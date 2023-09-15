use crate::objeto_mapa::ObjetoMapa;
use crate::coordenada::Coordenada;
use crate::juego::Juego;

pub struct Pared {
    coordenada_actual: Coordenada,
    juego: Juego
}

impl Pared {
    pub fn new(coordenada_actual: Coordenada, juego: Juego) -> Pared {
        Pared { coordenada_actual, juego}
    }
}

impl ObjetoMapa for Pared {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> &Coordenada {
        &self.coordenada_actual
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga(&self){
        //? Se debe bloquear que la explosion siga...
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga_traspaso(&self){
        //? Se debe bloquear que la explosion siga...
    }

    fn detonar(&mut self) -> Result<(), String>{
        Err("No se puede detonar una pared".to_string())
    }

    fn obtener_representacion(&self) -> String{
        //? Se debe corregir la representacion...
        " ".to_string()
    }
}