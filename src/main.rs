use crate::laberinto::Laberinto;

mod arg_handler;
mod coordenada;
mod enemigo;
mod file_handler;
mod laberinto;
mod casillero;

fn main() -> Result<(), String> {
    //? Consigo los argumentos. Path del archivo buscado y coordenadas a explotar.                                             --FUNCIONA OK
    let arg_handler = arg_handler::ArgHandler::new(std::env::args().collect());
    arg_handler.chequear_cant_args()?;
    let coordenada_bomba_a_detonar = arg_handler.get_coordenada_bomba_a_detonar()?;
    let full_path = arg_handler.concatenar_path_y_nombre_archivo();

    //? Consigo los datos del archivo                                                                                          --FUNCIONA OK
    let file_handler = file_handler::FileHandler::new(full_path);
    let contenido_archivo = file_handler.read()?;

    //? Creo un laberinto. Al crearlo se inicializa con todos los casilleros de tipo Vacio.                                    --FUNCIONA OK
    let mut lab = Laberinto::new(contenido_archivo.len());

    //? Lo inicializo con los datos leidos del archivo.                                                                        --FUNCIONA OK
    let resultado_inicializacion = lab.inicializar_laberinto_con_datos(contenido_archivo);

    //? Chequeo si hubo un error en la inicializacion previa. Si hubo, imprimo en el archivo y devuelvo error por consola.     --FUNCIONA OK
    if resultado_inicializacion.is_err() {
        file_handler.write_error(resultado_inicializacion.err().unwrap_or("".to_string()))?;
        return Err("Se ha detallado el error en el archivo.".to_string());
    };

    //? Le digo al laberinto que detone las coordenadas que recibi por consola.                                                 --POR TERMINAR!
    lab.detonar_objeto(coordenada_bomba_a_detonar)?;

    //? Imprimo la representacion del laberinto en el archivo.                                                                  --FUNCIONA OK
    file_handler.write(lab.obtener_visualizacion())?;





    //? FOR TESTING PURPOSES
    /*
    let hola = juego.obtener_visualizacion();
    for i in 0..hola.len() {
        for j in 0..hola[i].len() {
            print!("{}", hola[i][j]);
        }
        println!();
    }
*/

    Ok(())
}
