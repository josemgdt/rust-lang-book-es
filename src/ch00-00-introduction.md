# Introducción

> Nota: esta es la traducción del libro [The Rust Programming
> Language][nsprust] . No está disponible en formato impreso.

[nsprust]: https://nostarch.com/rust

Bienvenido a *El Lenguaje de Programación Rust*, un libro introductorio sobre Rust.
El lenguaje de programación Rust le ayudara a escribir software más rápido y confiable.
La ergonomía de alto nivel y el control de bajo nivel a menudo están en desacuerdo en el 
diseño de lenguajes de programación; Rust desafía ese conflicto. A través del equilibrio 
entre una poderosa capacidad técnica y una grata experiencia durante el desarrollo, Rust 
te da la opción de controlar los detalles de bajo nivel (como el uso de la memoria) sin 
todas las molestias tradicionalmente asociadas con dicho control.

## Para Quién Es Rust

Rust es ideal para muchas personas por diversas razones. Veamos algunos de los 
grupos más importantes.

### Equipos de Desarrolladores

Rust está demostrando ser una herramienta productiva para colaborar entre grandes equipos de
desarrolladores con distintos niveles de conocimientos de programación de sistemas. El código 
de bajo nivel es propenso a una variedad de errores sutiles, que en la mayoría de idiomas pueden ser
capturados sólo a través de pruebas exhaustivas y la revisión cuidadosa del código por 
desarrolladores experimentados. En Rust, el compilador juega un papel de guardián al negarse a
compilar código con estos errores esquivos, incluidos los errores de concurrencia. Trabajando
junto con el compilador, el equipo puede dedicar su tiempo a centrarse en la lógica del programa
en lugar de perseguir errores.

Rust también brinda herramientas de desarrollo coetaneas al mundo de la programación de sistemas:

* Cargo, el administrador de dependencias y la herramienta de compilación, hace que agregar,
  compilar y gestionar dependencias sea sencillo y coherente en todo el ecosistema Rust.
* Rustfmt garantiza un estilo de codificación coherente entre los desarrolladores.
* Rust Language Server impulsa la integración del entorno de desarrollo integrado (IDE) para 
  completar el código y dar mensajes de error en línea.

Al utilizar estas y otras herramientas en el ecosistema Rust, los desarrolladores pueden ser
productivos al escribir código a nivel de sistemas.

### Estudiantes

Rust es para estudiantes y aquellos que estén interesados en aprender sobre conceptos de sistemas.
Con Rust, muchas personas han aprendido sobre temas como desarrollo de sistemas operativos. La comunidad 
es muy acogedora y presta a responder preguntas de los estudiantes. A través de esfuerzos como este libro, 
los equipos de Rust quieren hacer que los conceptos de sistemas sean más accesibles para más personas, 
especialmente para los nuevos en programación.

### Compañías

Cientos de empresas, grandes y pequeñas, utilizan Rust en la producción de variedad de tareas. 
Esas tareas incluyen herramientas de línea de comandos, servicios web, herramientas DevOps,
dispositivos integrados, análisis y transcodificación de audio y video, criptomonedas,
bioinformática, motores de búsqueda, aplicaciones de Internet de las cosas, "machine learning", 
e incluso partes importantes del navegador web Firefox.

### Desarrolladores de Código Abierto

Rust es para personas que quieren construir el lenguaje de programación Rust, la comunidad,
las herramientas de desarrollo y bibliotecas. Nos encantaría que contribuyeses al lenguaje Rust.

### Personas que Valoran la Velocidad y la Estabilidad

Rust es para personas que anhelan velocidad y estabilidad en un lenguaje. Por velocidad, nosotros
entendemos la velocidad de los programas que puede crear con Rust y la velocidad a la que Rust 
te permite escribirlos. Las comprobaciones del compilador de Rust garantizan la estabilidad
mediante la incorporación de caracteristicas y la refactorización. Esto contrasta con el frágil
código heredado en lenguajes sin estas comprobaciones, que los desarrolladores suelen no modificar 
por temor. Al esforzarse por lograr abstracciones de coste cero, características de nivel superior 
que se compilan en código de nivel inferior tan rápido como el código escrito manualmente, Rust 
se esfuerza por hacer que el código seguro sea también código rápido.

El lenguaje Rust espera ser compatible también con muchos otros usuarios; los mencionados son solo 
algunos de los principales interesados. En general, la mayor ambición de Rust es eliminar las 
concesiones que los programadores han aceptado durante décadas, brindando seguridad *y* productividad, 
velocidad *y* ergonomía. Prueba Rust y comprueba si sus opciones funcionan para ti.

## Para Quien Es Este Libro

Este libro asume que has escrito código en otro lenguaje de programación pero no hace ninguna 
suposición sobre cuál. Intentamos hacer el material ampliamente accesible para aquellos con una amplia 
variedad de antecedentes en programación. No dedicaremos mucho tiempo a hablar sobre qué *es* la 
programación o cómo pensar sobre ella. Si es completamente nuevo en programación, estaría mejor leer 
un libro que proporcione específicamente una introducción a la programación.

## Como Usar Este Libro

En general, este libro asume que lo está leyendo en secuencia, desde el principio hasta el final. 
Los capítulos se basan en conceptos de capítulos anteriores y es posible que los capítulos no profundicen 
en los detalles de un tema; normalmente volvemos a visitar el tema en un capítulo posterior.

Encontrará dos tipos de capítulos en este libro: capítulos de conceptos y capítulos de proyectos. En 
los capítulos de conceptos, aprenderá sobre aspectos de Rust. En los capítulos de proyectos crearemos pequeños 
programas, aplicando lo que ha aprendido antes. Los capítulos 2, 12 y 20 son capítulos de proyectos; el resto 
son capítulos de conceptos.

Chapter 1 explains how to install Rust, how to write a “Hello, world!” program,
and how to use Cargo, Rust’s package manager and build tool. Chapter 2 is a
hands-on introduction to the Rust language. Here we cover concepts at a high
level, and later chapters will provide additional detail. If you want to get
your hands dirty right away, Chapter 2 is the place for that. At first, you
might even want to skip Chapter 3, which covers Rust features similar to those
of other programming languages, and head straight to Chapter 4 to learn about
Rust’s ownership system. However, if you’re a particularly meticulous learner
who prefers to learn every detail before moving on to the next, you might want
to skip Chapter 2 and go straight to Chapter 3, returning to Chapter 2 when
you’d like to work on a project applying the details you’ve learned.

Chapter 5 discusses structs and methods, and Chapter 6 covers enums, `match`
expressions, and the `if let` control flow construct. You’ll use structs and
enums to make custom types in Rust.

In Chapter 7, you’ll learn about Rust’s module system and about privacy rules
for organizing your code and its public Application Programming Interface
(API). Chapter 8 discusses some common collection data structures that the
standard library provides, such as vectors, strings, and hash maps. Chapter 9
explores Rust’s error-handling philosophy and techniques.

Chapter 10 digs into generics, traits, and lifetimes, which give you the power
to define code that applies to multiple types. Chapter 11 is all about testing,
which even with Rust’s safety guarantees is necessary to ensure your program’s
logic is correct. In Chapter 12, we’ll build our own implementation of a subset
of functionality from the `grep` command line tool that searches for text
within files. For this, we’ll use many of the concepts we discussed in the
previous chapters.

Chapter 13 explores closures and iterators: features of Rust that come from
functional programming languages. In Chapter 14, we’ll examine Cargo in more
depth and talk about best practices for sharing your libraries with others.
Chapter 15 discusses smart pointers that the standard library provides and the
traits that enable their functionality.

In Chapter 16, we’ll walk through different models of concurrent programming
and talk about how Rust helps you to program in multiple threads fearlessly.
Chapter 17 looks at how Rust idioms compare to object-oriented programming
principles you might be familiar with.

Chapter 18 is a reference on patterns and pattern matching, which are powerful
ways of expressing ideas throughout Rust programs. Chapter 19 contains a
smorgasbord of advanced topics of interest, including unsafe Rust, macros, and
more about lifetimes, traits, types, functions, and closures.

In Chapter 20, we’ll complete a project in which we’ll implement a low-level
multithreaded web server!

Finally, some appendices contain useful information about the language in a
more reference-like format. Appendix A covers Rust’s keywords, Appendix B
covers Rust’s operators and symbols, Appendix C covers derivable traits
provided by the standard library, Appendix D covers some useful development
tools, and Appendix E explains Rust editions.

There is no wrong way to read this book: if you want to skip ahead, go for it!
You might have to jump back to earlier chapters if you experience any
confusion. But do whatever works for you.

<span id="ferris"></span>

An important part of the process of learning Rust is learning how to read the
error messages the compiler displays: these will guide you toward working code.
As such, we’ll provide many examples that don’t compile along with the error
message the compiler will show you in each situation. Know that if you enter
and run a random example, it may not compile! Make sure you read the
surrounding text to see whether the example you’re trying to run is meant to
error. Ferris will also help you distinguish code that isn’t meant to work:

| Ferris                                                                 | Meaning                                          |
|------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain"/>    | This code does not compile!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain"/>              | This code panics!                                |
| <img src="img/ferris/unsafe.svg" class="ferris-explain"/>              | This code block contains unsafe code.            |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain"/>| This code does not produce the desired behavior. |

In most situations, we’ll lead you to the correct version of any code that
doesn’t compile.

## Source Code

The source files from which this book is generated can be found on
[GitHub][book].

[book]: https://github.com/rust-lang/book/tree/master/src
