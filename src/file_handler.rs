use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct FileHandler {
    target: String,
}

impl FileHandler {
    pub fn new(target: String) -> FileHandler {
        FileHandler { target }
    }

    pub fn read(&self) -> Result<Vec<String>, String> {
        let archivo = match File::open(&self.target) {
            Ok(archivo) => archivo,
            Err(e) => return Err(e.to_string()),
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

    pub fn write(&self, data: Vec<String>) -> Result<(), String> {
        //? Debe implementarse...
        Ok(())
    }
}