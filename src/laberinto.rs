use crate::casillero;
use crate::casillero::Casillero;
use crate::casillero::Casillero::{
    CasilleroBombaNormal, CasilleroBombaTraspaso, CasilleroDesvio, CasilleroEnemigo,
    CasilleroPared, CasilleroRoca, CasilleroVacio,
};
use crate::coordenada::Coordenada;
use crate::desvio::Desvio;
use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};
use crate::objeto_mapa::ResultadoRafaga::Insignificante;
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

    pub fn reemplazar_casillero_en_tablero(&self, casillero: casillero::Casillero<'a>) {
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

        let casillero: &mut Casillero<'a> = &mut self.tablero[coordenada_a_rafagear.get_x()][coordenada_a_rafagear.get_y()];
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
                resultado_rafaga = self.detonar_coordenada(bomba_normal.get_coordenada_actual());
            }
            CasilleroBombaTraspaso(bomba_traspaso) => {
                self.detonar_coordenada(bomba_traspaso.get_coordenada_actual());
            }
        }

        resultado_rafaga
    }

    pub fn rafagear_coordenada_traspaso(&mut self, coordenada_a_rafagear: &Coordenada) -> ResultadoRafaga {
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

    pub fn detonar_coordenada(&mut self, coordenada_a_detonar: &Coordenada) -> Result<ResultadoRafaga, String> {
        if self.coordenadas_fuera_de_rango(coordenada_a_detonar) {
            return Err("No se puede detonar fuera del mapa!".to_string());
        }

        let casillero_a_detonar: &mut Casillero<'a> = &mut self.tablero[coordenada_a_detonar.get_x()][coordenada_a_detonar.get_y()];
        let resultado_detonacion: Result<ResultadoRafaga, String>;

        match casillero_a_detonar {
            CasilleroVacio(_) => {
                resultado_detonacion = Err("No se puede detonar un vacio".to_string())
            }
            CasilleroRoca(_) => {
                resultado_detonacion = Err("No se puede detonar una roca".to_string())
            }
            CasilleroPared(_) => {
                resultado_detonacion = Err("No se puede detonar una pared".to_string())
            }
            CasilleroEnemigo(_) => {
                resultado_detonacion = Err("No se puede detonar un enemigo".to_string())
            }
            CasilleroDesvio(_) => {
                resultado_detonacion = Err("No se puede detonar un desvio".to_string())
            }
            CasilleroBombaNormal(bomba_normal) => {
                resultado_detonacion = Ok(self.iniciar_rafagas(casillero_a_detonar, bomba_normal.get_alcance()));
            }
            CasilleroBombaTraspaso(bomba_traspaso) => {
                resultado_detonacion = Ok(self.iniciar_rafagas_traspaso(coordenada_a_detonar));
            }
        };

        resultado_detonacion
    }

    pub fn iniciar_rafagas(&mut self, casillero_inicial: &mut Casillero, alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        self.rafagear_arriba(casillero_inicial.obtener_coordenada(), alcance_restante_rafagas.clone());
        self.rafagear_abajo(casillero_inicial.obtener_coordenada(), alcance_restante_rafagas.clone());
        self.rafagear_derecha(casillero_inicial.obtener_coordenada(), alcance_restante_rafagas.clone());
        self.rafagear_izquierda(casillero_inicial.obtener_coordenada(), alcance_restante_rafagas.clone())
    }

    pub fn rafagear_arriba(&mut self, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial;
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);

        while self.rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &resultado_rafaga ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_y() + 1);
            alcance_restante_rafagas -= 1;

            resultado_rafaga = self.rafagear_coordenada(&coordenada_a_rafagear);
        }

        if self.rafaga_continua_chocando_obstaculo(coordenada_a_rafagear, &resultado_rafaga) {
            resultado_rafaga = self.evaluar_camino_a_seguir(coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga));
        }

        resultado_rafaga
    }

    pub fn rafagear_abajo(&mut self, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> ResultadoRafaga{
        let mut coordenada_a_rafagear = coordenada_inicial;
        let mut resultado_rafaga = Insignificante;

        while self.rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &resultado_rafaga) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_y() - 1);
            alcance_restante_rafagas -= 1;

            resultado_rafaga = self.rafagear_coordenada(&coordenada_a_rafagear);
        }

        if self.rafaga_continua_chocando_obstaculo(coordenada_a_rafagear, &resultado_rafaga, ) {
            resultado_rafaga = self.evaluar_camino_a_seguir(coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga);
        }

        resultado_rafaga
    }

    pub fn rafagear_derecha(&mut self, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> ResultadoRafaga{
        let mut coordenada_a_rafagear = coordenada_inicial;
        let mut resultado_rafaga = Insignificante;

        while self.rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &resultado_rafaga) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_x() + 1);
            alcance_restante_rafagas -= 1;

            resultado_rafaga = self.rafagear_coordenada(&coordenada_a_rafagear);
        }

        if self.rafaga_continua_chocando_obstaculo(coordenada_a_rafagear, &resultado_rafaga, ) {
            resultado_rafaga = self.evaluar_camino_a_seguir(coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga);
        }

        resultado_rafaga
    }

    pub fn rafagear_izquierda(&mut self, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> ResultadoRafaga{
        let mut coordenada_a_rafagear = coordenada_inicial;
        let mut resultado_rafaga = Insignificante;

        while self.rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &resultado_rafaga) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_x() - 1);
            alcance_restante_rafagas -= 1;

            resultado_rafaga = self.rafagear_coordenada(&coordenada_a_rafagear);
        }

        if self.rafaga_continua_chocando_obstaculo(coordenada_a_rafagear, &resultado_rafaga, ) {
            resultado_rafaga = self.evaluar_camino_a_seguir(coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga);
        }

        resultado_rafaga
    }

    fn evaluar_camino_a_seguir(&mut self, coordenada_inicial: &Coordenada, alcance_restante: i32, resultado_rafaga: ResultadoRafaga) -> Result<ResultadoRafaga, String> {
        match resultado_rafaga {
            ResultadoRafaga::DesvioArriba => self.rafagear_arriba(coordenada_inicial, alcance_restante),
            ResultadoRafaga::DesvioAbajo => self.rafagear_abajo(coordenada_inicial, alcance_restante),
            ResultadoRafaga::DesvioIzquierda => { self.rafagear_izquierda(coordenada_inicial, alcance_restante) },
            ResultadoRafaga::DesvioDerecha => { self.rafagear_derecha(coordenada_inicial, alcance_restante) },
            _ => {}
        }
    }

    fn rafaga_continua_sin_chocar_obstaculo(&self, alcance_restante: i32, resultado_rafaga: &ResultadoRafaga) -> bool {
        alcance_restante > 0 && self.rafaga_no_choca_obstaculo(resultado_rafaga)
    }

    fn rafaga_no_choca_obstaculo(resultado_rafaga: &ResultadoRafaga) -> bool {
        !matches!(resultado_rafaga,ResultadoRafaga::DesvioArriba | ResultadoRafaga::DesvioAbajo | ResultadoRafaga::DesvioIzquierda | ResultadoRafaga::DesvioDerecha | crate::objeto_mapa::ResultadoRafaga::Choque)
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
