use crate::game_logic::objects::Object2D;
use eframe::egui::{self, Pos2, Vec2};

#[derive(Debug)]
pub struct Moneda {
    posicion: Pos2,
    grid_posision: Vec2,
    colider: super::Colider,
    size: f32,
    colected: bool,
}

impl Object2D for Moneda {
    fn posision(&self) -> Pos2 {
        self.posicion
    }
    fn set_posision(&mut self, pos: Pos2) {
        self.posicion = pos;
    }
    fn colider(&self) -> &super::Colider {
        &self.colider
    }
    fn draw(&self, p: &eframe::egui::Painter) {
        if !self.colected {
            p.circle_filled(self.posicion, self.size, egui::Color32::WHITE);
        }
    }
    fn init(&mut self) {
        use super::pared::PARED_SIZE;
        self.colider.radio = self.size;

        self.posicion = (PARED_SIZE / 2.0 + (PARED_SIZE * self.grid_posision)).to_pos2();
    }
    fn cast(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Moneda {
    pub fn new(grid_i: usize, grid_j: usize) -> Self {
        let mut moneda = Self {
            colider: super::Colider::new(0.0, super::TipoColider::Moneda),
            posicion: Pos2::ZERO,
            grid_posision: Vec2 {
                x: grid_j as f32,
                y: grid_i as f32,
            },
            size: 5.0,
            colected: false,
        };
        moneda.init();
        moneda
    }

    pub fn tomar_moneda(&mut self) {
        self.colected = true;
    }
    pub fn tomada(&self) -> bool {
        self.colected
    }
}
