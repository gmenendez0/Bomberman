use crate::objeto_mapa::ObjetoMapa;
use crate::coordenada::Coordenada;

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

    fn get_coordenada_actual(&self) -> &Coordenada {
        &self.coordenada_actual
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga(&self){
        //? No se debe hacer nada...
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga_traspaso(&self){
        //? No se debe hacer nada...
    }

    fn detonar(&mut self) -> Result<(), String>{
        Err("No se puede detonar un vacio".to_string())
    }

    fn obtener_representacion(&self) -> String{
        //? Se debe corregir la representacion...
        " ".to_string()
    }
}