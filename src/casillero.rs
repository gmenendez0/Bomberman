use crate::resultado_rafaga::ResultadoRafaga::{Choque, ChoqueFuerte, DesvioAbajo, DesvioArriba, DesvioDerecha, DesvioIzquierda, Detonacion, EnemigoEliminado, EnemigoTocado, Insignificante};
use crate::coordenada::Coordenada;
use crate::enemigo::Enemigo;
use crate::laberinto::Laberinto;
use crate::resultado_rafaga::ResultadoRafaga;

//? Representa un casillero del tablero. Puede contener o no valores, segun corresponda. Siempre contiene su coordenada.
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
    //? Devuelve true en caso de que la rafaga no haya chocado un obstaculo de bomba normal.
    fn rafaga_no_choca_obstaculo_traspaso(resultado_rafaga: &ResultadoRafaga) -> bool {
        !matches!(resultado_rafaga,ResultadoRafaga::DesvioArriba | ResultadoRafaga::DesvioAbajo | ResultadoRafaga::DesvioIzquierda | ResultadoRafaga::DesvioDerecha | ResultadoRafaga::ChoqueFuerte)
    }

    //? Devuelve true en caso de que la rafaga aun tenga alcance y no haya chocado un obstaculo de bomba normal.
    fn rafaga_continua_sin_chocar_obstaculo_traspaso(alcance_restante: i32, resultado_rafaga: &ResultadoRafaga) -> bool {
        alcance_restante > 0 && Casillero::rafaga_no_choca_obstaculo_traspaso(resultado_rafaga)
    }

    //? Devuelve true en caso de que la rafaga aun tenga alcance pero haya chocado un obstaculo de bomba normal.
    fn rafaga_continua_chocando_obstaculo_traspaso(alcance_restante: i32, resultado_rafaga: &ResultadoRafaga) -> bool {
        alcance_restante > 0 && !Casillero::rafaga_no_choca_obstaculo_traspaso(resultado_rafaga)
    }

    //? Devuelve true en caso de que la rafaga no haya chocado un obstaculo de bomba normal.
    fn rafaga_no_choca_obstaculo(resultado_rafaga: &ResultadoRafaga) -> bool {
        !matches!(resultado_rafaga,ResultadoRafaga::DesvioArriba | ResultadoRafaga::DesvioAbajo | ResultadoRafaga::DesvioIzquierda | ResultadoRafaga::DesvioDerecha | ResultadoRafaga::Choque | ResultadoRafaga::ChoqueFuerte)
    }

    //? Devuelve true en caso de que la rafaga aun tenga alcance y no haya chocado un obstaculo de bomba normal.
    fn rafaga_continua_sin_chocar_obstaculo(alcance_restante: i32, resultado_rafaga: &ResultadoRafaga) -> bool {
        alcance_restante > 0 && Casillero::rafaga_no_choca_obstaculo(resultado_rafaga)
    }

    //? Devuelve true en caso de que la rafaga aun tenga alcance pero haya chocado un obstaculo de bomba normal.
    fn rafaga_continua_chocando_obstaculo(alcance_restante: i32, resultado_rafaga: &ResultadoRafaga) -> bool {
        alcance_restante > 0 && !Casillero::rafaga_no_choca_obstaculo(resultado_rafaga)
    }

    //? Evalua el camino a seguir dependiendo del obstaculo chocado. Devuelve el resultado del rafageo de la ultima casilla rafageada.
    fn evaluar_camino_a_seguir(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, alcance_restante: i32, resultado_rafaga: ResultadoRafaga) -> Result<ResultadoRafaga, String> {
        match resultado_rafaga {
            DesvioArriba => { self.rafagear_arriba(lab, coordenada_inicial, alcance_restante) },
            DesvioAbajo => { self.rafagear_abajo(lab, coordenada_inicial, alcance_restante) },
            DesvioIzquierda => { self.rafagear_izquierda(lab, coordenada_inicial, alcance_restante) },
            DesvioDerecha => { self.rafagear_derecha(lab, coordenada_inicial, alcance_restante) },
            ChoqueFuerte => { Ok(ChoqueFuerte)},
            _ => { Ok(Choque) }
        }
    }

    //? Evalua el camino a seguir dependiendo del obstaculo chocado. Devuelve el resultado del rafageo de la ultima casilla rafageada.
    fn evaluar_camino_a_seguir_traspaso(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, alcance_restante: i32, resultado_rafaga: ResultadoRafaga) -> Result<ResultadoRafaga, String> {
        match resultado_rafaga {
            DesvioArriba => { self.rafagear_arriba_traspaso(lab, coordenada_inicial, alcance_restante) },
            DesvioAbajo => { self.rafagear_abajo_traspaso(lab, coordenada_inicial, alcance_restante) },
            DesvioIzquierda => { self.rafagear_izquierda_traspaso(lab, coordenada_inicial, alcance_restante) },
            DesvioDerecha => { self.rafagear_derecha_traspaso(lab, coordenada_inicial, alcance_restante) },
            _ => { Ok(ChoqueFuerte)},
        }
    }

    //? Aplica la rafaga a la coordenada superior a la recibida.
    pub fn aplicar_rafaga_arriba(coordenada_base: &mut Coordenada, lab: &mut Laberinto, alcance_restante_rafagas: &mut i32, resultado_rafaga: &mut Result<ResultadoRafaga, String>){
        if coordenada_base.get_y() == 0 {
            *resultado_rafaga = Ok(ChoqueFuerte);
        } else {
            coordenada_base.set_y(coordenada_base.get_y() - 1);
            *alcance_restante_rafagas -= 1;

            *resultado_rafaga = lab.rafagear_coordenada(coordenada_base);
        }
    }

    //? Aplica la rafaga a la coordenada izquierda a la recibida.
    pub fn aplicar_rafaga_izquierda(coordenada_base: &mut Coordenada, lab: &mut Laberinto, alcance_restante_rafagas: &mut i32, resultado_rafaga: &mut Result<ResultadoRafaga, String>){
        if coordenada_base.get_x() == 0 {
            *resultado_rafaga = Ok(ChoqueFuerte);
        } else {
            coordenada_base.set_x(coordenada_base.get_x() - 1);
            *alcance_restante_rafagas -= 1;

            *resultado_rafaga = lab.rafagear_coordenada(coordenada_base);
        }
    }

    //? Rafagea hacia arriba. Devuelve el resultado del rafageo de la ultima casilla rafageada.
    pub fn rafagear_arriba(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);
        let referencia_mutable_resultado_rafaga = &mut resultado_rafaga;

        while Casillero::rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &referencia_mutable_resultado_rafaga.clone() ?) {
            Casillero::aplicar_rafaga_arriba(&mut coordenada_a_rafagear, lab, &mut alcance_restante_rafagas, referencia_mutable_resultado_rafaga);
        }

        if Casillero::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga.clone() ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(lab, &coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga
    }

    //? Rafagea hacia abajo. Devuelve el resultado del rafageo de la ultima casilla rafageada.
    pub fn rafagear_abajo(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
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

    //? Rafagea a la derecha. Devuelve el resultado del rafageo de la ultima casilla rafageada.
    pub fn rafagear_derecha(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);
        let referencia_mutable_resultado_rafaga = &mut resultado_rafaga;

        while Casillero::rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &referencia_mutable_resultado_rafaga.clone() ?) {
            coordenada_a_rafagear.set_x(coordenada_a_rafagear.get_x() + 1);
            alcance_restante_rafagas -= 1;

            *referencia_mutable_resultado_rafaga = lab.rafagear_coordenada(&coordenada_a_rafagear);
        }

        if Casillero::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga.clone() ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(lab, &coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga
    }

    //? Rafagea a la izquierda. Devuelve el resultado del rafageo de la ultima casilla rafageada.
    pub fn rafagear_izquierda(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);
        let referencia_mutable_resultado_rafaga = &mut resultado_rafaga;

        while Casillero::rafaga_continua_sin_chocar_obstaculo(alcance_restante_rafagas, &referencia_mutable_resultado_rafaga.clone() ?) {
            Casillero::aplicar_rafaga_izquierda(&mut coordenada_a_rafagear, lab, &mut alcance_restante_rafagas, referencia_mutable_resultado_rafaga);
        }

        if Casillero::rafaga_continua_chocando_obstaculo(alcance_restante_rafagas, &resultado_rafaga.clone() ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir(lab, &coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga
    }

    //? Inicia los rafageos en todas las direcciones. Devuelve el resultado del rafageo de la ultima casilla rafageada.
    fn iniciar_rafagas(&self, lab: &mut Laberinto, alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        self.rafagear_arriba(lab, &self.get_coordenada(), alcance_restante_rafagas) ?;
        self.rafagear_abajo(lab, &self.get_coordenada(), alcance_restante_rafagas) ?;
        self.rafagear_derecha(lab, &self.get_coordenada(), alcance_restante_rafagas) ?;
        self.rafagear_izquierda(lab, &self.get_coordenada(), alcance_restante_rafagas)
    }

    //? Rafagea hacia arriba. Devuelve el resultado del rafageo de la ultima casilla rafageada.
    pub fn rafagear_arriba_traspaso(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);
        let referencia_mutable_resultado_rafaga = &mut resultado_rafaga;

        while Casillero::rafaga_continua_sin_chocar_obstaculo_traspaso(alcance_restante_rafagas, &referencia_mutable_resultado_rafaga.clone() ?) {
            Casillero::aplicar_rafaga_arriba(&mut coordenada_a_rafagear, lab, &mut alcance_restante_rafagas, referencia_mutable_resultado_rafaga);
        }

        if Casillero::rafaga_continua_chocando_obstaculo_traspaso(alcance_restante_rafagas, &resultado_rafaga.clone() ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir_traspaso(lab, &coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga
    }

    //? Rafagea hacia abajo. Devuelve el resultado del rafageo de la ultima casilla rafageada.
    pub fn rafagear_abajo_traspaso(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);
        let referencia_mutable_resultado_rafaga = &mut resultado_rafaga;

        while Casillero::rafaga_continua_sin_chocar_obstaculo_traspaso(alcance_restante_rafagas, &referencia_mutable_resultado_rafaga.clone() ?) {
            coordenada_a_rafagear.set_y(coordenada_a_rafagear.get_y() + 1);
            alcance_restante_rafagas -= 1;

            *referencia_mutable_resultado_rafaga = lab.rafagear_coordenada(&coordenada_a_rafagear);
        }

        if Casillero::rafaga_continua_chocando_obstaculo_traspaso(alcance_restante_rafagas, &resultado_rafaga.clone() ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir_traspaso(lab, &coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga
    }

    //? Rafagea a la derecha. Devuelve el resultado del rafageo de la ultima casilla rafageada.
    pub fn rafagear_derecha_traspaso(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);
        let referencia_mutable_resultado_rafaga = &mut resultado_rafaga;

        while Casillero::rafaga_continua_sin_chocar_obstaculo_traspaso(alcance_restante_rafagas, &referencia_mutable_resultado_rafaga.clone() ?) {
            coordenada_a_rafagear.set_x(coordenada_a_rafagear.get_x() + 1);
            alcance_restante_rafagas -= 1;

            *referencia_mutable_resultado_rafaga = lab.rafagear_coordenada(&coordenada_a_rafagear);
        }

        if Casillero::rafaga_continua_chocando_obstaculo_traspaso(alcance_restante_rafagas, &resultado_rafaga.clone() ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir_traspaso(lab, &coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga
    }

    //? Rafagea a la izquierda. Devuelve el resultado del rafageo de la ultima casilla rafageada.
    pub fn rafagear_izquierda_traspaso(&self, lab: &mut Laberinto, coordenada_inicial: &Coordenada, mut alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        let mut coordenada_a_rafagear = coordenada_inicial.clone();
        let mut resultado_rafaga: Result<ResultadoRafaga, String> = Ok(Insignificante);
        let referencia_mutable_resultado_rafaga = &mut resultado_rafaga;

        while Casillero::rafaga_continua_sin_chocar_obstaculo_traspaso(alcance_restante_rafagas, &referencia_mutable_resultado_rafaga.clone() ?) {
            Casillero::aplicar_rafaga_izquierda(&mut coordenada_a_rafagear, lab, &mut alcance_restante_rafagas, referencia_mutable_resultado_rafaga);
        }

        if Casillero::rafaga_continua_chocando_obstaculo_traspaso(alcance_restante_rafagas, &resultado_rafaga.clone() ?) {
            resultado_rafaga = self.evaluar_camino_a_seguir_traspaso(lab, &coordenada_a_rafagear, alcance_restante_rafagas, resultado_rafaga ?);
        }

        resultado_rafaga
    }

    //? Inicia los rafageos traspaso en todas las direcciones. Devuelve el resultado del rafageo de la ultima casilla rafageada.
    fn iniciar_rafagas_traspaso(&self, lab: &mut Laberinto, alcance_restante_rafagas: i32) -> Result<ResultadoRafaga, String>{
        self.rafagear_arriba_traspaso(lab, &self.get_coordenada(), alcance_restante_rafagas) ?;
        self.rafagear_abajo_traspaso(lab, &self.get_coordenada(), alcance_restante_rafagas) ?;
        self.rafagear_derecha_traspaso(lab, &self.get_coordenada(), alcance_restante_rafagas) ?;
        self.rafagear_izquierda_traspaso(lab, &self.get_coordenada(), alcance_restante_rafagas)
    }

    //? Detona la casilla si es posible y devuelve el resultado final de la detonacion (resultado del ultimo rafageo), en caso de no poder detonar se devuelve un error.
    pub fn detonar(&self, lab: &mut Laberinto) -> Result<ResultadoRafaga, String> {
        match self {
            Casillero::Vacio(_) => Err("Error: No se puede detonar un vacio".to_string()),
            Casillero::Roca(_) => Err("Error: No se puede detonar una roca".to_string()),
            Casillero::Pared(_) => Err("Error: No se puede detonar una pared".to_string()),
            Casillero::Enemigoo(..) => Err("Error: No se puede detonar un enemigo".to_string()),
            Casillero::Desvio(..) => Err("Error: No se puede detonar un desvio".to_string()),
            Casillero::BombaNormal(_, alcance) => {
                lab.reemplazar_objeto_en_tablero(Casillero::Vacio(self.get_coordenada()), self.get_coordenada());
                self.iniciar_rafagas(lab, *alcance)
            },
            Casillero::BombaTraspaso(_, alcance) => {
                lab.reemplazar_objeto_en_tablero(Casillero::Vacio(self.get_coordenada()), self.get_coordenada());
                self.iniciar_rafagas_traspaso(lab, *alcance)
            },
        }
    }

    //? Devuelve el resultado de recibir una rafaga sobre la casilla.
    pub fn recibir_rafaga(&self) -> ResultadoRafaga {
        match self {
            Casillero::Vacio(_) => Insignificante,
            Casillero::Roca(_) => Choque,
            Casillero::Pared(_) => ChoqueFuerte,
            Casillero::Enemigoo(_, enemigo) => {
                if enemigo.get_vida() - 1 == 0 {
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

    //? Devuelve la representacion de la casilla.
    pub(crate) fn obtener_representacion(&self) -> String {
        match self {
            Casillero::Vacio(_) => String::from("_"),
            Casillero::Roca(_) => String::from("R"),
            Casillero::Pared(_) => String::from("W"),
            Casillero::Enemigoo(_, enemigo) => {
                String::from("F") + &enemigo.get_vida().to_string()
            },
            Casillero::Desvio(_, direccion) => {
                println!("Direccion: {}", String::from("D") + direccion);
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

    //? Devuelve la coordenada de la casilla.
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