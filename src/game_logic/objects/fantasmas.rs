use crate::game_logic::{
    laberinto::Direcciones,
    objects::{Character, Colider, Object2D},
};
use eframe::egui::{self, Pos2, Vec2, vec2};
use std::collections::VecDeque;

pub use utils::TipoFantasma;
mod planning;
mod utils;

static TEMP_V: f32 = {
    let distancia: f32 = 1080.0 / 19.0;
    distancia / 160.0
};
static TEMP_H: f32 = {
    let distancia: f32 = 1920.0 / 31.0;
    distancia / 160.0
};

#[derive(Debug)]
pub struct Fantasma {
    tipo: utils::TipoFantasma,
    temp: f32,
    size: f32,
    posision: Pos2,
    velocity: Vec2,
    speed: f32,
    colider: Colider,
    ruta: VecDeque<Direcciones>,
    chanel: utils::Chanel<Option<VecDeque<Direcciones>>>,
    estado: utils::State,
}

impl Object2D for Fantasma {
    fn colider(&self) -> &Colider {
        &self.colider
    }
    fn posision(&self) -> Pos2 {
        self.posision
    }
    fn cast(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn set_posision(&mut self, pos: Pos2) {
        self.posision = pos;
    }
    fn draw(&self, p: &eframe::egui::Painter) {
        use egui::Color32;
        let color = match self.tipo {
            TipoFantasma::Azul => Color32::from_rgb(0, 100, 250),
            TipoFantasma::Naranja => Color32::ORANGE,
            TipoFantasma::Rojo => Color32::from_rgb(250, 30, 30),
            TipoFantasma::Rosa => Color32::from_rgb(255, 200, 200),
        };
        p.circle_filled(self.posision, self.size, color);
    }
    fn init(&mut self) {
        use super::pared::PARED_SIZE;
        self.size = (PARED_SIZE).length() / 3.4;
        self.colider.radio = self.size;

        self.set_posision(
            self.posision()
                + (PARED_SIZE / 2.0
                    + PARED_SIZE
                        * Vec2::from({
                            match self.tipo {
                                TipoFantasma::Azul => (3.0, 1.0),
                                TipoFantasma::Naranja => (27.0, 1.0),
                                TipoFantasma::Rojo => (3.0, 17.0),
                                TipoFantasma::Rosa => (27.0, 17.0),
                            }
                        })),
        );
    }
}

impl Character for Fantasma {
    fn add_next_direction(&mut self, direction: Direcciones) {
        self.ruta.push_back(direction);
    }
    fn consume_direction(&mut self) -> Option<Direcciones> {
        self.ruta.pop_front()
    }
    fn next_direction(&self) -> Option<&Direcciones> {
        self.ruta.iter().next()
    }
    fn colision(&mut self, _other: &mut dyn Object2D) {}
    fn set_velocity(&mut self, vel: Vec2) {
        self.velocity = vel * self.speed;
    }
    fn velocity(&self) -> Vec2 {
        self.velocity
    }
    fn logic(&mut self, i: &egui::InputState) {
        if self.temp > 0.0 {
            self.temp -= i.stable_dt;
            return;
        }
        match self.estado {
            utils::State::Execute => {
                if let Some(direccion) = self.next_direction() {
                    match direccion {
                        Direcciones::Este | Direcciones::Oeste => {
                            let dist_anclaje =
                                (self.posision().y / (super::pared::PARED_SIZE.y)).fract();
                            if dist_anclaje <= 0.58 && dist_anclaje >= 0.46 {
                                if *direccion == Direcciones::Este {
                                    self.set_velocity(Vec2 { x: 1.0, y: 0.0 });
                                } else {
                                    self.set_velocity(Vec2 { x: -1.0, y: 0.0 });
                                }
                                self.consume_direction();
                                self.temp = TEMP_H;
                            }
                        }
                        Direcciones::Norte | Direcciones::Sur => {
                            let dist_anclaje =
                                (self.posision().x / (super::pared::PARED_SIZE.x)).fract();
                            if dist_anclaje <= 0.58 && dist_anclaje >= 0.46 {
                                if *direccion == Direcciones::Norte {
                                    self.set_velocity(vec2(0.0, -1.0));
                                } else {
                                    self.set_velocity(vec2(0.0, 1.0));
                                }
                                self.consume_direction();
                                self.temp = TEMP_V;
                            }
                        }
                    }
                } else {
                    self.estado = utils::State::Standby
                }
            }
            utils::State::Standby => {
                let dist_anclaje = (
                    (self.posision().x / (super::pared::PARED_SIZE.x)).fract(),
                    (self.posision().y / (super::pared::PARED_SIZE.y)).fract(),
                );
                if dist_anclaje.0 <= 0.58
                    && dist_anclaje.0 >= 0.46
                    && dist_anclaje.1 <= 0.58
                    && dist_anclaje.1 >= 0.46
                {
                    self.velocity = Vec2::ZERO;
                }
            }
            utils::State::Planning => (),
        }
    }

    fn grid_pos(&self) -> (usize, usize) {
        (
            (self.posision().y / (super::pared::PARED_SIZE.y)).trunc() as usize,
            (self.posision().x / (super::pared::PARED_SIZE.x)).trunc() as usize,
        )
    }
}

impl Fantasma {
    pub fn new(tipo: TipoFantasma) -> Self {
        print!("temp h: {} | ", TEMP_H);
        println!("temp v: {}", TEMP_V);
        let mut fantasma = Self {
            temp: 0.0,
            speed: 160.0,
            chanel: utils::Chanel::new(),
            colider: Colider {
                radio: 0.0,
                tipo: super::TipoColider::Enemigo,
            },
            posision: Pos2::ZERO,
            ruta: VecDeque::new(),
            size: 0.0,
            tipo,
            velocity: Vec2::ZERO,
            estado: utils::State::Standby,
        };
        fantasma.init();
        fantasma
    }

    pub fn is_planning(&self) -> bool {
        self.estado == utils::State::Planning
    }
}
