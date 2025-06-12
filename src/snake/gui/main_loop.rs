use std::time::{Duration, Instant};
use eframe::{App, egui, Frame};
use eframe::egui::{Color32, Context, Id, Rect, Stroke, StrokeKind};
use crate::snake::{APPLE_COLOR, BACKGROUND_COLOR, FRAME_MS, GRID_SIZE, Snake, STROKE_WEIGHT};
use crate::snake::bodypart::Direction;

impl Snake {
    fn handle_input(&mut self, ctx: &Context) {
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
}

impl App for Snake {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.handle_input(ctx);

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
                    .rect_filled(screen_rect, 0.0, BACKGROUND_COLOR);

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
                    egui::pos2(rect.left(), rect.top()),
                    egui::vec2(min, min),
                );
                painter.rect_stroke(wrapping_rect, 0.0, Stroke::new(STROKE_WEIGHT, Color32::from_gray(0)), StrokeKind::Outside);
            }

            let cell_size = rect.width().min(rect.height()) / GRID_SIZE as f32;

            self.paint_body(&painter, cell_size, &rect);


            let x = rect.left() + self.apple.0 as f32 * cell_size;
            let y = rect.top() + self.apple.1 as f32 * cell_size;

            let cell = Rect::from_min_size(
                egui::pos2(x, y),
                egui::vec2(cell_size, cell_size),
            );
            painter.circle_filled(cell.center(), cell_size / 2.0, APPLE_COLOR);
        });

        ctx.request_repaint();
    }
}