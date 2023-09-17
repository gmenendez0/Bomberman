use crate::bomba_normal::BombaNormal;
use crate::bomba_traspaso::BombaTraspaso;
use crate::casillero::Casillero::{
    CasilleroBombaNormal, CasilleroBombaTraspaso, CasilleroDesvio, CasilleroEnemigo,
    CasilleroPared, CasilleroRoca, CasilleroVacio,
};
use crate::desvio::Desvio;
use crate::enemigo::Enemigo;
use crate::objeto_mapa::ObjetoMapa;
use crate::pared::Pared;
use crate::roca::Roca;
use crate::vacio::Vacio;

pub(crate) enum Casillero<'a> {
    CasilleroVacio(Vacio),
    CasilleroRoca(Roca),
    CasilleroPared(Pared),
    CasilleroEnemigo(Enemigo<'a>),
    CasilleroDesvio(Desvio),
    CasilleroBombaNormal(BombaNormal<'a>),
    CasilleroBombaTraspaso(BombaTraspaso<'a>),
}

impl<'a> Casillero<'a> {
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
}
