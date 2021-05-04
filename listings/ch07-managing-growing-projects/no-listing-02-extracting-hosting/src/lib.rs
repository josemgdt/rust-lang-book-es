mod fachada;

pub use crate::fachada::recepcion;

pub fn comer_en_restaurant() {
    hosting::poner_en_espera();
    hosting::poner_en_espera();
    hosting::poner_en_espera();
}
