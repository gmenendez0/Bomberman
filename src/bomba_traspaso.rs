use crate::coordenada::Coordenada;
use crate::juego::Juego;
use crate::objeto_mapa::ObjetoMapa;

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

impl<'a> ObjetoMapa for BombaTraspaso<'a> {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada) {
        self.coordenada_actual = coordenada;
    }

    fn get_coordenada_actual(&self) -> &Coordenada {
        &self.coordenada_actual
    }

    fn recibir_rafaga(&mut self) -> Result<(), String> {
        self.detonar()
    }

    fn recibir_rafaga_traspaso(&mut self) -> Result<(), String> {
        self.detonar()
    }

    fn detonar(&mut self) -> Result<(), String> {
        //? Hasta ahora, la bomba explota sin importar si se da contra una pared, un desvio o una roca.
        let mut coordenada_derecha_a_rafagear = self.coordenada_actual.clone();
        let mut coordenada_izquierda_a_rafagear = self.coordenada_actual.clone();
        let mut coordenada_arriba_a_rafagear = self.coordenada_actual.clone();
        let mut coordenada_abajo_a_rafagear = self.coordenada_actual.clone();

        for i in 1..self.alcance + 1 {
            coordenada_derecha_a_rafagear.set_x(self.coordenada_actual.get_x() + i);
            coordenada_izquierda_a_rafagear.set_x(self.coordenada_actual.get_x() - i);
            coordenada_arriba_a_rafagear.set_y(self.coordenada_actual.get_y() + i);
            coordenada_abajo_a_rafagear.set_y(self.coordenada_actual.get_y() - i);

            self.juego
                .rafagear_coordenada_traspaso(&coordenada_derecha_a_rafagear);
            self.juego
                .rafagear_coordenada_traspaso(&coordenada_izquierda_a_rafagear);
            self.juego
                .rafagear_coordenada_traspaso(&coordenada_arriba_a_rafagear);
            self.juego
                .rafagear_coordenada_traspaso(&coordenada_abajo_a_rafagear);
        }

        Ok(())
    }

    fn obtener_representacion(&self) -> String {
        let representacion_bomba_traspaso = String::from("S");
        let representacion_de_alcance = self.alcance.to_string();

        representacion_bomba_traspaso + &representacion_de_alcance
    }
}
