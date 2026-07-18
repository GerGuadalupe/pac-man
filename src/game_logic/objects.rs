use super::laberinto::Direcciones;
use eframe::egui::{self, Pos2, Vec2};
use std::any::Any;
use std::fmt::Debug;

pub mod fantasmas;
pub mod monedas;
pub mod pared;
pub mod personaje;

pub trait Object2D: Debug + Any {
    fn posision(&self) -> Pos2;

    fn draw(&self, p: &eframe::egui::Painter);

    fn colider(&self) -> &Colider;

    fn init(&mut self);

    fn set_posision(&mut self, pos: Pos2);

    fn cast(&mut self) -> &mut dyn Any;
}

pub trait Character: Object2D {
    /// # Recordatorio
    /// NO mover directamente los personajes desde esta función,
    ///
    /// solo se puede definir la velocidad
    fn logic(&mut self, i: &egui::InputState);

    fn set_velocity(&mut self, vel: Vec2);

    fn velocity(&self) -> Vec2;

    fn colision(&mut self, other: &mut dyn Object2D);

    fn add_next_direction(&mut self, direction: Direcciones);

    fn next_direction(&self) -> Option<&Direcciones>;

    fn consume_direction(&mut self) -> Option<Direcciones>;

    fn grid_pos(&self) -> (usize, usize);

    fn r#move(&mut self, delta: f32) {
        let vel = self.velocity() * delta;

        let pos = self.posision() + vel;

        self.set_posision(pos);
    }
}

#[derive(Debug)]
pub struct Colider {
    radio: f32,
    tipo: TipoColider,
}

impl Colider {
    pub fn new(radio: f32, tipo: TipoColider) -> Self {
        Self { radio, tipo }
    }
    pub fn radio(&self) -> f32 {
        self.radio
    }
}

#[derive(Debug)]
pub enum TipoColider {
    Obstaculo,
    Enemigo,
    Jugador,
    Moneda,
}
