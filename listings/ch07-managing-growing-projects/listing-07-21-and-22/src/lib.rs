mod fachada;

pub use crate::front_of_house::recepcion;

pub fn comer_en_restaurant() {
    hosting::poner_en_espera();
    hosting::poner_en_espera();
    hosting::poner_en_espera();
}
