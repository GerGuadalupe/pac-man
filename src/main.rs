mod game_app;
mod game_logic;
use game_app::{ALTURA_L, ANCHO_L};
use game_logic::MAPA_LABERINTO;

fn main() -> eframe::Result {
    game_app::launch()
}
