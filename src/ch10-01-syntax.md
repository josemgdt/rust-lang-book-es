## Tipos de Datos Genéricos

Podemos usar genéricos para crear definiciones para elementos como firmas de funciones o
structs, que luego podemos usar con muchos tipos de datos concretos diferentes. Vamos
a ver primero cómo definir funciones, estructuras, enumeraciones y métodos usando
genéricos. Luego, analizaremos cómo los genéricos afectan el rendimiento del código.

### En Definiciones de Funciones

Al definir una función que usa genéricos, colocamos los genéricos en la definición
de la función donde normalmente especificaríamos los tipos de datos de
parámetros y valor de retorno. Esto hace que nuestro código sea más flexible y proporciona
más funcionalidad para las llamadas a nuestra función al tiempo que evita la duplicación de código.

Continuando con nuestra función `largest`, el Listado 10-4 muestra dos funciones que
encuentran el valor más grande en un slice.

<span class="filename">​​Nombre de archivo:src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

<span class="caption">Listado 10-4: dos funciones que solo difieren en su
nombres y tipos en sus definiciones</span>

La función `larger_i32` es la que extrajimos en el Listado 10-3 que encuentra
el `i32` más grande en un slice. La función `larger_char` encuentra el `char` más grande
en un slice. Los cuerpos de las funciones tienen el mismo código, así que eliminemos
la duplicación mediante la introducción de un parámetro de tipo genérico en una sola función.

Para parametrizar los tipos en la nueva función que definiremos, necesitamos nombrar el tipo de
parámetro, al igual que lo hacemos con los parámetros de valor de una función. Usted puede
utilizar cualquier identificador como nombre de tipo de parámetro. Pero usaremos "T" porque, por
convención, los nombres de los parámetros en Rust son cortos, a menudo solo una letra, y
la convención de nomenclatura de tipos es CamelCase (separación de palabras con mayusculas)). La abreviatura de "tipo",
`T` es la elección de valor predeterminado de la mayoría de los programadores de Rust.

Cuando usamos un parámetro en el cuerpo de la función, tenemos que declarar el
nombre del parámetro en la declaración para que el compilador sepa lo que significa ese nombre.
De manera similar, cuando usamos un nombre de tipo de parámetro en una declaración de función, tenemos
para declarar el nombre de tipo del parámetro antes de usarlo. Para definir la función genérica
`largest`, coloque las declaraciones del nombre del tipo entre paréntesis angulares,`<>`,
entre el nombre de la función y la lista de parámetros, así:

```rust,ignore
fn largest<T>(list: &[T]) -> T {
```

Leemos esta definición como "la función `largest` es genérica sobre algún tipo
`T`. Esta función tiene un parámetro llamado `list`, que es un slice de valores
de tipo `T`. La función `largest` devolverá un valor del mismo tipo `T`."

El Listado 10-5 muestra la definición combinada de la función `largest` usando el
tipo de datos generico en su definición. El listado también muestra cómo podemos llamar a la función
con un slice de valores `i32` o valores `char`. Tenga en cuenta que este código no
compilar todavía, pero lo arreglaremos más adelante en este capítulo.

<span class="filename">​​Nombre de archivo:src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

<span class="caption">Listado 10-5: una definición de la función `largest` que
usa parámetros de tipo genérico pero aún no compila</span>

Si compilamos este código ahora mismo, obtendremos este error:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

La nota menciona `std::cmp::PartialOrd`, que es un *trait*. Hablaremos de
traits en la siguiente sección. Por ahora, este error establece que el cuerpo de
`largest` no funcionará para todos los tipos posibles de `T`. Como queremos
comparar valores de tipo `T` en el cuerpo, solo podemos usar tipos cuyos valores
se pueden ordenar. Para permitir las comparaciones, la biblioteca estándar tiene el trait
`std::cmp::PartialOrd` que puede implementar en tipos (consulte el Apéndice C
para obtener más información sobre este trait). Aprenderá como especificar que un tipo genérico tiene un
trait particular en la sección ["Traits como parámetros"][traits-as-parameters]<!--
ignore -->, pero primero exploremos otras formas de usar parámetros de tipo genérico.

### En Definiciones de Estructuras

También podemos definir estructuras para usar un parámetro de tipo genérico en uno o más
campos usando la sintaxis `<>`. El Listado 10-6 muestra cómo definir una struct `Point<T>`
para contener valores de coordenadas `x` e` y` de cualquier tipo.

<span class="filename">​​Nombre de archivo:src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

<span class="caption">Listado 10-6: Una estructura `Point <T>` que contiene valores `x` e` y`
de tipo `T`</span>

La sintaxis para usar genéricos en las definiciones de estructuras es similar a la que se usa en
definiciones de funciones. Primero, declaramos el nombre de tipo del parámetro dentro
corchetes angulares justo después del nombre de la estructura. Entonces podemos usar el tipo genérico
en la definición de la estructura donde de otra manera especificaríamos tipos de datos concretos.

Tenga en cuenta que debido a que solo hemos utilizado un tipo genérico para definir `Point<T>`, esta
definición dice que la estructura `Point<T>` es genérica sobre algún tipo `T`, y
los campos `x` e `y` son *ambos* del mismo tipo, cualquiera que sea ese tipo. Si
creamos una instancia de un `Point<T>` que tiene valores de diferentes tipos, como en
Listado 10-7, nuestro código no se compilará.

<span class="filename">​​Nombre de archivo:src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

<span class="caption">Listado 10-7: Los campos `x` e `y` deben ser de tipos iguales
porque ambos tienen el mismo tipo de datos genérico `T`.</span>

En este ejemplo, cuando asignamos el valor entero 5 a `x`, permitimos que el
compilador sepa que el tipo genérico `T` será un número entero para esta instancia de
`Point<T>`. Luego, cuando especificamos 4.0 para `y`, que hemos definido para tener el
mismo tipo que `x`, obtendremos un error de discrepancia de tipo como este:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

Para definir una estructura `Point` donde` x` e `y` son genéricos pero podrían tener
diferentes tipos, podemos usar parámetros múltiples de tipo genérico. Por ejemplo, en
Listado 10-8, podemos cambiar la definición de `Point` para que sea genérico sobre los tipos
`T` y `U` donde `x` es de tipo `T` e `y` es de tipo` U`.

<span class="filename">​​Nombre de archivo:src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

<span class="caption">Listado 10-8: Un `Point<T, U>` genérico sobre dos tipos, por lo
que `x` e` y` pueden ser valores de diferentes tipos</span>

Now all the instances of `Point` shown are allowed! You can use as many generic
type parameters in a definition as you want, but using more than a few makes
your code hard to read. When you need lots of generic types in your code, it
could indicate that your code needs restructuring into smaller pieces.
¡Ahora se permiten todas las instancias mostradas de `Point`! Puede utilizar tantos tipos genéricos
de parámetros en una definición como desee, pero utilizando demasiadas hace
su código difícil de leer. Cuando necesite muchos tipos genéricos en su código,
podría indicar que su código necesita reestructurarse en partes más pequeñas.

### En Definiciones de Enumeración

Como hicimos con las estructuras, podemos definir enumeraciones para contener tipos de datos genéricos en sus
variantes. Echemos otro vistazo a la enumeración `Option <T>` que proporciona la libreria
estándar, que usamos en el Capítulo 6:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Esta definición ahora debería tener más sentido. Como puede ver, `Option <T>`
es una enumeración que es genérica sobre el tipo `T` y tiene dos variantes: `Some`, que
contiene un valor de tipo `T` y una variante `None` que no contiene ningún valor.
Al usar la enumeración `Option <T>`, podemos expresar el concepto abstracto de tener un
valor opcional, y como `Option <T>` es genérico, podemos usar esta abstracción
no importa cuál sea el tipo de valor opcional.

Las enumeraciones también pueden usar varios tipos genéricos. La definición de la
enum `Result` que usamos en el Capítulo 9 es un ejemplo:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

La enumeración `Result` es genérica en dos tipos, `T` y `E`, y tiene dos variantes:
`Ok`, que contiene un valor de tipo `T`, y `Err`, que contiene un valor de tipo
`E`. Esta definición hace que sea conveniente utilizar la enumeración `Result` en cualquier lugar
con una operación que podría tener éxito (devolver un valor de algún tipo `T`) o fallar
(devuelve un error de algún tipo `E`). De hecho, esto es lo que usamos para abrir un archivo
en el Listado 9-3, donde `T` se completó con el tipo `std::fs::File` cuando
el archivo se abrió con éxito y `E` se completó con el tipo
`std::io::Error` cuando hubo problemas para abrir el archivo.

Cuando reconoce situaciones en su código con múltiples estructuras o definición de enumeraciones
que difieren sólo en los tipos de valores que contienen, puede
evitar la duplicación utilizando tipos genéricos en su lugar.

### En Definiciones de Métodos

We can implement methods on structs and enums (as we did in Chapter 5) and use
generic types in their definitions, too. Listing 10-9 shows the 
struct we defined in Listing 10-6 with a method named `x` implemented on it.
Podemos implementar métodos en estructuras y enumeraciones (como hicimos en el Capítulo 5) y también usar
tipos genéricos en sus definiciones. El Listado 10-9 muestra la estructura `Point<T>`
que definimos en el Listado 10-6 con un método llamado `x` implementado en ella.

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

<span class="caption">Listado 10-9: Implementación de un método llamado `x` en la
estructura `Point<T>` que devolverá una referencia al campo `x` de tipo
`T`</span>

Aquí, hemos definido un método llamado `x` en` Point<T>` que devuelve una referencia
a los datos en el campo `x`.

Tenga en cuenta que tenemos que declarar `T` justo después de `impl` para que podamos usarlo para especificar
que estamos implementando métodos en el tipo `Point<T>`. Declarando `T` como
tipo genérico después de `impl`, Rust puede identificar que el tipo en el 
los corchetes angulares en `Point` son un tipo genérico en lugar de un tipo concreto.

Podríamos, por ejemplo, implementar métodos solo en instancias `Point<f32>` en lugar de
en instancias `Point<T>` con cualquier tipo genérico. En el Listado 10-10 usamos el
tipo concreto `f32`, lo que significa que no declaramos ningún tipo después de `impl`.

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

<span class="caption">Listado 10-10: un bloque `impl` que solo se aplica a estructuras con un tipo concreto particular para el parámetro de tipo genérico `T`</span>

Este código significa que el tipo `Point<f32>` tendrá un método llamado
`distance_from_origin` y otras instancias de `Point<T>` donde `T` no es del
tipo `f32` no tendrá este método definido. El método mide qué tan lejos
está el punto desde las coordenadas (0.0, 0.0) y utiliza
operaciones matemáticas que están disponibles solo para tipos de coma flotante.

Los parámetros de tipo genérico en una definición de estructura no siempre son los mismos que los
que usa en las declaraciones de métodos de esa estructura. Por ejemplo, el Listado 10-11 define
el método `mixup` en la estructura `Point<T, U>` del Listado 10-8. El método
toma otro `Point` como parámetro, que puede tener diferentes tipos de
`self` `Point` con el que estamos llamando a `mixup`. El método crea una nueva instancia `Point`
con el valor `x` del ` self` `Point` (de tipo `T`) y el valor `y`
del `Point` pasado (de tipo `W`).

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

<span class="caption">Listado 10-11: un método que usa diferentes tipos genéricos
a partir de la definición de su estructura</span>

En `main`, hemos definido un `Point` que tiene un `i32` para `x` (con valor `5`)
y una `f64` para `y` (con valor `10.4`). La variable `p2` es una estructura `Point`
que tiene un string slice para `x` (con valor `"Hola"`) y un `char` para `y`
(con valor `c`). Llamar a `mixup` en `p1` con el argumento `p2` nos da `p3`,
que tendrá un `i32` para `x`, porque `x` proviene de `p1`. La variable `p3`
tendrá un `char` para `y`, porque `y` proviene de `p2`. La macro `println!`
la llamada imprimirá `p3.x = 5, p3.y = c`.

El propósito de este ejemplo es demostrar una situación en la que algunos
los parámetros se declaran con `impl` y algunos se declaran con la definición del método.
Aquí, los parámetros genéricos `T` y `U` se declaran después de `impl`,
porque van con la definición de estructura. Los parámetros genéricos `V` y `W`
se declaran después de `fn mixup`, porque solo son relevantes para el método.

### Rendimiento del Código Usando Genéricos

Quizás se pregunte si hay un costo de tiempo de ejecución cuando usa
parámetros de tipo genérico. La buena noticia es que Rust implementa genéricos
de una manera que su código no se ejecute más lento usando tipos genéricos de lo que lo haría
con tipos concretos.

Rust logra esto realizando una monomorfización del código que está usando
genéricos en tiempo de compilación. *Monomorfización* es el proceso de volver codigo genérico
a código específico completando los tipos concretos que se utilizan cuando se
compila.

En este proceso, el compilador hace lo contrario de los pasos que usamos para crear
la función genérica en el Listado 10-5: el compilador mira todos los lugares
donde se llama al código genérico y genera código para los tipos concretos con que
se llama al código genérico.

Veamos cómo funciona esto con un ejemplo que usa la enumeración de biblioteca estándar `Option <T>`:

```rust
let integer = Some(5);
let float = Some(5.0);
```

Cuando Rust compila este código, realiza una monomorfización. Durante el
proceso, el compilador lee los valores que se han utilizado en las instancias de `Option<T>`
e identifica dos tipos de `Option<T>`: uno es `i32` y el otro
es `f64`. Como tal, expande la definición genérica de `Option<T>` en
`Option_i32` y `Option_f64`, reemplazando así la definición genérica con
los específicos.

La versión monomorfizada del código tiene el siguiente aspecto. El genérico
`Option<T>` se reemplaza con las definiciones específicas creadas por el compilador:

<span class="filename">​​Nombre de archivo:src/main.rs</span>

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

Debido a que Rust compila código genérico en código que especifica el tipo en cada instancia,
no pagamos ningún costo de tiempo de ejecución por el uso de genéricos. Cuando se ejecuta el código,
funciona como lo haría si hubiéramos duplicado cada definición a mano.
El proceso de monomorfización hace que los genéricos de Rust sean extremadamente eficientes en
tiempo de ejecución.

[traits-as-parameters]: ch10-02-traits.html#traits-as-parameters
