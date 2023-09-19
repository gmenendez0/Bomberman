use crate::resultado_rafaga::ResultadoRafaga::EnemigoTocado;

//? Resultado que representa el resultado de rafagear una casilla.
//? Debug esta implementado para uso en tests.
#[derive(Clone, Copy, PartialEq, Debug)]
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
    //? Devuelve la vida del EnemigoTocado si corresponde, si no 0.
    pub fn get_vida_enemigo(&self) -> i32 {
        match self {
            EnemigoTocado(vida) => *vida,
            _ => 0,
        }
    }
}
