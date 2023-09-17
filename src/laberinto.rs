use crate::casillero::Casillero;
use crate::casillero::Casillero::{
    CasilleroBombaNormal, CasilleroBombaTraspaso, CasilleroDesvio, CasilleroEnemigo,
    CasilleroPared, CasilleroRoca, CasilleroVacio,
};
use crate::coordenada::Coordenada;
use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};
use crate::vacio::Vacio;

pub struct Laberinto<'a> {
    tablero: Vec<Vec<Casillero<'a>>>,
}

impl<'a> Laberinto<'a> {
    pub fn new(dimension_tablero: usize) -> Laberinto<'a> {
        let mut tablero: Vec<Vec<Casillero>> = Vec::new();

        for i in 0..dimension_tablero {
            let mut fila: Vec<Casillero> = Vec::new();

            for j in 0..dimension_tablero {
                fila.push(CasilleroVacio(Vacio::new(Coordenada::new(i, j))));
            }

            tablero.push(fila);
        }

        Laberinto { tablero }
    }

    pub fn reemplazar_casillero_en_tablero(&self, casillero: Casillero) {
        if self.coordenadas_fuera_de_rango(casillero.obtener_coordenada()) {
            return;
        }

        self.tablero[casillero.obtener_coordenada().get_x()]
            [casillero.obtener_coordenada().get_y()] = casillero;
    }

    pub fn coordenadas_fuera_de_rango(&self, coordenada: &Coordenada) -> bool {
        coordenada.get_x() >= self.tablero.len() || coordenada.get_y() >= self.tablero.len()
    }

    pub fn rafagear_coordenada(&mut self, coordenada_a_rafagear: &Coordenada) -> ResultadoRafaga {
        if self.coordenadas_fuera_de_rango(coordenada_a_rafagear) {
            return ResultadoRafaga::ChoqueFuerte;
        }

        let casillero: &mut Casillero<'a> =
            &mut self.tablero[coordenada_a_rafagear.get_x()][coordenada_a_rafagear.get_y()];
        let resultado_rafaga: ResultadoRafaga;

        match casillero {
            CasilleroVacio(vacio) => {
                resultado_rafaga = vacio.recibir_rafaga();
            }
            CasilleroRoca(roca) => {
                resultado_rafaga = roca.recibir_rafaga();
            }
            CasilleroPared(pared) => {
                resultado_rafaga = pared.recibir_rafaga();
            }
            CasilleroEnemigo(enemigo) => {
                resultado_rafaga = enemigo.recibir_rafaga();
            }
            CasilleroDesvio(desvio) => {
                resultado_rafaga = desvio.recibir_rafaga();
            }
            CasilleroBombaNormal(bomba_normal) => {
                resultado_rafaga = bomba_normal.recibir_rafaga();
            }
            CasilleroBombaTraspaso(bomba_traspaso) => {
                resultado_rafaga = bomba_traspaso.recibir_rafaga();
            }
        }

        resultado_rafaga
    }

    pub fn rafagear_coordenada_traspaso(
        &mut self,
        coordenada_a_rafagear: &Coordenada,
    ) -> ResultadoRafaga {
        if self.coordenadas_fuera_de_rango(coordenada_a_rafagear) {
            return ResultadoRafaga::ChoqueFuerte;
        }

        let casillero: &mut Casillero<'a> =
            &mut self.tablero[coordenada_a_rafagear.get_x()][coordenada_a_rafagear.get_y()];
        let resultado_rafaga: ResultadoRafaga;

        match casillero {
            CasilleroVacio(vacio) => {
                resultado_rafaga = vacio.recibir_rafaga_traspaso();
            }
            CasilleroRoca(roca) => {
                resultado_rafaga = roca.recibir_rafaga_traspaso();
            }
            CasilleroPared(pared) => {
                resultado_rafaga = pared.recibir_rafaga_traspaso();
            }
            CasilleroEnemigo(enemigo) => {
                resultado_rafaga = enemigo.recibir_rafaga_traspaso();
            }
            CasilleroDesvio(desvio) => {
                resultado_rafaga = desvio.recibir_rafaga_traspaso();
            }
            CasilleroBombaNormal(bomba_normal) => {
                resultado_rafaga = bomba_normal.recibir_rafaga_traspaso();
            }
            CasilleroBombaTraspaso(bomba_traspaso) => {
                resultado_rafaga = bomba_traspaso.recibir_rafaga_traspaso();
            }
        };

        resultado_rafaga
    }

    pub fn detonar_coordenada(&mut self, coordenada_a_detonar: &Coordenada) -> Result<(), String> {
        if self.coordenadas_fuera_de_rango(coordenada_a_detonar) {
            return Err("No se puede detonar fuera del mapa!".to_string());
        }

        let casillero_a_detonar: &mut Casillero<'a> =
            &mut self.tablero[coordenada_a_detonar.get_x()][coordenada_a_detonar.get_y()];
        let resultado_detonacion: Result<(), String>;

        match casillero_a_detonar {
            CasilleroVacio(vacio) => {
                resultado_detonacion = vacio.detonar();
            }
            CasilleroRoca(roca) => {
                resultado_detonacion = roca.detonar();
            }
            CasilleroPared(pared) => {
                resultado_detonacion = pared.detonar();
            }
            CasilleroEnemigo(enemigo) => {
                resultado_detonacion = enemigo.detonar();
            }
            CasilleroDesvio(desvio) => {
                resultado_detonacion = desvio.detonar();
            }
            CasilleroBombaNormal(bomba_normal) => {
                resultado_detonacion = bomba_normal.detonar();
            }
            CasilleroBombaTraspaso(bomba_traspaso) => {
                resultado_detonacion = bomba_traspaso.detonar();
            }
        };

        resultado_detonacion
    }

    pub fn vaciar_coordenada(&mut self, coordenada_a_vaciar: &Coordenada) {
        self.tablero[coordenada_a_vaciar.get_x()][coordenada_a_vaciar.get_y()] =
            CasilleroVacio(Vacio::new(Coordenada::new(
                coordenada_a_vaciar.get_x(),
                coordenada_a_vaciar.get_y(),
            )));
    }

    pub fn obtener_visualizacion(&self) -> Vec<Vec<String>> {
        let mut tablero_visualizacion: Vec<Vec<String>> = Vec::new();

        for i in 0..self.tablero.len() {
            let mut fila: Vec<String> = Vec::new();

            for j in 0..self.tablero.len() {
                fila.push(self.tablero[i][j].obtener_representacion());
                fila.push(" ".to_string());
            }

            tablero_visualizacion.push(fila);
        }

        tablero_visualizacion
    }
}
