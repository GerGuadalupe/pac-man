use crate::game_logic::laberinto::{Arc, Casilla, RwLock, Nodo, casillas};


pub fn maze_init(lab: &mut Vec<Vec<Casilla>>) {
    let mut check_list = [[false; 31]; 19];
    for i in 0..lab.len(){
        for j in 0..lab[0].len(){
            recursive_init(lab, &mut check_list, (i,j));
        }
    }
}




fn recursive_init<'a>(
    lab: &'a mut Vec<Vec<Casilla>>,
    check_list: &mut [[bool; 31]; 19],
    casilla: (usize, usize),
) -> Option<Arc<RwLock<Nodo>>> {

    use casillas::Direcciones;
    let (i, j) = casilla;

    let Casilla::Nodo(n) = &lab[i][j] else {
        check_list[i][j] = true;
        return None;
    };

    let n = Arc::clone(n);

    if check_list[i][j] {
        return Some(n);
    }

    check_list[i][j] = true;

    let obtener_ij = |dir| {
        let i_limit = lab.len() - 1;
        let j_limit = lab[0].len() - 1;

        match dir {
            Direcciones::Norte => {
                if i == 0 {
                    (i_limit, j)
                } else {
                    (i - 1, j)
                }
            }

            Direcciones::Sur => {
                if i == i_limit {
                    (0, j)
                } else {
                    (i + 1, j)
                }
            }
            Direcciones::Este => {
                if j == j_limit {
                    (i, 0)
                } else {
                    (i, j + 1)
                }
            }
            Direcciones::Oeste => {
                if j == 0 {(i, j_limit)
                } else {
                    (i, j - 1)
                }
            }
        }
    };

    let (nort, sur, est, ost) = (
        obtener_ij(Direcciones::Norte),
        obtener_ij(Direcciones::Sur),
        obtener_ij(Direcciones::Este),
        obtener_ij(Direcciones::Oeste),
    );

    if let Some(conection) = recursive_init(lab, check_list, nort) {
        let mut n = n.write().unwrap();
        n.add_conection(&conection, Direcciones::Norte);
    }
    if let Some(conection) = recursive_init(lab, check_list, sur) {
        let mut n = n.write().unwrap();
        n.add_conection(&conection, Direcciones::Sur);
    }
    if let Some(conection) = recursive_init(lab, check_list, est) {
        let mut n = n.write().unwrap();
        n.add_conection(&conection, Direcciones::Este);
    }
    if let Some(conection) = recursive_init(lab, check_list, ost) {
        let mut n = n.write().unwrap();
        n.add_conection(&conection, Direcciones::Oeste);
    }

    return Some(n);
}
