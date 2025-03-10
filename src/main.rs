mod creation_screen;

use eframe::egui;
use egui::Id;
use crate::creation_screen::creation_screen_mod::CreationScreen;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Shader Maker",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::<WindowState>::default())
        }),
    )
}

enum WindowState {
    CreationWindow(CreationScreen)
}

impl Default for WindowState {
    fn default() -> Self {
        Self::CreationWindow(CreationScreen::default())
    }
}

impl eframe::App for CreationScreen {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_controls(ui);
            self.ui_canvas(ui);
        });
    }
}

impl eframe::App for WindowState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top(Id::new("state")).show(ctx, |ui| {
            ui.label("Let there be dragons")
            /*if ui.button("Switch between Gallery and Drawing").clicked() {
                *self = match self {
                    WindowState::GalleryWindow(_) => WindowState::CreationWindow(CreationScreen::default()),
                }
            }*/
        });
        match self {
            WindowState::CreationWindow(w) => w.update(ctx, frame),
        }
    }
}