use crate::casillero::ResultadoRafaga::{Choque, ChoqueFuerte, EnemigoEliminado, Insignificante};
use crate::coordenada::Coordenada;
use crate::enemigo::Enemigo;
use crate::laberinto::Laberinto;

#[derive(Clone)]
pub enum ResultadoRafaga {
    DesvioArriba,
    DesvioAbajo,
    DesvioIzquierda,
    DesvioDerecha,
    Choque,
    ChoqueFuerte,
    Insignificante,
    EnemigoEliminado,
}

#[derive(Clone)]
pub enum Casillero {
    Vacio(Coordenada),
    Roca(Coordenada),
    Pared(Coordenada),
    Enemigoo(Coordenada, Enemigo), //Hay que agregarle la vida...
    Desvio(Coordenada, String),
    BombaNormal(Coordenada, i32),
    BombaTraspaso(Coordenada, i32),
}

impl Casillero {
    pub fn detonar(&self, x: &mut Laberinto) -> Result<(), String> {
        match self {
            Casillero::Vacio(_) => Err("No se puede detonar un vacio".to_string()),
            Casillero::Roca(_) => Err("No se puede detonar una roca".to_string()),
            Casillero::Pared(_) => Err("No se puede detonar una pared".to_string()),
            Casillero::Enemigoo(..) => Err("No se puede detonar un enemigo".to_string()),
            Casillero::Desvio(..) => Err("No se puede detonar un desvio".to_string()),
            Casillero::BombaNormal(_, _) => Ok(()),
            Casillero::BombaTraspaso(_, _) => Ok(()),
        }
    }

    pub fn recibir_rafaga(&self) -> ResultadoRafaga {
        match self {
            Casillero::Vacio(_) => Insignificante,
            Casillero::Roca(_) => Choque,
            Casillero::Pared(_) => ChoqueFuerte,
            Casillero::Enemigoo(_, enemigo) => {
                //enemigo.reducir_vida();

                if enemigo.esta_muerto() {
                    return EnemigoEliminado;
                }

                EnemigoEliminado
            },
            Casillero::Desvio(..) => Insignificante,
            Casillero::BombaNormal(_, _) => Insignificante,
            Casillero::BombaTraspaso(_, _) => Insignificante,
        }
    }

    pub(crate) fn obtener_representacion(&self) -> String {
        match self {
            Casillero::Vacio(_) => String::from("_"),
            Casillero::Roca(_) => String::from("R"),
            Casillero::Pared(_) => String::from("W"),
            Casillero::Enemigoo(_, enemigo) => {
                String::from("E") + &enemigo.get_vida().to_string()
            },
            Casillero::Desvio(_, direccion) => {
                String::from("D") + direccion
            },
            Casillero::BombaNormal(_, alcance) => {
                let representacion_bomba = String::from("B");
                let representacion_alcance= alcance.to_string();

                representacion_bomba + &representacion_alcance
            },
            Casillero::BombaTraspaso(_, alcance) => {
                let representacion_bomba_traspaso = String::from("S");
                let representacion_alcance= alcance.to_string();

                representacion_bomba_traspaso + &representacion_alcance
            },
        }
    }

    pub fn get_coordenada(&self) -> Coordenada {
        match self {
            Casillero::Vacio(coordenada) => coordenada.clone(),
            Casillero::Roca(coordenada) => coordenada.clone(),
            Casillero::Pared(coordenada) => coordenada.clone(),
            Casillero::Enemigoo(coordenada, ..) => coordenada.clone(),
            Casillero::Desvio(coordenada, _) => coordenada.clone(),
            Casillero::BombaNormal(coordenada, _) => coordenada.clone(),
            Casillero::BombaTraspaso(coordenada, _) => coordenada.clone(),
        }
    }
}