const CANT_ARGS: usize = 5;
const POSICION_X: usize = 3;
const POSICION_Y: usize = 4;
const POSICION_NOMBRE_ARCHIVO: usize = 1;
const POSICION_PATH: usize = 2;

//? ArgHandler es el encargado de manejar los argumentos recibidos por el programa.
pub struct ArgHandler {
    args: Vec<String>,
}

impl ArgHandler {
    //? Crea un nuevo ArgHandler a partir de los argumentos recibidos por el programa.
    pub(crate) fn new(args: Vec<String>) -> ArgHandler {
        ArgHandler { args }
    }

    //? Chequea que la cantidad de argumentos recibidos sea la esperada.
    pub(crate) fn chequear_cant_args(&self) -> Result<(), String> {
        if self.args.len() != CANT_ARGS {
            return Err("ERROR: Se esperaban 4 argumentos pero se recibió otra cantidad. Cerrando el programa...".to_string());
        }
        Ok(())
    }

    //? Parsea el argumento 3 a un i32.
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

    //? Parsea el argumento 4 a un i32.
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

    //? Concatena el path recibido (2do argumento) con el nombre del archivo recibido (1er argumento).
    pub fn concatenar_path_y_nombre_archivo(&self) -> String {
        let mut path = self.args[POSICION_PATH].clone();
        path.push('/');
        path.push_str(&self.args[POSICION_NOMBRE_ARCHIVO]);
        path
    }
}

#[cfg(test)]
mod tests {
    use crate::arg_handler::ArgHandler;

    #[test]
    fn test_chequear_cant_args() {
        let args = vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string(), "arg4".to_string(), "arg5".to_string()];
        let arg_handler = ArgHandler::new(args);
        assert!(arg_handler.chequear_cant_args().is_ok());

        let args = vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string(), "arg4".to_string()];
        let arg_handler = ArgHandler::new(args);
        assert!(arg_handler.chequear_cant_args().is_err());
    }

    #[test]
    fn test_parse_x() {
        let args = vec!["arg1".to_string(), "arg2".to_string(), "123".to_string(), "456".to_string(), "arg5".to_string()];
        let arg_handler = ArgHandler::new(args);
        assert_eq!(arg_handler.parse_x(), Ok(456));

        let args = vec!["arg1".to_string(), "arg2".to_string(), "not_an_integer".to_string(), "dfgdfg".to_string(), "arg5".to_string()];
        let arg_handler = ArgHandler::new(args);
        assert!(arg_handler.parse_x().is_err());
    }

    #[test]
    fn test_parse_y() {
        let args = vec!["arg1".to_string(), "arg2".to_string(), "123".to_string(), "456".to_string(), "456".to_string()];
        let arg_handler = ArgHandler::new(args);
        assert_eq!(arg_handler.parse_y(), Ok(456));

        let args = vec!["arg1".to_string(), "arg2".to_string(), "123".to_string(), "not_an_integer".to_string(), "not_an_integer".to_string()];
        let arg_handler = ArgHandler::new(args);
        assert!(arg_handler.parse_y().is_err());
    }

    #[test]
    fn test_concatenar_path_y_nombre_archivo() {
        let args = vec!["arg1".to_string(), "file.txt".to_string(), "path_cualquiera".to_string(), "435".to_string(), "arg5".to_string()];
        let arg_handler = ArgHandler::new(args);
        assert_eq!(arg_handler.concatenar_path_y_nombre_archivo(), "path_cualquiera/file.txt");
    }
}
