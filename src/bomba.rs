use crate::coordenada::Coordenada;
use crate::objeto_mapa::ResultadoRafaga;

pub trait Bomba {
    fn rafagear_arriba(&mut self, coordenada_inicial: Coordenada, alcance_restante: i32);

    fn rafagear_abajo(&mut self, coordenada_inicial: Coordenada, alcance_restante: i32);

    fn rafagear_derecha(&mut self, coordenada_inicial: Coordenada, alcance_restante: i32);

    fn rafagear_izquierda(&mut self, coordenada_inicial: Coordenada, alcance_restante: i32);

    fn rafaga_no_choca_obstaculo(resultado_rafaga: &ResultadoRafaga) -> bool;

    fn rafaga_continua_sin_chocar_obstaculo(
        alcance_restante: i32,
        resultado_rafaga: &ResultadoRafaga,
    ) -> bool;

    fn rafaga_continua_chocando_obstaculo(
        alcance_restante: i32,
        resultado_rafaga: &ResultadoRafaga,
    ) -> bool;

    fn evaluar_camino_a_seguir(
        &mut self,
        coordenada_inicial: Coordenada,
        alcance_restante: i32,
        resultado_rafaga: ResultadoRafaga,
    );

    fn procesar_detonacion(&mut self);
}
