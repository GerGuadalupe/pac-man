use std::sync::mpsc;
#[derive(Debug)]
pub struct Chanel<T> {
    sender: mpsc::SyncSender<T>,
    receiber: mpsc::Receiver<T>,
}
impl<T> Chanel<T> {
    pub fn new() -> Self {
        let (sender, receiber) = mpsc::sync_channel(0);
        Chanel { sender, receiber }
    }
    pub fn sender(&self) -> mpsc::SyncSender<T> {
        self.sender.clone()
    }
    pub fn receiber(&self) -> &mpsc::Receiver<T> {
        &self.receiber
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Planning,
    Execute,
    Standby,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TipoFantasma {
    Rojo,
    Azul,
    Rosa,
    Naranja,
}

#[derive(Debug)]
pub struct Timer {
    plan_timer: f32,
    consume_timer: f32,
}
impl Timer {
    pub fn cuentra_atras(&mut self, delta: f32) {
        if self.plan_timer > 0.0 {
            self.plan_timer -= delta
        }
        if self.consume_timer > 0.0 {
            self.consume_timer -= delta;
        }
    }
    pub fn new() -> Self {
        Self {
            plan_timer: 0.0,
            consume_timer: 0.0,
        }
    }
    pub fn plan(&self) -> bool {
        self.plan_timer <= 0.0
    }
    pub fn consume(&self) -> bool {
        self.consume_timer <= 0.0
    }

    pub fn set_consume_time(&mut self, time: f32) {
        self.consume_timer = time;
    }
    pub fn set_plan_time(&mut self, time: f32) {
        self.plan_timer = time;
    }
}
