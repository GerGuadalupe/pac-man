use super::Direcciones;
use eframe::egui::Pos2;

use crate::game_logic::{
    laberinto::Casilla,
    objects::{
        Character,
        fantasmas::{Fantasma, planning::EstadoRecorrido::Interrump, utils::TipoFantasma},
    },
};
use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;
type Matriz<T> = Vec<Vec<T>>;

impl Fantasma {
    pub fn planning(
        &mut self,
        mapa: Arc<Matriz<Casilla>>,
        pacman_pos: Pos2,
        pacman_vel: eframe::egui::Vec2,
    ) -> Result<(), String> {
        let objetivo;
        match self.tipo {
            super::TipoFantasma::Rojo => objetivo = pacman_pos,
            super::TipoFantasma::Rosa => objetivo = pacman_pos + (pacman_vel.normalized() * 2.0),
            super::TipoFantasma::Azul => {
                let pos_1 = pacman_pos + (pacman_vel.normalized() * 2.0);
                let dist = pos_1 - self.grid_pos();
                let mut obj = self.grid_pos() + (dist * 2.0);
                objetivo = loop {
                    if obj.x < 0.0 || obj.y < 0.0 {
                        break pacman_pos;
                    }
                    let vec = Vec::new();
                    if let Some(&Casilla::Nodo(_)) =
                        mapa.get(obj.y as usize).unwrap_or(&vec).get(obj.x as usize)
                    {
                        break obj;
                    }
                    if obj.x > obj.y {
                        obj.x -= 1.0;
                    } else {
                        obj.y -= 1.0;
                    }
                }
            }
            super::TipoFantasma::Naranja => objetivo = pacman_pos,
        }
        self.estado = super::utils::State::Planning;
        let retorno = self.chanel.sender();
        let tipo = self.tipo;
        let objetivo = {
            let Pos2 { x, y } = objetivo;
            (y as usize, x as usize)
        };
        let self_pos = {
            let Pos2 { x, y } = self.grid_pos();
            (y as usize, x as usize)
        };

        match &mapa[self_pos.0][self_pos.1] {
            &Casilla::Nodo(_) => (),
            _ => return Err(String::from("por cula del fantasma")),
        }

        thread::spawn(move || {
            let mut mapa_distancias: Matriz<f32> =
                vec![vec![f32::INFINITY; mapa[0].len()]; mapa.len()];

            mapa_distancias[self_pos.0][self_pos.1] = 0.0;
            match recorrido(&*mapa, &mut mapa_distancias, self_pos, objetivo, tipo) {
                EstadoRecorrido::Interrump => retorno.send(None),
                EstadoRecorrido::Error(err) => panic!("error al calcular posisiones => {}", err),
                EstadoRecorrido::Exito(ruta) => retorno.send(ruta),
            }
        });
        self.temp.set_plan_time(5.0);
        Ok(())
    }
}

fn recorrido(
    mapa: &Matriz<Casilla>,
    mapa_distancias: &mut Matriz<f32>,
    inicio: (usize, usize),
    objetivo: (usize, usize),
    tipo: TipoFantasma,
) -> EstadoRecorrido {
    if objetivo == inicio {
        return EstadoRecorrido::Exito(None);
    }

    let mut posibles_caminos: Vec<VecDeque<Direcciones>> = Vec::new();
    let (i, j) = inicio;
    let Casilla::Nodo(nodo_actual) = &mapa[i][j] else {
        return EstadoRecorrido::Error("error, la casilla actual no es un nodo");
    };
    let nodo_actual = nodo_actual.read().unwrap();

    for (coneccion, _) in nodo_actual.conections() {
        let mut n_i = i;
        let mut n_j = j;
        match coneccion {
            Direcciones::Este => {
                if n_j == mapa[0].len() - 1 {
                    n_j = 0;
                } else {
                    n_j += 1;
                }
            }
            Direcciones::Norte => {
                if n_i == 0 {
                    n_i = mapa.len() - 1;
                } else {
                    n_i -= 1
                }
            }
            Direcciones::Oeste => {
                if n_j == 0 {
                    n_j = mapa[0].len() - 1;
                } else {
                    n_j -= 1;
                }
            }
            Direcciones::Sur => {
                if n_i == mapa.len() - 1 {
                    n_i = 0;
                } else {
                    n_i += 1
                }
            }
        }

        if (mapa_distancias[i][j] + 1.0) < mapa_distancias[n_i][n_j] {
            mapa_distancias[n_i][n_j] = mapa_distancias[i][j] + 1.0;
            if let EstadoRecorrido::Exito(recorrido) =
                recorrido(mapa, mapa_distancias, (n_i, n_j), objetivo, tipo)
            {
                match recorrido {
                    Some(mut recorrido) => {
                        recorrido.push_front(coneccion.clone());
                        posibles_caminos.push(recorrido);
                    }
                    None => posibles_caminos.push(VecDeque::from([coneccion.clone()])),
                }
            }
        }
    }
    if posibles_caminos.len() == 0 {
        return Interrump;
    }
    posibles_caminos.sort_by_key(|v| match tipo {
        TipoFantasma::Naranja => (v.get(0).unwrap() != &Direcciones::Norte, v.len()),
        TipoFantasma::Rosa => (v.get(0).unwrap() != &Direcciones::Sur, v.len()),
        TipoFantasma::Rojo => (v.get(0).unwrap() != &Direcciones::Este, v.len()),
        TipoFantasma::Azul => (v.get(0).unwrap() != &Direcciones::Oeste, v.len()),
    });

    let camino_corto: usize = 0;

    return EstadoRecorrido::Exito(Some(posibles_caminos.remove(camino_corto)));
}

enum EstadoRecorrido {
    Error(&'static str),
    Interrump,
    Exito(Option<VecDeque<Direcciones>>),
}

impl Fantasma {
    pub fn try_get_plan(&mut self) {
        if let Ok(plan) = self.chanel.receiber().try_recv() {
            if let Some(ruta) = plan {
                self.ruta = ruta;
            }
            self.estado = super::utils::State::Execute;
        }
    }
}
