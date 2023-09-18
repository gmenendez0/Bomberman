use crate::bomba_normal::BombaNormal;
use crate::bomba_traspaso::BombaTraspaso;
use crate::casillero::Casillero::{
    BombaNormall, BombaTraspasoo, Desvioo, Enemigoo,
    Paredd, Rocaa, Vacioo,
};
use crate::coordenada::Coordenada;
use crate::desvio::Desvio;
use crate::enemigo::Enemigo;
use crate::laberinto::Laberinto;
use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};
use crate::objeto_mapa::ResultadoRafaga::Insignificante;
use crate::pared::Pared;
use crate::roca::Roca;
use crate::vacio::Vacio;

pub enum Casillero {
    Vacioo(Vacio),
    Rocaa(Roca),
    Paredd(Pared),
    Enemigoo(Enemigo),
    Desvioo(Desvio),
    BombaNormall(BombaNormal),
    BombaTraspasoo(BombaTraspaso),
}

impl Casillero {
    pub fn obtener_representacion(&self) -> String {
        let representacion: String;

        match &self {
            Vacioo(vacio) => {
                representacion = vacio.obtener_representacion();
            }
            Rocaa(roca) => {
                representacion = roca.obtener_representacion();
            }
            Paredd(pared) => {
                representacion = pared.obtener_representacion();
            }
            Enemigoo(enemigo) => {
                representacion = enemigo.obtener_representacion();
            }
            Desvioo(desvio) => {
                representacion = desvio.obtener_representacion();
            }
            BombaNormall(bomba_normal) => {
                representacion = bomba_normal.obtener_representacion();
            }
            BombaTraspasoo(bomba_traspaso) => {
                representacion = bomba_traspaso.obtener_representacion();
            }
        };

        representacion
    }

    pub fn obtener_coordenada(&self) -> Coordenada {
        let coordenada: Coordenada;

        match &self {
            Vacioo(vacio) => {
                coordenada = vacio.get_coordenada_actual();
            }
            Rocaa(roca) => {
                coordenada = roca.get_coordenada_actual();
            }
            Paredd(pared) => {
                coordenada = pared.get_coordenada_actual();
            }
            Enemigoo(enemigo) => {
                coordenada = enemigo.get_coordenada_actual();
            }
            Desvioo(desvio) => {
                coordenada = desvio.get_coordenada_actual();
            }
            BombaNormall(bomba_normal) => {
                coordenada = bomba_normal.get_coordenada_actual();
            }
            BombaTraspasoo(bomba_traspaso) => {
                coordenada = bomba_traspaso.get_coordenada_actual();
            }
        };

        coordenada
    }

    pub fn detonar(&mut self, laberinto: &mut Laberinto) -> Result<(), String> {
        let result: Result<(), String>;

        match self {
            Vacioo(vacio) => {
                result = vacio.detonar(laberinto);
            }
            Rocaa(roca) => {
                result = roca.detonar(laberinto);
            }
            Paredd(pared) => {
                result = pared.detonar(laberinto);
            }
            Enemigoo(enemigo) => {
                result = enemigo.detonar(laberinto);
            }
            Desvioo(desvio) => {
                result = desvio.detonar(laberinto);
            }
            BombaNormall(bomba_normal) => {
                result = bomba_normal.detonar(laberinto);
            }
            BombaTraspasoo(bomba_traspaso) => {
                result = bomba_traspaso.detonar(laberinto);
            }
        };

        result

    }

}
