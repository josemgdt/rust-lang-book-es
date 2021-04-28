# El lenguaje de programación Rust

*por Steve Klabnik y Carol Nichols, con contribuciones de la Comunidad Rust*

*traducción al castellano por J.M. García*

Esta versión del texto asume que está usando Rust 1.50 o posterior con la declaración
`edition = "2018"` en el encabezado [package] del archivo *Cargo.toml* de todos los proyectos para usar el estilo
de la Edición Rust 2018. Consulte la sección [“Instalación”][instalar]<!-- ignore -->
del Capitulo 1 para instalar o actualizar Rust, y vea el nuevo [Apéndice E][ediciones]<!-- ignore --> para información sobre ediciones.

La edición 2018 del lenguaje Rust incluye una serie de mejoras que
hacen que Rust sea más ergonómico y fácil de aprender. Esta edición del libro
contiene una serie de cambios que reflejan esas mejoras:

- El Capítulo 7, "Gestión de Proyectos en Crecimiento con Paquetes, Cajas y Módulos",
  se ha reescrito en su mayor parte. El sistema de módulos y la forma en que funcionan las rutas es
  más consitente en la edición 2018.
- El capítulo 10 tiene nuevas secciones tituladas "Traits como Parámetros" y "Retornando
  Tipos que Implementan Traits” que explican la nueva sintaxis de los traits implícitos.
- El Capítulo 11 tiene una nueva sección titulada "Uso de `Result<T, E>` en Tests" que
  muestra cómo escribir tests que utilizan el operador `?`.
- La sección "Lifetimes avanzados" del Capítulo 19 se eliminó porque las mejoras
  del compilador han hecho que las construcciones en esa sección se usen raramente.
- El antiguo Apéndice D, "Macros", se ha ampliado para incluir procedimientos
  macros y se ha movido a la sección "Macros" en el Capítulo 19.
- El Apéndice A, "Palabras Clave", también explica la nueva función de identificadores sin procesar que
  permite que el código escrito en las ediciones 2015 y 2018 interoperen.
- El Apéndice D ahora se titula "Herramientas de Desarrollo Útiles" y cubre
  herramientas publicadas recientemente que le ayudarán a escribir código Rust.
- Corregimos una serie de pequeños errores y alguna redacción imprecisa a lo largo del libro.
  ¡Gracias a los lectores que los advirtieron!

Tenga en cuenta que cualquier código en ediciones anteriores de *The Rust Programming Language*
que compilaban, continuarán haciendolo sin `edition = "2018"` en el archivo
*Cargo.toml* del proyecto, incluso si actualiza la versión del compilador Rust
utilizada. ¡Eso garantíza la compatibilidad con versiones anteriores de Rust!

El formato HTML (en inglés) está disponible en línea en
[https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)
y offline con instalaciones de Rust hechas con "rustup"; ejecutar `rustup docs
--book` para abrirlo.

Este texto (en inglés) está disponible en [formato de bolsillo y libro electrónico de No Starch
Press][nsprust].



[instalar]: ch01-01-installation.html
[ediciones]: appendix-05-editions.html
[nsprust]: https://nostarch.com/rust

