mod fachada {
    mod recepcion {
        fn poner_en_espera() {}
    }
}

pub fn comer_en_restaurant() {
    // Absolute path
    crate::fachada::recepcion::poner_en_espera();

    // Relative path
    fachada::recepcion::poner_en_espera();
}
