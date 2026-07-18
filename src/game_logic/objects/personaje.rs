use eframe::egui::{self, Pos2, Vec2, vec2};

use crate::game_logic::objects::{Character, Colider, Direcciones, Object2D, monedas::Moneda};

#[derive(Debug)]
pub struct Player {
    posision: Pos2,
    colider: Colider,
    size: f32,
    frames: Vec<fn(&egui::Painter)>,
    frame: usize,
    velocity: Vec2,
    speed: f32,
    rotation: f32,
    next_direction: Option<Direcciones>,
    puntaje: u32,
}

impl Object2D for Player {
    fn posision(&self) -> Pos2 {
        self.posision
    }
    fn colider(&self) -> &Colider {
        &self.colider
    }
    fn set_posision(&mut self, pos: Pos2) {
        self.posision = pos;
    }
    fn init(&mut self) {
        use super::pared::PARED_SIZE;
        self.size = (PARED_SIZE).length() / 3.4;
        self.colider.radio = self.size;

        self.set_posision(
            self.posision() + (PARED_SIZE / 2.0 + PARED_SIZE * Vec2::from((16.0, 16.0))),
        );
    }
    fn draw(&self, p: &eframe::egui::Painter) {
        p.circle_filled(self.posision(), self.size, egui::Color32::YELLOW);
    }
    fn cast(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Character for Player {
    fn set_velocity(&mut self, vel: Vec2) {
        self.velocity = vel.normalized() * self.speed;
    }

    fn velocity(&self) -> Vec2 {
        self.velocity
    }

    fn add_next_direction(&mut self, direction: Direcciones) {
        self.next_direction = Some(direction);
    }
    fn next_direction(&self) -> Option<&Direcciones> {
        if let Some(dir) = &self.next_direction {
            Some(dir)
        } else {
            None
        }
    }
    fn consume_direction(&mut self) -> Option<Direcciones> {
        let dirección = self.next_direction.clone();
        self.next_direction = None;
        dirección
    }

    fn colision(&mut self, other: &mut dyn Object2D) {
        let vector_distancia = self.posision() - other.posision();

        let dist_colision = self.colider().radio() + other.colider().radio();
        if vector_distancia.length() <= dist_colision {
            match other.colider().tipo {
                super::TipoColider::Obstaculo => {
                    let posision = |d| {
                        let v = self.velocity.normalized()
                            * -1.0
                            * if d {
                                super::pared::PARED_SIZE.x
                            } else {
                                super::pared::PARED_SIZE.y
                            };
                        let posision_uncorrected = other.posision() + v;

                        if d {
                            let Pos2 { x: _, y } = self.posision;
                            let Pos2 { x, y: _ } = posision_uncorrected;
                            Pos2 { x, y }
                        } else {
                            let Pos2 { x, y: _ } = self.posision;
                            let Pos2 { x: _, y } = posision_uncorrected;
                            Pos2 { x, y }
                        }
                    };
                    self.set_posision(posision(self.velocity.y == 0.0));

                    self.set_velocity(Vec2::ZERO);
                }
                super::TipoColider::Moneda => {
                    let Some(moneda) = other.cast().downcast_mut::<Moneda>() else {
                        panic!(
                            "un objeto no moneda tiene un colider tipo moneda\n
                            datos: {:#?}",
                            other
                        );
                    };

                    if !moneda.tomada() {
                        self.puntaje += 1;
                    }
                    moneda.tomar_moneda();
                }
                _ => todo!(),
            }
        }
    }

    fn logic(&mut self, i: &egui::InputState) {
        use egui::Key;
        for event in &i.events {
            if let egui::Event::Key {
                key,
                modifiers,
                pressed,
                repeat,
                ..
            } = *event
            {
                if pressed && !repeat && !modifiers.any() {
                    match key {
                        Key::A | Key::ArrowLeft => {
                            self.add_next_direction(Direcciones::Oeste);
                        }
                        Key::S | Key::ArrowDown => {
                            self.add_next_direction(Direcciones::Sur);
                        }
                        Key::D | Key::ArrowRight => {
                            self.add_next_direction(Direcciones::Este);
                        }
                        Key::W | Key::ArrowUp => {
                            self.add_next_direction(Direcciones::Norte);
                        }
                        _ => (),
                    }
                }
            }
        }

        if let Some(direccion) = self.next_direction() {
            match direccion {
                Direcciones::Este | Direcciones::Oeste => {
                    let dist_anclaje = (self.posision().y / (super::pared::PARED_SIZE.y)).fract();
                    if dist_anclaje <= 0.58 && dist_anclaje >= 0.46 {
                        if *direccion == Direcciones::Este {
                            self.set_velocity(Vec2 { x: 1.0, y: 0.0 });
                        } else {
                            self.set_velocity(Vec2 { x: -1.0, y: 0.0 });
                        }
                        self.consume_direction();
                    }
                }
                Direcciones::Norte | Direcciones::Sur => {
                    let dist_anclaje = (self.posision().x / (super::pared::PARED_SIZE.x)).fract();
                    if dist_anclaje <= 0.58 && dist_anclaje >= 0.46 {
                        if *direccion == Direcciones::Norte {
                            self.set_velocity(vec2(0.0, -1.0));
                        } else {
                            self.set_velocity(vec2(0.0, 1.0));
                        }
                        self.consume_direction();
                    }
                }
            }
        }
    }

    fn grid_pos(&self) -> (usize, usize) {
        (
            (self.posision().y / (super::pared::PARED_SIZE.y)).trunc() as usize,
            (self.posision().x / (super::pared::PARED_SIZE.x)).trunc() as usize,
        )
    }
}

impl Player {
    pub fn new() -> Self {
        let mut jugador = Self {
            posision: Pos2::ZERO,
            colider: Colider {
                radio: 0.0,
                tipo: super::TipoColider::Jugador,
            },
            size: 0.0,
            frames: Vec::new(),
            frame: 0,
            velocity: Vec2::ZERO,
            speed: 160.0,
            rotation: 0.0,
            next_direction: None,
            puntaje: 0,
        };

        jugador.init();

        jugador
    }
    pub fn puntaje(&self) -> u32 {
        self.puntaje
    }
}
