use crate::coordenada::Coordenada;
use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};
use crate::objeto_mapa::ResultadoRafaga::{EnemigoEliminado, Insignificante};
use crate::pared::Pared;
use crate::vacio::Vacio;

//? Laberinto es la estructura destinada a manejar el tablero del juego.
pub struct Laberinto{
    tablero: Vec<Vec<Box<dyn ObjetoMapa>>>,
}

impl Laberinto {
    //? Crea un laberinto lleno de Vacio y lo devuelve.
    pub fn new(dimension_tablero: usize) -> Laberinto {
        let mut tablero: Vec<Vec<Box<dyn ObjetoMapa>>> = Vec::new();

        for i in 0..dimension_tablero {
            let mut fila: Vec<Box<dyn ObjetoMapa>> = Vec::new();

            for j in 0..dimension_tablero {
                let vacio: Box<dyn ObjetoMapa> = Box::new(Vacio::new(Coordenada::new(i, j)));
                fila.push(vacio);
            }

            tablero.push(fila);
        }

        Laberinto { tablero }
    }

    //? Devuelve la visualizacion del estado actual del laberinto.
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



    //? Reemplaza el casillero ubicado en las coordenadas del casillero recibido por el casillero recibido.
    pub fn reemplazar_objeto_en_tablero(&mut self, objeto: Box<dyn ObjetoMapa>) {
        if self.coordenadas_fuera_de_rango(&objeto.get_coordenada_actual()) {
            return;
        }

        let x = objeto.get_coordenada_actual().get_x();
        let y = objeto.get_coordenada_actual().get_y();

        self.tablero[y][x] = objeto;
    }


    //? Devuelve true en caso de que las coordenadas recibidas esten fuera del tablero, false caso contrario.
    pub fn coordenadas_fuera_de_rango(&self, coordenada: &Coordenada) -> bool {
        coordenada.get_x() > self.tablero.len() || coordenada.get_y() > self.tablero.len()
    }
    
    fn obtener_objeto(tablero: &mut Vec<Vec<Box<dyn ObjetoMapa>>>, coordenada_a_detonar: Coordenada) -> &mut Box<dyn ObjetoMapa> {
        &mut tablero[coordenada_a_detonar.get_x()][coordenada_a_detonar.get_y()]
    }

    pub fn detonar_coordenadas(&mut self, coordenada_a_detonar: &Coordenada) -> Result<(), String> {
        if self.coordenadas_fuera_de_rango(coordenada_a_detonar) {
            return Err("No se puede detonar fuera del mapa!".to_string());
        }

        let objeto = Laberinto::obtener_objeto(&mut self.tablero, coordenada_a_detonar.clone());
        objeto.detonar(self)
    }


    /*pub fn detonar_coordenada(&mut self, coordenada_a_detonar: &Coordenada) -> Result<(), String> {
        if self.coordenadas_fuera_de_rango(coordenada_a_detonar) {
                return Err("No se puede detonar fuera del mapa!".to_string());
        }

        let resultado_detonacion = self.tablero[coordenada_a_detonar.get_x()][coordenada_a_detonar.get_y()].detonar(self);

        resultado_detonacion
     }


    pub fn vaciar_coordenada(&mut self, coordenada_a_vaciar: &Coordenada) {
        self.tablero[coordenada_a_vaciar.get_x()][coordenada_a_vaciar.get_y()] = CasilleroVacio(Vacio::new(Coordenada::new(coordenada_a_vaciar.get_x(), coordenada_a_vaciar.get_y(), )));
    }

    pub fn rafagear_coordenada(&mut self, coordenada_a_rafagear: &Coordenada) -> Result<ResultadoRafaga, String> {
        if self.coordenadas_fuera_de_rango(coordenada_a_rafagear) {
            return Ok(ResultadoRafaga::ChoqueFuerte);
        }

        let casillero: &mut Casillero = &mut self.tablero[coordenada_a_rafagear.get_x()][coordenada_a_rafagear.get_y()];
        let resultado_rafaga: Result<ResultadoRafaga, String>;

        match casillero {
            CasilleroVacio(vacio) => {
                resultado_rafaga = Ok(vacio.recibir_rafaga());
            }
            CasilleroRoca(roca) => {
                resultado_rafaga = Ok(roca.recibir_rafaga());
            }
            CasilleroPared(pared) => {
                resultado_rafaga = Ok(pared.recibir_rafaga());
            }
            CasilleroEnemigo(enemigo) => {
                let resultado_rafaga_sobre_enemigo = enemigo.recibir_rafaga();

                if let EnemigoEliminado = resultado_rafaga_sobre_enemigo {
                    self.vaciar_coordenada(&enemigo.get_coordenada_actual());
                }

                resultado_rafaga = Ok(resultado_rafaga_sobre_enemigo);
            }
            CasilleroDesvio(desvio) => {
                resultado_rafaga = Ok(desvio.recibir_rafaga());
            }
            CasilleroBombaNormal(bomba_normal) => {
                self.detonar_coordenada(&bomba_normal.get_coordenada_actual());
            }
            CasilleroBombaTraspaso(bomba_traspaso) => {
                self.detonar_coordenada(&bomba_traspaso.get_coordenada_actual());
            }
        }

        //resultado_rafaga
        Ok(Insignificante)
    }

    fn evaluar_camino_a_seguir_traspaso(&mut self, coordenada_inicial: &Coordenada, alcance_restante: i32, resultado_rafaga: ResultadoRafaga) -> Result<ResultadoRafaga, String> {
        match resultado_rafaga {
            ResultadoRafaga::DesvioArriba => self.rafagear_arriba_traspaso(coordenada_inicial, alcance_restante),
            ResultadoRafaga::DesvioAbajo => self.rafagear_abajo_traspaso(coordenada_inicial, alcance_restante),
            ResultadoRafaga::DesvioIzquierda => { self.rafagear_izquierda_traspaso(coordenada_inicial, alcance_restante) },
            ResultadoRafaga::DesvioDerecha => { self.rafagear_derecha_traspaso(coordenada_inicial, alcance_restante) },
            _ => { Err("Error al ejecutar desvio.".to_string()) }
        }
    }

    fn rafaga_no_choca_obstaculo(resultado_rafaga: &ResultadoRafaga) -> bool {
        !matches!(resultado_rafaga,ResultadoRafaga::DesvioArriba | ResultadoRafaga::DesvioAbajo | ResultadoRafaga::DesvioIzquierda | ResultadoRafaga::DesvioDerecha | crate::objeto_mapa::ResultadoRafaga::Choque | crate::objeto_mapa::ResultadoRafaga::ChoqueFuerte)
    }

    fn rafaga_continua_sin_chocar_obstaculo_traspaso(&self, alcance_restante: i32, resultado_rafaga: &ResultadoRafaga) -> bool {
        alcance_restante > 0 && Laberinto::rafaga_no_choca_obstaculo_traspaso(resultado_rafaga)
    }

    fn rafaga_no_choca_obstaculo_traspaso(resultado_rafaga: &ResultadoRafaga) -> bool {
        !matches!(resultado_rafaga,ResultadoRafaga::DesvioArriba | ResultadoRafaga::DesvioAbajo | ResultadoRafaga::DesvioIzquierda | ResultadoRafaga::DesvioDerecha | crate::objeto_mapa::ResultadoRafaga::ChoqueFuerte)
    }

    fn rafaga_continua_sin_chocar_obstaculo(&self, alcance_restante: i32, resultado_rafaga: &ResultadoRafaga) -> bool {
        alcance_restante > 0 && Laberinto::rafaga_no_choca_obstaculo(resultado_rafaga)
    }

    fn rafaga_continua_chocando_obstaculo(alcance_restante: i32, resultado_rafaga: &ResultadoRafaga) -> bool {
        alcance_restante > 0 && !Laberinto::rafaga_no_choca_obstaculo(resultado_rafaga)
    }

    fn rafaga_continua_chocando_obstaculo_traspaso(alcance_restante: i32, resultado_rafaga: &ResultadoRafaga) -> bool {
        alcance_restante > 0 && !Laberinto::rafaga_no_choca_obstaculo_traspaso(resultado_rafaga)
    }


    // ? ----------------------------------------- NO TRASPASO --------------------------------------------------------------------------------------------

    pub fn iniciar_rafagas(&mut self, casillero_inicial: &mut Casillero, alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        self.rafagear_arriba(&casillero_inicial.obtener_coordenada(), alcance_restante_rafagas.clone()) ?;
        self.rafagear_abajo(&casillero_inicial.obtener_coordenada(), alcance_restante_rafagas.clone()) ?;
        self.rafagear_derecha(&casillero_inicial.obtener_coordenada(), alcance_restante_rafagas.clone()) ?;
        self.rafagear_izquierda(&casillero_inicial.obtener_coordenada(), alcance_restante_rafagas.clone())
    }

    pub fn rafagear_arriba(&mut self, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);

        while self.rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &resultado_rafaga ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_y() + 1);
            alcance_restante_rafagas -= 1;

            resultado_rafaga = self.rafagear_coordenada(&coordenada_a_rafagear);
        }

        if Laberinto::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(&coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga
    }

    pub fn rafagear_abajo(&mut self, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);

        while self.rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &resultado_rafaga ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_y() - 1);
            alcance_restante_rafagas -= 1;

            resultado_rafaga = self.rafagear_coordenada(&coordenada_a_rafagear);
        }

        /*if Laberinto::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(&coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga*/

        Ok(Insignificante)
    }

    pub fn rafagear_derecha(&mut self, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);

        while self.rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &resultado_rafaga ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_x() + 1);
            alcance_restante_rafagas -= 1;

            resultado_rafaga = self.rafagear_coordenada(&coordenada_a_rafagear);
        }

        /*if Laberinto::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(&coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga*/

        Ok(Insignificante)
    }

    pub fn rafagear_izquierda(&mut self, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);

        while self.rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &resultado_rafaga ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_x() - 1);
            alcance_restante_rafagas -= 1;

            resultado_rafaga = self.rafagear_coordenada(&coordenada_a_rafagear);
        }

        /*if Laberinto::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(&coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga*/

        Ok(Insignificante)
    }

    fn evaluar_camino_a_seguir(&mut self, coordenada_inicial: &Coordenada, alcance_restante: i32, resultado_rafaga: ResultadoRafaga) -> Result<ResultadoRafaga, String> {
        match resultado_rafaga {
            ResultadoRafaga::DesvioArriba => self.rafagear_arriba(coordenada_inicial, alcance_restante),
            ResultadoRafaga::DesvioAbajo => self.rafagear_abajo(coordenada_inicial, alcance_restante),
            ResultadoRafaga::DesvioIzquierda => { self.rafagear_izquierda(coordenada_inicial, alcance_restante) },
            ResultadoRafaga::DesvioDerecha => { self.rafagear_derecha(coordenada_inicial, alcance_restante) },
            _ => { Err("Error al ejecutar el desvio".to_string())}
        }
    }

    // ? ----------------------------------------- TRASPASO --------------------------------------------------------------------------------------------

    pub fn iniciar_rafagas_traspaso(&mut self, casillero_inicial: &mut Casillero, alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        self.rafagear_arriba_traspaso(&casillero_inicial.obtener_coordenada(), alcance_restante_rafagas.clone()) ?;
        self.rafagear_abajo_traspaso(&casillero_inicial.obtener_coordenada(), alcance_restante_rafagas.clone()) ?;
        self.rafagear_derecha_traspaso(&casillero_inicial.obtener_coordenada(), alcance_restante_rafagas.clone()) ?;
        self.rafagear_izquierda_traspaso(&casillero_inicial.obtener_coordenada(), alcance_restante_rafagas.clone())
    }

    pub fn rafagear_arriba_traspaso(&mut self, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);

        while self.rafaga_continua_sin_chocar_obstaculo_traspaso(alcance_restante_rafagas, &resultado_rafaga ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_y() + 1);
            alcance_restante_rafagas -= 1;

            resultado_rafaga = self.rafagear_coordenada(&coordenada_a_rafagear);
        }

        /*if Laberinto::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(&coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga*/

        Ok(Insignificante)
    }

    pub fn rafagear_abajo_traspaso(&mut self, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);

        while self.rafaga_continua_sin_chocar_obstaculo_traspaso(alcance_restante_rafagas, &resultado_rafaga ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_y() - 1);
            alcance_restante_rafagas -= 1;

            resultado_rafaga = self.rafagear_coordenada(&coordenada_a_rafagear);
        }

        /*if Laberinto::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(&coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga*/

        Ok(Insignificante)
    }

    pub fn rafagear_derecha_traspaso(&mut self, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);

        while self.rafaga_continua_sin_chocar_obstaculo_traspaso(alcance_restante_rafagas, &resultado_rafaga ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_x() + 1);
            alcance_restante_rafagas -= 1;

            resultado_rafaga = self.rafagear_coordenada(&coordenada_a_rafagear);
        }

        /*if Laberinto::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(&coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga*/

        Ok(Insignificante)
    }

    pub fn rafagear_izquierda_traspaso(&mut self, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);

        while self.rafaga_continua_sin_chocar_obstaculo_traspaso(alcance_restante_rafagas, &resultado_rafaga ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_x() - 1);
            alcance_restante_rafagas -= 1;

            resultado_rafaga = self.rafagear_coordenada(&coordenada_a_rafagear);
        }

        /*if Laberinto::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(&coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga*/

        Ok(Insignificante)
    }*/
}

/*

#[cfg(test)]
mod tests {
    use crate::coordenada::Coordenada;
    use crate::laberinto::Laberinto;

    #[test]
    fn test_chequear_coordenadas_fuera_de_rango() {
        let laberinto = Laberinto::new(3);
        let coordenada = Coordenada::new(4, 3);
        assert!(laberinto.coordenadas_fuera_de_rango(&coordenada));
    }

    #[test]
    fn test_chequear_coordenadas_dentro_de_rango() {
        let laberinto = Laberinto::new(3);
        let coordenada = Coordenada::new(2, 2);
        assert!(!laberinto.coordenadas_fuera_de_rango(&coordenada));
    }
}

 */