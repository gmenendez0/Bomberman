use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub struct FileHandler {
    target: String,
}

impl FileHandler {
    pub fn new(target: String) -> FileHandler {
        FileHandler { target }
    }

    pub fn crear_archivo(&self) -> Result<(), String> {
        let archivo = match File::create(&self.target) {
            Ok(archivo) => archivo,
            Err(e) => return Err(e.to_string()),
        };

        let mut escritor = BufWriter::new(archivo);
        match escritor.write(b"ERROR: Se ha creado este archivo ya que no se ha podido abrir el archivo target especificado.") {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn read(&self) -> Result<Vec<String>, String> {
        let archivo = match File::open(&self.target) {
            Ok(archivo) => archivo,
            Err(_) => {
                self.crear_archivo()?;
                return Err("Error al abrir el archivo. Se ha creado uno y detallado el error dentro de el.".to_string());
            }
        };

        let lector = BufReader::new(archivo);
        let mut lineas = Vec::new();

        for line in lector.lines() {
            match line {
                Ok(linea) => lineas.push(linea),
                Err(e) => return Err(e.to_string()),
            }
        }

        Ok(lineas)
    }

    pub fn write(&self, data: Vec<Vec<String>>) -> Result<(), String> {
        let archivo = match File::open(&self.target) {
            Ok(archivo) => archivo,
            Err(e) => return Err(e.to_string()),
        };
        let escritor = BufWriter::new(archivo);

        //? Debe implementarse...

        Ok(())
    }
}
