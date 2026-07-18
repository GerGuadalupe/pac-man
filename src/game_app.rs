use crate::game_logic::Game;
use eframe::egui::{self, Vec2};
use egui::emath::TSTransform;

pub const ALTURA_L: f32 = 1080.0;
pub const ANCHO_L: f32 = 1920.0;

pub fn launch() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(false)
            .with_transparent(true),

        ..Default::default()
    };

    eframe::run_native(
        "Pac-Man",
        native_options,
        Box::new(|cc| Ok(Box::new(Game::new(cc)))),
    )
}

impl eframe::App for Game {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let window_rect = ui.available_rect_before_wrap();
        let size = window_rect.size();
        if self.actual_size != Some(size) {
            self.actual_size = Some(size);

            let Vec2 { x, y } = size;
            let scale = (x / ANCHO_L).min(y / ALTURA_L);

            let tx = (x - (ANCHO_L * scale)) * 0.5;
            let ty = (y - (ALTURA_L * scale)) * 0.5;

            self.set_transform(TSTransform::new((tx, ty).into(), scale));
        }
        let capa_juego = egui::LayerId::new(egui::Order::Background, "Juego".into());

        ui.set_transform_layer(capa_juego, self.transform());
        let mut painter = ui.layer_painter(capa_juego);
        painter.set_clip_rect(self.transform().inverse() * window_rect);
        self.draw(&painter);
        ui.horizontal_top(|ui| {
            ui.add(egui::Label::new(
                egui::RichText::new(format!("Puntaje: {}", self.puntaje()))
                    .color(egui::Color32::WHITE)
                    .size(24.0)
                    .strong(),
            ))
        });

        ui.request_repaint();
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::BLACK.to_array()
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let i = ctx.input(|i| i.clone());
        self.logic(i.unstable_dt, &i);
    }
}
