use crate::coordenada::Coordenada;
use crate::laberinto::Laberinto;
use crate::objeto_mapa::ResultadoRafaga;

pub struct Juego {
    laberinto: Laberinto,
}

impl Juego {
    pub fn new(filas_del_tablero: Vec<String>) -> Juego {
        Juego {
            laberinto: Laberinto::new(filas_del_tablero),
        }
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

    // ? Devolver el resultado de la detonacion!!
    pub fn detonar_coordenada(&mut self, coordenada_a_detonar: &Coordenada) {
        self.laberinto.detonar_coordenada(coordenada_a_detonar);
    }

    pub fn vaciar_coordenada(&mut self, coordenada_a_vaciar: &Coordenada) {
        self.laberinto.vaciar_coordenada(coordenada_a_vaciar);
    }

    pub fn obtener_visualizacion(&self) -> Vec<Vec<String>> {
        self.laberinto.obtener_visualizacion()
    }
}
