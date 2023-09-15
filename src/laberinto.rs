use crate::coordenada::Coordenada;
use crate::objeto_mapa::ObjetoMapa;

pub struct Laberinto{
    //tablero: Vec<Vec<Box<dyn ObjetoMapa>>>,
}

impl Laberinto{
    pub fn new(filasDelTablero: Vec<String>) -> Laberinto{
        //? Crear tablero basándose en las filas...
        Laberinto{}
    }

   pub fn explotar_coordenada(self, coordenada_a_explotar: Coordenada) {
       //? Acceder a la coordenada recibida dentro del tablero y decirle que recibaExplosion...
   }

    pub fn explotar_coordenada_traspaso(self, coordenada_a_explotar: Coordenada) {
         //? Acceder a la coordenada recibida dentro del tablero y decirle que recibaExplosionTraspaso...
    }

    // ? Devolver el resultado de la detonacion!!
    pub fn detonar_coordenada(self, coordenada_a_detonar: Coordenada) {
        //? Acceder a la coordenada recibida dentro del tablero y decirle que detone.
    }

    pub fn vaciar_coordenada(self, coordenada_a_vaciar: Coordenada) {
        //? Acceder a la coordenada recibida dentro del tablero y poner alli un nuevo Vacio.
    }

    pub fn obtener_visualizacion(&self) -> Vec<Vec<String>> {
        //? Recorrer el tablero y obtener la representacion de cada objeto.
        //? Armar una matriz de strings con esas representaciones y devolverla.

        let mut matriz: Vec<Vec<String>> = Vec::new();
        matriz
    }
}