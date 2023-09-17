#[derive(Clone)]
pub struct Coordenada {
    x: usize,
    y: usize,
}

impl Coordenada {
    pub fn new(x: usize, y: usize) -> Coordenada {
        Coordenada { x, y }
    }

    pub fn get_x(&self) -> usize {
        self.x.clone()
    }

    pub fn get_y(&self) -> usize {
        self.y.clone()
    }

    pub fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }
}
