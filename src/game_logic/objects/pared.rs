const SHOW_COLIDER: bool = true;
use eframe::egui::{self, Pos2, Vec2};

use crate::game_logic::objects::{Colider, Object2D};
use crate::{ALTURA_L, ANCHO_L, MAPA_LABERINTO};

pub static PARED_SIZE: Vec2 = {
    let num_rows: f32 = MAPA_LABERINTO.len() as f32;
    let num_cols = MAPA_LABERINTO[0].len() as f32;
    Vec2::new(ANCHO_L / num_cols, ALTURA_L / num_rows)
};

#[derive(Debug)]
pub struct Pared {
    posicion: Pos2,
    grid_posision: Vec2,
    colider: Colider,
    size: Vec2,
}

impl Object2D for Pared {
    fn posision(&self) -> Pos2 {
        self.posicion
    }
    fn set_posision(&mut self, pos: Pos2) {
        self.posicion = pos;
    }
    fn draw(&self, p: &eframe::egui::Painter) {
        let dist = self.size;

        //println!("{:#?}", self);

        p.rect_filled(
            egui::Rect::from_center_size(self.posicion, dist),
            egui::CornerRadius::default(),
            egui::Color32::from_rgb(0, 20, 150),
        );
        if SHOW_COLIDER {
            p.circle_filled(
                self.posision(),
                self.colider.radio,
                egui::Color32::from_rgba_unmultiplied(0, 100, 100, 100),
            );
        }
    }

    fn colider(&self) -> &Colider {
        &self.colider
    }

    fn init(&mut self) {
        self.size = PARED_SIZE;

        self.posicion = (self.size / 2.0 + (self.size * self.grid_posision)).to_pos2();
        self.colider.radio = (self.size / 3.1).length();
    }
    fn cast(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Pared {
    pub fn new(grid_i: f32, grid_j: f32) -> Self {
        let mut pared = Self {
            colider: Colider::new(0.0, super::TipoColider::Obstaculo),
            grid_posision: Vec2::from((grid_j, grid_i)),
            posicion: Pos2::ZERO,
            size: Vec2::ZERO,
        };

        pared.init();

        pared
    }
}
