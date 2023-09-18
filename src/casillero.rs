use crate::bomba_normal::BombaNormal;
use crate::bomba_traspaso::BombaTraspaso;
use crate::casillero::Casillero::{
    CasilleroBombaNormal, CasilleroBombaTraspaso, CasilleroDesvio, CasilleroEnemigo,
    CasilleroPared, CasilleroRoca, CasilleroVacio,
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
    CasilleroVacio(Vacio),
    CasilleroRoca(Roca),
    CasilleroPared(Pared),
    CasilleroEnemigo(Enemigo),
    CasilleroDesvio(Desvio),
    CasilleroBombaNormal(BombaNormal),
    CasilleroBombaTraspaso(BombaTraspaso),
}

impl Casillero {
    pub fn obtener_representacion(&self) -> String {
        let representacion: String;

        match &self {
            CasilleroVacio(vacio) => {
                representacion = vacio.obtener_representacion();
            }
            CasilleroRoca(roca) => {
                representacion = roca.obtener_representacion();
            }
            CasilleroPared(pared) => {
                representacion = pared.obtener_representacion();
            }
            CasilleroEnemigo(enemigo) => {
                representacion = enemigo.obtener_representacion();
            }
            CasilleroDesvio(desvio) => {
                representacion = desvio.obtener_representacion();
            }
            CasilleroBombaNormal(bomba_normal) => {
                representacion = bomba_normal.obtener_representacion();
            }
            CasilleroBombaTraspaso(bomba_traspaso) => {
                representacion = bomba_traspaso.obtener_representacion();
            }
        };

        representacion
    }

    pub fn obtener_coordenada(&self) -> Coordenada {
        let coordenada: Coordenada;

        match &self {
            CasilleroVacio(vacio) => {
                coordenada = vacio.get_coordenada_actual();
            }
            CasilleroRoca(roca) => {
                coordenada = roca.get_coordenada_actual();
            }
            CasilleroPared(pared) => {
                coordenada = pared.get_coordenada_actual();
            }
            CasilleroEnemigo(enemigo) => {
                coordenada = enemigo.get_coordenada_actual();
            }
            CasilleroDesvio(desvio) => {
                coordenada = desvio.get_coordenada_actual();
            }
            CasilleroBombaNormal(bomba_normal) => {
                coordenada = bomba_normal.get_coordenada_actual();
            }
            CasilleroBombaTraspaso(bomba_traspaso) => {
                coordenada = bomba_traspaso.get_coordenada_actual();
            }
        };

        coordenada
    }

    pub fn detonar(&mut self, laberinto: &mut Laberinto) -> Result<(), String> {
        let result: Result<(), String>;

        match self {
            CasilleroVacio(vacio) => {
                result = vacio.detonar(laberinto);
            }
            CasilleroRoca(roca) => {
                result = roca.detonar(laberinto);
            }
            CasilleroPared(pared) => {
                result = pared.detonar(laberinto);
            }
            CasilleroEnemigo(enemigo) => {
                result = enemigo.detonar(laberinto);
            }
            CasilleroDesvio(desvio) => {
                result = desvio.detonar(laberinto);
            }
            CasilleroBombaNormal(bomba_normal) => {
                result = bomba_normal.detonar(laberinto);
            }
            CasilleroBombaTraspaso(bomba_traspaso) => {
                result = bomba_traspaso.detonar(laberinto);
            }
        };

        result

    }

}
