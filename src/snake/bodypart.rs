use eframe::epaint::CornerRadiusF32;
use crate::snake::RADIUS;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub(super) fn is_vertical(self) -> bool {
        self == Direction::Down || self == Direction::Up
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    None,
}

impl Corner {
    pub(super) fn get_corner_radius(&self) -> CornerRadiusF32 {
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
pub(super) struct BodyPart {
    pub(super) x: u32,
    pub(super) y: u32,
    pub(super) direction: Direction,
    pub(super) corner: Corner,
}

impl BodyPart {
    pub(super) fn new(x: u32, y: u32, direction: Direction) -> Self {
        Self {
            x,
            y,
            direction,
            corner: Corner::None,
        }
    }

    pub(super) fn position(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    pub(super) fn position_eq(&self, other: &BodyPart) -> bool {
        self.x == other.x && self.y == other.y
    }

    pub(super) fn is_bend(&self) -> bool {
        self.corner != Corner::None
    }
}