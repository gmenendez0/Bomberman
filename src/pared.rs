use crate::coordenada::Coordenada;
use crate::objeto_mapa::ResultadoRafaga::ChoqueFuerte;
use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};

pub struct Pared {
    coordenada_actual: Coordenada,
}

impl Pared {
    pub fn new(coordenada_actual: Coordenada) -> Pared {
        Pared { coordenada_actual }
    }
}

impl ObjetoMapa for Pared {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> &Coordenada {
        &self.coordenada_actual
    }

    fn recibir_rafaga(&mut self) -> ResultadoRafaga {
        ChoqueFuerte
    }

    fn recibir_rafaga_traspaso(&mut self) -> ResultadoRafaga {
        ChoqueFuerte
    }

    fn detonar(&mut self) -> Result<(), String> {
        Err("No se puede detonar una pared".to_string())
    }

    fn obtener_representacion(&self) -> String {
        String::from("W ")
    }
}
