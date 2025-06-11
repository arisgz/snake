use eframe::egui;
use eframe::egui::Context;
use rand::Rng;
use crate::snake::bodypart::{BodyPart, Corner, Direction};
use crate::snake::{Snake, GRID_SIZE};

impl Snake {
    pub(super) fn handle_input(&mut self, ctx: &Context) {
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
    }


    pub(super) fn generate_fruit(&mut self) {
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

    pub(super) fn step(&mut self) {
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

        self.body.push_front(new_head);

        if self.body.front().unwrap().position() == self.apple {
            self.score += 1;
            self.generate_fruit();
            self.growing = true;
            self.body.pop_back();
        } else if !self.growing {
            self.body.pop_back();
        } else {
            self.growing = false;
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