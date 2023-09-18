use crate::bomba_normal::BombaNormal;
use crate::bomba_traspaso::BombaTraspaso;
use crate::coordenada::Coordenada;
use crate::desvio::Desvio;
use crate::enemigo::Enemigo;
use crate::objeto_mapa::{ObjetoMapa};
use crate::pared::Pared;
use crate::roca::Roca;
use crate::vacio::Vacio;

const UN_CARACTER: usize = 1;
const ASCII_DIF: i32 = 48;

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

    //? Crea objetos representados unicamente por un unico caracter.
    fn crear_objeto_un_caracter(&mut self, caracter: &str, coordenada_objeto: Coordenada) -> Result<Box<dyn ObjetoMapa>, String> {
        let mut result: Result<Box<dyn ObjetoMapa>, String> = Err("Caracter representado no valido".to_string());

        if caracter == "_" {
            result = Ok(Box::new(Vacio::new(coordenada_objeto)));
        } else if caracter == "R" {
            result = Ok(Box::new(Roca::new(coordenada_objeto)));
        } else if caracter == "W" {
            result = Ok(Box::new(Pared::new(coordenada_objeto)));
        };

        result
    }

    //? Crea objetos representados unicamente por dos caracteres.
    fn crear_objeto_dos_caracteres(&mut self, parte: &str, coordenada_objeto: Coordenada) -> Result<Box<dyn ObjetoMapa>, String> {
        let mut result: Result<Box<dyn ObjetoMapa>, String> = Err("Caracter representado no valido".to_string());
        let segundo_caracter = parte.as_bytes()[1];

        if let Some(primer_caracter) = parte.chars().next() {
            if primer_caracter == 'B' {
                if segundo_caracter as i32  - ASCII_DIF < 1 {
                    return Err("Error: El alcance de la bomba no puede ser menor a 1".to_string());
                }

                result = Ok(Box::new(BombaNormal::new(coordenada_objeto, segundo_caracter as i32  - ASCII_DIF)));
            } else if primer_caracter == 'S' {
                if segundo_caracter as i32  - ASCII_DIF < 1 {
                    return Err("Error: El alcance de la bomba traspaso no puede ser menor a 1".to_string());
                }

                result = Ok(Box::new(BombaTraspaso::new(coordenada_objeto, segundo_caracter as i32  - ASCII_DIF)));
            } else if primer_caracter == 'F' {
                if (segundo_caracter as i32  - ASCII_DIF < 1) || (segundo_caracter as i32  - ASCII_DIF > 3) {
                    return Err("Error: La vida del enemigo no puede ser menor a 1 ni mayor a 3".to_string());
                }

                result = Ok(Box::new(Enemigo::new(coordenada_objeto, segundo_caracter as i32 - ASCII_DIF)));
            } else if primer_caracter == 'D' {
                result = Ok(Box::new(Desvio::new(coordenada_objeto, segundo_caracter.to_string())));
            }
        };

        result
    }

    //? Recibe un vector de strings, donde cada string representa una fila del laberinto y cada caracter representa un objeto.
    //? A partir de estos datos, actualiza el tablero.
    pub fn inicializar_laberinto_con_datos(&mut self, datos: Vec<String>) -> Result<(), String> {
        let mut objeto: Box<dyn ObjetoMapa>;
        let mut coordenada_casillero: Coordenada;

        for (coordenada_y, dato) in datos.iter().enumerate() {
            let partes = dato.split_whitespace().collect::<Vec<&str>>();

            for(coordenada_x, parte) in partes.iter().enumerate() {
                coordenada_casillero = Coordenada::new(coordenada_x, coordenada_y);

                if parte.len() == UN_CARACTER {
                    objeto = self.crear_objeto_un_caracter(parte, coordenada_casillero)?;
                } else {
                    objeto = self.crear_objeto_dos_caracteres(parte, coordenada_casillero)?;
                }

                self.reemplazar_objeto_en_tablero(objeto);
            }
        }

        Ok(())
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

    //? Devuelve el objeto ubicado en las coordenadas recibidas.
    fn obtener_objeto(&mut self, coordenada_a_detonar: Coordenada) -> &mut Box<dyn ObjetoMapa> {
        &mut self.tablero[coordenada_a_detonar.get_x()][coordenada_a_detonar.get_y()]
    }

    //? Detonar el objeto ubicado en las coordenadas recibidas. Devuelve un error en caso de que no se pueda detonar.
    pub fn detonar_objeto(&mut self, coordenada_a_detonar: Coordenada) -> Result<(), String> {
        if self.coordenadas_fuera_de_rango(&coordenada_a_detonar) {
            return Err("No se puede detonar fuera del mapa!".to_string());
        }

        let objeto = self.obtener_objeto(coordenada_a_detonar);
        //objeto.detonar(self);

        //1. Tener un tablero de casillas de objetos en vez de un tablero de objetos.
        //2. Que los objetos que tienen que interactuar con el laberinto tengan al laberinto como campo en su struct.
        //3. Que el algoritmo de simular la explosion este en Laberinto. Entonces al hacer "objeto.Detonar()", esto devuelve un resultado
        //Dependiendo del resultado devuelto, se ejecuta el algoritmo de simular la explosion o no. Pero aca se vuelve a generar el mismo issue de compilacion.

        Ok(())
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

    #[test]
    fn test_chequear_inicializacion_con_datos() {
        let mut lab = Laberinto::new(3);
        let datos = vec!["R _ _".to_string(), "_ W _".to_string(), "_ _ _".to_string()];
        assert!(lab.inicializar_laberinto_con_datos(datos).is_ok());

        let mut lab2 = Laberinto::new(3);
        let datos = vec!["R _ _".to_string(), "_ Z _".to_string(), "_ _ _".to_string()];
        assert!(lab2.inicializar_laberinto_con_datos(datos).is_err());
    }

    #[test]
    fn test_chequear_visualizacion_datos() {
        let mut lab = Laberinto::new(3);
        let datos = vec!["R _ _".to_string(), "_ W _".to_string(), "_ _ _".to_string()];
        lab.inicializar_laberinto_con_datos(datos).unwrap();
        let visualizacion = lab.obtener_visualizacion();
        assert_eq!(visualizacion[0][0], "R");
        assert_eq!(visualizacion[0][2], "_");
        assert_eq!(visualizacion[1][2], "W");
    }

    #[test]
    fn test_crear_objeto_un_caracter() {
        let mut lab = Laberinto::new(3);
        let caracter = "P";
        let casillero = lab.crear_objeto_un_caracter(caracter, Coordenada::new(2,2));
        let es_error = casillero.is_err();
        assert!(es_error);
    }
}

