mod fachada {
    pub mod recepcion {
        pub fn poner_en_espera() {}
    }
}

use self::fachada::recepcion;

pub fn comer_en_restaurant() {
    recepcion::poner_en_espera();
    recepcion::poner_en_espera();
    recepcion::poner_en_espera();
}
