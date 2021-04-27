## Traer rutas al alcance con la palabra clave `use`

Puede parecer que las rutas que hemos escrito hasta ahora para llamar a funciones son
inconvenientemente largas y repetitivas. Por ejemplo, en el Listado 7-7, si
eligió la ruta absoluta o relativa a la función `add_to_waitlist`, cada
vez que queríamos llamar a `add_to_waitlist` teníamos que especificar `front_of_house` y
también `hosting`. Afortunadamente, existe una forma de simplificar este proceso. Podemos
traer una ruta a un alcance una vez y luego llamar a los elementos en esa ruta como
items locales con la palabra clave `use`.

En el Listado 7-11, traemos el módulo `crate::front_of_house::hosting` al
alcance de la función `eat_at_restaurant` por lo que solo tenemos que especificar
`hosting::add_to_waitlist` para llamar a la función `add_to_waitlist` en
`eat_at_restaurant`.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

<span class="caption"> Listado 7-11: Incorporación de un módulo al alcance con
`use`</span>

Agregar `use` y una ruta en un alcance es similar a crear un enlace simbólico en
el sistema de archivos. Añadiendo `use crate::front_of_house::hosting` en la caja
root, `hosting` es ahora un nombre válido en ese ámbito, como si el módulo `hosting`
se hubiese definido en la caja raíz. Caminos traídos al alcance con `use`
también verifican la privacidad, como cualquier otro camino.

También puede traer un elemento al alcance con `use` y una ruta relativa. El listado
7-12 muestra cómo especificar una ruta relativa para obtener el mismo comportamiento que en
el listado 7-11.

<span class="filename">Nombre de archivo: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

<span class="caption">Listado 7-12: Traer un módulo al alcance con `use` y
una ruta relativa</span>

### Creando caminos idiomaticos `use`

En el Listado 7-11, es posible que se haya preguntado por qué especificamos `usecrate::front_of_house::hosting`
y luego llamamos a `hosting::add_to_waitlist` en
`eat_at_restaurant` en lugar de especificar la ruta `use` hasta
la función `add_to_waitlist` para lograr el mismo resultado, como en el Listado 7-13.

<span class="filename"> ​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

<span class="caption">Listado 7-13: Incorporación de la función `add_to_waitlist`
en el alcance con `use`, que es unidiomático</span>

Aunque tanto el Listado 7-11 como el 7-13 realizan la misma tarea, el Listado 7-11 es
la forma idiomática de llevar una función al alcance con `use`. Trayendo el
el módulo principal de la función en el alcance con `use`, por lo que tenemos que especificar el módulo principal
al llamar a la función deja en claro que la función no está localmente
definida sin dejar de minimizar la repetición de la ruta completa. El código en el listado
7-13 no es claro sobre dónde se define `add_to_waitlist`.

Por otro lado, al traer estructuras, enumeraciones y otros elementos con `use`,
es idiomático especificar la ruta completa. El listado 7-14 muestra la forma idiomática
para llevar la estructura `HashMap` de la biblioteca estándar al alcance de una caja binaria.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```
<span class = "caption"> Listado 7-14: Llevando `HashMap` al alcance en una
forma idiomática </span>

No hay una razón sólida detrás de este modismo: es solo la convención la que
surgió, y la gente se ha acostumbrado a leer y escribir código Rust de esta manera.

La excepción a este modismo es si traemos dos items con el mismo nombre
dentro del alcance con declaraciones `use`, porque Rust no lo permite. El listado 7-15
muestra cómo traer dos tipos `Result` al alcance que tienen el mismo nombre pero
diferentes módulos principales y cómo hacer referencia a ellos.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

<span class="caption"> Listado 7-15: Incorporación de dos tipos con el mismo nombre en
el mismo alcance requiere el uso de sus módulos principales.</span>

Como puede ver, el uso de los módulos principales distingue los dos tipos de `Result`.
Si en cambio especificamos `use std::fmt::Result` y `use std::io::Result`,
tendriamos dos tipos de `Result` en el mismo ámbito y Rust no sabría a cuál de ellos
se referia cuando usamos `Result`.

### Proporcionar nuevos nombres con la palabra clave `as`

Existe otra solución al problema de traer dos tipos con el mismo nombre
en el mismo ámbito con `use`; después de la ruta, podemos especificar `as` y un nuevo
nombre local, o alias, para el tipo. El listado 7-16 muestra otra forma de escribir el
Listado 7-15 cambiando el nombre de uno de los dos tipos de `Result` usando `as`.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

<span class="caption">Listado 7-16: Cambiar el nombre de un tipo cuando se lleva a
alcance con la palabra clave `as` </span>

En la segunda declaración `use`, elegimos el nuevo nombre `IoResult` para el tipo
`std::io::Result`, que no entrará en conflicto con el `Result` de `std::fmt`
que también hemos incluido en el ámbito de aplicación. Listado 7-15 y Listado 7-16 son
considerados idiomáticos, ¡así que la elección depende de usted!

### Re-exporting Names with `pub use`

When we bring a name into scope with the `use` keyword, the name available in
the new scope is private. To enable the code that calls our code to refer to
that name as if it had been defined in that code’s scope, we can combine `pub`
and . This technique is called *re-exporting* because we’re bringing
an item into scope but also making that item available for others to bring into
their scope.

Listing 7-17 shows the code in Listing 7-11 with `use` in the root module
changed to .

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

<span class="caption">Listing 7-17: Making a name available for any code to use
from a new scope with `pub use`</span>

### Reexportación de nombres con `pub use`

Cuando traemos un nombre al alcance con la palabra clave `use`, el nombre disponible en
el nuevo ámbito es privado. Para habilitar el código que llama a nuestro código para hacer referencia a
ese nombre como si se hubiera definido en el alcance de ese código, podemos combinar `pub`
y `use`. Esta técnica se llama *reexportación* porque estamos trayendo
un elemento en el alcance, pero también haciendo que ese elemento esté disponible para que otros lo traigan
su alcance.

El Listado 7-17 muestra el código del Listado 7-11 con `use` en el módulo raíz
cambiado a `pub use`.

<span class="filename">Nombre de archivo: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

<span class="caption">Listado 7-17: Hacer que un nombre esté disponible para que lo use cualquier código
desde un nuevo alcance con `pub use`</span>

Al usar `pub use`, el código externo ahora puede llamar a la función `add_to_waitlist`
usando `hosting::add_to_waitlist`. Si no hubiéramos especificado `pub use`,
la función `eat_at_restaurant` podría llamar a `hosting::add_to_waitlist` en su
alcance, pero el código externo no podría aprovechar esta nueva ruta.

La reexportación es útil cuando la estructura interna de su código es diferente
a lo que los programadores que llaman a su código pensarían sobre el dominio.
Por ejemplo, en esta metáfora del restaurante, las personas que dirigen el restaurante piensan
en el "frente de la casa" y la "parte trasera de la casa". Pero los clientes que visitan un restaurante
probablemente no pensará en las partes del restaurante en esos términos. Con
`pub use`, podemos escribir nuestro código con una estructura pero exponer una diferente. 
Esto hace que nuestra biblioteca esté bien organizada para los programadores que trabajan en
la biblioteca y los programadores que llaman a la biblioteca.

### Uso de paquetes externos

En el capítulo 2, programamos un proyecto de juego de adivinanzas que utilizaba un
paquete llamado `rand` para obtener números aleatorios. Para usar `rand` en nuestro proyecto,
agregó esta línea a *Cargo.toml*:

<!-- Al actualizar la versión de `rand` utilizada, actualice también la versión de
`rand` usado en estos archivos para que todos coincidan:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<span class = "filename">Nombre de archivo: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

Then, to bring `rand` definitions into the scope of our package, we added a
`use` line starting with the name of the crate, `rand`, and listed the items
we wanted to bring into scope. Recall that in the [“Generating a Random
Number”][rand]<!-- ignore --> section in Chapter 2, we brought the `Rng` trait
into scope and called the `rand::thread_rng` function:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```
Agregar `rand` como una dependencia en * Cargo.toml * le dice a Cargo que descargue el
paquete `rand` y cualquier dependencia de [crates.io](https://crates.io/) y
ponga `rand` a disposición de nuestro proyecto.

Luego, para traer las definiciones de `rand` al alcance de nuestro paquete, agregamos una
línea `use` que comienza con el nombre de la caja, `rand`, y enumera los elementos
que queremos llevar al alcance. Recuerde que en la sección ["Generación de un Número aleatorio ”] [rand] <!-- ignore -->
en el Capítulo 2, trajimos el trait `Rng` en el alcance y llamó a la función `rand::thread_rng`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Los miembros de la comunidad de Rust han puesto a disposición muchos paquetes en
[crates.io](https://crates.io/), y colocar cualquiera de ellos en su paquete
implica estos mismos pasos: incluirlos en el archivo *Cargo.toml* de su paquete y
utilizar `use` para traer items de sus cajas al alcance.

Tenga en cuenta que la biblioteca estándar (`std`) también es una caja externa a nuestro
paquete. Debido a que la biblioteca estándar se incluye con el lenguaje Rust,
no es necesario cambiar *Cargo.toml* para incluir `std`. Pero necesitamos referirnos a ella
con "use" para traer elementos desde allí al alcance de nuestro paquete. Por ejemplo,
con `HashMap` usaríamos esta línea:

```rust
use std::collections::HashMap;
```

Esta es una ruta absoluta que comienza con `std`, el nombre de la caja de biblioteca estándar

### Uso de rutas anidadas para limpiar grandes listas de `use`

Si usamos varios elementos definidos en la misma caja o en el mismo módulo,
enumerar cada elemento en su propia línea puede ocupar mucho espacio en nuestros
archivos. Por ejemplo, estas dos declaraciones de `use` que teníamos en el juego de adivinanzas en
el listado 2-4 trae elementos de `std` al alcance:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

En su lugar, podemos usar rutas anidadas para traer los mismos elementos al alcance en una
línea. Hacemos esto especificando la parte común de la ruta, seguida de dos
dos puntos y, a continuación, corchetes alrededor de una lista de las partes de las rutas que
difieren, como se muestra en el Listado 7-18.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

<span class="caption">Listado 7-18: Especificación de una ruta anidada para traer múltiples
elementos con el mismo prefijo en el alcance</span>

En programas más grandes, traer muchos elementos al alcance de la misma caja o
módulo usando rutas anidadas puede reducir el número de declaraciones de `use` independientes
mucho!

Podemos usar una ruta anidada en cualquier nivel de una ruta, lo cual es útil al combinar
dos declaraciones de `use` que comparten una ruta secundaria. Por ejemplo, el Listado 7-19 muestra dos
sentencias `use`: una que trae `std::io` al alcance y otra que trae
`std::io::Write`.

<span class="filename">Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

<span class="caption">Listado 7-19: Dos declaraciones `use` donde una es una subruta
de la otra</span>

La parte común de estas dos rutas es `std::io`, y es el primer camino completo.
Para fusionar estas dos rutas en una declaración `use`, podemos usar `self` en
la ruta anidada, como se muestra en el listado 7-20.

<span class="filename">Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

<span class="caption">Listing 7-20: Listado 7-20: Combinando las rutas del Listado 7-19 en
una declaración `use`</span>

Esta línea trae `std::io` y `std::io::Write` al alcance.

### El operador Glob

Si queremos traer *todos* los elementos públicos definidos en una ruta al alcance, podemos
especificar esa ruta seguida de `*`, el operador glob:

```rust
use std::collections::*;
```

Esta sentencia `use` trae todos los elementos públicos definidos en `std::collections`
al alcance actual. ¡Tenga cuidado al utilizar el operador glob! Glob puede hacer
más difícil saber qué nombres están dentro del alcance y dónde fue definido un nombre que se usa en su programa.

El operador glob se utiliza a menudo al realizar test para poner todo a prueba
en el módulo `tests`; hablaremos de eso en la sección ["Cómo escribir Pruebas ”][writing-tests]<!-- ignore -->
en el Capítulo 11. El operador glob también se utiliza a veces como parte del patrón 
de preludio: consulte [la documentación de la biblioteca estándar](../std/prelude/index.html#other-preludes)<!-- ignore -->
para obtener más información sobre ese patrón.

[rand]: ch02-00-guessing-game-tutorial.html#generando-un-numero-aleatorio
[writing-tests]: ch11-01-writing-tests.html#cómo-escribir-pruebas
