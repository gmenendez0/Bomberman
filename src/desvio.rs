use crate::coordenada::Coordenada;
use crate::juego::Juego;
use crate::objeto_mapa::ObjetoMapa;

pub enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}

pub struct Desvio {
    coordenada_actual: Coordenada,
    juego: Juego,
    direccion: Direccion,
}

impl Desvio {
    pub fn new(coordenada_actual: Coordenada, juego: Juego, direccion: Direccion) -> Desvio {
        Desvio {
            coordenada_actual,
            juego,
            direccion,
        }
    }
}

impl ObjetoMapa for Desvio {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> &Coordenada {
        &self.coordenada_actual
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga(&self) {
        //? Se debe desviar la rafaga de la explosion...
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga_traspaso(&self) {
        //? Se debe desviar la rafaga de la explosion...
    }

    fn detonar(&mut self) -> Result<(), String> {
        Err("No se puede detonar un desvio".to_string())
    }

    fn obtener_representacion(&self) -> String {
        let representacion_desvio = String::from("D");

        let representacion_de_direccion = match self.direccion {
            Direccion::Arriba => String::from("U"),
            Direccion::Abajo => String::from("D"),
            Direccion::Izquierda => String::from("L"),
            Direccion::Derecha => String::from("R"),
        };

        representacion_desvio + &representacion_de_direccion
    }
}
