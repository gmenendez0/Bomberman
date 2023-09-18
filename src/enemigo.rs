use crate::coordenada::Coordenada;
use crate::laberinto::Laberinto;
use crate::objeto_mapa::ResultadoRafaga::Insignificante;
use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};

pub struct Enemigo {
    coordenada_actual: Coordenada,
    vida: i32,
}

impl Enemigo {
    pub fn new(coordenada_actual: Coordenada, vida: i32) -> Enemigo {
        Enemigo {
            coordenada_actual,
            vida,
        }
    }

    fn reducir_vida(&mut self) {
        self.vida -= 1;
    }
}

impl ObjetoMapa for Enemigo {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> Coordenada {
        self.coordenada_actual.clone()
    }

    fn recibir_rafaga(&mut self) -> ResultadoRafaga {
        self.reducir_vida();

        if self.vida <= 0 {
            //? Se debe actuar!
        }

        Insignificante
    }

    fn detonar(&mut self, laberinto: &mut Laberinto) -> Result<(), String> {
        Err("No se puede detonar un enemigo".to_string())
    }

    fn obtener_representacion(&self) -> String {
        let representacion_enemigo = String::from("F");
        let representacion_de_vida = self.vida.to_string();

        representacion_enemigo + &representacion_de_vida
    }
}
