#[derive(Clone)]
pub struct Coordenada {
    x: i32,
    y: i32,
}

impl Coordenada {
    pub fn new(x: i32, y: i32) -> Coordenada {
        Coordenada { x, y }
    }

    pub fn get_x(&self) -> i32 {
        self.x.clone()
    }

    pub fn get_y(&self) -> i32 {
        self.y.clone()
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}
