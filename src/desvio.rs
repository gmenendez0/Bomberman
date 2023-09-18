use crate::coordenada::Coordenada;
use crate::objeto_mapa::ResultadoRafaga::{
    DesvioAbajo, DesvioArriba, DesvioDerecha, DesvioIzquierda,
};
use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};

pub enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}

pub struct Desvio {
    coordenada_actual: Coordenada,
    direccion: Direccion,
}

impl Desvio {
    pub fn new(coordenada_actual: Coordenada, letra_direccion: String) -> Desvio {
        let direccion = match letra_direccion.as_str() {
            "U" => Direccion::Arriba,
            "D" => Direccion::Abajo,
            "L" => Direccion::Izquierda,
            "R" => Direccion::Derecha,
            _ => Direccion::Arriba,
        };

        Desvio {
            coordenada_actual,
            direccion,
        }
    }

    fn chequear_desvio(&self) -> ResultadoRafaga {
        match self.direccion {
            Direccion::Arriba => DesvioArriba,
            Direccion::Abajo => DesvioAbajo,
            Direccion::Izquierda => DesvioIzquierda,
            Direccion::Derecha => DesvioDerecha,
        }
    }
}

impl ObjetoMapa for Desvio {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> Coordenada {
        self.coordenada_actual.clone()
    }

    fn recibir_rafaga(&mut self) -> ResultadoRafaga {
        self.chequear_desvio()
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
