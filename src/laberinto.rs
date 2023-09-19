use crate::casillero::Casillero;
use crate::coordenada::Coordenada;
use crate::enemigo::Enemigo;
use crate::resultado_rafaga::ResultadoRafaga;
use crate::resultado_rafaga::ResultadoRafaga::{Detonacion, EnemigoEliminado, EnemigoTocado};

const UN_CARACTER: usize = 1;
const ASCII_DIF: i32 = 48;

//? Laberinto es la estructura destinada a manejar el tablero del juego.
pub struct Laberinto {
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
    fn crear_objeto_un_caracter(
        &mut self,
        caracter: &str,
        coordenada_objeto: Coordenada,
    ) -> Result<Casillero, String> {
        let mut result: Result<Casillero, String> =
            Err("Caracter representado no valido".to_string());

        if caracter == "_" {
            result = Ok(Casillero::Vacio(coordenada_objeto));
        } else if caracter == "R" {
            result = Ok(Casillero::Roca(coordenada_objeto));
        } else if caracter == "W" {
            result = Ok(Casillero::Pared(coordenada_objeto));
        };

        result
    }

    //? Crea una bomba normal a partir del segundo caracter recibido, o devuelve error.
    fn crear_bomba_normal(
        segundo_caracter: u8,
        coordenada_objeto: Coordenada,
    ) -> Result<Casillero, String> {
        if (segundo_caracter as i32 - ASCII_DIF < 1) || (segundo_caracter as i32 - ASCII_DIF > 9) {
            return Err("Error: El alcance de la bomba no puede ser menor a 1".to_string());
        }

        Ok(Casillero::BombaNormal(
            coordenada_objeto,
            segundo_caracter as i32 - ASCII_DIF,
        ))
    }

    //? Crea una bomba traspaso a partir del segundo caracter recibido, o devuelve error.
    fn crear_bomba_traspaso(
        segundo_caracter: u8,
        coordenada_objeto: Coordenada,
    ) -> Result<Casillero, String> {
        if (segundo_caracter as i32 - ASCII_DIF < 1) || (segundo_caracter as i32 - ASCII_DIF > 9) {
            return Err(
                "Error: El alcance de la bomba traspaso no puede ser menor a 1".to_string(),
            );
        }

        Ok(Casillero::BombaTraspaso(
            coordenada_objeto,
            segundo_caracter as i32 - ASCII_DIF,
        ))
    }

    //? Crea un enemigo a partir del segundo caracter recibido, o devuelve error.
    fn crear_enemigo(
        segundo_caracter: u8,
        coordenada_objeto: Coordenada,
    ) -> Result<Casillero, String> {
        if (segundo_caracter as i32 - ASCII_DIF < 1) || (segundo_caracter as i32 - ASCII_DIF > 3) {
            return Err(
                "Error: La vida del enemigo no puede ser menor a 1 ni mayor a 3".to_string(),
            );
        }

        let enemigo = Enemigo::new(segundo_caracter as i32 - ASCII_DIF);

        Ok(Casillero::Enemigoo(coordenada_objeto, enemigo))
    }

    //? Crea un desvio a partir del segundo caracter recibido, o devuelve error.
    fn crear_desvio(
        segundo_caracter: u8,
        coordenada_objeto: Coordenada,
    ) -> Result<Casillero, String> {
        let direccion = String::from(segundo_caracter as char);
        if direccion != "U" && direccion != "D" && direccion != "L" && direccion != "R" {
            return Err("Error: La direccion del desvio no es valida".to_string());
        }

        Ok(Casillero::Desvio(coordenada_objeto, direccion))
    }

    //? Crea objetos representados únicamente por dos caracteres.
    fn crear_objeto_dos_caracteres(
        &mut self,
        parte: &str,
        coordenada_objeto: Coordenada,
    ) -> Result<Casillero, String> {
        let mut result: Result<Casillero, String> =
            Err("Caracter representado no valido".to_string());
        let segundo_caracter = parte.as_bytes()[1];

        if let Some(primer_caracter) = parte.chars().next() {
            if primer_caracter == 'B' {
                result = Laberinto::crear_bomba_normal(segundo_caracter, coordenada_objeto)
            } else if primer_caracter == 'S' {
                result = Laberinto::crear_bomba_traspaso(segundo_caracter, coordenada_objeto)
            } else if primer_caracter == 'F' {
                result = Laberinto::crear_enemigo(segundo_caracter, coordenada_objeto)
            } else if primer_caracter == 'D' {
                result = Laberinto::crear_desvio(segundo_caracter, coordenada_objeto)
            }
        };

        result
    }

    //? Crea  el objeto correspondiente y lo agrega al mapa.
    pub fn crear_objeto_correspondiente(
        &mut self,
        parte: &str,
        coordenada_x: usize,
        coordenada_y: usize,
    ) -> Result<(), String> {
        let objeto: Casillero;
        let coordenada_casillero = Coordenada::new(coordenada_x, coordenada_y);
        let coordenada_casillero_copia = coordenada_casillero.clone();

        if parte.len() == UN_CARACTER {
            objeto = self.crear_objeto_un_caracter(parte, coordenada_casillero)?;
        } else {
            objeto = self.crear_objeto_dos_caracteres(parte, coordenada_casillero)?;
        }

        self.reemplazar_objeto_en_tablero(objeto.clone(), coordenada_casillero_copia);

        Ok(())
    }

    //? Recibe un vector de strings, donde cada string representa una fila del laberinto y cada caracter representa un objeto.
    //? A partir de estos datos, actualiza el tablero.
    pub fn inicializar_laberinto_con_datos(&mut self, datos: Vec<String>) -> Result<(), String> {
        for (coordenada_y, dato) in datos.iter().enumerate() {
            let partes = dato.split_whitespace().collect::<Vec<&str>>();

            for (coordenada_x, parte) in partes.iter().enumerate() {
                self.crear_objeto_correspondiente(parte, coordenada_x, coordenada_y)?;
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
        coordenada.get_x() >= self.tablero.len() || coordenada.get_y() >= self.tablero.len()
    }

    //? Devuelve el objeto ubicado en las coordenadas recibidas.
    fn obtener_objeto(&mut self, coordenada_buscada: Coordenada) -> Casillero {
        self.tablero[coordenada_buscada.get_y()][coordenada_buscada.get_x()].clone()
    }

    //? Detonar el objeto ubicado en las coordenadas recibidas. Devuelve un error en caso de que no se pueda detonar.
    pub fn detonar_objeto(
        &mut self,
        coordenada_a_detonar: Coordenada,
    ) -> Result<ResultadoRafaga, String> {
        if self.coordenadas_fuera_de_rango(&coordenada_a_detonar) {
            return Err("No se puede detonar fuera del mapa!".to_string());
        }

        let objeto = self.obtener_objeto(coordenada_a_detonar.clone());
        objeto.detonar(self)
    }

    //? Ordena al objeto correspondiente que reciba la rafaga, aplica las consecuencias y devuelve el resultado.
    pub fn rafagear_coordenada(
        &mut self,
        coordenada_a_rafagear: &Coordenada,
    ) -> Result<ResultadoRafaga, String> {
        if self.coordenadas_fuera_de_rango(coordenada_a_rafagear) {
            return Ok(ResultadoRafaga::ChoqueFuerte);
        }

        let mut resultado_rafaga = Ok(self.tablero[coordenada_a_rafagear.get_y()]
            [coordenada_a_rafagear.get_x()]
        .recibir_rafaga());

        if resultado_rafaga.clone()? == EnemigoEliminado {
            self.reemplazar_objeto_en_tablero(
                Casillero::Vacio(coordenada_a_rafagear.clone()),
                coordenada_a_rafagear.clone(),
            );
        } else if resultado_rafaga.clone()? == Detonacion {
            resultado_rafaga = self.detonar_objeto(coordenada_a_rafagear.clone());
        } else if resultado_rafaga.clone()? == EnemigoTocado(1)
            || resultado_rafaga.clone()? == EnemigoTocado(2)
        {
            let enemigo_nuevo = Enemigo::new(resultado_rafaga.clone()?.get_vida_enemigo());
            self.reemplazar_objeto_en_tablero(
                Casillero::Enemigoo(coordenada_a_rafagear.clone(), enemigo_nuevo),
                coordenada_a_rafagear.clone(),
            );
        }

        resultado_rafaga
    }
}

#[cfg(test)]
mod tests {
    use crate::coordenada::Coordenada;
    use crate::laberinto::Laberinto;
    use crate::resultado_rafaga::ResultadoRafaga;

    #[test]
    fn test_detonar() {
        let mut laberinto = Laberinto::new(3);
        let bomba = Laberinto::crear_bomba_normal(49, Coordenada::new(1, 1)).unwrap();
        laberinto.reemplazar_objeto_en_tablero(bomba, Coordenada::new(1, 1));
        let resultado_detonacion = laberinto.detonar_objeto(Coordenada::new(1, 1));
        assert!(resultado_detonacion.unwrap() == ResultadoRafaga::Insignificante);
    }

    #[test]
    fn test_rafagear() {
        let mut laberinto = Laberinto::new(3);
        let bomba = Laberinto::crear_enemigo(51, Coordenada::new(1, 1)).unwrap();
        laberinto.reemplazar_objeto_en_tablero(bomba, Coordenada::new(1, 1));
        let resultado_rafaga = laberinto.rafagear_coordenada(&Coordenada::new(1, 1));
        assert!(resultado_rafaga.unwrap() == ResultadoRafaga::EnemigoTocado(2));
    }

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
        let datos = vec![
            "R _ _".to_string(),
            "_ W _".to_string(),
            "_ _ _".to_string(),
        ];
        assert!(lab.inicializar_laberinto_con_datos(datos).is_ok());

        let mut lab2 = Laberinto::new(3);
        let datos = vec![
            "R _ _".to_string(),
            "_ Z _".to_string(),
            "_ _ _".to_string(),
        ];
        assert!(lab2.inicializar_laberinto_con_datos(datos).is_err());
    }

    #[test]
    fn test_chequear_visualizacion_datos() {
        let mut lab = Laberinto::new(3);
        let datos = vec![
            "R _ _".to_string(),
            "_ W _".to_string(),
            "_ _ _".to_string(),
        ];
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
        let casillero = lab.crear_objeto_un_caracter(caracter, Coordenada::new(2, 2));
        let es_error = casillero.is_err();
        assert!(es_error);
    }
}
