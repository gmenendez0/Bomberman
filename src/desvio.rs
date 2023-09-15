use crate::objeto_mapa::ObjetoMapa;
use crate::coordenada::Coordenada;
use crate::juego::Juego;

pub struct Desvio {
    coordenada_actual: Coordenada,
    juego: Juego
}

impl Desvio {
    pub fn new(coordenada_actual: Coordenada, juego: Juego) -> Desvio {
        Desvio { coordenada_actual, juego}
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
    fn recibir_rafaga(&self){
        //? Se debe desviar la rafaga de la explosion...
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga_traspaso(&self){
        //? Se debe desviar la rafaga de la explosion...
    }

    fn detonar(&mut self) -> Result<(), String>{
        Err("No se puede detonar un desvio".to_string())
    }

    fn obtener_representacion(&self) -> String{
        //? Se debe corregir la representacion...
        " ".to_string()
    }
}