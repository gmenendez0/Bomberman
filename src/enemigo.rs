use crate::coordenada::Coordenada;
use crate::juego::Juego;
use crate::objeto_mapa::ObjetoMapa;

pub struct Enemigo {
    coordenada_actual: Coordenada,
    juego: Juego,
    vida: i32,
}

impl Enemigo {
    pub fn new(coordenada_actual: Coordenada, juego: Juego, vida: i32) -> Enemigo {
        Enemigo {
            coordenada_actual,
            juego,
            vida,
        }
    }
}

impl ObjetoMapa for Enemigo {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> &Coordenada {
        &self.coordenada_actual
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga(&self) {
        //? Se debe reducir la vida...
        //? Se debe chequear si la vida llego a 0...
        //? Si la vida llego a 0, se le debe decir al juego que elimine al jugador del mapa...
    }

    //? Seguramente debería recibir una referencia mutable...
    fn recibir_rafaga_traspaso(&self) {
        //? Se debe reducir la vida...
        //? Se debe chequear si la vida llego a 0...
        //? Si la vida llego a 0, se le debe decir al juego que elimine al jugador del mapa...
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
