use crate::coordenada::Coordenada;

pub trait ObjetoMapa {
    fn set_coordenada_actual(&mut self, coordenada: Coordenada);

    fn get_coordenada_actual(&self) -> &Coordenada;

    fn recibir_rafaga(&mut self) -> Result<(), String>;

    fn recibir_rafaga_traspaso(&mut self) -> Result<(), String>;

    fn detonar(&mut self) -> Result<(), String>;

    fn obtener_representacion(&self) -> String;
}
