use crate::coordenada::Coordenada;
use crate::juego::Juego;
use crate::objeto_mapa::ObjetoMapa;

pub struct Pared<'a> {
    coordenada_actual: Coordenada,
    juego: &'a mut Juego,
}

impl<'a> Pared<'a> {
    pub fn new(coordenada_actual: Coordenada, juego: &mut Juego) -> Pared {
        Pared {
            coordenada_actual,
            juego,
        }
    }
}

impl<'a> ObjetoMapa for Pared<'a> {
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
        //? Se debe bloquear que la explosion siga...
        Ok(())
    }

    fn detonar(&mut self) -> Result<(), String> {
        Err("No se puede detonar una pared".to_string())
    }

    fn obtener_representacion(&self) -> String {
        String::from("W ")
    }
}
