mod fachada {
    pub mod recepcion {
        fn poner_en_espera() {}
    }
}

pub fn comer_en_restaurant() {
    // path absoluto
    crate::fachada::recepcion::poner_en_espera();

    // path relativo 
    fachada::recepcion::poner_en_espera();
}
