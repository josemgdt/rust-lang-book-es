## ¡Hola, Cargo!

Cargo es el sistema de construcción y el administrador de paquetes de Rust. La mayoría de los rustáceos utilizan esta herramienta para administrar sus proyectos de Rust porque Cargo maneja muchas tareas por ti,
como crear tu código, descargar las bibliotecas de las que depende tu código y
construir esas bibliotecas. (Las bibliotecas que tu código necesita las llamamos *dependencias*.)

Los programas Rust más simples, como el que hemos escrito hasta ahora, no tienen
dependencias. Si hubiéramos construido el proyecto "¡Hola, mundo!" con Cargo,
solo usaría la parte de Cargo que maneja la construcción de tu código. Cuando escribas
programas Rust más complejos, agregarás dependencias y, si comienzas un proyecto
usando Cargo, agregar dependencias será mucho más fácil.

Dado que la gran mayoría de los proyectos Rust utilizan Cargo, el resto de este libro
asume que también estás usando Cargo. Cargo viene instalado con Rust si
utilizaste los instaladores oficiales discutidos en la sección [“Instalación”][installation]<!-- ignore -->. 
Si instalaste Rust a través de algún otro medio, verifica si Cargo está instalado ingresando lo
siguiente en tu terminal:

```console
$ cargo --version
```

Si ves un número de versión, ¡lo tienes! Si ves un error, como `comando
no encontrado`, consulta la documentación de tu método de instalación para
determinar cómo instalar Cargo por separado.

### Creando un Proyecto con Cargo

Creemos un nuevo proyecto con Cargo y observemos en qué se diferencia de nuestro
proyecto original "¡Hola, mundo!". Vuelve a tu directorio *projects* (o
donde decidiste almacenar tu código). Luego, en cualquier sistema operativo, ejecuta
lo siguiente:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

El primer comando crea un nuevo directorio llamado *hello_cargo*. Hemos nombrado
nuestro proyecto *hello_cargo*, y Cargo crea sus archivos en un directorio del
mismo nombre.

Ve al directorio *hello_cargo* y enumera los archivos. Verás que Cargo
ha generado dos archivos y un directorio para nosotros: un archivo *Cargo.toml* y un
directorio *src* con un archivo *main.rs* dentro.

También ha inicializado un nuevo repositorio de Git junto con un archivo *.gitignore*.
Los archivos Git no se generarán si ejecuta `cargo new` dentro de un repositorio Git 
existente; puedes anular este comportamiento usando `cargo new --vcs=git`.

> Nota: Git es un sistema de control de versiones común. Puedes cambiar `cargo new` y
> utilizar un sistema de control de versiones diferente o ningún sistema de control de versiones utilizando
> la bandera `--vcs`. Ejecuta `cargo new --help` para ver las opciones disponibles.

Abre *Cargo.toml* en el editor de texto que prefieras. Debería verse similar al
código en el Listado 1-2.

<span class="filename">Nombre de archivo: Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

<span class="caption">Listado 1-2: Contenido de *Cargo.toml* generado por `cargo
new` </span>

Este archivo está en [*TOML*](https://toml.io)<!-- ignore --> (*Tom's Obvious,
Minimal Language*), que es el formato de configuración de Cargo.

La primera línea, `[package]`, es un encabezado de sección que indica que
las siguientes declaraciones están configurando un paquete. A medida que agregamos más información a
este archivo, agregaremos otras secciones.

Las siguientes cuatro líneas establecen la información de configuración que Cargo necesita para compilar
tu programa: el nombre, la versión, quién lo escribió y la edición de Rust a
usar. Cargo obtiene tu nombre e información de correo electrónico de tu entorno, así que si
la información no es correcta, corrije la información ahora y luego guarda el
archivo. Hablaremos sobre la clave `edition` en el Apéndice E.

La última línea, `[dependencies]`, es el comienzo de una sección que enumera cualquier
dependencia de tu proyecto. En Rust, los paquetes de código se denominan
*cajas (crates)*. No necesitaremos otras cajas para este proyecto, pero en el
primer proyecto en el Capítulo 2, usaremos esta sección de dependencias.

Ahora abre *src/main.rs* y echa un vistazo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo ha generado un programa "¡Hola, mundo!" para ti, como el que
escribiste en el Listado 1-1! Hasta ahora, las diferencias entre nuestro proyecto anterior y
el proyecto Cargo es que Cargo colocó el código en el directorio *src*
, y tenemos un archivo de configuración *Cargo.toml* en el directorio superior.

Cargo espera que los archivos fuente esten dentro del directorio *src*.
El directorio de proyectos de nivel superior es solo para archivos README, información de licencia,
archivos de configuración y cualquier otra cosa que no esté relacionada con tu código. Usar Cargo
te ayuda a organizar tus proyectos. Hay un lugar para todo y todo está en su lugar.

Si iniciaste un proyecto que no utiliza Cargo, como hicimos con el proyecto "Hola,
¡mundo!", puedes convertirlo en un proyecto que utilice Cargo. Mueve el
proyecto al directorio *src* y crea un archivo *Cargo.toml* apropiado.

### Construcción y Ejecución de un Proyecto Cargo

Ahora veamos qué es diferente cuando creamos y ejecutamos el programa "¡Hola, mundo!"
con Cargo! Desde tu directorio *hello_cargo*, construye el proyecto
ingresando el siguiente comando:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Este comando crea un archivo ejecutable en *target/debug/hello_cargo* (o
*target\debug\hello_cargo.exe* en Windows) en lugar de en su actual
directorio. Puedes ejecutarlo con este comando:

```console
$ ./target/debug/hello_cargo # o .\target\debug\hello_cargo.exe en Windows
Hello, world!
```

Si todo va bien, debería imprimirse "¡Hola, mundo!" en la terminal. Ejecutando `cargo
build` por primera vez también hace que Cargo cree un nuevo archivo en el nivel superior
: *Cargo.lock*. Este archivo realiza un seguimiento de las versiones exactas de
dependencias en tu proyecto. Este proyecto no tiene dependencias, por lo que
el archivo es un poco escaso. Nunca necesitará cambiar este archivo manualmente; Cargo
gestiona su contenido por ti.

Acabamos de construir un proyecto con "cargo build" y lo ejecutamos con
`./target/debug/hello_cargo`, pero también podemos usar `cargo run` para compilar el
código y luego correr el ejecutable resultante, todo en un comando:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Ten en cuenta que esta vez no vimos resultados que indicaran que Cargo estaba compilando
`hello_cargo`. Cargo descubrió que los archivos no habían cambiado, por lo que simplemente se ejecutó
el binario. Si hubieras modificado tu código fuente, Cargo habría reconstruido el
proyecto antes de ejecutarlo, y habrías visto este resultado:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo también proporciona un comando llamado `cargo check`. Este comando comprueba rápidamente
el código para asegurarse de que se compila pero no produce un ejecutable:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

¿Por qué no querrías un ejecutable? A menudo, `cargo check` es mucho más rápido que
`cargo build`, porque omite el paso de producir un ejecutable. Si estas
verificando continuamente tu trabajo mientras escribes el código, usar `cargo check`
¡acelera el proceso! Como tal, muchos rustáceos ejecutan `cargo check` periódicamente
mientras escriben su programa para asegurarse de que se compile. Luego ejecutan `cargo
build` cuando estan listos para usar el ejecutable.

Recapitulemos lo que hemos aprendido hasta ahora sobre Cargo:

* Podemos construir un proyecto usando `cargo build`.
* Podemos construir y ejecutar un proyecto en un solo paso usando "cargo run".
* Podemos construir un proyecto sin producir un binario para verificar errores usando
  `cargo build`.
* En lugar de guardar el resultado de la compilación en el mismo directorio que nuestro código,
  Cargo lo almacena en el directorio *target/debug*.

Una ventaja adicional de usar Cargo es que los comandos son los mismos, no
importa en qué sistema operativo estes trabajando. No se
proporcionan instrucciones específicas para Linux y macOS frente a Windows.

### Construyendo para Release

Cuando tu proyecto esté listo para su lanzamiento, puedes usar `cargo build
--release` para compilarlo con optimizaciones. Este comando creará un
ejecutable en *target/release* en lugar de en *target/debug*. Las optimizaciones
haran que el código Rust se ejecute más rápido, pero activarlas alarga el tiempo de
compilación de tu programa. Por eso hay dos perfiles diferentes: uno
para el desarrollo, cuando deseas reconstruir de forma rápida y frecuente, y otro para
crear el programa final que se dará a un usuario que no se reconstruirá
repetidamente y que se ejecutará lo más rápido posible. Si estás midiendo (benchmarking)
tiempo de ejecución del código, asegúrate de ejecutar `cargo build --release` y medir con
el ejecutable en *target/release *.

### Cargo como Convención

Con proyectos simples, Cargo no aporta mucho valor por encima del simple
`rustc`, pero demostrará su valor a medida que tus programas se vuelvan más complejos.
Con proyectos complejos compuestos por varias cajas, es mucho más fácil dejar que
Cargo coordine la construcción.

Aunque el proyecto `hello_cargo` es simple, ahora usa gran parte de las
herramientas que utilizará en el resto de su carrera en Rust. De hecho, para trabajar en cualquier
proyecto existente, puedes usar los siguientes comandos para verificar el código
usando Git, cambia al directorio de ese proyecto y compila:

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

Para obtener más información sobre Cargo, consulte [su documentación].

[su documentación]: https://doc.rust-lang.org/cargo/

## Resumen

¡Has tenido un gran comienzo en tu viaje por Rust! En este capítulo,
has aprendido a:

* Instalar la última versión estable de Rust usando `rustup`
* Actualizar a una versión más reciente de Rust
* Abrir documentación instalada localmente
* Escribir y ejecutar un programa "¡Hola, mundo!" usando `rustc` directamente
* Crear y ejecutar un nuevo proyecto usando las convenciones de Cargo

Este es un buen momento para crear un programa más sustancial para acostumbrarse a la lectura
y escritura de código Rust. En el Capítulo 2, crearemos un programa de juego de adivinanzas.
Si prefieres comenzar por aprender cómo funcionan los conceptos de programación comunes en
Rust, ve al Capítulo 3 y luego regresa al Capítulo 2.

[installation]: ch01-01-installation.html#Instalacion
