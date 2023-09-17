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
use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};
use crate::pared::Pared;
use crate::roca::Roca;
use crate::vacio::Vacio;

const UN_CARACTER: usize = 1;

pub struct Juego<'a> {
    laberinto: Laberinto<'a>,
}

impl<'a> Juego<'a> {
    pub fn new(dimension_tablero: usize) -> Juego<'a> {
        Juego {
            laberinto: Laberinto::new(dimension_tablero),
        }
    }

    fn crear_objeto_un_caracter(
        &mut self,
        caracter: &str,
        coordenada_objeto: Coordenada,
    ) -> Result<Casillero<'a>, String> {
        let mut result: Result<Casillero<'a>, String> =
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

    fn crear_objeto_dos_caracteres(
        &mut self,
        parte: &str,
        coordenada_objeto: Coordenada,
    ) -> Result<Casillero<'a>, String> {
        let mut result: Result<Casillero<'a>, String> =
            Err("Caracter representado no valido".to_string());
        let mut segundo_caracter = parte.as_bytes()[1];

        if let Some(primer_caracter) = parte.chars().next() {
            if primer_caracter == 'B' {
                result = Ok(CasilleroBombaNormal(BombaNormal::new(
                    coordenada_objeto,
                    self,
                    segundo_caracter as i32,
                )));
            } else if primer_caracter == 'S' {
                result = Ok(CasilleroBombaTraspaso(BombaTraspaso::new(
                    coordenada_objeto,
                    self,
                    segundo_caracter as i32,
                )));
            } else if primer_caracter == 'F' {
                result = Ok(CasilleroEnemigo(Enemigo::new(
                    coordenada_objeto,
                    self,
                    segundo_caracter as i32,
                )));
            } else if primer_caracter == 'D' {
                result = Ok(CasilleroDesvio(Desvio::new(
                    coordenada_objeto,
                    segundo_caracter.to_string(),
                )));
            }
        };

        result
    }

    pub fn inicializar_laberinto_con_datos(&mut self, datos: Vec<String>) -> Result<(), String> {
        let mut coordenada_x = 0;
        let mut coordenada_y = 0;
        let mut casillero: Casillero<'a>;
        let mut coordenada_casillero: Coordenada;

        for dato in datos {
            let partes = dato.split_whitespace().collect::<Vec<&str>>();

            for parte in partes {
                coordenada_casillero = Coordenada::new(coordenada_x, coordenada_y);

                if parte.len() == UN_CARACTER {
                    casillero = self.crear_objeto_un_caracter(parte, coordenada_casillero)?;
                } else {
                    casillero = self.crear_objeto_dos_caracteres(parte, coordenada_casillero)?;
                }

                self.laberinto.reemplazar_casillero_en_tablero(casillero);
                coordenada_x = coordenada_x + 1;
            }

            coordenada_y = coordenada_y + 1;
            coordenada_x = 0;
        }

        Ok(())
    }

    pub fn rafagear_coordenada(&mut self, coordenada_a_rafagear: &Coordenada) -> ResultadoRafaga {
        self.laberinto.rafagear_coordenada(coordenada_a_rafagear)
    }

    pub fn rafagear_coordenada_traspaso(
        &mut self,
        coordenada_a_rafagear: &Coordenada,
    ) -> ResultadoRafaga {
        self.laberinto
            .rafagear_coordenada_traspaso(coordenada_a_rafagear)
    }

    pub fn detonar_coordenada(&mut self, coordenada_a_detonar: &Coordenada) -> Result<(), String> {
        self.laberinto.detonar_coordenada(coordenada_a_detonar)
    }

    pub fn vaciar_coordenada(&mut self, coordenada_a_vaciar: &Coordenada) {
        self.laberinto.vaciar_coordenada(coordenada_a_vaciar);
    }

    pub fn obtener_visualizacion(&self) -> Vec<Vec<String>> {
        self.laberinto.obtener_visualizacion()
    }
}
