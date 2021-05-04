fn servir_comanda() {}

mod parte_de_atras {
    fn correjir_comanda() {
        cocinar_comanda();
        super::servir_comanda();
    }

    fn cocinar_comanda() {}
}
