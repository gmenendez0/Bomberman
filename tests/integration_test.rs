#[path = "../src/laberinto.rs"]
mod laberinto;
use laberinto::Laberinto;

#[path = "../src/casillero.rs"]
mod casillero;

#[path = "../src/resultado_rafaga.rs"]
mod resultado_rafaga;

#[path = "../src/enemigo.rs"]
mod enemigo;

#[path = "../src/coordenada.rs"]
mod coordenada;
use coordenada::Coordenada;

#[path = "../src/file_handler.rs"]
mod file_handler;
use file_handler::FileHandler;

#[test]
fn integration1(){
    let mut lab = Laberinto::new(7);

    let tablero_inicial = vec![
        String::from("B2 R R _ F1 _ _"),
        String::from("_ W R W _ W _"),
        String::from("B5 _ _ _ B2 _ _"),
        String::from("_ W _ W _ W _"),
        String::from("_ _ _ _ _ _ _"),
        String::from("_ W _ W _ W _"),
        String::from("_ _ _ _ _ _ _"),
    ];

    lab.inicializar_laberinto_con_datos(tablero_inicial).expect("TODO: panic message");
    lab.detonar_objeto(Coordenada::new(0, 0)).expect("TODO: panic message");
    let visualizacion = lab.obtener_visualizacion();

    let file_handler = FileHandler::new("tests/lab_test.txt".to_string(), "tests/lab_test.txt".to_string());
    file_handler.write(visualizacion).expect("TODO: panic message");

    let tablero_esperado = vec![
        String::from("_ R R _ _ _ _ "),
        String::from("_ W R W _ W _ "),
        String::from("_ _ _ _ _ _ _ "),
        String::from("_ W _ W _ W _ "),
        String::from("_ _ _ _ _ _ _ "),
        String::from("_ W _ W _ W _ "),
        String::from("_ _ _ _ _ _ _ "),
    ];

    let contenido_archivo = file_handler.read().unwrap();

    assert_eq!(tablero_esperado, contenido_archivo);
}