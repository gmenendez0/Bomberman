use crate::juego::Juego;

mod arg_handler;
mod bomba_normal;
mod bomba_traspaso;
mod casillero;
mod coordenada;
mod desvio;
mod enemigo;
mod file_handler;
mod juego;
mod laberinto;
mod objeto_mapa;
mod pared;
mod roca;
mod vacio;

fn main() -> Result<(), String> {
    //cargo run -- maze.txt /path/to/output_dir/ x y

    let args: Vec<String> = std::env::args().collect();
    let arg_handler = arg_handler::ArgHandler::new(args);

    arg_handler.chequear_cant_args()?;
    let coordenada_bomba_a_detonar = coordenada::Coordenada::new(
        arg_handler.parse_x()? as usize,
        arg_handler.parse_y()? as usize,
    );
    let full_path = arg_handler.concatenar_path_y_nombre_archivo();

    let file_handler = file_handler::FileHandler::new(full_path);
    let contenido_archivo = file_handler.read()?;

    let mut juego = Juego::new(contenido_archivo.len());
    println!("Contenido del archivo: {:?}", contenido_archivo);


    let resultado = juego.inicializar_laberinto_con_datos(contenido_archivo) ?;
    let hola = juego.obtener_visualizacion();

    //? FOR TESTING PURPOSES
/*    println!("Contenido del archivo: {:?}", contenido_archivo);

    let mut lab = laberinto::Laberinto::new(contenido_archivo.len());
    let hola = lab.obtener_visualizacion();*/

    for i in 0..hola.len() {
        for j in 0..hola[i].len() {
            print!("{}", hola[i][j]);
        }
        println!();
    }

    //lab.detonar_coordenada(&coordenada_bomba_a_detonar)?;

    Ok(())
}
