# El Lenguaje de Programación Rust

[El lenguaje de Programación Rust](title-page.md)

[Prólogo](foreword.md)

[Introducción](ch00-00-introduction.md)

## Empezando

- [Introducción](ch01-00-getting-started.md)
    - [Instalación](ch01-01-installation.md)
    - [¡Hola, Mundo!](ch01-02-hello-world.md)
    - [¡Hola, Cargo!](ch01-03-hello-cargo.md)

- [Programación de un Juego de Adivinanzas](ch02-00-guessing-game-tutorial.md)

- [Conceptos Comunes de Programación](ch03-00-common-programming-concepts.md)
    - [Variables y Mutabilidad](ch03-01-variables-and-mutability.md)
    - [Tipos de Datos](ch03-02-data-types.md)
    - [Funciones](ch03-03-how-functions-work.md)
    - [Comentarios](ch03-04-comments.md)
    - [Control de Flujo](ch03-05-control-flow.md)

- [Entendiendo la Propiedad](ch04-00-understanding-ownership.md)
    - [¿Qué es Propiedad?](ch04-01-what-is-ownership.md)
    - [Referencias y Préstamos](ch04-02-references-and-borrowing.md)
    - [El Tipo Slice](ch04-03-slices.md)

- [Uso de Structs para Estructurar Datos Relacionados](ch05-00-structs.md)
    - [Definición e Instanciación de Estructuras](ch05-01-defining-structs.md)
    - [Un Programa de Ejemplo que Usa Estructuras](ch05-02-example-structs.md)
    - [Sintaxis de Métodos](ch05-03-method-syntax.md)

- [Coincidencia de Patrones y Enumeraciones](ch06-00-enums.md)
    - [Definición de una Enumeración](ch06-01-defining-an-enum.md)
    - [El Operador de Control de Flujo `match`](ch06-02-match.md)
    - [Flujo de Control Conciso con `if let`](ch06-03-if-let.md)

## Conocimientos Básicos sobre Rust

- [Gestión de Proyectos en Crecimiento con Paquetes, Cajas y Módulos](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
    - [Paquetes y Cajas](ch07-01-packages-and-crates.md)
    - [Definición de Módulos para Controlar Alcance y Privacidad](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [Rutas para Referencia a un Elemento en el Arbol de Módulos](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [Poner Rutas Dentro del Alcance con la Palabra Clave `use`](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [Separación de Módulos en Diferentes Archivos](ch07-05-separating-modules-into-different-files.md)

- [Colecciones Comunes](ch08-00-common-collections.md)
    - [Almacenamiento de Listas de Valores con Vectores](ch08-01-vectors.md)
    - [Almacenamiento de Texto Codificado en UTF-8 con Cadenas](ch08-02-strings.md)
    - [Almacenamiento de Claves con Valores Asociados en Mapas Hash](ch08-03-hash-maps.md)

- [Manejo de Errores](ch09-00-error-handling.md)
    - [Errores Irrecuperables con `¡panic!`](ch09-01-unrecoverable-errors-with-panic.md)
    - [Errores Recuperables con `Result`](ch09-02-recoverable-errors-with-result.md)
    - [`panic!` o no `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [Tipos Genéricos, Caracteristicas y Tiempos de Vida](ch10-00-generics.md)
    - [Tipos de Datos Genéricos](ch10-01-syntax.md)
    - [Caracteristicas: Definición de Comportamiento Compartido](ch10-02-traits.md)
    - [Validación de Referencias con Tiempos de Vida](ch10-03-lifetime-syntax.md)

- [Escritura Automatizada de Tests](ch11-00-testing.md)
    - [Cómo Escribir Tests](ch11-01-writing-tests.md)
    - [Controlando la Ejecución de los Tests](ch11-02-running-tests.md)
    - [Organización del Test](ch11-03-test-organization.md)

- [Un Proyecto de E/S: Creación de un Programa de Línea de Comandos](ch12-00-an-io-project.md)
    - [Aceptación de Argumentos en la Línea de Comandos](ch12-01-accepting-command-line-arguments.md)
    - [Leyendo un Archivo](ch12-02-reading-a-file.md)
    - [Refactorización para Mejorar la Modularidad y el Manejo de Errores](ch12-03-improving-error-handling-and-modularity.md)
    - [Desarrollo de la Funcionalidad de la Biblioteca con Desarrollo Basado en Tests](ch12-04-testing-the-librarys-functionality.md)
    - [Trabajar con Variables de Entorno](ch12-05-working-with-environment-variables.md)
    - [Escribir Mensajes de Error en Stderr en Lugar de Stdout](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Pensando en Rust

- [Características Funcionales del Lenguaje: Iteradores y Cierres](ch13-00-functional-features.md)
    - [Cierres: Funciones Anónimas que Pueden Capturar su Entorno](ch13-01-closures.md)
    - [Procesando una Serie de Elementos con Iteradores](ch13-02-iterators.md)
    - [Mejorando Nuestro Proyecto de E/S](ch13-03-improving-our-io-project.md)
    - [Comparación de Rendimiento: Bucles frente a Iteradores](ch13-04-performance.md)

- [Más sobre Cargo y Crates.io](ch14-00-more-about-cargo.md)
    - [Personalización de Compilaciones con Perfiles de Release](ch14-01-release-profiles.md)
    - [Publicar una Caja en Crates.io](ch14-02-publishing-to-crates-io.md)
    - [Espacio de Trabajo de Cargo](ch14-03-cargo-workspaces.md)
    - [Instalación de Binarios desde Crates.io con `cargo install`](ch14-04-installing-binaries.md)
    - [Extensión de Cargo con Comandos Personalizados](ch14-05-extending-cargo.md)

- [Punteros Inteligentes](ch15-00-smart-pointers.md)
    - [Uso de `Box<T>` para Apuntar los Datos en el Heap](ch15-01-box.md)
    - [Tratar los Punteros Inteligentes como Referencias Regulares con la Caracteristica `Deref`](ch15-02-deref.md)
    - [Ejecutar Código en la Limpieza con la Caracteristica `Drop`](ch15-03-drop.md)
    - [`Rc<T>`, el Puntero Inteligente Contado de Referencia](ch15-04-rc.md)
    - [`RefCell<T>` y el Patrón de Mutabilidad Interior](ch15-05-interior-mutability.md)
    - [Los Ciclos de Referencia Pueden Perder Memoria](ch15-06-reference-cycles.md)

- [Concurrencia sin miedo](ch16-00-concurrency.md)
    - [Uso de Hilos para Ejecutar Código Simultáneamente](ch16-01-threads.md)
    - [Uso de Paso de Mensajes para Transferir Datos entre Hilos](ch16-02-message-passing.md)
    - [Concurrencia de Estado Compartido](ch16-03-shared-state.md)
    - [Concurrencia Extensible con las Caracteristicas `Sync` y` Send`](ch16-04-extensible-concurrency-sync-and-send.md)

- [Caracteristicas de la Programación Orientadas a Objetos de Rust](ch17-00-oop.md)
    - [Características de los Lenguajes Orientados a Objetos](ch17-01-what-is-oo.md)
    - [Uso de Objetos Trait que Permiten Valores de Diferentes Tipos](ch17-02-trait-objects.md)
    - [Implementación de un Patrón de Diseño Orientado a Objetos](ch17-03-oo-design-patterns.md)

## Temas avanzados

- [Patrones y Coincidencias](ch18-00-patterns.md)
    - [Se pueden usar todos los patrones de lugares](ch18-01-all-the-places-for-patterns.md)
    - [Refutabilidad: Cuando un Patrón Puede no Coincidir](ch18-02-refutability.md)
    - [Sintaxis de Patrónes](ch18-03-pattern-syntax.md)

- [Funciones Avanzadas](ch19-00-advanced-features.md)
    - [Rust Inseguro](ch19-01-unsafe-rust.md)
    - [Traits Avanzados](ch19-03-advanced-traits.md)
    - [Tipos Avanzados](ch19-04-advanced-types.md)
    - [Funciones Avanzadas y Cierres](ch19-05-advanced-functions-and-closures.md)
    - [Macros](ch19-06-macros.md)

- [Proyecto Final: Creación de un Servidor Web Multiproceso](ch20-00-final-project-a-web-server.md)
    - [Creación de un Servidor Web de un Solo Hilo](ch20-01-single-threaded.md)
    - [Convertir Nuestro Servidor de un Solo Hilo en un Servidor de Múltiples Hilos](ch20-02-multithreaded.md)
    - [Cierre y Limpieza Elegantes](ch20-03-graceful-shutdown-and-cleanup.md)

- [Apéndice](appendix-00.md)
    - [A - Palabras Clave](appendix-01-keywords.md)
    - [B - Operadores y Símbolos](appendix-02-operators.md)
    - [C - Traits derivables](appendix-03-derivable-traits.md)
    - [D - Herramientas de Desarrollo Utiles](appendix-04-useful-development-tools.md)
    - [E - Ediciones](appendix-05-editions.md)
    - [F - Traducciones del Libro](appendix-06-translation.md)
    - [G - Cómo se Produce Rust y “Nightly Rust”](appendix-07-nightly-rust.md)
