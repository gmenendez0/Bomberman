const CANT_ARGS: usize = 5;
const POSICION_X: usize = 3;
const POSICION_Y: usize = 4;
const POSICION_NOMBRE_ARCHIVO: usize = 1;
const POSICION_PATH: usize = 2;

pub struct ArgHandler {
    args: Vec<String>,
}

impl ArgHandler {
    pub(crate) fn new(args: Vec<String>) -> ArgHandler {
        ArgHandler { args }
    }

    pub(crate) fn chequear_cant_args(&self) -> Result<(), String> {
        if self.args.len() != CANT_ARGS {
            return Err("ERROR: Se esperaban 4 argumentos pero se recibió otra cantidad. Cerrando el programa...".to_string());
        }
        Ok(())
    }

    pub fn parse_x(&self) -> Result<i32, String> {
        let x = match self.args[POSICION_X].parse::<i32>() {
            Ok(x) => x,
            Err(_) => {
                return Err(
                    "ERROR: El argumento 3 debe ser un numero entero. Cerrando el programa..."
                        .to_string(),
                )
            }
        };

        Ok(x)
    }

    pub fn parse_y(&self) -> Result<i32, String> {
        let y = match self.args[POSICION_Y].parse::<i32>() {
            Ok(y) => y,
            Err(_) => {
                return Err(
                    "ERROR: El argumento 4 debe ser un numero entero. Cerrando el programa..."
                        .to_string(),
                )
            }
        };

        Ok(y)
    }

    pub fn concatenar_path_y_nombre_archivo(&self) -> String {
        let mut path = self.args[POSICION_PATH].clone();
        path.push('/');
        path.push_str(&self.args[POSICION_NOMBRE_ARCHIVO]);
        path
    }
}
