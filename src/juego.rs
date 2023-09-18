use crate::bomba_normal::BombaNormal;
use crate::bomba_traspaso::BombaTraspaso;
use crate::coordenada::Coordenada;
use crate::desvio::Desvio;
use crate::enemigo::Enemigo;
use crate::laberinto::Laberinto;
use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};
use crate::pared::Pared;
use crate::roca::Roca;
use crate::vacio::Vacio;

const UN_CARACTER: usize = 1;
const ASCII_DIF: i32 = 48;

//? Juego es el encargado de manejar el juego en generar, el laberinto y los objetos que se encuentran en el mismo.
pub struct Juego {
    laberinto: Laberinto,
}

impl Juego {
    //? Crea un juego inicializado con el laberinto lleno de Vacio y lo devuelve. El laberinto sera cuadrado, de las dimensiones recibidas.
    pub fn new(dimension_tablero: usize) -> Juego {
        Juego {
            laberinto: Laberinto::new(dimension_tablero),
        }
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
    //? A partir de estos datos, cambia el estado del laberinto.
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

                self.laberinto.reemplazar_objeto_en_tablero(objeto);
            }
        }

        Ok(())
    }

    //? Recibe una coordenada del tablero y ordena al laberinto que la detone.
    pub fn detonar_coordenada(&mut self, coordenada_a_detonar: &Coordenada) -> Result<(), String> {
        //self.laberinto.detonar_coordenadas(coordenada_a_detonar)
        Ok(())
    }

    //? Devuelve la visualizacion del laberinto.
    pub fn obtener_visualizacion(&self) -> Vec<Vec<String>> {
        self.laberinto.obtener_visualizacion()
    }
}

#[cfg(test)]
mod tests {
    use crate::coordenada::Coordenada;
    use crate::juego::Juego;

    #[test]
    fn test_chequear_inicializacion_con_datos() {
        let mut juego = Juego::new(3);
        let datos = vec!["R _ _".to_string(), "_ W _".to_string(), "_ _ _".to_string()];
        assert!(juego.inicializar_laberinto_con_datos(datos).is_ok());

        let mut juego2 = Juego::new(3);
        let datos = vec!["R _ _".to_string(), "_ Z _".to_string(), "_ _ _".to_string()];
        assert!(juego2.inicializar_laberinto_con_datos(datos).is_err());
    }

    #[test]
    fn test_chequear_visualizacion_datos() {
        let mut juego = Juego::new(3);
        let datos = vec!["R _ _".to_string(), "_ W _".to_string(), "_ _ _".to_string()];
        juego.inicializar_laberinto_con_datos(datos).unwrap();
        let visualizacion = juego.obtener_visualizacion();
        assert_eq!(visualizacion[0][0], "R");
        assert_eq!(visualizacion[0][2], "_");
        assert_eq!(visualizacion[1][2], "W");
    }

    #[test]
    fn test_crear_objeto_un_caracter() {
        let mut juego = Juego::new(3);
        let caracter = "P";
        let casillero = juego.crear_objeto_un_caracter(caracter, Coordenada::new(2,2));
        let es_error = casillero.is_err();
        assert!(es_error);
    }
}