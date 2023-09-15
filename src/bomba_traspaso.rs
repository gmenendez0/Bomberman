use crate::objeto_mapa::ObjetoMapa;
use crate::coordenada::Coordenada;
use crate::juego::Juego;

pub struct BombaTraspaso {
    coordenada_actual: Coordenada,
    juego: Juego,
    alcance: i32,
}

impl BombaTraspaso {
    pub fn new(coordenada_actual: Coordenada, juego: Juego, alcance: i32) -> BombaTraspaso {
        BombaTraspaso { coordenada_actual, juego, alcance }
    }
}

impl ObjetoMapa for BombaTraspaso {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> &Coordenada {
        &self.coordenada_actual
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga(&self){
        //? Se debe explotar la bomba utilizando la explosion traspaso...
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga_traspaso(&self){
        //? Se debe explotar la bomba utilizando la explosion traspaso...
    }

    fn detonar(&mut self) -> Result<(), String>{
        //? Se debe explotar la bomba utilizando la explosion traspaso...
        Ok(())
    }

    fn obtener_representacion(&self) -> String{
        //? Se debe corregir la representacion...
        " ".to_string()
    }
}