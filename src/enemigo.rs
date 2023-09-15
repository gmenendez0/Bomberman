use crate::coordenada::Coordenada;
use crate::juego::Juego;
use crate::objeto_mapa::ObjetoMapa;

pub struct Enemigo<'a> {
    coordenada_actual: Coordenada,
    juego: &'a mut Juego,
    vida: i32,
}

impl<'a> Enemigo<'a> {
    pub fn new(coordenada_actual: Coordenada, juego: &mut Juego, vida: i32) -> Enemigo {
        Enemigo {
            coordenada_actual,
            juego,
            vida,
        }
    }

    fn reducir_vida(&mut self) {
        self.vida -= 1;
    }
}

impl<'a> ObjetoMapa for Enemigo<'a> {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> &Coordenada {
        &self.coordenada_actual
    }

    fn recibir_rafaga(&mut self) -> Result<(), String> {
        self.reducir_vida();

        if self.vida <= 0 {
            self.juego.vaciar_coordenada(&self.coordenada_actual);
        }

        Ok(())
    }

    fn recibir_rafaga_traspaso(&mut self) -> Result<(), String> {
        self.reducir_vida();

        if self.vida <= 0 {
            self.juego.vaciar_coordenada(&self.coordenada_actual);
        }

        Ok(())
    }

    fn detonar(&mut self) -> Result<(), String> {
        Err("No se puede detonar un enemigo".to_string())
    }

    fn obtener_representacion(&self) -> String {
        let representacion_enemigo = String::from("F");
        let representacion_de_vida = self.vida.to_string();

        representacion_enemigo + &representacion_de_vida
    }
}
