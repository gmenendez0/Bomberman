use crate::coordenada::Coordenada;
use crate::laberinto::Laberinto;

use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};
use crate::objeto_mapa::ResultadoRafaga::Insignificante;

pub struct BombaNormal {
    coordenada_actual: Coordenada,
    alcance: i32,
}

impl BombaNormal {
    pub fn new(coordenada_actual: Coordenada, alcance: i32) -> BombaNormal {
        BombaNormal {
            coordenada_actual,
            alcance,
        }
    }

    pub fn get_alcance(&self) -> i32 {
        self.alcance
    }
}

impl ObjetoMapa for BombaNormal {
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
        Ok(())


    }

    fn obtener_representacion(&self) -> String {
        let representacion_bomba = String::from("B");
        let representacion_de_alcance = self.alcance.to_string();

        representacion_bomba + &representacion_de_alcance
    }
}
