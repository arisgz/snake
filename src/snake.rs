use eframe::egui::{Color32, Context, Id, Painter, Rect, Stroke};
use eframe::epaint::{CornerRadiusF32};
use eframe::{App, Frame, egui};
use rand::Rng;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use crate::snake::Corner::{BottomLeft, BottomRight, TopLeft, TopRight};

const GRID_SIZE: usize = 15;
const FRAME_MS: u64 = 150;
const RADIUS: f32 = 10.0;
const APPLE_COLOR: Color32 = Color32::RED;
const BODY_COLOR: Color32 = Color32::YELLOW;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, PartialEq)]
enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub struct Snake {
    body: VecDeque<(u32, u32)>,
    apple: (u32, u32),
    direction: Direction,
    last_update: Instant,
    directions_queue: VecDeque<Direction>,
    game_over: bool,
    score: u32,
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
        };
        snake
            .body
            .push_front((GRID_SIZE as u32 / 2, GRID_SIZE as u32 / 2));
        snake
            .body
            .push_front((GRID_SIZE as u32 / 2, GRID_SIZE as u32 / 2 - 1));
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

            let cell_size = rect.width().min(rect.height()) / GRID_SIZE as f32;

            for i in 0..GRID_SIZE as u32 {
                for j in 0..GRID_SIZE as u32 {
                    let x = rect.left() + j as f32 * cell_size;
                    let y = rect.top() + i as f32 * cell_size;

                    let cell = Rect::from_min_size(
                        egui::pos2(x, y),
                        egui::vec2(cell_size, cell_size),
                    );

                    // Print backgroud grid
                    // painter.rect_stroke(
                    //     cell,
                    //     0.0,
                    //     Stroke::new(0.5, Color32::LIGHT_GRAY),
                    //     StrokeKind::Inside,
                    // );

                    if self.apple == (j, i) {
                        painter.circle_filled(cell.center(), cell_size / 2.0, APPLE_COLOR);
                    }

                    if self.body.contains(&(j, i)) {
                        self.paint_body(&painter, j, i, cell);
                    }
                }
            }
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

    fn paint_body(&self, painter: &Painter, x: u32, y: u32, mut rect: Rect) {
        let front = self.body.front().unwrap();
        let back = self.body.back().unwrap();
        let offset = rect.width() - ((Instant::now() - self.last_update).as_millis() as f32 * rect.width() / 150.0);
        if (x, y) == *front {
            match self.direction {
                Direction::Up => rect.min.y += offset,
                Direction::Down => rect.max.y -= offset,
                Direction::Left => rect.min.x += offset,
                Direction::Right => rect.max.x -= offset,
            };

            painter.rect_filled(rect, 0.0, BODY_COLOR);

            let succ = self.body[1];
            if front.0 == succ.0 {
                Snake::paint_lr_border(painter, rect);
            } else {
                Snake::paint_tb_border(painter, rect);
            }
        } else if (x, y) == *back {
            let prev = self.body[self.body.len() - 2];
            let dir = if prev.0 < x {
                Direction::Left
            } else if prev.0 > x {
                Direction::Right
            } else if prev.1 < y {
                Direction::Up
            } else {
                Direction::Down
            };
            match dir {
                Direction::Up => rect.max.y += offset-rect.width(),
                Direction::Down => rect.min.y -= offset-rect.width(),
                Direction::Left => rect.max.x += offset-rect.width(),
                Direction::Right => rect.min.x -= offset-rect.width(),
            };
            painter.rect_filled(rect, 0.0, BODY_COLOR);
            if back.0 == prev.0 {
                Snake::paint_lr_border(painter, rect);
            } else {
                Snake::paint_tb_border(painter, rect);
            }
        } else {
            // bend
            if let Some(index) = self.body.iter().position(|&r| r == (x, y)) {
                let prev = self.body[index - 1];
                let succ = self.body[index + 1];
                let current = self.body[index];

                if prev.0 == succ.0 && prev.1 != succ.1 {
                    Snake::paint_lr_border(painter, rect);
                    painter.rect_filled(rect, 0.0, BODY_COLOR);
                } else if prev.1 == succ.1 && prev.0 != succ.0 {
                    Snake::paint_tb_border(painter, rect);
                    painter.rect_filled(rect, 0.0, BODY_COLOR);
                } else {
                    let corner = Snake::get_corner(&prev, &current, &succ);
                    Snake::draw_one_rounded_corner_rect(painter, rect, corner);
                };
            }
        }
    }

    fn get_corner(prev: &(u32, u32), current: &(u32, u32), succ: &(u32, u32)) -> Corner {
        let dx = prev.0 as i32 - succ.0 as i32;
        let dy = prev.1 as i32 - succ.1 as i32;
        let last_dir_vertical = current.0 == prev.0;

        match (dx.signum(), dy.signum(), last_dir_vertical) {
            (-1, -1, true) => BottomLeft, // left -> top, last top
            (-1, -1, false) => TopRight, // left -> top, last left
            (-1, 1, true) => TopLeft, // left -> bottom, last bottom
            (-1, 1, false) => BottomRight, // left -> bottom, last left
            (1, 1, true) => TopRight, // right -> bottom, last bottom
            (1, 1, false) => BottomLeft, // right -> bottom, last right
            (1, -1, true) => BottomRight, // right -> top, last top
            (1, -1, false) => TopLeft, // right -> top, last right
            _ => unreachable!(),
        }
    }

    fn draw_one_rounded_corner_rect(painter: &Painter, rect: Rect, corner: Corner) {
        let rounding = match corner {
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
        };

        painter.rect_filled(rect, rounding, BODY_COLOR);
    }

    fn generate_fruit(&mut self) {
        let mut rng = rand::rng();

        loop {
            let x = rng.random_range(0..GRID_SIZE as u32);
            let y = rng.random_range(0..GRID_SIZE as u32);
            if !self.body.contains(&(x, y)) {
                self.apple = (x, y);
                break;
            }
        }
    }

    fn step(&mut self) {
        let (x, y) = self.body.front().copied().unwrap();

        // with wraparound
        // let new_head = match self.direction {
        //     Direction::Left => ((x + GRID_SIZE as u32 - 1) % GRID_SIZE as u32, y),
        //     Direction::Right => ((x + 1) % GRID_SIZE as u32, y),
        //     Direction::Up => (x, (y + GRID_SIZE as u32 - 1) % GRID_SIZE as u32),
        //     Direction::Down => (x, (y + 1) % GRID_SIZE as u32),
        // };

        if let Some(direction) = self.directions_queue.pop_front() {
            self.direction = direction;
        }

        let new_head = match self.direction {
            Direction::Left => {
                if x == 0 {
                    self.game_over = true;
                    return;
                }
                (x - 1, y)
            }
            Direction::Right => {
                if x + 1 == GRID_SIZE as u32 {
                    self.game_over = true;
                    return;
                }
                (x + 1, y)
            }
            Direction::Up => {
                if y == 0 {
                    self.game_over = true;
                    return;
                }
                (x, y - 1)
            }
            Direction::Down => {
                if y + 1 == GRID_SIZE as u32 {
                    self.game_over = true;
                    return;
                }
                (x, y + 1)
            }
        };

        if self.body.contains(&new_head) && self.body.back().unwrap() != &new_head {
            self.game_over = true;
            return;
        }

        self.body.push_front(new_head);

        if new_head == self.apple {
            self.score += 1;
            self.generate_fruit();
        } else {
            self.body.pop_back();
        }
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
