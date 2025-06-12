use std::time::Instant;
use eframe::egui;
use eframe::egui::{Painter, Rect, Stroke};
use crate::snake::{BACKGROUND_COLOR, BODY_COLOR, FRAME_MS, RADIUS, Snake, STROKE_WEIGHT};
use crate::snake::bodypart::{BodyPart, Corner, Direction};

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

    pub(super) fn paint_body(&self, painter: &Painter, cell_size: f32, rect: &Rect) {
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