use crate::casillero::{Casillero};
use crate::resultado_rafaga::ResultadoRafaga::{Detonacion, EnemigoEliminado, EnemigoTocado};
use crate::coordenada::Coordenada;
use crate::enemigo::Enemigo;
use crate::resultado_rafaga::ResultadoRafaga;

const UN_CARACTER: usize = 1;
const ASCII_DIF: i32 = 48;

//? Laberinto es la estructura destinada a manejar el tablero del juego.
pub struct Laberinto{
    tablero: Vec<Vec<Casillero>>,
}

impl Laberinto {
    //? Crea un laberinto lleno de Vacio y lo devuelve.
    pub fn new(dimension_tablero: usize) -> Laberinto {
        let mut tablero: Vec<Vec<Casillero>> = Vec::new();

        for i in 0..dimension_tablero {
            let mut fila: Vec<Casillero> = Vec::new();

            for j in 0..dimension_tablero {
                fila.push(Casillero::Vacio(Coordenada::new(i, j)));
            }

            tablero.push(fila);
        }

        Laberinto { tablero }
    }

    //? Crea objetos representados unicamente por un unico caracter.
    fn crear_objeto_un_caracter(&mut self, caracter: &str, coordenada_objeto: Coordenada) -> Result<Casillero, String> {
        let mut result: Result<Casillero, String> = Err("Caracter representado no valido".to_string());

        if caracter == "_" {
            result = Ok(Casillero::Vacio(coordenada_objeto));
        } else if caracter == "R" {
            result = Ok(Casillero::Roca(coordenada_objeto));
        } else if caracter == "W" {
            result = Ok(Casillero::Pared(coordenada_objeto));
        };

        result
    }

    //? Crea objetos representados únicamente por dos caracteres.
    fn crear_objeto_dos_caracteres(&mut self, parte: &str, coordenada_objeto: Coordenada) -> Result<Casillero, String> {
        let mut result: Result<Casillero, String> = Err("Caracter representado no valido".to_string());
        let segundo_caracter = parte.as_bytes()[1];

        if let Some(primer_caracter) = parte.chars().next() {
            if primer_caracter == 'B' {
                if segundo_caracter as i32  - ASCII_DIF < 1 {
                    return Err("Error: El alcance de la bomba no puede ser menor a 1".to_string());
                }

                result = Ok(Casillero::BombaNormal(coordenada_objeto, segundo_caracter as i32  - ASCII_DIF));
            } else if primer_caracter == 'S' {
                if segundo_caracter as i32  - ASCII_DIF < 1 {
                    return Err("Error: El alcance de la bomba traspaso no puede ser menor a 1".to_string());
                }

                result = Ok(Casillero::BombaTraspaso(coordenada_objeto, segundo_caracter as i32  - ASCII_DIF));
            } else if primer_caracter == 'F' {
                if (segundo_caracter as i32  - ASCII_DIF < 1) || (segundo_caracter as i32  - ASCII_DIF > 3) {
                    return Err("Error: La vida del enemigo no puede ser menor a 1 ni mayor a 3".to_string());
                }

                let enemigo = Enemigo::new(segundo_caracter as i32  - ASCII_DIF);

                result = Ok(Casillero::Enemigoo(coordenada_objeto, enemigo));
            } else if primer_caracter == 'D' {
                result = Ok(Casillero::Desvio(coordenada_objeto, segundo_caracter.to_string()));
            }
        };

        result
    }

    //? Recibe un vector de strings, donde cada string representa una fila del laberinto y cada caracter representa un objeto.
    //? A partir de estos datos, actualiza el tablero.
    pub fn inicializar_laberinto_con_datos(&mut self, datos: Vec<String>) -> Result<(), String> {
        let mut objeto: Casillero;
        let mut coordenada_casillero: Coordenada;
        let mut coordenada_casillero_copia: Coordenada;

        for (coordenada_y, dato) in datos.iter().enumerate() {
            let partes = dato.split_whitespace().collect::<Vec<&str>>();

            for(coordenada_x, parte) in partes.iter().enumerate() {
                coordenada_casillero = Coordenada::new(coordenada_x, coordenada_y);
                coordenada_casillero_copia = coordenada_casillero.clone();

                if parte.len() == UN_CARACTER {
                    objeto = self.crear_objeto_un_caracter(parte, coordenada_casillero)?;
                } else {
                    objeto = self.crear_objeto_dos_caracteres(parte, coordenada_casillero)?;
                }

                self.reemplazar_objeto_en_tablero(objeto, coordenada_casillero_copia);
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
    pub fn reemplazar_objeto_en_tablero(&mut self, casillero: Casillero, coordenada: Coordenada) {
        if self.coordenadas_fuera_de_rango(&coordenada) {
            return;
        }

        self.tablero[coordenada.get_y()][coordenada.get_x()] = casillero;
    }

    //? Devuelve true en caso de que las coordenadas recibidas esten fuera del tablero, false caso contrario.
    pub fn coordenadas_fuera_de_rango(&self, coordenada: &Coordenada) -> bool {
        (coordenada.get_x() >= self.tablero.len() || coordenada.get_y() >= self.tablero.len()) || (coordenada.get_x() < 0 || coordenada.get_y() < 0)
    }

    //? Devuelve el objeto ubicado en las coordenadas recibidas.
    fn obtener_objeto(&mut self, coordenada_buscada: Coordenada) -> Casillero {
        self.tablero[coordenada_buscada.get_y()][coordenada_buscada.get_x()].clone()
    }

    //? Detonar el objeto ubicado en las coordenadas recibidas. Devuelve un error en caso de que no se pueda detonar.
    pub fn detonar_objeto(&mut self, coordenada_a_detonar: Coordenada) -> Result<ResultadoRafaga, String> {
        if self.coordenadas_fuera_de_rango(&coordenada_a_detonar) {
            return Err("No se puede detonar fuera del mapa!".to_string());
        }

        let objeto = self.obtener_objeto(coordenada_a_detonar.clone());
        objeto.detonar(self)
    }

    //? Ordena al objeto correspondiente que reciba la rafaga, aplica las consecuencias y devuelve el resultado.
    pub fn rafagear_coordenada(&mut self, coordenada_a_rafagear: &Coordenada) -> Result<ResultadoRafaga, String> {
        if self.coordenadas_fuera_de_rango(coordenada_a_rafagear) {
            return Ok(ResultadoRafaga::ChoqueFuerte);
        }

        let mut resultado_rafaga = Ok(self.tablero[coordenada_a_rafagear.get_y()][coordenada_a_rafagear.get_x()].recibir_rafaga());

        if resultado_rafaga.clone()? == EnemigoEliminado {
            self.reemplazar_objeto_en_tablero(Casillero::Vacio(coordenada_a_rafagear.clone()), coordenada_a_rafagear.clone());
        } else if resultado_rafaga.clone()? == Detonacion {
            resultado_rafaga = self.detonar_objeto(coordenada_a_rafagear.clone());
        } else if resultado_rafaga.clone()? == EnemigoTocado(1) || resultado_rafaga.clone()? == EnemigoTocado(2){
            let enemigo_nuevo = Enemigo::new(resultado_rafaga.clone()?.get_vida_enemigo());
            self.reemplazar_objeto_en_tablero(Casillero::Enemigoo(coordenada_a_rafagear.clone(), enemigo_nuevo), coordenada_a_rafagear.clone());
        }

        resultado_rafaga
    }


























    // * ------------------------------------------------- BASURA! -----------------------------------------------------


/*    pub fn detonar_objeto(&mut self, coordenada_a_detonar: Coordenada) -> Result<(), String> {
        if self.coordenadas_fuera_de_rango(&coordenada_a_detonar) {
            return Err("No se puede detonar fuera del mapa!".to_string());
        }
        
        let objeto = self.obtener_objeto(coordenada_a_detonar.clone());
        objeto.detonar(self)?;
        /*
        //Supongamos que si es una bomba devuelve un valor particular, tipo Ok()...
        match objeto.detonar(){
            Ok(()) => {
                drop(objeto);
                self.reemplazar_objeto_en_tablero(Box::new(Vacio::new(coordenada_a_detonar)));
            },
            Err(error) => {
                return Err(error);
            }
        }
        
        //let mut vector: Vec<Desvio> = vec![Desvio::new(coordenada_a_detonar.clone(), "R".to_string()),Desvio::new(coordenada_a_detonar.clone(), "R".to_string()),Desvio::new(coordenada_a_detonar.clone(), "R".to_string())];
        //let hola = &mut vector[1];
        //let hola2 = &mut vector[2]

        //1. Tener un tablero de casillas de objetos en vez de un tablero de objetos.
        //2. Que los objetos que tienen que interactuar con el laberinto tengan al laberinto como campo en su struct.
        //3. Que el algoritmo de simular la explosion este en Laberinto. Entonces al hacer "objeto.Detonar()", esto devuelve un resultado
        //Dependiendo del resultado devuelto, se ejecuta el algoritmo de simular la explosion o no. Pero aca se vuelve a generar el mismo issue de compilacion.
        */
        Ok(())
    }*/

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

