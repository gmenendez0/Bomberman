use crate::coordenada::Coordenada;
use crate::laberinto::Laberinto;

pub struct Juego{
    laberinto: Laberinto,
}

impl Juego{
    pub fn new(filas_del_tablero: Vec<String>) -> Juego{
        Juego{
            laberinto: Laberinto::new(filas_del_tablero),
        }
    }

    pub fn explotar_coordenada(self, coordenada_a_explotar: Coordenada) {
        self.laberinto.explotar_coordenada(coordenada_a_explotar);
    }

    pub fn explotar_coordenada_traspaso(self, coordenada_a_explotar: Coordenada) {
        self.laberinto.explotar_coordenada_traspaso(coordenada_a_explotar);
    }

    // ? Devolver el resultado de la detonacion!!
    pub fn detonar_coordenada(self, coordenada_a_detonar: Coordenada) {
        self.laberinto.detonar_coordenada(coordenada_a_detonar);
    }

    pub fn vaciar_coordenada(self, coordenada_a_vaciar: Coordenada) {
        self.laberinto.vaciar_coordenada(coordenada_a_vaciar);
    }

    pub fn obtener_visualizacion(&self) -> Vec<Vec<String>> {
        self.laberinto.obtener_visualizacion()
    }
}