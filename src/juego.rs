use crate::bomba_normal::BombaNormal;
use crate::bomba_traspaso::BombaTraspaso;
use crate::casillero::Casillero;
use crate::casillero::Casillero::{
    CasilleroBombaNormal, CasilleroBombaTraspaso, CasilleroDesvio, CasilleroEnemigo,
    CasilleroPared, CasilleroRoca, CasilleroVacio,
};
use crate::coordenada::Coordenada;
use crate::desvio::Desvio;
use crate::enemigo::Enemigo;
use crate::laberinto::Laberinto;
use crate::objeto_mapa::{ResultadoRafaga};
use crate::pared::Pared;
use crate::roca::Roca;
use crate::vacio::Vacio;

const UN_CARACTER: usize = 1;
const ASCII_DIF: i32 = 48;

pub struct Juego {
    laberinto: Laberinto,
}

impl Juego {
    pub fn new(dimension_tablero: usize) -> Juego {
        Juego {
            laberinto: Laberinto::new(dimension_tablero),
        }
    }

    fn crear_objeto_un_caracter(&mut self, caracter: &str, coordenada_objeto: Coordenada) -> Result<Casillero, String> {
        let mut result: Result<Casillero, String> =
            Err("Caracter representado no valido".to_string());

        if caracter == "_" {
            result = Ok(CasilleroVacio(Vacio::new(coordenada_objeto)));
        } else if caracter == "R" {
            result = Ok(CasilleroRoca(Roca::new(coordenada_objeto)));
        } else if caracter == "W" {
            result = Ok(CasilleroPared(Pared::new(coordenada_objeto)));
        };

        result
    }

    fn crear_objeto_dos_caracteres(&mut self, parte: &str, coordenada_objeto: Coordenada) -> Result<Casillero, String> {
        let mut result: Result<Casillero, String> = Err("Caracter representado no valido".to_string());
        let segundo_caracter = parte.as_bytes()[1];

        if let Some(primer_caracter) = parte.chars().next() {
            if primer_caracter == 'B' {
                if segundo_caracter as i32  - ASCII_DIF < 1 {
                    return Err("Error: El alcance de la bomba no puede ser menor a 1".to_string());
                }

                result = Ok(CasilleroBombaNormal(BombaNormal::new(coordenada_objeto, segundo_caracter as i32  - ASCII_DIF)));
            } else if primer_caracter == 'S' {
                if segundo_caracter as i32  - ASCII_DIF < 1 {
                    return Err("Error: El alcance de la bomba traspaso no puede ser menor a 1".to_string());
                }

                result = Ok(CasilleroBombaTraspaso(BombaTraspaso::new(coordenada_objeto, segundo_caracter as i32  - ASCII_DIF)));
            } else if primer_caracter == 'F' {
                if (segundo_caracter as i32  - ASCII_DIF < 1) || (segundo_caracter as i32  - ASCII_DIF > 3) {
                    return Err("Error: La vida del enemigo no puede ser menor a 1 ni mayor a 3".to_string());
                }

                result = Ok(CasilleroEnemigo(Enemigo::new(coordenada_objeto, segundo_caracter as i32 - ASCII_DIF)));
            } else if primer_caracter == 'D' {
                result = Ok(CasilleroDesvio(Desvio::new(coordenada_objeto, segundo_caracter.to_string())));
            }
        };

        result
    }

    pub fn inicializar_laberinto_con_datos(&mut self, datos: Vec<String>) -> Result<(), String> {
        let mut casillero: Casillero;
        let mut coordenada_casillero: Coordenada;


        for (coordenada_y, dato) in datos.iter().enumerate() {
            let partes = dato.split_whitespace().collect::<Vec<&str>>();

            for(coordenada_x, parte) in partes.iter().enumerate() {
                coordenada_casillero = Coordenada::new(coordenada_x, coordenada_y);

                if parte.len() == UN_CARACTER {
                    casillero = self.crear_objeto_un_caracter(parte, coordenada_casillero)?;
                } else {
                    casillero = self.crear_objeto_dos_caracteres(parte, coordenada_casillero)?;
                }

                self.laberinto.reemplazar_casillero_en_tablero(casillero);
            }
        }

        Ok(())
    }

    pub fn detonar_coordenada(&mut self, coordenada_a_detonar: &Coordenada) -> Result<(), String> {
        //self.laberinto.detonar_coordenada(coordenada_a_detonar)
        Ok(())
    }

    pub fn obtener_visualizacion(&self) -> Vec<Vec<String>> {
        self.laberinto.obtener_visualizacion()
    }
}
