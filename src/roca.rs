use crate::coordenada::Coordenada;
use crate::juego::Juego;
use crate::objeto_mapa::ObjetoMapa;

pub struct Roca<'a> {
    coordenada_actual: Coordenada,
    juego: &'a mut Juego,
}

impl<'a> Roca<'a> {
    pub fn new(coordenada_actual: Coordenada, juego: &mut Juego) -> Roca {
        Roca {
            coordenada_actual,
            juego,
        }
    }
}

impl<'a> ObjetoMapa for Roca<'a> {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> &Coordenada {
        &self.coordenada_actual
    }

    fn recibir_rafaga(&mut self) -> Result<(), String> {
        //? Se debe bloquear que la explosion siga...
        Ok(())
    }

    fn recibir_rafaga_traspaso(&mut self) -> Result<(), String> {
        //? No se debe hacer nada...
        Ok(())
    }

    fn detonar(&mut self) -> Result<(), String> {
        Err("No se puede detonar una roca".to_string())
    }

    fn obtener_representacion(&self) -> String {
        String::from("R ")
    }
}
