use crate::objeto_mapa::ObjetoMapa;
use crate::coordenada::Coordenada;
use crate::juego::Juego;

pub struct BombaNormal {
    coordenada_actual: Coordenada,
    juego: Juego,
    alcance: i32,
}

impl BombaNormal {
    pub fn new(coordenada_actual: Coordenada, juego: Juego, alcance: i32) -> BombaNormal {
        BombaNormal { coordenada_actual, juego, alcance }
    }
}

impl ObjetoMapa for BombaNormal {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> &Coordenada {
        &self.coordenada_actual
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga(&self){
        //? Se debe detonar la bomba utilizando explosion normal...
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga_traspaso(&self){
        //? Se debe detonar la bomba utilizando explosion normal...
    }

    fn detonar(&mut self) -> Result<(), String>{
        //? Se debe detonar la bomba utilizando explosion normal...
        Ok(())
    }

    fn obtener_representacion(&self) -> String{
        //? Se debe corregir la representacion...
        " ".to_string()
    }
}