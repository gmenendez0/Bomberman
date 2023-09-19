use crate::resultado_rafaga::ResultadoRafaga::EnemigoTocado;

#[derive(Clone, Copy, PartialEq)]
pub enum ResultadoRafaga {
    DesvioArriba,
    DesvioAbajo,
    DesvioIzquierda,
    DesvioDerecha,
    Choque,
    ChoqueFuerte,
    Insignificante,
    EnemigoTocado(i32),
    EnemigoEliminado,
    Detonacion,
}

impl ResultadoRafaga {
    pub fn get_vida_enemigo(&self) -> i32 {
        match self {
            EnemigoTocado(vida) => *vida,
            _ => 0,
        }
    }
}