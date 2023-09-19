use crate::casillero::ResultadoRafaga::{Choque, ChoqueFuerte, DesvioAbajo, DesvioArriba, DesvioDerecha, DesvioIzquierda, Detonacion, EnemigoEliminado, EnemigoTocado, Insignificante};
use crate::coordenada::Coordenada;
use crate::enemigo::Enemigo;
use crate::laberinto::Laberinto;

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

#[derive(Clone)]
pub enum Casillero {
    Vacio(Coordenada),
    Roca(Coordenada),
    Pared(Coordenada),
    Enemigoo(Coordenada, Enemigo),
    Desvio(Coordenada, String),
    BombaNormal(Coordenada, i32),
    BombaTraspaso(Coordenada, i32),
}

impl Casillero {
    fn rafaga_no_choca_obstaculo(resultado_rafaga: &ResultadoRafaga) -> bool {
        !matches!(resultado_rafaga,ResultadoRafaga::DesvioArriba | ResultadoRafaga::DesvioAbajo | ResultadoRafaga::DesvioIzquierda | ResultadoRafaga::DesvioDerecha | ResultadoRafaga::Choque | ResultadoRafaga::ChoqueFuerte)
    }

    fn rafaga_continua_sin_chocar_obstaculo(alcance_restante: i32, resultado_rafaga: &ResultadoRafaga) -> bool {
        alcance_restante > 0 && Casillero::rafaga_no_choca_obstaculo(resultado_rafaga)
    }

    fn rafaga_continua_chocando_obstaculo(alcance_restante: i32, resultado_rafaga: &ResultadoRafaga) -> bool {
        alcance_restante > 0 && !Casillero::rafaga_no_choca_obstaculo(resultado_rafaga)
    }

    fn evaluar_camino_a_seguir(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, alcance_restante: i32, resultado_rafaga: ResultadoRafaga) -> Result<ResultadoRafaga, String> {
        match resultado_rafaga {
            DesvioArriba => { self.rafagear_arriba(lab, coordenada_inicial, alcance_restante) },
            DesvioAbajo => { self.rafagear_abajo(lab, coordenada_inicial, alcance_restante) },
            DesvioIzquierda => { self.rafagear_izquierda(lab, coordenada_inicial, alcance_restante) },
            DesvioDerecha => { self.rafagear_derecha(lab, coordenada_inicial, alcance_restante) },
            _ => { Err("Error al ejecutar el desvio".to_string()) }
        }
    }

    pub fn rafagear_arriba(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);
        let referencia_mutable_resultado_rafaga = &mut resultado_rafaga;

        while Casillero::rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &referencia_mutable_resultado_rafaga.clone() ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_y() + 1);
            alcance_restante_rafagas -= 1;

            *referencia_mutable_resultado_rafaga = lab.rafagear_coordenada(&coordenada_a_rafagear);
        }

        if Casillero::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga.clone() ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(lab, &coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga
    }

    pub fn rafagear_abajo(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);
        let referencia_mutable_resultado_rafaga = &mut resultado_rafaga;

        while Casillero::rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &referencia_mutable_resultado_rafaga.clone() ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_y() - 1);
            alcance_restante_rafagas -= 1;

            *referencia_mutable_resultado_rafaga = lab.rafagear_coordenada(&coordenada_a_rafagear);
        }

        if Casillero::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga.clone() ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(lab, &coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga
    }

    pub fn rafagear_derecha(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);
        let referencia_mutable_resultado_rafaga = &mut resultado_rafaga;

        while Casillero::rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &referencia_mutable_resultado_rafaga.clone() ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_x() + 1);
            alcance_restante_rafagas -= 1;

            *referencia_mutable_resultado_rafaga = lab.rafagear_coordenada(&coordenada_a_rafagear);
        }

        if Casillero::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga.clone() ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(lab, &coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga
    }

    pub fn rafagear_izquierda(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);
        let referencia_mutable_resultado_rafaga = &mut resultado_rafaga;

        while Casillero::rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &referencia_mutable_resultado_rafaga.clone() ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_x() - 1);
            alcance_restante_rafagas -= 1;

            *referencia_mutable_resultado_rafaga = lab.rafagear_coordenada(&coordenada_a_rafagear);
        }

        if Casillero::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga.clone() ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(lab, &coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga
    }

    fn iniciar_rafagas(&self, lab: &mut Laberinto, alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        self.rafagear_arriba(lab, &self.get_coordenada(), alcance_restante_rafagas) ?;
        self.rafagear_abajo(lab, &self.get_coordenada(), alcance_restante_rafagas) ?;
        self.rafagear_derecha(lab, &self.get_coordenada(), alcance_restante_rafagas) ?;
        self.rafagear_izquierda(lab, &self.get_coordenada(), alcance_restante_rafagas)
    }

    pub fn detonar(&self, lab: &mut Laberinto) -> Result<ResultadoRafaga, String> {
        match self {
            Casillero::Vacio(_) => Err("No se puede detonar un vacio".to_string()),
            Casillero::Roca(_) => Err("No se puede detonar una roca".to_string()),
            Casillero::Pared(_) => Err("No se puede detonar una pared".to_string()),
            Casillero::Enemigoo(..) => Err("No se puede detonar un enemigo".to_string()),
            Casillero::Desvio(..) => Err("No se puede detonar un desvio".to_string()),
            Casillero::BombaNormal(_, alcance) => {
                lab.reemplazar_objeto_en_tablero(Casillero::Vacio(self.get_coordenada()), self.get_coordenada());
                self.iniciar_rafagas(lab, *alcance)
            },
            Casillero::BombaTraspaso(_, _) => {
                lab.reemplazar_objeto_en_tablero(Casillero::Vacio(self.get_coordenada()), self.get_coordenada());
                Ok(Insignificante)
            },
        }
    }

    pub fn recibir_rafaga(&self) -> ResultadoRafaga {
        match self {
            Casillero::Vacio(_) => Insignificante,
            Casillero::Roca(_) => Choque,
            Casillero::Pared(_) => ChoqueFuerte,
            Casillero::Enemigoo(_, enemigo) => {
                if enemigo.esta_muerto() {
                    EnemigoEliminado
                } else {
                    EnemigoTocado(enemigo.get_vida() - 1)
                }
            },
            Casillero::Desvio(_, direccion) => {
                if direccion == "U" {
                    DesvioArriba
                } else if direccion == "D" {
                    DesvioAbajo
                } else if direccion == "L" {
                    DesvioIzquierda
                } else {
                    DesvioDerecha
                }
            },
            Casillero::BombaNormal(_, _) => Detonacion,
            Casillero::BombaTraspaso(_, _) => Detonacion,
        }
    }

    pub(crate) fn obtener_representacion(&self) -> String {
        match self {
            Casillero::Vacio(_) => String::from("_"),
            Casillero::Roca(_) => String::from("R"),
            Casillero::Pared(_) => String::from("W"),
            Casillero::Enemigoo(_, enemigo) => {
                String::from("E") + &enemigo.get_vida().to_string()
            },
            Casillero::Desvio(_, direccion) => {
                String::from("D") + direccion
            },
            Casillero::BombaNormal(_, alcance) => {
                let representacion_bomba = String::from("B");
                let representacion_alcance= alcance.to_string();

                representacion_bomba + &representacion_alcance
            },
            Casillero::BombaTraspaso(_, alcance) => {
                let representacion_bomba_traspaso = String::from("S");
                let representacion_alcance= alcance.to_string();

                representacion_bomba_traspaso + &representacion_alcance
            },
        }
    }

    pub fn get_coordenada(&self) -> Coordenada {
        match self {
            Casillero::Vacio(coordenada) => coordenada.clone(),
            Casillero::Roca(coordenada) => coordenada.clone(),
            Casillero::Pared(coordenada) => coordenada.clone(),
            Casillero::Enemigoo(coordenada, ..) => coordenada.clone(),
            Casillero::Desvio(coordenada, _) => coordenada.clone(),
            Casillero::BombaNormal(coordenada, _) => coordenada.clone(),
            Casillero::BombaTraspaso(coordenada, _) => coordenada.clone(),
        }
    }
}