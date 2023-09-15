use crate::coordenada::Coordenada;
use crate::juego::Juego;
use crate::objeto_mapa::ObjetoMapa;

pub struct Roca {
    coordenada_actual: Coordenada,
    juego: Juego,
}

impl Roca {
    pub fn new(coordenada_actual: Coordenada, juego: Juego) -> Roca {
        Roca {
            coordenada_actual,
            juego,
        }
    }
}

impl ObjetoMapa for Roca {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> &Coordenada {
        &self.coordenada_actual
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga(&self) {
        //? Se debe bloquear que la explosion siga...
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga_traspaso(&self) {
        //? No se debe hacer nada...
    }

    fn detonar(&mut self) -> Result<(), String> {
        Err("No se puede detonar una roca".to_string())
    }

    fn obtener_representacion(&self) -> String {
        //? Se debe corregir la representacion...
        " ".to_string()
    }
}
