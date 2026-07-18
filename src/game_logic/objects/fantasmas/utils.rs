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

#[derive(Debug, PartialEq)]
pub enum TipoFantasma {
    Rojo,
    Azul,
    Rosa,
    Naranja,
}
