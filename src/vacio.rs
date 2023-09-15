use crate::coordenada::Coordenada;
use crate::objeto_mapa::ObjetoMapa;

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

    fn recibir_rafaga(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn recibir_rafaga_traspaso(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn detonar(&mut self) -> Result<(), String> {
        Err("No se puede detonar un vacio".to_string())
    }

    fn obtener_representacion(&self) -> String {
        String::from("  ")
    }
}
