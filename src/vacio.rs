use crate::coordenada::Coordenada;
use crate::laberinto::Laberinto;
use crate::objeto_mapa::ResultadoRafaga::Insignificante;
use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};

pub struct Vacio {
    coordenada_actual: Coordenada,
}

impl Vacio {
    pub fn new(coordenada_actual: Coordenada) -> Vacio {
        Vacio { coordenada_actual }
    }
}

impl ObjetoMapa for Vacio {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> Coordenada {
        self.coordenada_actual.clone()
    }

    fn recibir_rafaga(&mut self) -> ResultadoRafaga {
        Insignificante
    }

    fn detonar(&self, laberinto: &mut Laberinto) -> Result<(), String> {
        Err("No se puede detonar un vacio".to_string())
    }

    fn obtener_representacion(&self) -> String {
        String::from("_")
    }
}
