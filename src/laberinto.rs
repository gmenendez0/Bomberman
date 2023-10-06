use crate::casillero::Casillero;
use crate::coordenada::Coordenada;
use crate::enemigo::Enemigo;
use crate::resultado_rafaga::ResultadoRafaga;
use crate::resultado_rafaga::ResultadoRafaga::{Detonacion, EnemigoEliminado, EnemigoTocado};

const UN_CARACTER: usize = 1;
const ASCII_DIF: i32 = 48;

///? Laberinto es la estructura destinada a manejar el tablero del juego.
pub struct Laberinto {
    tablero: Vec<Vec<Casillero>>,
}

impl Laberinto {
    ///? Crea un laberinto lleno de Vacio y lo devuelve.
    /// # Arguments
    /// * `dimension_tablero` - Dimension del tablero.
    pub fn new(dimension_tablero: usize) -> Laberinto {
        let mut tablero: Vec<Vec<Casillero>> = Vec::new();

        for i in 0..dimension_tablero {
            let mut fila: Vec<Casillero> = Vec::new();

            for j in 0..dimension_tablero {
                fila.push(Casillero::Vacio(Coordenada::new(i, j)));
            }

            tablero.push(fila);
        }

        Laberinto { tablero }
    }

    ///? Crea objetos representados unicamente por un unico caracter.
    /// # Arguments
    /// * `caracter` - Caracter que representa el objeto.
    /// * `coordenada_objeto` - Coordenada del objeto.
    /// # Returns
    /// * `Ok(Casillero)` - Casillero creado.
    /// * `Err(String)` - Error.
    /// # Errors
    /// * `ERROR: Caracter representado no valido` - En caso de que el caracter no sea valido.
    fn crear_objeto_un_caracter(
        &mut self,
        caracter: &str,
        coordenada_objeto: Coordenada,
    ) -> Result<Casillero, String> {
        let mut result: Result<Casillero, String> =
            Err("ERROR: Caracter representado no valido".to_string());

        if caracter == "_" {
            result = Ok(Casillero::Vacio(coordenada_objeto));
        } else if caracter == "R" {
            result = Ok(Casillero::Roca(coordenada_objeto));
        } else if caracter == "W" {
            result = Ok(Casillero::Pared(coordenada_objeto));
        };

        result
    }

    ///? Crea una bomba normal a partir del segundo caracter recibido, o devuelve error.
    /// # Arguments
    /// * `segundo_caracter` - Segundo caracter que representa el alcance de la bomba.
    /// * `coordenada_objeto` - Coordenada del objeto.
    /// * `id_bomba_actual` - Id de la bomba.
    /// # Returns
    /// * `Ok(Casillero)` - Casillero creado.
    /// * `Err(String)` - Error.
    /// # Errors
    /// * `ERROR: El alcance de la bomba  no puede ser menor a 1 o mayor a 9` - En caso de que el alcance sea menor a 1 o mayor a 9.
    fn crear_bomba_normal(
        segundo_caracter: u8,
        coordenada_objeto: Coordenada,
        id_bomba_actual: &mut i32,
    ) -> Result<Casillero, String> {
        if (segundo_caracter as i32 - ASCII_DIF < 1) || (segundo_caracter as i32 - ASCII_DIF > 9) {
            return Err("ERROR: El alcance de la bomba no puede ser menor a 1 o mayor a 9".to_string());
        }

        let bomba = Ok(Casillero::BombaNormal(
            coordenada_objeto,
            segundo_caracter as i32 - ASCII_DIF,
            *id_bomba_actual,
        ));

        *id_bomba_actual += 1;

        bomba
    }

    ///? Crea una bomba traspaso a partir del segundo caracter recibido, o devuelve error.
    /// # Arguments
    /// * `segundo_caracter` - Segundo caracter que representa el alcance de la bomba.
    /// * `coordenada_objeto` - Coordenada del objeto.
    /// * `id_bomba_actual` - Id de la bomba.
    /// # Returns
    /// * `Ok(Casillero)` - Casillero creado.
    /// * `Err(String)` - Error.
    /// # Errors
    /// * `ERROR: El alcance de la bomba traspaso no puede ser menor a 1 o mayor a 9` - En caso de que el alcance sea menor a 1 o mayor a 9.
    fn crear_bomba_traspaso(
        segundo_caracter: u8,
        coordenada_objeto: Coordenada,
        id_bomba_actual: &mut i32,
    ) -> Result<Casillero, String> {
        if (segundo_caracter as i32 - ASCII_DIF < 1) || (segundo_caracter as i32 - ASCII_DIF > 9) {
            return Err(
                "ERROR: El alcance de la bomba traspaso no puede ser menor a 1 o mayor a 9".to_string(),
            );
        }

        let bomba = Ok(Casillero::BombaTraspaso(
            coordenada_objeto,
            segundo_caracter as i32 - ASCII_DIF,
            *id_bomba_actual,
        ));

        *id_bomba_actual += 1;

        bomba
    }

    ///? Crea un enemigo a partir del segundo caracter recibido, o devuelve error.
    /// # Arguments
    /// * `segundo_caracter` - Segundo caracter que representa la vida del enemigo.
    /// * `coordenada_objeto` - Coordenada del objeto.
    /// # Returns
    /// * `Ok(Casillero)` - Casillero creado.
    /// * `Err(String)` - Error.
    /// # Errors
    /// * `ERROR: La vida del enemigo no puede ser menor a 1 ni mayor a 3` - En caso de que la vida sea menor a 1 o mayor a 3.
    fn crear_enemigo(
        segundo_caracter: u8,
        coordenada_objeto: Coordenada,
    ) -> Result<Casillero, String> {
        if (segundo_caracter as i32 - ASCII_DIF < 1) || (segundo_caracter as i32 - ASCII_DIF > 3) {
            return Err(
                "ERROR: La vida del enemigo no puede ser menor a 1 ni mayor a 3".to_string(),
            );
        }

        let enemigo = Enemigo::new(segundo_caracter as i32 - ASCII_DIF);

        Ok(Casillero::Enemigoo(coordenada_objeto, enemigo, Vec::new()))
    }

    ///? Crea un desvio a partir del segundo caracter recibido, o devuelve error.
    /// # Arguments
    /// * `segundo_caracter` - Segundo caracter que representa la direccion del desvio.
    /// * `coordenada_objeto` - Coordenada del objeto.
    /// # Returns
    /// * `Ok(Casillero)` - Casillero creado.
    /// * `Err(String)` - Error.
    /// # Errors
    /// * `ERROR: La direccion del desvio no es valida` - En caso de que la direccion no sea valida.
    /// # Examples
    /// ```
    /// use bomberman::laberinto::Laberinto;
    /// use bomberman::coordenada::Coordenada;
    /// let desvio = Laberinto::crear_desvio(85, Coordenada::new(1, 1));
    /// assert!(desvio.is_ok());
    /// ```
    fn crear_desvio(
        segundo_caracter: u8,
        coordenada_objeto: Coordenada,
    ) -> Result<Casillero, String> {
        let direccion = String::from(segundo_caracter as char);
        if direccion != "U" && direccion != "D" && direccion != "L" && direccion != "R" {
            return Err("ERROR: La direccion del desvio no es valida".to_string());
        }

        Ok(Casillero::Desvio(coordenada_objeto, direccion))
    }

    ///? Crea objetos representados únicamente por dos caracteres.
    /// # Arguments
    /// * `parte` - Parte del string que representa el objeto.
    /// * `coordenada_objeto` - Coordenada del objeto.
    /// * `id_bomba_actual` - Id de la bomba.
    /// # Returns
    /// * `Ok(Casillero)` - Casillero creado.
    /// * `Err(String)` - Error.
    /// # Errors
    /// * `ERROR: Caracter representado no valido` - En caso de que el caracter no sea valido.
    /// # Examples
    /// ```
    /// use bomberman::laberinto::Laberinto;
    /// use bomberman::coordenada::Coordenada;
    /// let objeto = Laberinto::crear_objeto_dos_caracteres("B2", Coordenada::new(1, 1), &mut 1);
    /// assert!(objeto.is_ok());
    /// ```
    fn crear_objeto_dos_caracteres(
        &mut self,
        parte: &str,
        coordenada_objeto: Coordenada,
        id_bomba_actual: &mut i32,
    ) -> Result<Casillero, String> {
        let mut result: Result<Casillero, String> =
            Err("ERROR: Caracter representado no valido".to_string());
        let segundo_caracter = parte.as_bytes()[1];

        if let Some(primer_caracter) = parte.chars().next() {
            if primer_caracter == 'B' {
                result = Laberinto::crear_bomba_normal(
                    segundo_caracter,
                    coordenada_objeto,
                    id_bomba_actual,
                )
            } else if primer_caracter == 'S' {
                result = Laberinto::crear_bomba_traspaso(
                    segundo_caracter,
                    coordenada_objeto,
                    id_bomba_actual,
                )
            } else if primer_caracter == 'F' {
                result = Laberinto::crear_enemigo(segundo_caracter, coordenada_objeto)
            } else if primer_caracter == 'D' {
                result = Laberinto::crear_desvio(segundo_caracter, coordenada_objeto)
            }
        };

        result
    }

    ///? Crea  el objeto correspondiente y lo agrega al mapa.
    /// # Arguments
    /// * `parte` - Parte del string que representa el objeto.
    /// * `coordenada_x` - Coordenada x del objeto.
    /// * `coordenada_y` - Coordenada y del objeto.
    /// * `id_bomba_actual` - Id de la bomba.
    /// # Returns
    /// * `Ok(())` - Casillero creado.
    /// * `Err(String)` - Error.
    /// # Errors
    /// * `ERROR: Caracter representado no valido` - En caso de que el caracter no sea valido.
    /// # Examples
    /// ```
    /// use bomberman::laberinto::Laberinto;
    /// use bomberman::coordenada::Coordenada;
    /// let mut lab = Laberinto::new(3);
    /// let objeto = lab.crear_objeto_correspondiente("B2", 1, 1, &mut 1);
    /// assert!(objeto.is_ok());
    /// ```
    pub fn crear_objeto_correspondiente(
        &mut self,
        parte: &str,
        coordenada_x: usize,
        coordenada_y: usize,
        id_bomba_actual: &mut i32,
    ) -> Result<(), String> {
        let coordenada_casillero = Coordenada::new(coordenada_x, coordenada_y);
        let coordenada_casillero_copia = coordenada_casillero.clone();

        let objeto = if parte.len() == UN_CARACTER {
            self.crear_objeto_un_caracter(parte, coordenada_casillero)?
        } else {
            self.crear_objeto_dos_caracteres(parte, coordenada_casillero, id_bomba_actual)?
        };

        self.reemplazar_objeto_en_tablero(objeto.clone(), &coordenada_casillero_copia);

        Ok(())
    }

    ///? Recibe un vector de strings, donde cada string representa una fila del laberinto y cada caracter representa un objeto.
    ///? A partir de estos datos, actualiza el tablero.
    /// # Arguments
    /// * `datos` - Vector de strings que representa el laberinto.
    /// # Returns
    /// * `Ok(())` - Casillero creado.
    /// * `Err(String)` - Error.
    /// # Errors
    /// * `ERROR: Caracter representado no valido` - En caso de que el caracter no sea valido.
    /// # Examples
    /// ```
    /// use bomberman::laberinto::Laberinto;
    /// let mut lab = Laberinto::new(3);
    /// let datos = vec![
    ///    "R _ _".to_string(),
    ///   "_ W _".to_string(),
    ///   "_ _ _".to_string(),
    /// ];
    /// assert!(lab.inicializar_laberinto_con_datos(datos).is_ok());
    /// ```
    pub fn inicializar_laberinto_con_datos(&mut self, datos: Vec<String>) -> Result<(), String> {
        let mut id_bomba_actual = 1;

        for (coordenada_y, dato) in datos.iter().enumerate() {
            let partes = dato.split_whitespace().collect::<Vec<&str>>();

            for (coordenada_x, parte) in partes.iter().enumerate() {
                self.crear_objeto_correspondiente(
                    parte,
                    coordenada_x,
                    coordenada_y,
                    &mut id_bomba_actual,
                )?;
            }
        }

        Ok(())
    }

    ///? Devuelve la visualizacion del estado actual del laberinto.
    /// # Returns
    /// * `Vec<Vec<String>>` - Visualizacion del laberinto.
    /// # Examples
    /// ```
    /// use bomberman::laberinto::Laberinto;
    /// let mut lab = Laberinto::new(3);
    /// let datos = vec![
    ///   "R _ _".to_string(),
    ///  "_ W _".to_string(),
    /// "_ _ _".to_string(),
    /// ];
    /// lab.inicializar_laberinto_con_datos(datos).unwrap();
    /// let visualizacion = lab.obtener_visualizacion();
    /// assert_eq!(visualizacion[0][0], "R");
    /// assert_eq!(visualizacion[0][2], "_");
    /// assert_eq!(visualizacion[1][2], "W");
    /// ```
    pub fn obtener_visualizacion(&self) -> Vec<Vec<String>> {
        let mut tablero_visualizacion: Vec<Vec<String>> = Vec::new();

        for i in 0..self.tablero.len() {
            let mut fila: Vec<String> = Vec::new();

            for j in 0..self.tablero.len() {
                fila.push(self.tablero[i][j].obtener_representacion());
                fila.push(" ".to_string());
            }

            tablero_visualizacion.push(fila);
        }

        tablero_visualizacion
    }

    ///? Reemplaza el casillero ubicado en las coordenadas del casillero recibido por el casillero recibido.
    /// # Arguments
    /// * `casillero` - Casillero a reemplazar.
    /// * `coordenada` - Coordenada del casillero a reemplazar.
    /// # Examples
    /// ```
    /// use bomberman::laberinto::Laberinto;
    /// use bomberman::casillero::Casillero;
    /// use bomberman::coordenada::Coordenada;
    /// let mut lab = Laberinto::new(3);
    /// let bomba = Laberinto::crear_bomba_normal(49, Coordenada::new(1, 1), &mut 1).unwrap();
    /// lab.reemplazar_objeto_en_tablero(bomba, &Coordenada::new(1, 1));
    /// ```
    pub fn reemplazar_objeto_en_tablero(&mut self, casillero: Casillero, coordenada: &Coordenada) {
        if self.coordenadas_fuera_de_rango(coordenada) {
            return;
        }

        self.tablero[coordenada.get_y()][coordenada.get_x()] = casillero;
    }

    ///? Devuelve true en caso de que las coordenadas recibidas esten fuera del tablero, false caso contrario.
    /// # Arguments
    /// * `coordenada` - Coordenada a chequear.
    /// # Returns
    /// * `bool` - True en caso de que las coordenadas esten fuera del tablero, false caso contrario.
    /// # Examples
    /// ```
    /// use bomberman::laberinto::Laberinto;
    /// use bomberman::coordenada::Coordenada;
    /// let lab = Laberinto::new(3);
    /// let coordenada = Coordenada::new(4, 3);
    /// assert!(lab.coordenadas_fuera_de_rango(&coordenada));
    /// ```
    pub fn coordenadas_fuera_de_rango(&self, coordenada: &Coordenada) -> bool {
        coordenada.get_x() >= self.tablero.len() || coordenada.get_y() >= self.tablero.len()
    }

    ///? Devuelve el objeto ubicado en las coordenadas recibidas.
    /// # Arguments
    /// * `coordenada_buscada` - Coordenada del objeto a buscar.
    /// # Returns
    /// * `Casillero` - Casillero ubicado en las coordenadas recibidas.
    /// # Examples
    /// ```
    /// use bomberman::laberinto::Laberinto;
    /// use bomberman::casillero::Casillero;
    /// use bomberman::coordenada::Coordenada;
    /// let mut lab = Laberinto::new(3);
    /// let bomba = Laberinto::crear_bomba_normal(49, Coordenada::new(1, 1), &mut 1).unwrap();
    /// lab.reemplazar_objeto_en_tablero(bomba, &Coordenada::new(1, 1));
    /// let objeto = lab.obtener_objeto(&Coordenada::new(1, 1));
    /// assert_eq!(objeto, Casillero::BombaNormal(Coordenada::new(1, 1), 1, 1));
    /// ```
    fn obtener_objeto(&mut self, coordenada_buscada: &Coordenada) -> Casillero {
        self.tablero[coordenada_buscada.get_y()][coordenada_buscada.get_x()].clone()
    }

    ///? Detonar el objeto ubicado en las coordenadas recibidas. Devuelve un error en caso de que no se pueda detonar, o el ResultadoRafaga de la ultima rafaga en caso de todo OK.
    /// # Arguments
    /// * `coordenada_a_detonar` - Coordenada del objeto a detonar.
    /// # Returns
    /// * `Result<ResultadoRafaga, String>` - ResultadoRafaga de la ultima rafaga en caso de todo OK, o un error en caso de que no se pueda detonar.
    /// # Errors
    /// * `ERROR: No se puede detonar fuera del mapa!` - En caso de que la coordenada a detonar este fuera del mapa.
    /// # Examples
    /// ```
    /// use bomberman::laberinto::Laberinto;
    /// use bomberman::coordenada::Coordenada;
    /// use bomberman::resultado_rafaga::ResultadoRafaga;
    /// use bomberman::resultado_rafaga::ResultadoRafaga::Insignificante;
    /// let mut lab = Laberinto::new(3);
    /// let bomba = Laberinto::crear_bomba_normal(49, Coordenada::new(1, 1), &mut 1).unwrap();
    /// lab.reemplazar_objeto_en_tablero(bomba, &Coordenada::new(1, 1));
    /// let resultado_detonacion = lab.detonar_objeto(Coordenada::new(1, 1));
    /// assert_eq!(resultado_detonacion.unwrap(), Insignificante);
    /// ```
    pub fn detonar_objeto(
        &mut self,
        coordenada_a_detonar: Coordenada,
    ) -> Result<ResultadoRafaga, String> {
        if self.coordenadas_fuera_de_rango(&coordenada_a_detonar) {
            return Err("No se puede detonar fuera del mapa!".to_string());
        }

        let objeto = self.obtener_objeto(&coordenada_a_detonar);
        objeto.detonar(self)
    }

    ///? Ordena al objeto correspondiente que reciba la rafaga, aplica las consecuencias y devuelve el ResultadoRafaga en caso de OK, un string en caso de Err.
    /// # Arguments
    /// * `coordenada_a_rafagear` - Coordenada del objeto a rafagear.
    /// * `id_bomba_rafageadora` - Id de la bomba que rafagea.
    /// # Returns
    /// * `Result<ResultadoRafaga, String>` - ResultadoRafaga en caso de OK, un string en caso de Err.
    /// # Errors
    /// * `ERROR: No se puede rafagear fuera del mapa!` - En caso de que la coordenada a rafagear este fuera del mapa.
    /// # Examples
    /// ```
    /// use bomberman::laberinto::Laberinto;
    /// use bomberman::coordenada::Coordenada;
    /// use bomberman::resultado_rafaga::ResultadoRafaga;
    /// use bomberman::resultado_rafaga::ResultadoRafaga::EnemigoTocado;
    /// let mut lab = Laberinto::new(3);
    /// let enemigo = Laberinto::crear_enemigo(51, Coordenada::new(1, 1)).unwrap();
    /// lab.reemplazar_objeto_en_tablero(enemigo, &Coordenada::new(1, 1));
    /// let resultado_rafaga = lab.rafagear_coordenada(&Coordenada::new(1, 1), &1);
    /// assert_eq!(resultado_rafaga.unwrap(), EnemigoTocado(2));
    /// ```
    pub fn rafagear_coordenada(
        &mut self,
        coordenada_a_rafagear: &Coordenada,
        id_bomba_rafageadora: &i32,
    ) -> Result<ResultadoRafaga, String> {
        if self.coordenadas_fuera_de_rango(coordenada_a_rafagear) {
            return Ok(ResultadoRafaga::ChoqueFuerte);
        }

        let mut resultado_rafaga = Ok(self.tablero[coordenada_a_rafagear.get_y()]
            [coordenada_a_rafagear.get_x()]
        .recibir_rafaga());
        let coordenada_rafageada = coordenada_a_rafagear.clone();
        let resultado_rafaga_copia = resultado_rafaga.clone()?;

        if resultado_rafaga_copia == Detonacion {
            resultado_rafaga = self.detonar_objeto(coordenada_rafageada);
        } else if resultado_rafaga_copia == EnemigoEliminado {
            if !self.tablero[coordenada_a_rafagear.get_y()][coordenada_a_rafagear.get_x()]
                .ya_recibio_rafaga_de_bomba_actual(id_bomba_rafageadora)
                .unwrap_or(false)
            {
                self.reemplazar_objeto_en_tablero(
                    Casillero::Vacio(coordenada_rafageada),
                    coordenada_a_rafagear,
                );
            }
        } else if (resultado_rafaga_copia == EnemigoTocado(1)
            || resultado_rafaga_copia == EnemigoTocado(2))
            && !self.tablero[coordenada_a_rafagear.get_y()][coordenada_a_rafagear.get_x()]
                .ya_recibio_rafaga_de_bomba_actual(id_bomba_rafageadora)
                .unwrap_or(false)
        {
            let enemigo_nuevo = Enemigo::new(resultado_rafaga_copia.get_vida_enemigo());
            let mut ids_bombas_recibidas = self.tablero[coordenada_a_rafagear.get_y()]
                [coordenada_a_rafagear.get_x()]
            .get_ids_bombas_sufridas()
            .unwrap_or(Vec::new());
            ids_bombas_recibidas.push(*id_bomba_rafageadora);
            self.reemplazar_objeto_en_tablero(
                Casillero::Enemigoo(coordenada_rafageada, enemigo_nuevo, ids_bombas_recibidas),
                coordenada_a_rafagear,
            );
        }

        resultado_rafaga
    }
}

#[cfg(test)]
mod tests {
    use crate::coordenada::Coordenada;
    use crate::laberinto::Laberinto;
    use crate::resultado_rafaga::ResultadoRafaga;

    #[test]
    fn test_detonar() {
        let mut laberinto = Laberinto::new(3);
        let bomba = Laberinto::crear_bomba_normal(49, Coordenada::new(1, 1), &mut 1).unwrap();
        laberinto.reemplazar_objeto_en_tablero(bomba, &Coordenada::new(1, 1));
        let resultado_detonacion = laberinto.detonar_objeto(Coordenada::new(1, 1));
        assert_eq!(
            resultado_detonacion.unwrap(),
            ResultadoRafaga::Insignificante
        );
    }

    #[test]
    fn test_rafagear() {
        let mut laberinto = Laberinto::new(3);
        let enemigo = Laberinto::crear_enemigo(51, Coordenada::new(1, 1)).unwrap();
        laberinto.reemplazar_objeto_en_tablero(enemigo, &Coordenada::new(1, 1));
        let resultado_rafaga = laberinto.rafagear_coordenada(&Coordenada::new(1, 1), &1);
        assert_eq!(resultado_rafaga.unwrap(), ResultadoRafaga::EnemigoTocado(2));
    }

    #[test]
    fn test_chequear_coordenadas_fuera_de_rango() {
        let laberinto = Laberinto::new(3);
        let coordenada = Coordenada::new(4, 3);
        assert!(laberinto.coordenadas_fuera_de_rango(&coordenada));
    }

    #[test]
    fn test_chequear_coordenadas_dentro_de_rango() {
        let laberinto = Laberinto::new(3);
        let coordenada = Coordenada::new(2, 2);
        assert!(!laberinto.coordenadas_fuera_de_rango(&coordenada));
    }

    #[test]
    fn test_chequear_inicializacion_con_datos() {
        let mut lab = Laberinto::new(3);
        let datos = vec![
            "R _ _".to_string(),
            "_ W _".to_string(),
            "_ _ _".to_string(),
        ];
        assert!(lab.inicializar_laberinto_con_datos(datos).is_ok());

        let mut lab2 = Laberinto::new(3);
        let datos = vec![
            "R _ _".to_string(),
            "_ Z _".to_string(),
            "_ _ _".to_string(),
        ];
        assert!(lab2.inicializar_laberinto_con_datos(datos).is_err());
    }

    #[test]
    fn test_chequear_visualizacion_datos() {
        let mut lab = Laberinto::new(3);
        let datos = vec![
            "R _ _".to_string(),
            "_ W _".to_string(),
            "_ _ _".to_string(),
        ];
        lab.inicializar_laberinto_con_datos(datos).unwrap();
        let visualizacion = lab.obtener_visualizacion();
        assert_eq!(visualizacion[0][0], "R");
        assert_eq!(visualizacion[0][2], "_");
        assert_eq!(visualizacion[1][2], "W");
    }

    #[test]
    fn test_crear_objeto_un_caracter() {
        let mut lab = Laberinto::new(3);
        let caracter = "P";
        let casillero = lab.crear_objeto_un_caracter(caracter, Coordenada::new(2, 2));
        let es_error = casillero.is_err();
        assert!(es_error);
    }
}
