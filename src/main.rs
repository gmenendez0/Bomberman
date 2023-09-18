use crate::laberinto::Laberinto;

mod arg_handler;
mod bomba_normal;
mod bomba_traspaso;
mod coordenada;
mod desvio;
mod enemigo;
mod file_handler;
mod laberinto;
mod objeto_mapa;
mod pared;
mod roca;
mod vacio;

fn main() -> Result<(), String> {
    let arg_handler = arg_handler::ArgHandler::new(std::env::args().collect());
    arg_handler.chequear_cant_args()?;
    let coordenada_bomba_a_detonar = arg_handler.get_coordenada_bomba_a_detonar()?;
    let full_path = arg_handler.concatenar_path_y_nombre_archivo();

    let file_handler = file_handler::FileHandler::new(full_path);
    let contenido_archivo = file_handler.read()?;

    let mut lab = Laberinto::new(contenido_archivo.len());
    let resultado_inicializacion = lab.inicializar_laberinto_con_datos(contenido_archivo);
    if resultado_inicializacion.is_err() {
        file_handler.write_error(resultado_inicializacion.err().unwrap_or("".to_string()))?;
        return Err("Se ha detallado el error en el archivo.".to_string());
    };

    lab.detonar_objeto(coordenada_bomba_a_detonar)?;

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
