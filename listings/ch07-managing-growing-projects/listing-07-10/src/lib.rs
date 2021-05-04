mod parte_de_atras {
    pub enum Aperitivo {
        Sopa,
        Ensalada,
    }
}

pub fn comer_en_restaurant() {
    let order1 = back_of_house::Aperitivo::Sopa;
    let order2 = back_of_house::Aperitivo::Ensalada;
}
