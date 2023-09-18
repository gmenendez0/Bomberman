use crate::coordenada::Coordenada;
use crate::laberinto::Laberinto;

#[derive(Clone)]
pub enum ResultadoRafaga {
    DesvioArriba,
    DesvioAbajo,
    DesvioIzquierda,
    DesvioDerecha,
    Choque,
    ChoqueFuerte,
    Insignificante,
    EnemigoEliminado,
}

//? ObjetoMapa es un trait que define el comportamiento de los objetos que se pueden colocar en el mapa.
pub trait ObjetoMapa {
    //? Establece la coordenada actual del objeto.
    fn set_coordenada_actual(&mut self, coordenada: Coordenada);

    //? Devuelve una coordenada COPIA de la actual del objeto.
    fn get_coordenada_actual(&self) -> Coordenada;

    //? Hace que el objeto reciba una rafaga y se apliquen sus consecuencias.
    fn recibir_rafaga(&mut self) -> ResultadoRafaga;

    //? Detona el objeto, si es posible. Dependiendo de eso, devuelve un error o exito.
    fn detonar(&self, laberinto: &mut Laberinto) -> Result<(), String>;

    //? Devuelve la representacion del objeto en el mapa.
    fn obtener_representacion(&self) -> String;
}
