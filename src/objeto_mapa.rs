use crate::coordenada::Coordenada;

pub enum ResultadoRafaga {
    DesvioArriba,
    DesvioAbajo,
    DesvioIzquierda,
    DesvioDerecha,
    Choque,
    ChoqueFuerte,
    Insignificante,
}

pub trait ObjetoMapa {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada);

    fn get_coordenada_actual(&self) -> &Coordenada;

    fn recibir_rafaga(&mut self) -> ResultadoRafaga;

    fn recibir_rafaga_traspaso(&mut self) -> ResultadoRafaga;

    fn detonar(&mut self) -> Result<(), String>;

    fn obtener_representacion(&self) -> String;
}
