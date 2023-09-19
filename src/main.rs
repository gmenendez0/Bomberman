use crate::laberinto::Laberinto;

mod arg_handler;
mod casillero;
mod coordenada;
mod enemigo;
mod file_handler;
mod laberinto;
mod resultado_rafaga;

// * PARA CORRECTOR: Al compilar, arroja un warning de que el crate Bomberman debe estar en snake_case pero no encuentr dicho crate para cambiarle el nombre.
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
        return Err("Se ha detallado el error de inicializacion en el archivo.".to_string());
    };

    let resultado_detonacion = lab.detonar_objeto(coordenada_bomba_a_detonar);
    match resultado_detonacion {
        Ok(_) => (),
        Err(e) => {
            file_handler.write_error(e)?;
            return Err("Se ha detallado el error de detonacion en el archivo.".to_string());
        }
    }

    file_handler.write(lab.obtener_visualizacion())?;

    Ok(())
}
