mod parte_de_atras {
    pub struct Desayuno {
        pub tostada: String,
        fruta_de_temporada: String,
    }

    impl Desayuno {
        pub fn verano(tostada: &str) -> Desayuno {
            Desayuno {
                tostada: String::from(tostada),
                fruta_de_temporada: String::from("melocotones"),
            }
        }
    }
}

pub fn comer_en_restaurant() {
    // Pedir un desayuno en verano con tostadas de centeno
    let mut comer = parte_de_atras::Desayuno::verano("centeno");
    // Cambio de opinion sobre que pan queremos
    comer.tostada = String::from("trigo");
    println!("Quisiera tostadas de {} por favor", comer.tostada);

    // La siguiente línea no se compilará si descomentamos; no podemos 
    // ver ni modificar la fruta de temporada que viene con la comida. 
    // comer.fruta_de_temporada = String::from("arándanos");
}
