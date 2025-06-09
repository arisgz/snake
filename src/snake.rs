use eframe::egui::{pos2, vec2, Color32, Context, Id, Painter, Rect, Stroke};
use eframe::epaint::{CornerRadiusF32, StrokeKind};
use eframe::{App, Frame, egui};
use rand::Rng;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const GRID_SIZE: usize = 15;
const FRAME_MS: u64 = 130;
const RADIUS: f32 = 10.0;
const APPLE_COLOR: Color32 = Color32::RED;
const BODY_COLOR: Color32 = Color32::YELLOW;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn is_vertical(self) -> bool {
        self == Direction::Down || self == Direction::Up
    }

    fn is_horizontal(self) -> bool {
        self == Direction::Left || self == Direction::Right
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    None,
}

impl Corner {
    fn get_corner_radius(&self) -> CornerRadiusF32 {
        match self {
            Corner::TopLeft => CornerRadiusF32 {
                nw: RADIUS,
                ne: 0.0,
                sw: 0.0,
                se: 0.0,
            },
            Corner::TopRight => CornerRadiusF32 {
                nw: 0.0,
                ne: RADIUS,
                sw: 0.0,
                se: 0.0,
            },
            Corner::BottomLeft => CornerRadiusF32 {
                nw: 0.0,
                ne: 0.0,
                sw: RADIUS,
                se: 0.0,
            },
            Corner::BottomRight => CornerRadiusF32 {
                nw: 0.0,
                ne: 0.0,
                sw: 0.0,
                se: RADIUS,
            },
            Corner::None => CornerRadiusF32::default(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct BodyPart {
    x: u32,
    y: u32,
    direction: Direction,
    corner: Corner,
}

impl BodyPart {
    fn new(x: u32, y: u32, direction: Direction) -> Self {
        Self {
            x,
            y,
            direction,
            corner:Corner::None,
        }
    }

    fn position(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn position_eq(&self, other: &BodyPart) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Snake {
    body: VecDeque<BodyPart>,
    apple: (u32, u32),
    direction: Direction,
    last_update: Instant,
    directions_queue: VecDeque<Direction>,
    game_over: bool,
    score: u32,
    growing: bool,
}

impl Default for Snake {
    fn default() -> Self {
        let mut snake = Self {
            body: VecDeque::new(),
            apple: (0, 0),
            direction: Direction::Up,
            directions_queue: VecDeque::new(),
            last_update: Instant::now(),
            game_over: false,
            score: 0,
            growing: false,
        };
        let middle = GRID_SIZE as u32 / 2;
        snake
            .body
            .push_front(BodyPart::new(middle, middle + 1, Direction::Up));
        snake
            .body
            .push_front(BodyPart::new(middle, middle, Direction::Up));
        snake
            .body
            .push_front(BodyPart::new(middle, middle - 1, Direction::Up));
        snake.generate_fruit();
        snake
    }
}

impl App for Snake {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.input(|input| {
            if input.key_pressed(egui::Key::ArrowLeft) {
                self.queue_direction(Direction::Left);
            } else if input.key_pressed(egui::Key::ArrowRight) {
                self.queue_direction(Direction::Right);
            } else if input.key_pressed(egui::Key::ArrowUp) {
                self.queue_direction(Direction::Up);
            } else if input.key_pressed(egui::Key::ArrowDown) {
                self.queue_direction(Direction::Down);
            } else if input.key_pressed(egui::Key::R) {
                *self = Snake::default();
            }
        });

        let now = Instant::now();
        if !self.game_over && now - self.last_update >= Duration::from_millis(FRAME_MS) {
            self.step();
            self.last_update = now;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(
                egui::Vec2::new(ui.available_width(), ui.available_height()),
                egui::Sense::hover(),
            );

            if self.game_over {
                let screen_rect = ui.max_rect();
                ui.painter()
                    .rect_filled(screen_rect, 0.0, Color32::from_black_alpha(150));

                egui::Area::new(Id::from("game_over"))
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                    .show(ui.ctx(), |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Game Over");
                            ui.heading(format!("Score: {}", self.score));
                            ui.label("Press R to restart");
                        });
                    });
                return;
            }

            let rect = response.rect;

            {
                let min = rect.width().min(rect.height());
                let wrapping_rect = Rect::from_min_size(
                    pos2(rect.left(), rect.top()),
                    vec2(min, min),
                );
                painter.rect_stroke(wrapping_rect, 0.0, Stroke::new(1.0, Color32::from_gray(0)), StrokeKind::Outside);
            }

            let cell_size = rect.width().min(rect.height()) / GRID_SIZE as f32;

            for body_part in &self.body {
                let x = rect.left() + body_part.x as f32 * cell_size;
                let y = rect.top() + body_part.y as f32 * cell_size;

                let cell = Rect::from_min_size(
                    pos2(x, y),
                    vec2(cell_size, cell_size),
                );

                self.paint_body(&painter, body_part, cell);
            }

            let x = rect.left() + self.apple.0 as f32 * cell_size;
            let y = rect.top() + self.apple.1 as f32 * cell_size;

            let cell = Rect::from_min_size(
                pos2(x, y),
                vec2(cell_size, cell_size),
            );
            painter.circle_filled(cell.center(), cell_size / 2.0, APPLE_COLOR);
        });

        ctx.request_repaint();
    }
}

impl Snake {
    fn paint_lr_border(painter: &Painter, rect: Rect) {
        let y0 = rect.top();
        let y1 = rect.bottom();
        let x0 = rect.left() - 0.5;
        let x1 = rect.right() + 0.5;

        painter.line_segment(
            [egui::pos2(x0, y0), egui::pos2(x0, y1)],
            Stroke::new(1.0, Color32::BLACK),
        );
        painter.line_segment(
            [egui::pos2(x1, y0), egui::pos2(x1, y1)],
            Stroke::new(1.0, Color32::BLACK),
        );
    }

    fn paint_tb_border(painter: &Painter, rect: Rect) {
        let y0 = rect.top() - 0.5;
        let y1 = rect.bottom() + 0.5;
        let x0 = rect.left();
        let x1 = rect.right();

        painter.line_segment(
            [egui::pos2(x0, y0), egui::pos2(x1, y0)],
            Stroke::new(1.0, Color32::BLACK),
        );
        painter.line_segment(
            [egui::pos2(x0, y1), egui::pos2(x1, y1)],
            Stroke::new(1.0, Color32::BLACK),
        );
    }

    fn paint_body(& self, painter: &Painter, bodypart: &BodyPart, mut rect: Rect) {
        let front = self.body.front().unwrap();
        let back = self.body.back().unwrap();
        let offset = rect.width() - ((Instant::now() - self.last_update).as_millis() as f32 * rect.width() / FRAME_MS as f32);
        if *bodypart == *front {
            match bodypart.direction {
                Direction::Up => rect.min.y += offset,
                Direction::Down => rect.max.y -= offset,
                Direction::Left => rect.min.x += offset,
                Direction::Right => rect.max.x -= offset,
            };

            painter.rect_filled(rect, 0.0, BODY_COLOR);

            if bodypart.direction.is_vertical() {
                Snake::paint_lr_border(painter, rect);
            } else {
                Snake::paint_tb_border(painter, rect);
            }
        } else if *bodypart == *back {
            if !self.growing {
                match bodypart.direction {
                    Direction::Up => rect.max.y += offset - rect.width(),
                    Direction::Down => rect.min.y -= offset - rect.width(),
                    Direction::Left => rect.max.x += offset - rect.width(),
                    Direction::Right => rect.min.x -= offset - rect.width(),
                };
            }
            painter.rect_filled(rect, 0.0, BODY_COLOR);
            if bodypart.direction.is_vertical() {
                Snake::paint_lr_border(painter, rect);
            } else {
                Snake::paint_tb_border(painter, rect);
            }
        } else {
            painter.rect_filled(rect, bodypart.corner.get_corner_radius(), BODY_COLOR);
        }
    }

    fn generate_fruit(&mut self) {
        let mut rng = rand::rng();

        loop {
            let x = rng.random_range(0..GRID_SIZE as u32);
            let y = rng.random_range(0..GRID_SIZE as u32);
            if !self.body.iter().any(|b| b.position() == (x, y)) {
                self.apple = (x, y);
                break;
            }
        }
    }

    fn get_new_head(&mut self) -> Option<BodyPart> {
        let old_head = self.body.front_mut().unwrap();

        let (x, y) = match self.direction {
            Direction::Left => {
                if old_head.x == 0 {
                    self.game_over = true;
                    return None;
                }
                (old_head.x - 1, old_head.y)
            }
            Direction::Right => {
                if old_head.x + 1 == GRID_SIZE as u32 {
                    self.game_over = true;
                    return None;
                }
                (old_head.x + 1, old_head.y)
            }
            Direction::Up => {
                if old_head.y == 0 {
                    self.game_over = true;
                    return None;
                }
                (old_head.x, old_head.y - 1)
            }
            Direction::Down => {
                if old_head.y + 1 == GRID_SIZE as u32 {
                    self.game_over = true;
                    return None;
                }
                (old_head.x, old_head.y + 1)
            }
        };

        old_head.corner = match (old_head.direction, self.direction) {
            (Direction::Up, Direction::Left) => Corner::TopRight,
            (Direction::Up, Direction::Right) => Corner::TopLeft,
            (Direction::Down, Direction::Right) => Corner::BottomLeft,
            (Direction::Down, Direction::Left) => Corner::BottomRight,
            (Direction::Left, Direction::Down) => Corner::TopLeft,
            (Direction::Left, Direction::Up) => Corner::BottomLeft,
            (Direction::Right, Direction::Down) => Corner::TopRight,
            (Direction::Right, Direction::Up) => Corner::BottomRight,
            _ => Corner::None
        };

        if self.direction != old_head.direction {
            old_head.direction = self.direction;
        }

        Some(BodyPart::new(x, y, self.direction))
    }

    fn step(&mut self) {
        if let Some(direction) = self.directions_queue.pop_front() {
            self.direction = direction;
        }

        let new_head = if let Some(value) = self.get_new_head() {
            value
        } else {
            self.game_over = true;
            return;
        };


        if self.body.iter().any(|b| b.position_eq(&new_head)) && !self.body.back().unwrap().position_eq(&new_head) {
            self.game_over = true;
            return;
        }

        if new_head.position() == self.apple {
            self.score += 1;
            self.generate_fruit();
            self.growing = true;
            self.body.pop_back();
        } else if !self.growing {
            self.body.pop_back();
        } else {
            self.growing = false;
        }

        self.body.push_front(new_head);
    }

    fn queue_direction(&mut self, dir: Direction) {
        let last_dir = self.directions_queue.back().copied().unwrap_or(self.direction);

        let is_opposite = matches!(
        (last_dir, dir),
        (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
    );

        if !is_opposite && last_dir != dir && self.directions_queue.len() < 2 {
            self.directions_queue.push_back(dir);
        }
    }
}
