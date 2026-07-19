use std::sync::Arc;

use eframe::egui::{self, Vec2, emath::TSTransform};

use crate::game_logic::objects::{
    Character, Object2D, fantasmas::Fantasma, monedas::Moneda, pared::Pared, personaje::Player,
};

mod laberinto;
pub mod objects;
pub use laberinto::MAPA_LABERINTO;

pub struct Game {
    maze: Arc<Vec<Vec<laberinto::Casilla>>>,
    paredes: Vec<Pared>,
    jugador: Player,
    fantasmas: Vec<Fantasma>,
    pub actual_size: Option<Vec2>,
    actual_transform: TSTransform,
    monedas: Vec<Moneda>,
    game_over: bool,
}

impl Game {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Game::default()
    }

    pub fn logic(&mut self, delta: f32, i: &egui::InputState) {
        if !self.game_over {
            self.jugador.logic(i);
            self.jugador.r#move(delta);
            for fantasma in &mut self.fantasmas {
                fantasma.logic(i);
                fantasma.r#move(delta);
                if fantasma.velocity() == Vec2::ZERO && !fantasma.is_planning() {
                    fantasma
                        .planning(
                            Arc::clone(&self.maze),
                            self.jugador.grid_pos(),
                            self.jugador.velocity(),
                        )
                        .expect(
                            format!("el fantasma {:?} ha provocado una falla", fantasma).as_str(),
                        );
                }
                fantasma.try_get_plan();
            }
            self.resolve_colisions();
            if !self.jugador.vivo() {
                self.game_over = true
            }
        } else {
            if i.key_pressed(egui::Key::R) {
                self.reset();
            }
        }
    }
    pub fn set_transform(&mut self, transform: TSTransform) {
        self.actual_transform = transform;
    }
    pub fn transform(&self) -> TSTransform {
        self.actual_transform
    }
    pub fn draw(&self, p: &egui::Painter) {
        for pared in &self.paredes {
            pared.draw(p);
        }
        for moneda in &self.monedas {
            moneda.draw(p);
        }
        for fantasma in &self.fantasmas {
            fantasma.draw(p);
        }

        self.jugador.draw(p);
    }

    fn resolve_colisions(&mut self) {
        for pared in &mut self.paredes {
            self.jugador.colision(pared);
        }

        for moneda in &mut self.monedas {
            self.jugador.colision(moneda);
        }
        for fantasma in &mut self.fantasmas {
            self.jugador.colision(fantasma);
        }
    }
    pub fn puntaje(&self) -> u32 {
        self.jugador.puntaje()
    }
}

impl Default for Game {
    fn default() -> Self {
        let mut juego = Self {
            maze: Arc::new(laberinto::maze_maker()),
            paredes: Vec::new(),
            actual_size: None,
            actual_transform: TSTransform::default(),
            jugador: Player::new(),
            monedas: Vec::new(),
            fantasmas: Vec::new(),
            game_over: false,
        };
        juego.init();
        juego
    }
}

impl Game {
    fn init(&mut self) {
        for (i, fila) in self.maze.iter().enumerate() {
            for (j, casilla) in fila.iter().enumerate() {
                match casilla {
                    laberinto::Casilla::Pared => self.paredes.push(Pared::new(i as f32, j as f32)),
                    laberinto::Casilla::Nodo(_) => self.monedas.push(Moneda::new(i, j)),
                    _ => (),
                }
            }
        }
        self.fantasmas
            .push(Fantasma::new(objects::fantasmas::TipoFantasma::Azul));
        self.fantasmas
            .push(Fantasma::new(objects::fantasmas::TipoFantasma::Naranja));
        self.fantasmas
            .push(Fantasma::new(objects::fantasmas::TipoFantasma::Rojo));
        self.fantasmas
            .push(Fantasma::new(objects::fantasmas::TipoFantasma::Rosa));
    }
    fn reset(&mut self) {
        self.jugador = Player::new();
        self.fantasmas.clear();
        self.monedas.clear();
        self.game_over = false;
        self.paredes.clear();

        self.init();
    }
}
