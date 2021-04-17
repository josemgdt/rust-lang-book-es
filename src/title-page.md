# El lenguaje de programación Rust

*por Steve Klabnik y Carol Nichols, con contribuciones de la Comunidad Rust *

Esta versión del texto asume que estás usando Rust 1.50 o posterior con
`edition="2018 "` en el archivo *Cargo.toml* de todos los proyectos, para usar el estilo
Rust 2018 Edition. Consulte la sección [“Instalación” del Capítulo 1][instalar]<!-- ignore -->
para instalar o actualizar Rust, y vea el nuevo [Apéndice E][ediciones]<!-- ignore --> para información sobre ediciones.

La edición 2018 del lenguaje Rust incluye una serie de mejoras que
hacen que Rust sea más ergonómico y fácil de aprender. Esta iteración del libro
contiene una serie de cambios para reflejar esas mejoras:

- El Capítulo 7, "Gestión de Proyectos en Crecimiento con Paquetes, Cajas y Módulos",
  se ha reescrito en su mayor parte. El sistema de módulos y la forma en que funcionan las rutas en
  la edición 2018 se hizo más consistente.
- El capítulo 10 tiene nuevas secciones tituladas "Traits como parámetros" y "Retornando
  Tipos que Implementan Traits” que explican la nueva sintaxis de los Traits implícitos.
- El Capítulo 11 tiene una nueva sección titulada "Uso de `Result<T, E>` en Tests" que
  muestra cómo escribir tests que utilizan el operador `?`.
- La sección "Lifetimes avanzados" del Capítulo 19 se eliminó porque las mejoras
  del compilador han hecho que las construcciones en esa sección sean de uso raro.
- El Apéndice D anterior, "Macros", se ha ampliado para incluir procedimientos
  macros y se movió a la sección "Macros" en el Capítulo 19.
- El Apéndice A, "Palabras clave", también explica la nueva función de identificadores sin procesar que
  permite que el código escrito en la edición 2015 y la edición 2018 interoperen.
- El Apéndice D ahora se titula "Herramientas de desarrollo útiles" y cubre
  herramientas publicadas recientemente que le ayudarán a escribir código Rust.
- Corregimos una serie de pequeños errores y una redacción imprecisa a lo largo del libro.
  ¡Gracias a los lectores que los informaron!

Tenga en cuenta que cualquier código en iteraciones anteriores de *The Rust Programming Language*
que compilaba continuará compilando sin `edition="2018 "` en el archivo
*Cargo.toml* del proyecto, incluso si actualiza la versión del compilador de Rust,
utilizado. ¡Eso garantíza de compatibilidad con versiones anteriores de Rust!

El formato HTML (en inglés) está disponible en línea en
[https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)
y offline con instalaciones de Rust hechas con "rustup"; ejecutar `rustup docs
--book` para abrirlo.

Este texto (en inglés) está disponible en [formato de bolsillo y libro electrónico de No Starch
Press][nsprust].

[instalar]: ch01-01-installation.html
[ediciones]: appendix-05-editions.html
[nsprust]: https://nostarch.com/rust

