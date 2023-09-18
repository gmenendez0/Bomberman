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
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }
}

#[cfg(test)]
mod tests {
    use crate::coordenada::Coordenada;

    #[test]
    fn test_new_coordenada() {
        let coord = Coordenada::new(3, 4);
        assert_eq!(coord.get_x(), 3);
        assert_eq!(coord.get_y(), 4);
    }

    #[test]
    fn test_get_set_x() {
        let mut coord = Coordenada::new(3, 4);
        assert_eq!(coord.get_x(), 3);

        coord.set_x(5);
        assert_eq!(coord.get_x(), 5);
    }

    #[test]
    fn test_get_set_y() {
        let mut coord = Coordenada::new(3, 4);
        assert_eq!(coord.get_y(), 4);

        coord.set_y(6);
        assert_eq!(coord.get_y(), 6);
    }
}