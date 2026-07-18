use super::Direcciones;
use crate::game_logic::{
    laberinto::Casilla,
    objects::{
        Character,
        fantasmas::{Fantasma, planning::EstadoRecorrido::Interrump},
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
        pacman_pos: (usize, usize),
    ) -> Result<(), String> {
        self.estado = super::utils::State::Planning;
        let retorno = self.chanel.sender();
        let self_pos = self.grid_pos();

        match (
            &mapa[pacman_pos.0][pacman_pos.1],
            &mapa[self_pos.0][self_pos.1],
        ) {
            (&Casilla::Nodo(_), &Casilla::Nodo(_)) => (),
            _ => {
                return Err(format!(
                    r#"
posisión de pacman = {:#?}
posisión del fantasma = {:#?}
                "#,
                    pacman_pos, self_pos
                ));
            }
        }

        thread::spawn(move || {
            let mut mapa_distancias: Matriz<f32> =
                vec![vec![f32::INFINITY; mapa[0].len()]; mapa.len()];

            mapa_distancias[self_pos.0][self_pos.1] = 0.0;
            match recorrido(&*mapa, &mut mapa_distancias, self_pos, pacman_pos) {
                EstadoRecorrido::Interrump => panic!("el fantasma está softlock"),
                EstadoRecorrido::Error(err) => panic!("error al calcular posisiones => {}", err),
                EstadoRecorrido::Exito(ruta) => retorno.send(ruta),
            }
        });
        Ok(())
    }
}

fn recorrido(
    mapa: &Matriz<Casilla>,
    mapa_distancias: &mut Matriz<f32>,
    inicio: (usize, usize),
    objetivo: (usize, usize),
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
                recorrido(mapa, mapa_distancias, (n_i, n_j), objetivo)
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
    let mut camino_corto: usize = 0;
    for i in 1..posibles_caminos.len() {
        if posibles_caminos[i].len() < posibles_caminos[camino_corto].len() {
            camino_corto = i;
        }
    }

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
            } else {
                self.ruta = VecDeque::new();
            }
            self.estado = super::utils::State::Execute;
        }
    }
}
