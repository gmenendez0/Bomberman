use crate::coordenada::Coordenada;
use crate::laberinto::Laberinto;

use crate::objeto_mapa::ResultadoRafaga::Insignificante;

use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};

pub struct BombaTraspaso {
    coordenada_actual: Coordenada,
    alcance: i32,
}

impl BombaTraspaso {
    pub fn new(
        coordenada_actual: Coordenada,
        alcance: i32,
    ) -> BombaTraspaso{
        BombaTraspaso {
            coordenada_actual,
            alcance,
        }
    }

    pub fn get_alcance(&self) -> i32 {
        self.alcance
    }
}

impl ObjetoMapa for BombaTraspaso {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> Coordenada {
        self.coordenada_actual.clone()
    }

    fn recibir_rafaga(&mut self) -> ResultadoRafaga {
        Insignificante
    }

    fn detonar(&mut self, laberinto: &mut Laberinto) -> Result<(), String> {
        Ok(())
    }

    fn obtener_representacion(&self) -> String {
        let representacion_bomba_traspaso = String::from("S");
        let representacion_de_alcance = self.alcance.to_string();

        representacion_bomba_traspaso + &representacion_de_alcance
    }
}
