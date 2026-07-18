use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Weak},
};

#[derive(Debug)]
pub enum Casilla {
    Pared,
    Vacio,
    Spawn,
    Nodo(Arc<RwLock<Nodo>>),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Direcciones {
    Norte,
    Sur,
    Este,
    Oeste,
}

#[derive(Debug)]
pub struct Nodo {
    conections: HashMap<Direcciones, Weak<RwLock<Nodo>>>,
}

impl Default for Nodo {
    fn default() -> Self {
        Self {
            conections: HashMap::new(),
        }
    }
}

impl Nodo {
    pub fn add_conection(&mut self, conection: &Arc<RwLock<Nodo>>, direccion: Direcciones) {
        self.conections.insert(direccion, Arc::downgrade(conection));
    }
    pub fn conections(&self) -> &HashMap<Direcciones, Weak<RwLock<Nodo>>> {
        &self.conections
    }
}
