use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub struct FileHandler {
    target: String,
}

impl FileHandler {
    pub fn new(target: String) -> FileHandler {
        FileHandler { target }
    }

    pub fn crear_archivo(&self) -> Result<File, String> {
        let result: Result<File, String>;

        match File::create(&self.target) {
            Ok(archivo) => result = Ok(archivo),
            Err(e) => result = Err(e.to_string()),
        }

        result
    }

    pub fn archivo_no_encontrado(&self) -> Result<(), String> {
        let archivo = self.crear_archivo()?;

        let mut escritor = BufWriter::new(archivo);
        match escritor.write(b"ERROR: Se ha creado este archivo ya que no se ha podido abrir el archivo target especificado.") {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn abrir_archivo(&self) -> Result<File, String>{
        let archivo = match File::open(&self.target) {
            Ok(archivo) => archivo,
            Err(_) => {
                self.archivo_no_encontrado()?;
                return Err("Error al abrir el archivo. Se ha creado uno y detallado el error dentro de el.".to_string());
            }
        };

        Ok(archivo)
    }

    pub fn read(&self) -> Result<Vec<String>, String> {
        let archivo = self.abrir_archivo()?;

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

    pub fn write_error(&self, error: String) -> Result<(), String> {
        let archivo = self.crear_archivo()?;

        let mut escritor = BufWriter::new(archivo);
        match escritor.write(error.as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn write(&self, data: Vec<Vec<String>>) -> Result<(), String> {
        let archivo = self.crear_archivo()?;

        let mut escritor = BufWriter::new(archivo);

        for i in 0..data.len() {
            for j in 0..data[i].len() {
                match escritor.write(data[i][j].as_bytes()) {
                    Ok(_) => (),
                    Err(e) => return Err(e.to_string()),
                }
            }
            match escritor.write(b"\n") {
                Ok(_) => (),
                Err(e) => return Err(e.to_string()),
            }
        }

        Ok(())
    }
}
