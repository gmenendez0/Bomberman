use crate::bomba::Bomba;
use crate::coordenada::Coordenada;
use crate::juego::Juego;
use crate::objeto_mapa::ResultadoRafaga::{
    Choque, ChoqueFuerte, DesvioAbajo, DesvioArriba, Insignificante,
};
use crate::objeto_mapa::{ObjetoMapa, ResultadoRafaga};

pub struct BombaTraspaso<'a> {
    coordenada_actual: Coordenada,
    juego: &'a mut Juego,
    alcance: i32,
}

impl<'a> BombaTraspaso<'a> {
    pub fn new(coordenada_actual: Coordenada, juego: &mut Juego, alcance: i32) -> BombaTraspaso {
        BombaTraspaso {
            coordenada_actual,
            juego,
            alcance,
        }
    }
}

impl<'a> Bomba for BombaTraspaso<'a> {
    fn rafagear_arriba(&mut self, coordenada_inicial: Coordenada, mut alcance_restante: i32) {
        let mut coordenada_a_rafagear = coordenada_inicial;
        let mut resultado_rafaga = Insignificante;

        while BombaTraspaso::<'a>::rafaga_continua_sin_chocar_obstaculo(
            alcance_restante,
            &resultado_rafaga,
        ) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_y() + 1);
            alcance_restante -= 1;

            resultado_rafaga = self
                .juego
                .rafagear_coordenada_traspaso(&coordenada_a_rafagear);
        }

        if BombaTraspaso::<'a>::rafaga_continua_chocando_obstaculo(
            alcance_restante,
            &resultado_rafaga,
        ) {
            self.evaluar_camino_a_seguir(coordenada_a_rafagear, alcance_restante, resultado_rafaga);
        }
    }

    fn rafagear_abajo(&mut self, coordenada_inicial: Coordenada, mut alcance_restante: i32) {
        let mut coordenada_a_rafagear = coordenada_inicial;
        let mut resultado_rafaga = Insignificante;

        while BombaTraspaso::<'a>::rafaga_continua_sin_chocar_obstaculo(
            alcance_restante,
            &resultado_rafaga,
        ) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_y() - 1);
            alcance_restante -= 1;

            resultado_rafaga = self
                .juego
                .rafagear_coordenada_traspaso(&coordenada_a_rafagear);
        }

        if BombaTraspaso::<'a>::rafaga_continua_chocando_obstaculo(
            alcance_restante,
            &resultado_rafaga,
        ) {
            self.evaluar_camino_a_seguir(coordenada_a_rafagear, alcance_restante, resultado_rafaga);
        }
    }

    fn rafagear_derecha(&mut self, coordenada_inicial: Coordenada, mut alcance_restante: i32) {
        let mut coordenada_a_rafagear = coordenada_inicial;
        let mut resultado_rafaga = Insignificante;

        while BombaTraspaso::<'a>::rafaga_continua_sin_chocar_obstaculo(
            alcance_restante,
            &resultado_rafaga,
        ) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_x() + 1);
            alcance_restante -= 1;

            resultado_rafaga = self
                .juego
                .rafagear_coordenada_traspaso(&coordenada_a_rafagear);
        }

        if BombaTraspaso::<'a>::rafaga_continua_chocando_obstaculo(
            alcance_restante,
            &resultado_rafaga,
        ) {
            self.evaluar_camino_a_seguir(coordenada_a_rafagear, alcance_restante, resultado_rafaga);
        }
    }

    fn rafagear_izquierda(&mut self, coordenada_inicial: Coordenada, mut alcance_restante: i32) {
        let mut coordenada_a_rafagear = coordenada_inicial;
        let mut resultado_rafaga = Insignificante;

        while BombaTraspaso::<'a>::rafaga_continua_sin_chocar_obstaculo(
            alcance_restante,
            &resultado_rafaga,
        ) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_x() - 1);
            alcance_restante -= 1;

            resultado_rafaga = self
                .juego
                .rafagear_coordenada_traspaso(&coordenada_a_rafagear);
        }

        if BombaTraspaso::<'a>::rafaga_continua_chocando_obstaculo(
            alcance_restante,
            &resultado_rafaga,
        ) {
            self.evaluar_camino_a_seguir(coordenada_a_rafagear, alcance_restante, resultado_rafaga);
        }
    }

    fn rafaga_no_choca_obstaculo(resultado_rafaga: &ResultadoRafaga) -> bool {
        !matches!(
            resultado_rafaga,
            DesvioArriba
                | DesvioAbajo
                | ResultadoRafaga::DesvioIzquierda
                | ResultadoRafaga::DesvioDerecha
                | ChoqueFuerte
        )
    }

    fn rafaga_continua_sin_chocar_obstaculo(
        alcance_restante: i32,
        resultado_rafaga: &ResultadoRafaga,
    ) -> bool {
        alcance_restante > 0 && BombaTraspaso::<'a>::rafaga_no_choca_obstaculo(resultado_rafaga)
    }

    fn rafaga_continua_chocando_obstaculo(
        alcance_restante: i32,
        resultado_rafaga: &ResultadoRafaga,
    ) -> bool {
        alcance_restante > 0 && !BombaTraspaso::<'a>::rafaga_no_choca_obstaculo(resultado_rafaga)
    }

    fn evaluar_camino_a_seguir(
        &mut self,
        coordenada_inicial: Coordenada,
        alcance_restante: i32,
        resultado_rafaga: ResultadoRafaga,
    ) {
        match resultado_rafaga {
            DesvioArriba => self.rafagear_arriba(coordenada_inicial, alcance_restante),
            DesvioAbajo => self.rafagear_abajo(coordenada_inicial, alcance_restante),
            ResultadoRafaga::DesvioIzquierda => {
                self.rafagear_izquierda(coordenada_inicial, alcance_restante)
            }
            ResultadoRafaga::DesvioDerecha => {
                self.rafagear_derecha(coordenada_inicial, alcance_restante)
            }
            _ => {}
        }
    }

    fn procesar_detonacion(&mut self) {
        self.rafagear_arriba(self.coordenada_actual.clone(), self.alcance);
        self.rafagear_abajo(self.coordenada_actual.clone(), self.alcance);
        self.rafagear_izquierda(self.coordenada_actual.clone(), self.alcance);
        self.rafagear_derecha(self.coordenada_actual.clone(), self.alcance);
    }
}

impl<'a> ObjetoMapa for BombaTraspaso<'a> {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> &Coordenada {
        &self.coordenada_actual
    }

    fn recibir_rafaga(&mut self) -> ResultadoRafaga {
        self.procesar_detonacion();
        Insignificante
    }

    fn recibir_rafaga_traspaso(&mut self) -> ResultadoRafaga {
        self.procesar_detonacion();
        Insignificante
    }

    fn detonar(&mut self) -> Result<(), String> {
        self.procesar_detonacion();
        Ok(())
    }

    fn obtener_representacion(&self) -> String {
        let representacion_bomba_traspaso = String::from("S");
        let representacion_de_alcance = self.alcance.to_string();

        representacion_bomba_traspaso + &representacion_de_alcance
    }
}
