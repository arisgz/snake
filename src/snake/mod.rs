mod bodypart;
mod main_logic;
mod gui;

use eframe::egui::{Color32};
use std::collections::VecDeque;
use std::time::{Instant};
use crate::snake::bodypart::{BodyPart, Direction};

const GRID_SIZE: usize = 15;
const FRAME_MS: u64 = 130;
const RADIUS: f32 = 10.0;
const APPLE_COLOR: Color32 = Color32::RED;
const BODY_COLOR: Color32 = Color32::YELLOW;
const BACKGROUND_COLOR: Color32 = Color32::from_gray(27);
const STROKE_WEIGHT: f32 = 1.0;


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
