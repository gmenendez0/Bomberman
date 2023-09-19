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

#[cfg(test)]
mod tests {
    use crate::resultado_rafaga::ResultadoRafaga;

    #[test]
    fn test_get_vida_enemigo_enemigo_tocado() {
        let resultado = ResultadoRafaga::EnemigoTocado(42);
        assert_eq!(resultado.get_vida_enemigo(), 42);
    }

    #[test]
    fn test_get_vida_enemigo_desvio_arriba() {
        let resultado = ResultadoRafaga::DesvioArriba;
        assert_eq!(resultado.get_vida_enemigo(), 0);
    }

    #[test]
    fn test_get_vida_enemigo_enemigo_eliminado() {
        let resultado = ResultadoRafaga::EnemigoEliminado;
        assert_eq!(resultado.get_vida_enemigo(), 0);
    }

    #[test]
    fn test_get_vida_enemigo_detonacion() {
        let resultado = ResultadoRafaga::Detonacion;
        assert_eq!(resultado.get_vida_enemigo(), 0);
    }
}
