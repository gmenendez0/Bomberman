use crate::coordenada::Coordenada;
use crate::laberinto::Laberinto;
use crate::objeto_mapa::ResultadoRafaga::Choque;
use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};

pub struct Roca {
    coordenada_actual: Coordenada,
}

impl Roca {
    pub fn new(coordenada_actual: Coordenada) -> Roca {
        Roca { coordenada_actual }
    }
}

impl ObjetoMapa for Roca {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> Coordenada {
        self.coordenada_actual.clone()
    }

    fn recibir_rafaga(&mut self) -> ResultadoRafaga {
        Choque
    }

    fn detonar(&self, laberinto: &mut Laberinto) -> Result<(), String> {
        Err("No se puede detonar una roca".to_string())
    }

    fn obtener_representacion(&self) -> String {
        String::from("R")
    }
}
