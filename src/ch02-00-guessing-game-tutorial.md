# Programación de un Juego de Adivinanzas

¡Entremos en Rust trabajando en un proyecto práctico! Este capítulo le presenta algunos 
conceptos comunes de Rust mostrándole cómo utilizarlos en un programa real. Aprenderá 
acerca de "let", "match", métodos, funciones asociadas, uso de cajas externas, ¡y más! 
Los siguientes capítulos explorarán estas ideas con más detalle. En este capítulo, practicará los fundamentos.

Implementaremos un problema clásico de programación para principiantes: un juego de adivinanzas. Así es
cómo funciona; el programa generará un número entero aleatorio entre 1 y 100 y
le pedirá al jugador que ingrese una suposición. Después de ingresarla, el
programa indicará si la conjetura es demasiado baja o demasiado alta, pero si es
correcta el juego imprimirá un mensaje de felicitación y saldrá.

## Configuración de un Nuevo Proyecto

Para configurar un nuevo proyecto, vaya al directorio *projects* que creó en el
Capítulo 1 y prepare un nuevo proyecto usando Cargo:

```console
$ cargo new guessing_game
$ cd guessing_game
```

El primer comando, `cargo new`, toma el nombre del proyecto (`guess_game`)
como primer argumento. El segundo comando cambia al directorio del nuevo proyecto.

Mire el archivo *Cargo.toml* generado:

<span class="filename">Nombre de archivo: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Si la información del autor que Cargo obtuvo del entorno no es correcta, corríjalo en el archivo y guárdelo nuevamente.

Como vio en el Capítulo 1, `cargo new` genera un programa tipo "¡Hola, mundo!".
Consulte el archivo *src/main.rs*:

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Ahora compilemos este programa "¡Hola, mundo!" y ejecutemoslo en el mismo paso
usando el comando `cargo run`:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

El comando `run` es útil cuando necesita iterar rápidamente en un proyecto,
como haremos en este juego, probando rápidamente cada iteración antes de pasar a
la siguiente.

Vuelva a abrir el archivo *src/main.rs*. Escriba todo el código en este archivo.

## Procesando una Suposición

La primera parte del programa del juego de adivinanzas solicitará la entrada del usuario, procesará
esa entrada, y verificará que la entrada está en la forma esperada. Para empezar,
permitiremos que el jugador ingrese una suposición. Ingrese el código en el Listado 2-1 en
*src/main.rs*.

<span class="filename"> ​Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

<span class="caption">Listado 2-1: Código que obtiene una suposición del usuario y
la imprime</span>

Este código contiene mucha información, así que vamos a repasarlo línea por línea. Para
obtener la entrada del usuario y luego imprimir el resultado como salida, necesitamos traer la
biblioteca `io` (entrada/salida) dentro del alcance. La biblioteca `io` proviene de la
biblioteca estándar (que se conoce como `std`):

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Por defecto, Rust trae solo unos pocos tipos al alcance de cada programa en
[el *preludio*][prelude]<!-- ignore -->. Si un tipo que desea utilizar no está en el
preludio, tiene que traer ese tipo al alcance explícitamente con una declaración `use`.
El uso de la biblioteca `std::io` le proporciona una serie de características útiles, 
incluida la capacidad de aceptar entradas de usuario.

[prelude]: ../std/prelude/index.html

Como se vio en el Capítulo 1, la función `main` es el punto de entrada al programa:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

La sintaxis `fn` declara una nueva función, los paréntesis,`()`, indican que
no hay parámetros, y el corchete, `{`, inicia el cuerpo de la función.

Como también aprendimos en el Capítulo 1, `println!` es una macro que imprime una cadena
en la pantalla:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Este código imprime un mensaje que indica lo qué es el juego y solicita información
del usuario.

### Almacenamiento de Valores con Variables

A continuación, crearemos un lugar para almacenar la entrada del usuario:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

¡Ahora el programa se está poniendo interesante! Están sucediendo muchas cosas en esta pequeña
línea. Tenga en cuenta que esta es una declaración `let`, que se utiliza para crear una
*variable*. Aquí hay otro ejemplo:

```rust,ignore
let foo = bar;
```

Esta línea crea una nueva variable llamada "foo" y la vincula al valor de la
variable `bar`. En Rust, las variables son inmutables por defecto.
Discutiremos este concepto en detalle en la sección [“Variables y
Mutabilidad ”][variables-and-mutability]<!-- ignore --> en el Capítulo 3.
El siguiente ejemplo muestra cómo usar `mut` antes del nombre de la variable para hacer
una variable mutable:

```rust,ignore
let foo = 5; // immutable
let mut bar = 5; // mutable
```

> Nota: La sintaxis `//` inicia un comentario que continúa hasta el final de la
> línea. Rust ignora todos los comentarios, que se discuten con más detalle.
> en el Capítulo 3.

Volvamos al programa del juego de adivinanzas. Ahora sabemos que `let mut guess`
introducirá una variable mutable llamada `guess`. Al otro lado de la igualdad
(`=`) está el valor al que está vinculado `guess`, que es el resultado de
llamar a `String::new`, una función que devuelve una nueva instancia de una 
[`String`][string]<!-- ignore -->, es un tipo cadena proporcionado por la
biblioteca estándar; un fragmento de texto, codificado en UTF-8, que puede crecer.

[string]: ../std/string/struct.String.html

La sintaxis `::` en la línea `::new` indica que `new` es una *función 
asociada* del tipo `String`. Una función asociada se implementa en un tipo,
en este caso `String`, en lugar de en una instancia particular de una `String`. Algunos
lenguajes llaman a esto un *método estático*.

Esta función `new` crea una nueva cadena vacía. Encontrarás una función `new`
en muchos tipos, porque es un nombre común para que una función genere un nuevo valor
de algún tipo.

Para resumir, la línea `let mut guess = String::new ();` ha creado una variable
mutable que actualmente está vinculada a una instancia nueva y vacía de una `String`. ¡Uf!

Recuerde que incluimos la funcionalidad de entrada/salida de la biblioteca 
estándar con `use std::io;` en la primera línea del programa. Ahora llamaremos a
la función `stdin` del módulo `io`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Si no hubiéramos puesto la línea `use std::io` al principio del programa,
podríamos haber escrito esta llamada de función como `std::io::stdin`. La función `stdin`
devuelve una instancia de [`std::io::Stdin`][iostdin]<!-- ignore -->, que es un
tipo que representa un identificador de la entrada estándar de su terminal.

[iostdin]: ../std/io/struct.Stdin.html

La siguiente parte del código, `.read_line(&mut guess)`, llama al método
[`read_line`][read_line]<!-- ignore -->  en el manejador de entrada estándar para
obtener información del usuario. También estamos pasando un argumento a `read_line`: `&mut
guess`.

[read_line]: ../std/io/struct.Stdin.html#method.read_line

El trabajo de `read_line` es tomar lo que el usuario escriba en la entrada estándar
y agregarlo a una cadena (sin sobrescribir su contenido), por lo que se necesita
esa cadena como argumento. El argumento de cadena debe ser mutable para que el
método pueda cambiar el contenido de la cadena agregando la entrada del usuario.

El `&` indica que este argumento es una *referencia*, lo que da una forma de
permitir que múltiples partes del código accedan a una pieza de datos sin necesidad de
copiar esos datos en la memoria varias veces. Las referencias son una característica compleja,
y una de las principales ventajas de Rust es lo seguro y fácil de usar que son las
referencias. No es necesario que conozca muchos de esos detalles para terminar este
programa. Por ahora, todo lo que necesita saber es que, al igual que las variables, las referencias son
inmutable por defecto. Por lo tanto, debe escribir `&mut guess` en lugar de
`&guess` para hacerla mutable. (El Capítulo 4 explicará las referencias más
minuciosamente.)

### Manejo de Fallos Potenciales con el Tipo `Result`

Seguimos trabajando en esta línea de código. Aunque ahora estamos discutiendo una tercera
línea de texto, sigue siendo parte de una única línea lógica de código. La siguiente parte
es este método:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Cuando se llama a un método con la sintaxis `.foo()`, a menudo es aconsejable introducir una
newline y otros espacios en blanco para ayudar a dividir las líneas largas. Podríamos tener
escrito este código como:


```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

Sin embargo, una línea larga es difícil de leer, por lo que es mejor dividirla. Ahora
hablemos de lo que hace esta línea.

Como se mencionó anteriormente, `read_line` pone lo que el usuario escribe en la cadena
que estamos pasando, pero también devuelve un valor, en este caso, un
[`io::Result`][ioresult]<!-- ignore -->. Rust tiene varios tipos llamados
`Result` en su biblioteca estándar: un [`Result`][result]<!-- ignore --> generico
así como versiones específicas para submódulos, como `io::Result`.

[ioresult]: ../std/io/type.Result.html
[result]: ../std/result/enum.Result.html

Los tipos `Result` son [*enumeraciones*][enums]<!-- ignore -->, a menudo referidas
como *enums*. Una enumeración es un tipo que puede tener un conjunto fijo de valores,
que se denominan *variantes* de la enumeración. El capítulo 6 cubrirá las enumeraciones
con más detalle.

[enums]: ch06-00-enums.html

Para `Result`, las variantes son `Ok` o `Err`. La variante `Ok` indica que
la operación fue exitosa, y dentro de `Ok` está el valor generado exitosamente.
La variante `Err`significa que la operación falló y `Err` contiene información
sobre cómo o por qué falló la operación.

El propósito de estos tipos `Result` es codificar la información de manejo de errores.
Los valores del tipo `Result`, como los valores de cualquier tipo, tienen métodos definidos en
ellos. Una instancia de `io::Result` tiene un [método `expect`][expect]<!-- ignore
--> que se puede llamar. Si esta instancia de `io::Result` es un valor `Err`,
`expect` hará que el programa se bloquee y muestre el mensaje que se ha
pasado como argumento para `expect`. Si el método `read_line` devuelve un `Err`,
probablemente sea el resultado de un error proveniente del sistema operativo 
subyacente. Si esta instancia de `io::Result` es un valor `Ok`, `expect` tomará
el valor de retorno que tiene `Ok` y devuelve solo ese valor para que
pueda usarlo. En este caso, ese valor es el número de bytes que el usuario
ha ingresado en la entrada estándar.

[expect]: ../std/result/enum.Result.html#method.expect

Si no llama a `expect`, el programa se compilará, pero recibirá una advertencia:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust advierte que no ha utilizado el valor de `Result` devuelto por `read_line`,
indicando que el programa no ha manejado un posible error.

La forma correcta de suprimir la advertencia es escribir realmente el manejo de errores, pero
si solo desea bloquear este programa cuando ocurre un problema, puede usar
`expect`. Aprenderá a recuperar errores en el Capítulo 9.

### Impresión de Valores con Marcadores de Posición `println!`

Aparte del corchete de cierre, solo hay una línea más que discutir en
el código agregado hasta ahora, que es la siguiente:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Esta línea imprime la cadena en la que guardamos la entrada del usuario. El conjunto de
corchetes, `{}`, es un marcador de posición; piense en `{}` como pequeñas pinzas de cangrejo que
mantienen un valor en su lugar. Puede imprimir más de un valor utilizando llaves;
el primer conjunto de llaves contiene el primer valor enumerado después del formato
cadena, el segundo conjunto contiene el segundo valor, y así sucesivamente. Imprimir múltiples
valores en una llamada a `println!` se verían así:

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

Este código imprimirá `x = 5 and y = 10`.

### Testeando la Primera Parte

Probemos la primera parte del juego de adivinanzas. Ejecútelo usando `cargo run`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```
En este punto, la primera parte del juego está lista: está recibiendo información del
teclado y luego la imprime.

## Generando un Número Secreto

A continuación, necesitamos generar el número secreto que el usuario intentará adivinar.
El número secreto debe ser diferente cada vez para que sea divertido jugar mas
de una vez. Usemos un número aleatorio entre 1 y 100 para que el juego no sea demasiado
difícil. Rust aún no incluye la funcionalidad de números aleatorios en su biblioteca estándar.
Sin embargo, el equipo de Rust proporciona una [caja `rand`][randcrate].

[randcrate]: https://crates.io/crates/rand

### Uso de una Caja para Obtener más Funcionalidad

Recuerde que una caja es una colección de archivos de código fuente Rust.
El proyecto que hemos estado construyendo es una *caja binaria*, que es un ejecutable.
La caja `rand` es una *caja de biblioteca*, que contiene código destinado a ser
utilizado en otros programas.

El uso de cajas externas es donde realmente brilla Cargo. Antes de que podamos escribir el
código que usa `rand`, necesitamos modificar el archivo *Cargo.toml* para incluir la caja
`rand` como dependencia. Abra ese archivo ahora y agregue la siguiente línea a
la parte inferior debajo del encabezado de la sección `[dependencies]` que Cargo creó para
usted. Asegúrese de especificar `rand` exactamente como lo tenemos aquí, o los ejemplos de código en
este tutorial posiblemente no funcionen.

<!-- Al actualizar la versión de `rand` utilizada, actualice también la versión de
`rand` en estos archivos para que todos coincidan:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Nombre de archivo: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

En el archivo *Cargo.toml*, todo lo que sigue a un encabezado es parte de una sección,
que continúa hasta que comienza otra sección. La sección `[dependencies]` es
donde se le dice a Cargo de qué cajas externas depende su proyecto y que
versiones de esas cajas necesita. En este caso, especificaremos la caja `rand`
con el especificador de versión semántica `0.8.3`. Cargo entiende [Semantic
Versioning][semver]<!-- ignore --> (a veces llamado *SemVer*), que es un
estándar para escribir números de versión. El número `0.8.3` es en realidad una abreviatura
para `^0.8.3`, que significa cualquier versión que sea al menos `0.8.3` pero inferior
`0.9.0`. Cargo considera que estas versiones tienen API públicas compatibles con la
versión `0.8.3`, y esta especificación garantiza que obtendrá el último parche de
versión que aún se compilará con el código de este capítulo. Cualquier versión
`0.9.0` o superior no garantiza que tenga la misma API que los siguientes
ejemplos de uso.

[semver]: http://semver.org

Ahora, sin cambiar nada del código, compilemos el proyecto, como se muestra en
Listado 2-2.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo clean
cargo build -->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
   Compiling rand_core v0.6.2
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

<span class="caption">Listado 2-2: El resultado de ejecutar `cargo build` después
de agregar la caja rand como una dependencia</span>

Es posible que vea diferentes números de versión (pero todos serán compatibles con
el código, gracias a SemVer!), diferentes líneas (dependiendo del sistema
operativo), y las líneas pueden estar en un orden diferente.

Ahora que tenemos una dependencia externa, Cargo obtiene las últimas versiones de
todo desde el *registry*, que es una copia de los datos de
[Crates.io][cratesio]. Crates.io es donde las personas del ecosistema Rust publican
sus proyectos Rust de código abierto para que otros los usen.

[cratesio]:https://crates.io/

Después de actualizar el registro, Cargo verifica la sección `[dependencies]` y
descarga las cajas que aún no tiene. En este caso, aunque solo enumeramos
`rand` como una dependencia, Cargo también tomó otras cajas de las que depende `rand`
para trabajar. Después de descargar las cajas, Rust las compila y luego compila el
proyecto con las dependencias disponibles.

Si ejecuta inmediatamente `cargo build` nuevamente sin realizar ningún cambio,
no obtendrá ningún resultado aparte de la línea `Finished`. Cargo sabe que ya
descargó y compiló las dependencias, y no ha cambiado nada
sobre ellas en su archivo *Cargo.toml*. Cargo también sabe que no ha cambiado
nada en el código, por lo que tampoco lo vuelve a compilar. Sin nada que
hacer, simplemente sale.

Si abre el archivo *src/main.rs*, hace un cambio trivial y luego lo guarda
y compila de nuevo, solo verá dos líneas de salida:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

Estas líneas muestran que Cargo solo actualiza la construcción con el pequeño cambio en el
archivo *src/main.rs*. Sus dependencias no han cambiado, por lo que Cargo sabe que puede
reutilizar lo que ya ha descargado y compilado para ellas. Simplemente reconstruye
su parte del código.

#### Garantizar Compilaciones Reproducibles con el Archivo *Cargo.lock*

Cargo tiene un mecanismo que garantiza que pueda reconstruir el mismo artefacto,
en todo momento, cuando usted o cualquier otra persona construye su código; Cargo utilizará solo las versiones de
dependencias que especificó hasta que indique lo contrario. Por ejemplo, ¿qué
sucede si la próxima semana sale la versión 0.8.4 de la caja `rand` y
contiene una corrección de errores importante, pero también contiene una regresión que romperia
su codigo?

La respuesta a este problema es el archivo *Cargo.lock*, que se creó
la primera vez que ejecutó `cargo build` y ahora está en su directorio *guess_game*.
Cuando construye un proyecto por primera vez, Cargo descubre todos las
versiones de las dependencias que se ajustan a los criterios y luego las escribe en
el archivo *Cargo.lock*. Cuando construya su proyecto en el futuro, Cargo
verá que el archivo *Cargo.lock* existe y usará las versiones especificadas allí
en lugar de hacer todo el trabajo de averiguar las versiones de nuevo. Esto permite
tener una construcción reproducible automáticamente. En otras palabras, su proyecto
permanecerá con `0.8.3` hasta que actualice explícitamente, gracias al archivo *Cargo.lock*.

#### Actualización de una Caja para Obtener una Nueva Versión

Cuando *desea* actualizar una caja, Cargo proporciona otro comando, `update`,
que ignorará el archivo *Cargo.lock* y descubrirá las últimas versiones
que se ajusten a sus especificaciones en *Cargo.toml*. Si eso funciona, Cargo escribirá
esas versiones al archivo *Cargo.lock*.

Pero, de forma predeterminada, Cargo solo buscará versiones superiores a `0.8.3` y menores
que `0.9.0`. Si se han lanzado dos nuevas versiones de la caja `rand` ,` 0.8.4` y
`0.9.0`, vería lo siguiente si ejecutara `cargo update`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

En este punto, también notaría un cambio en su archivo *Cargo.lock* señalando
que la versión de la caja `rand` que está usando ahora es` 0.8.4`.

Si desea utilizar la versión `rand 0.9.0` o cualquier versión de la serie `0.9.x`
, tendrías que actualizar el archivo *Cargo.toml* para que tenga este aspecto:

```toml
[dependencies]
rand = "0.9.0"
```

La próxima vez que ejecute `cargo build`, Cargo actualizará el registro de cajas
disponible y reevaluara sus requisitos de "rand" de acuerdo con la nueva versión
especificada.

Hay mucho más que decir sobre [Cargo][doccargo]<!-- ignore --> y 
[su ecosistema][doccratesio]<!-- ignore --> que discutiremos en el Capítulo 14, pero
por ahora, eso es todo lo que necesita saber. Cargo hace que sea muy fácil reutilizar
bibliotecas, por lo que los rustáceos pueden escribir proyectos más pequeños que se ensamblan
desde varios paquetes.

[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html

### Generando un Número Aleatorio

Ahora que ha agregado la caja `rand` a *Cargo.toml*, comencemos a usar
`rand`. El siguiente paso es actualizar *src/main.rs*, como se muestra en el Listado 2-3.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<span class="caption">Listado 2-3: Agregar código para generar un
número aleatorio</span>

Primero, agregamos una línea `use`: `use rand::Rng`. El trait (rasgo) "Rng" define
métodos que implementan los generadores de números aleatorios, y este trait debe estar en
el alcance para que usemos esos métodos. El capítulo 10 cubrirá los traits en detalle.

A continuación, agregamos dos líneas en medio. La función `rand::thread_rng`
nos dará el generador de números aleatorios particular que vamos a utilizar:
uno que es local al hilo de ejecución actual y "sembrado" por el
sistema operativo. Luego llamamos al método `gen_range` en el generador de números
aleatorios. Este método está definido por el trait `Rng` que trajimos al alcance
con la instrucción `use rand::Rng`. El método `gen_range` toma una expresión rango
como argumento y genera un número aleatorio dentro del rango. El tipo
de la expresión rango que usamos aquí tiene la forma `start..end`. Es
inclusivo en el límite inferior pero exclusivo en el límite superior, por lo que necesitamos
especificar `1..101` para solicitar un número entre 1 y 100. Alternativamente, podríamos
pasa el rango `1..=100`, que es equivalente.

> Nota: no solo necesita saber qué traits usar y qué métodos y funciones
> llamar desde una caja. Las instrucciones para usar una caja están en la
> documentación de cada caja. Otra característica interesante de Cargo es que puede ejecutar el
> comando `cargo doc --open`, que construirá la documentación proporcionada por todas sus
> dependencias localmente y la abre en su navegador. Si está interesado en
> otra funcionalidad en la caja `rand`, por ejemplo, ejecute` cargo doc --open`
> y haga clic en "rand" en la barra lateral de la izquierda.

La segunda línea que agregamos a la mitad del código imprime el número
secreto. Esto es útil mientras desarrollamos el programa para poder probar
, pero lo eliminaremos de la versión final. No es un gran juego si el
programa imprime la respuesta tan pronto como se inicia.

Intente ejecutar el programa varias veces:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

Debe obtener diferentes números aleatorios, y todos deben ser números entre
1 y 100. ¡Buen trabajo!

## Comparando la Conjetura con el Numero Secreto

Ahora que tenemos la entrada del usuario y un número aleatorio, podemos compararlos. Ese paso
se muestra en el Listado 2-4. No compile este código aún, hasta que lo
expliquemos.

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<span class="caption">Listado 2-4: Manejo de los posibles valores devueltos al
comparar dos números</span>

Aquí, lo primero nuevo es otra declaración `use`, que trae un tipo llamado
`std::cmp::Ordering` en el alcance de la biblioteca estándar. Como `Result`,
`Ordering` es otra enumeración, pero las variantes de` Ordering` son `Less`,
`Greater` y `Equal`. Son los tres resultados posibles cuando
se comparan dos valores.

Luego agregamos cinco líneas nuevas en la parte inferior que usan el tipo `Ordering`.
El método `cmp` compara dos valores, y se puede llamar con cualquier cosa que pueda ser
comparada. Toma una referencia a lo que se quiera comparar; aquí está
comparando `guess` con `secret_number`. Luego devuelve una variante de
la enumeración `Ordering`, que incorporamos al alcance con la declaración `use`. Usamos una
expresión [`match`][match]<!-- ignore --> para decidir qué hacer a continuación según
qué variante de `Ordering` se devolvió de la llamada a `cmp` con los valores
de `guess` y `secret_number`.

[match]: ch06-02-match.html

Una expresión `match` se compone de *arms* (brazos). Un brazo consta de un *pattern* y
el código que debe ejecutarse si el valor dado al comienzo de la expresión `match`
se ajusta al patrón de ese brazo. Rust toma el valor dado para `match` y
mira a través del patrón de cada brazo por turno. La construcción y los patrones `match`
son funciones poderosas en Rust que le permiten expresar una variedad de situaciones que
el código puede encontrar y asegúrese de manejarlas todas. Estas características
se tratarán en detalle en el Capítulo 6 y el Capítulo 18, respectivamente.

Veamos un ejemplo de lo que sucedería con la expresión `match`
utilizada aquí. Digamos que el usuario conjetura 50 y el número secreto generado aleatoriamente
esta vez es 38. Cuando el código compara 50 y 38, el método `cmp`
retorna `Ordering::Greater`, porque 50 es mayor que 38. La expresión `match`
obtiene el valor `Ordering::Greater` y comienza a comprobar cada brazo
del patrón. Mira el patrón del primer brazo, `Ordering::Less`, y ve que
el valor `Ordering::Greater` no coincide con `Ordering::Less`, por lo que ignora
el código en ese brazo y pasa al siguiente brazo. El patrón del siguiente brazo,
`Ordering::Greater`, coincide con `Ordering::Greater`! El código asociado en
ese brazo se ejecutará e imprimirá "¡Demasiado grande!" en la pantalla. La expresión
`match` termina porque no tiene necesidad de mirar el último brazo en este escenario.

Sin embargo, el código del Listado 2-4 aún no se compilará. Vamos a intentarlo:

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

El núcleo del error indica que hay *mismatched types*. Rust tiene un
sistema de tipos fuerte y estático. Sin embargo, también tiene inferencia de tipos. Cuando escribimos
`let mut guess = String::new()`, Rust pudo inferir que `guess` debería ser
una `String` y no nos obligó a escribir el tipo. Pero `secret_number`, por otro
lado, es un tipo numérico. Algunos tipos numéricos pueden tener un valor entre 1 y 100:
`i32`, un número de 32 bits; `u32`, un número de 32 bits sin signo; `i64`, un número de 64 bits
; y otros. Rust tiene por defecto `i32`, que es el tipo de
`secret_number` a menos que agregue información de tipo en otro lugar que causaría que Rust
infiera un tipo numérico diferente. El motivo del error es que Rust
no puede comparar una cadena y un tipo numérico.

En última instancia, queremos convertir la `String`, que el programa lee como entrada, en un
tipo numérico real para que podamos compararlo numéricamente con el número secreto. Podemos
hacer eso agregando otra línea al cuerpo de la función `main`:

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

La linea es:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Creamos una variable llamada `guess`. Pero espere, ¿el programa ya tiene
una variable llamada `guess`? Si, la tiene, pero Rust nos permite *sombrear* el anterior
valor de `guess` con uno nuevo. Esta función se utiliza a menudo en situaciones en
que desea convertir un valor de un tipo a otro. El sombreado permite
reutilizar el nombre de la variable `guess` en lugar de obligarnos a crear dos
variables, como `guess_str` y `guess`, por ejemplo. (El Capítulo 3 cubre
sombreado con más detalle.)

Vinculamos `guess` a la expresión `guesss.trim().Parse()`. `guess` en la
expresión se refiere a la "suposición" original que era una "Cadena" con la entrada en
ella. El método `trim` en una instancia de `String` eliminará cualquier espacio en blanco
al principio y al final. Aunque `u32` solo puede contener caracteres numéricos,
el usuario debe presionar <span class="keystroke">enter</span> para satisfacer
`read_line`. Cuando el usuario presiona <span class="keystroke">enter</span>, se agrega a la cadena un
carácter de nueva línea. Por ejemplo, si el usuario escribe <span
class="keystroke">5</span> y presiona <span class="keystroke">enter</span>,
`guess` se ve así: `5\n`. `\n` representa "nueva línea", el resultado de
presionar <span class="keystroke">enter</span> (en Windows, presionando <span
class="keystroke">enter</span> da como resultado un retorno de carro y una nueva línea,
`\r\n`). El método `trim` elimina `\n` o `\r\n`, lo que da como resultado solo un `5`.

El método [`parse`][parse]<!-- ignore --> en cadenas analiza una cadena en algunos
tipos de número. Debido a que este método puede analizar una variedad de tipos de números,
necesitamos decirle a Rust el tipo de número exacto que queremos usando `let guess: u32`. Los
dos puntos (`:`) después de `guess` le dice a Rust que anotaremos el tipo de variable. Rust
tiene algunos tipos de números integrados; el `u32` que se ve aquí es un entero de 32 bits 
sin signo. Es una buena opción predeterminada para un número positivo pequeño. Usted aprenderá
sobre otros tipos de números en el Capítulo 3. Además, la notación `u32` en
este programa de ejemplo y la comparación con `secret_number` significa que Rust
inferirá que `secret_number` debería ser también un `u32`. Ahora 
¡la comparación será entre dos valores del mismo tipo!

[parse]: ../std/primitive.str.html#method.parse

La llamada a `parse` fácilmente podría causar un error. Si, por ejemplo, la cadena
contenía `A👍%`, no habría forma de convertir eso en un número. Ya que
puede fallar, el método `parse` devuelve un tipo `Result`, al igual que hace `read_line`
(discutido anteriormente en ["Manejo de fallos potenciales con el
tipo `Result`](#manejo-de-fallos-potenciales-con-el-tipo-result)<!-- ignore
-->). Trataremos este `Result` de la misma manera mediante el método `expect`
de nuevo. Si `parse` devuelve una variante de `Err` de `Result` porque no se pudo crear
un número desde la cadena, la llamada `expect` bloqueará el juego e imprimirá el
mensaje que le damos. Si `parse` puede convertir correctamente la cadena en un número,
devolverá la variante `Ok` de `Result`, y `expect` devolverá el
número que queremos del valor `Ok`.

¡Ejecutemos el programa ahora!

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

¡Bien! Aunque se agregaron espacios antes del valor conjeturado, el programa considera
que el usuario conjeturaba 76. Ejecute el programa varias veces para verificar
comportamientos diferentes con diferentes tipos de entrada: adivinar el número correcto,
adivinar un número demasiado alto y adivinar un número demasiado bajo.

Tenemos la mayor parte del juego funcionando ahora, pero el usuario solo puede hacer una suposición.
¡Cambiemos eso agregando un bucle!

## Permitir Múltiples Suposiciones con Un Bucle

La palabra clave `loop` crea un bucle infinito. Agregaremos un bucle para brindarles a los usuarios
mas oportunidades de adivinar el numero:

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Como puede ver, hemos movido todo en un bucle, desde el indicador de entrada de conjetura en
adelante. Asegúrese de sangrar las líneas dentro del bucle, otros cuatro espacios cada una,
y vuelva a ejecutar el programa. Observe que hay un nuevo problema porque el
programa está haciendo exactamente lo que le dijimos que hiciera: ¡pedir otra conjetura eternamente!
¡No parece que el usuario pueda salir!

El usuario siempre puede interrumpir el programa usando el atajo de teclado <span
class="keystroke">ctrl-c</span>. Pero hay otra forma de escapar de este
monstruo insaciable, como se menciona en la discusión de "parse" en ["Comparando 
la Conjetura con el Número Secreto”](#comparando-la-conjetura-con-el-numero-secreto)<!--
ignore -->: si el usuario ingresa una respuesta no numérica, el programa se bloqueará.
El usuario puede aprovechar eso para salir, como se muestra aquí:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
salir
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Escribir "salir" en realidad cierra el juego, pero también lo hará cualquier otra entrada que no sea un número.
Sin embargo, esto no es óptimo, por decir poco. Queremos que el juego automáticamente se
deténga cuando se adivine el número correcto.

### Salir Despues de una Suposicion Correcta

Programemos el juego para que se cierre cuando el usuario gane agregando una declaración "break":

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

Agregar la línea `break` después de `You win!' hace que el programa salga del ciclo cuando
el usuario adivina correctamente el número secreto. Salir del bucle también significa
salir del programa, porque el bucle es la última parte de `main`.

### Manejo de Entradas no válidas

Para refinar aún más el comportamiento del juego, en lugar de bloquear el programa cuando
el usuario ingresa algo que no es un número, hagamos que el juego lo ignore para que el
el usuario puede seguir adivinando. Podemos hacer eso alterando la línea donde `guess`
se convierte de un `String` a un `u32`, como se muestra en el Listado 2-5.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<span class="caption">Listado 2-5: Ignorar una suposición que no sea numérica y pedir
otra suposición en lugar de bloquear el programa</span>

Cambiar de una llamada `expect` a una expresión `match` es lo que generalmente
distingue fallar por error a manejar el error. Recuerde que `parse`
devuelve un tipo `Result` y `Result` es una enumeración que tiene las variantes `Ok` o
`Err`. Estamos usando una expresión `match` aquí, como hicimos con `Ordering`
resultado del método `cmp`.

Si `parse` es capaz de convertir correctamente la cadena en un número,
devuelve un valor `Ok` que contiene el número resultante. Ese valor de `Ok` debe
coincidir con el patrón del primer brazo, y la expresión `match` sólo devolverá el
valor `num` que produjo `parse` y se puso dentro del valor `Ok`. Ese número
terminará justo donde lo queremos; en la nueva variable `guess` que estamos creando.

Si `parse` *no* puede convertir la cadena en un número, devolverá un
valor `Err` que contiene más información sobre el error. El valor de `Err`
no coincide con el patrón `Ok(num)` en el primer brazo `match`, pero sí
coincide con el patrón `Err(_)` en el segundo brazo. El guión bajo, `_`, es un
valor de captura; en este ejemplo, estamos diciendo que queremos hacer coincidir todos los 
valores `Err`, sin importar la información que contengan. Entonces el programa
ejecutará el código del segundo brazo, `continue`, que le dice al programa que vaya a la
siguiente iteración de `loop` y pida otra conjetura. Entonces, efectivamente,
¡El programa ignora todos los errores que pueda encontrar "parse"!

Ahora todo en el programa debería funcionar como se esperaba. Vamos a intentarlo:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

¡Impresionante! Con un pequeño ajuste final, terminaremos el juego de adivinanzas. Recordar
que el programa todavía está imprimiendo el número secreto. Eso funcionó bien para
pruebas, pero arruina el juego. Eliminemos el `println!` que muestra el
número secreto. El listado 2-6 muestra el código final.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<span class="caption">Listado 2-6: código completo del juego de adivinanzas</span>

## Resumen

En este punto, ha creado con éxito el juego de adivinanzas. ¡Felicidades!

Este proyecto fue una forma práctica de presentarle muchos conceptos nuevos de Rust:
`let`,` match`, métodos, funciones asociadas, el uso de cajas externas y
más. En los próximos capítulos, aprenderá sobre estos conceptos con más
detalle. El capítulo 3 cubre conceptos que tienen la mayoría de los lenguajes de programación, como
variables, tipos de datos y funciones, y muestra cómo usarlos en Rust.
El capítulo 4 explora la propiedad, una característica que hace que Rust sea diferente de otros
lenguajes. El Capítulo 5 analiza las estructuras y la sintaxis de métodos, y el Capítulo 6
explica cómo funcionan las enumeraciones.

[variables-and-mutability]:
ch03-01-variables-and-mutability.html#variables-y-mutabilidad
