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

pub trait ObjetoMapa {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada);

    fn get_coordenada_actual(&self) -> Coordenada;

    fn recibir_rafaga(&mut self) -> ResultadoRafaga;

    fn detonar(&mut self, laberinto: &mut Laberinto) -> Result<(), String>;

    fn obtener_representacion(&self) -> String;
}
