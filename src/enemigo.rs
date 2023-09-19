//? Representa al enemigo. Podría eliminarse y simplemente representarse con un i32 dentro de CasilleroEnemigo pero no llego con el tiempo.
#[derive(Clone)]
pub struct Enemigo {
    vida: i32,
}

impl Enemigo {
    pub fn new(vida: i32) -> Enemigo {
        Enemigo {
            vida,
        }
    }

    //? Devuelve la vida del enemigo
    pub fn get_vida(&self) -> i32 {
        self.vida
    }
}
