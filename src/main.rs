#![cfg_attr(
    all(
        target_os = "windows",
        not(debug_assertions),
    ),
    windows_subsystem = "windows"
)]
mod snake;

use eframe::egui;
use crate::snake::Snake;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 600.0])
            .with_title("Snake"),
        ..Default::default()
    };
    eframe::run_native(
        "Snake",
        options,
        Box::new(|_cc| Ok(Box::new(Snake::default()))),
    )
}
