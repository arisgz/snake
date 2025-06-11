use std::time::{Duration, Instant};
use eframe::{egui, App, Frame};
use eframe::egui::{Color32, Context, Id, Painter, Rect, Stroke, StrokeKind};
use crate::snake::{Snake, APPLE_COLOR, BACKGROUND_COLOR, BODY_COLOR, FRAME_MS, GRID_SIZE, RADIUS, STROKE_WEIGHT};
use crate::snake::bodypart::{BodyPart, Corner, Direction};

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

impl Snake {
    fn paint_lr_border(painter: &Painter, rect: Rect) {
        let y0 = rect.top();
        let y1 = rect.bottom();
        let x0 = rect.left() - 0.5;
        let x1 = rect.right() + 0.5;

        painter.line_segment(
            [egui::pos2(x0, y0), egui::pos2(x0, y1)],
            Stroke::new(STROKE_WEIGHT, BACKGROUND_COLOR),
        );
        painter.line_segment(
            [egui::pos2(x1, y0), egui::pos2(x1, y1)],
            Stroke::new(STROKE_WEIGHT, BACKGROUND_COLOR),
        );
    }

    fn paint_tb_border(painter: &Painter, rect: Rect) {
        let y0 = rect.top() - 0.5;
        let y1 = rect.bottom() + 0.5;
        let x0 = rect.left();
        let x1 = rect.right();

        painter.line_segment(
            [egui::pos2(x0, y0), egui::pos2(x1, y0)],
            Stroke::new(STROKE_WEIGHT, BACKGROUND_COLOR),
        );
        painter.line_segment(
            [egui::pos2(x0, y1), egui::pos2(x1, y1)],
            Stroke::new(STROKE_WEIGHT, BACKGROUND_COLOR),
        );
    }

    fn paint_bend_border(painter: &Painter, rect: Rect, corner: &Corner) {
        let y0 = rect.top();
        let y1 = rect.bottom();
        let x0 = rect.left();
        let x1 = rect.right();
        let radius_offset = RADIUS / 2.0;
        let half_stroke = STROKE_WEIGHT / 2.0;

        let (pos1, pos2) = match corner {
            Corner::TopLeft => ([egui::pos2(x0 + radius_offset, y0 - half_stroke), egui::pos2(x1, y0 - half_stroke)],
                                [egui::pos2(x0 - half_stroke, y0 + radius_offset), egui::pos2(x0 - half_stroke, y1)]),
            Corner::BottomLeft => ([egui::pos2(x0 + radius_offset, y1 + half_stroke), egui::pos2(x1, y1 + half_stroke)],
                                   [egui::pos2(x0 - half_stroke, y0), egui::pos2(x0 - half_stroke, y1 - radius_offset)]),
            Corner::TopRight => ([egui::pos2(x0, y0 - half_stroke), egui::pos2(x1 - radius_offset, y0 - half_stroke)],
                                 [egui::pos2(x1 + half_stroke, y0 + radius_offset), egui::pos2(x1 + half_stroke, y1)]),
            Corner::BottomRight => ([egui::pos2(x0, y1 + half_stroke), egui::pos2(x1 - radius_offset, y1 + half_stroke)],
                                    [egui::pos2(x1 + half_stroke, y0), egui::pos2(x1 + half_stroke, y1 - radius_offset)]),
            _ => unreachable!(),
        };

        let stroke = Stroke::new(STROKE_WEIGHT, BACKGROUND_COLOR);
        painter.line_segment(pos1, stroke);
        painter.line_segment(pos2, stroke);
    }

    fn paint_front(painter: &Painter, bodypart: &BodyPart, cell: &mut Rect, offset: f32) {
        match bodypart.direction {
            Direction::Up => cell.min.y += offset,
            Direction::Down => cell.max.y -= offset,
            Direction::Left => cell.min.x += offset,
            Direction::Right => cell.max.x -= offset,
        };

        painter.rect_filled(*cell, 0.0, BODY_COLOR);
    }

    fn paint_back(&self, painter: &Painter, bodypart: &BodyPart, cell: &mut Rect, offset: f32) {
        if !self.growing {
            match bodypart.direction {
                Direction::Up => cell.max.y += offset - cell.width(),
                Direction::Down => cell.min.y -= offset - cell.width(),
                Direction::Left => cell.max.x += offset - cell.width(),
                Direction::Right => cell.min.x -= offset - cell.width(),
            };
        }

        painter.rect_filled(*cell, 0.0, BODY_COLOR);
    }

    fn paint_body(&self, painter: &Painter, cell_size: f32, rect: &Rect) {
        for bodypart in &self.body {
            let x = rect.left() + bodypart.x as f32 * cell_size;
            let y = rect.top() + bodypart.y as f32 * cell_size;

            let mut cell = Rect::from_min_size(
                egui::pos2(x, y),
                egui::vec2(cell_size, cell_size),
            );

            let front = self.body.front().unwrap();
            let back = self.body.back().unwrap();
            let offset = cell.width() - ((Instant::now() - self.last_update).as_millis() as f32 * cell.width() / FRAME_MS as f32);
            if *bodypart == *front {
                Snake::paint_front(painter, bodypart, &mut cell, offset);
            } else if *bodypart == *back {
                self.paint_back(painter, bodypart, &mut cell, offset);
            } else {
                painter.rect_filled(cell, bodypart.corner.get_corner_radius(), BODY_COLOR);
            }

            if !bodypart.is_bend() {
                if bodypart.direction.is_vertical() {
                    Snake::paint_lr_border(painter, cell);
                } else {
                    Snake::paint_tb_border(painter, cell);
                }
            } else {
                Snake::paint_bend_border(painter, cell, &bodypart.corner);
            }
        }
    }
}