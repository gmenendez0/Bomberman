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

    pub fn get_vida(&self) -> i32 {
        self.vida
    }
}
