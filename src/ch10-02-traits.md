## Traits: Definiendo Comportamientos Compartidos

Un *trait* (rasgo, caracteristica especial) informa al compilador de Rust sobre la funcionalidad 
que tiene un tipo en particular y que puede compartir con otros tipos. Podemos usar traits para 
definir comportamientos compartidos en una forma abstracta. Podemos usar límites de traits para 
especificar que un genérico puede ser cualquier tipo que tenga cierto comportamiento.

> Nota: Los traits son similares a una característica a menudo llamadas *interfaces* en otros
> lenguajes, aunque con algunas diferencias.

### Definiendo un Trait

El comportamiento de un tipo queda definido por los métodos que podemos llamar con ese tipo. Diferentes
tipos comparten el mismo comportamiento si podemos llamar a los mismos métodos en todos ellos.
Las definiciones de traits son una forma de agrupar las declaraciones de métodos para
definir un conjunto de comportamientos necesarios para lograr algún propósito.

Por ejemplo, digamos que tenemos varias estructuras que contienen varios tipos y
cantidades de texto; una estructura `NewsArticle` que contiene una noticia archivada en una
ubicación particular y una `Tweet` que puede tener como máximo 280 caracteres
con metadatos que indican si se trataba de un nuevo tweet, un retweet o una respuesta
a otro tweet.

Queremos hacer una biblioteca de agregación de medios que pueda mostrar resúmenes de datos
que pueden estar almacenados en una instancia de `NewsArticle` o en una `Tweet`. Para hacer esto,
necesitamos un resumen de cada tipo, y necesitamos solicitar ese resumen llamando a un
método `summarize` en una instancia. El Listado 10-12 muestra la definición de un
trait `Summary` que expresa este comportamiento.

<span class="filename"> ​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

<span class="caption">Listado 10-12: Un trait `Summary` que consiste en un
comportamiento proporcionado por un método `summarize`</span>

Aquí, declaramos un trait usando la palabra clave `trait` y luego el nombre del trait,
que es `Summary` en este caso. Dentro de las llaves, declaramos los
métodos que describen los comportamientos de los tipos que implementan este
trait, que en este caso es `fn summarize(&self) -> String`.

Después de la declaración del método, en lugar de proporcionar una implementación dentro de
llaves, usamos un punto y coma. Cada tipo que implemente este trait debe proporcionar
su propio comportamiento personalizado para el cuerpo del método. El compilador hará cumplir
que cualquier tipo que tenga el trait `Summary` tendrá el método `summarize`
definido con exactamente esta declaración.

Un trait puede tener varios métodos en su cuerpo; las declaraciones de método se enumeran
una por línea y cada línea termina en punto y coma.

### Implementar un Trait en un Tipo

Ahora que hemos definido el comportamiento deseado usando el trait `Summary`, podemos
implementarlo en los tipos en nuestro agregador de medios. El listado 10-13 muestra una
implementación del trait `Summary` en la estructura `NewsArticle` que usa el
título, el autor y la ubicación para crear el valor de retorno de
`summarize`. Para la estructura `Tweet`, definimos `summarize` como el nombre de usuario
seguido del texto completo del tweet, asumiendo que el contenido del tweet está
ya limitado a 280 caracteres.

<span class="filename"> ​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

<span class="caption">Listado 10-13: Implementación del trait `Summary` en los
tipos `NewsArticle` y `Tweet`</span>

Implementar un trait en un tipo es similar a implementar métodos regulares.
La diferencia es que después de `impl`, ponemos el nombre del trait que queremos
implementar, y luego se usa la palabra clave `for`, que aqui no representa un bucle, y se especifica 
el nombre del tipo para el que se desea implementar el trait. Dentro del bloque `impl`, ponemos 
las declaraciones de método que la definición del trait habia definido. En lugar de agregar un punto y coma
después de cada declaración, usamos llaves y completamos el cuerpo del método con
el comportamiento específico que queremos que tengan los métodos del trait para el
tipo particular.

Después de implementar el trait, podemos llamar a los métodos en instancias de
`NewsArticle` y `Tweet` de la misma manera que llamamos a métodos regulares, como esto:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs:here}}
```

Este código imprime `1 new tweet: horse_ebooks: of course, as you probably already
know, people`.

Tenga en cuenta que debido a que definimos el trait `Summary` y los tipos `NewsArticle` y
`Tweet` en la misma *lib.rs* en el Listado 10-13, todos están en el mismo
alcance. Supongamos que esta *lib.rs* es para una caja que llamamos `aggregator` y
alguien más quiere usar la funcionalidad de nuestra caja para implementar el trait `Summary`
en una estructura definida dentro del alcance de su biblioteca. Primero necesitaría
llevar el trait a su alcance. Lo harían especificando `use aggregator::Summary;`, que 
le permitiría implementar `Summary` para su tipo. El trait `Summary` también debería ser un 
trait público para que otra caja lo implemente, motivo por el que pusimos la palabra
clave `pub` antes de `trait` en el Listado 10-12.

Una restricción a tener en cuenta con las implementaciones de traits es que podemos implementar un
trait en un tipo solo si el trait o el tipo es local a nuestra caja.
Por ejemplo, podemos implementar características de biblioteca estándar como `Display` en un
tipo personalizado como `Tweet` como parte de nuestra funcionalidad de la caja `aggregator`,
porque el tipo `Tweet` es local a nuestra caja `aggregator`. También podemos
implementar `Summary` en `Vec<T>` en nuestra caja `aggregator`, porque el
trait `Summary` es local a nuestra caja `aggregator`.

Pero no podemos implementar traits externos en tipos externos. Por ejemplo, no podemos
implementar el trait `Display` en `Vec<T>` dentro de nuestra caja `aggregator`,
porque `Display` y `Vec<T>` se definen en la biblioteca estándar y no son
locales a nuestra caja `aggregator`. Esta restricción es parte de una propiedad de los
programas llamada *coherencia*, y más específicamente *regla huérfana*, así llamada
porque el tipo padre no está presente. Esta regla asegura que el código de otras personas
no puede romper nuestro código y viceversa. Sin la regla, dos cajas podrían
implementar el mismo trait para el mismo tipo, y Rust no sabría qué
implementación utilizar.

### Implementaciones Predeterminadas

A veces es útil tener un comportamiento predeterminado para algunos o todos los métodos
en un trait en lugar de requerir implementaciones para todos los métodos en cada tipo.
Luego, a medida que implementamos el trait en un tipo en particular, podemos mantener o modificar
el comportamiento predeterminado de cada método.

El Listado 10-14 muestra cómo especificar una cadena predeterminada para el método `summarize`
del trait `Summary` en lugar de solo definir la declaración del método, como hicimos
en el Listado 10-12.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

<span class="caption">Listado 10-14: Definición de un trait `Summary` con una
implementación predeterminada del método `summarize`</span>

Para usar una implementación predeterminada para resumir instancias de `NewsArticle` en lugar
de definir una implementación personalizada, especificamos un bloque `impl` vacío con
`impl Summary for NewsArticle {}`.

Aunque ya no estamos definiendo el método `summarize` en `NewsArticle`
directamente, proporcionamos una implementación predeterminada y especificamos que
`NewsArticle` implementa el trait `Summary`. Como resultado, todavía podemos llamar
el método `summarize` en una instancia de` NewsArticle`, como este:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

Este código imprime `¡New article available! (Read more...)`.

La creación de una implementación predeterminada para `summarize` no requiere que cambiemos
nada sobre la implementación de `Summary` en `Tweet` en el Listado 10-13.
La razón es que la sintaxis para modificar una implementación predeterminada es la misma
que la sintaxis para implementar un método de trait que no tiene un valor predeterminado de
implementación.

Las implementaciones predeterminadas pueden llamar a otros métodos en el mismo trait, incluso 
si esos otros métodos no tienen una implementación predeterminada. De esta forma, un trait puede
proporcionan una gran cantidad de funciones útiles y solo se requiere que los implementadores
especifiquen una pequeña parte de ella. Por ejemplo, podríamos definir el trait `Summary`
para tener un método `summarize_author` cuya implementación es requerida, y luego definir un
método `summarize` que tiene una implementación predeterminada que llama al método
`summarize_author`:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

Para usar esta versión de `Summary`, solo necesitamos definir `summarize_author`
cuando implementamos el trait en un tipo:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

Después de definir `summarize_author`, podemos llamar a `summarize` en instancias de la
estructura `Tweet`, y la implementación predeterminada de `summarize` llamará a la
definición de `summarize_author` que hemos proporcionado. Ya que hemos implementado
`summarize_author`, el trait `Summary` nos ha dado el comportamiento del
método `summarize` sin que tengamos que escribir más código.

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

Este código imprime `1 new tweet: (Read more from @horse_ebooks...)`.

Tenga en cuenta que no es posible llamar a la implementación predeterminada desde una
implementación rehecha de ese mismo método.

### Traits como Parametros

Ahora que sabe cómo definir e implementar traits, podemos explorar cómo usarlos
para definir funciones que aceptan muchos tipos diferentes.

Por ejemplo, en el Listado 10-13, implementamos el trait `Summary` en los
tipos `NewsArticle` y `Tweet`. Podemos definir una función `notify` que llame
al método `summarize` en su parámetro `item`, que es de algún tipo que
implementa el trait `Summary`. Para hacer esto, podemos usar la sintaxis `impl Trait`,
como esta:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

En lugar de un tipo concreto para el parámetro `item`, especificamos la
palabra clave `impl` y el nombre del trait. Este parámetro acepta cualquier tipo que implemente el
trait especificado. En el cuerpo de `notify`, podemos llamar a cualquier método en `item`
que provienen del trait `Summary`, como `summarize`. Podemos llamar a `notify`
y pasar cualquier instancia de `NewsArticle` o `Tweet`. El código que llame a la
función con cualquier otro tipo, como un `String` o un `i32`, no se compilará
porque esos tipos no implementan `Summary`.

#### Sintaxis de Traits Limitados

La sintaxis `impl Trait` funciona para casos sencillos, pero en realidad es
una sintaxis resumida para una forma más larga, que se llama *trait limitado*, parecida a
esto:

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

Esta forma más larga es equivalente al ejemplo de la sección anterior, pero es
más detallada. Colocamos traits limitados con la declaración del parámetro de tipo genérico
después de dos puntos y dentro de paréntesis angulares.

La sintaxis `impl Trait` es conveniente y permite un código más conciso en
casos simples. La sintaxis de traits limitados puede expresar más complejidad en otros casos.
Por ejemplo, podemos tener dos parámetros que implementen `Summary`. Usando `impl Trait`
la sintaxis se ve así:

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

Si quisiéramos que esta función permitiera que `item1` y` item2` tengan diferentes
tipos, sería apropiado usar `impl Trait` (siempre que ambos tipos implementen
`Summary`). Si quisiéramos forzar que ambos parámetros tuvieran el mismo tipo, eso
solo es posible expresar usando traits limitados, como esto:

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

El tipo genérico `T` especificado como el tipo de los parámetros `item1` e `item2`
*restringe la función* de modo que los tipos concretos de los valores
pasados como argumentos a `item1` e `item2` deben ser iguales.

#### Especificación de Traits Limitados Múltiples con la Sintaxis `+`

También podemos especificar más de un trait limitado. Digamos que necesitamos usar `notify`
para mostrar el formato en `item`, así como el método `summarize`; especificamos en
la definición de `notify` que `item` debe implementar tanto `Display` como
`Summary`. Podemos hacerlo usando la sintaxis `+`:

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

La sintaxis `+` también es válida con traits limitados en tipos genéricos:

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

Con los dos traits limitados especificados, el cuerpo de `notify` puede llamar a `summarize`
y usar `{}` para formatear `item`.


#### Traits Limitados más Claros con Cláusulas `where`

Usar demasiados traits limitados tiene sus desventajas. Cada genérico tiene su propios traits limitados,
por lo que las funciones con múltiples parámetros de tipo genérico pueden contener demasiada
información de traits limitados entre el nombre de la función y su lista de parámetros,
haciendo que la declaración de la función sea difícil de leer. Por esta razón, Rust tiene sintaxis alternativas
para especificar traits limitados dentro de una cláusula `where` después de la declaración de la función.
Por tanto, en lugar de escribir esto:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

podemos usar una cláusula `where`, como esta:

```rust,ignore
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

La declaración de esta función está menos desordenada; el nombre de la función, la lista de parámetros
y el tipo de retorno están juntos, similar a una función sin tantos traits limitados.
.

### Retornando Tipos que Implementan Traits

También podemos usar la sintaxis `impl Trait` en la posición de retorno para devolver un
valor de algún tipo que implemente un trait, como se muestra aquí:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

Al usar `impl Summary` para el tipo de retorno, especificamos que 
la función `returns_summarizable` devuelve algún tipo que implementa el trait `Summary`
sin nombrar el tipo concreto. En este caso, `returns_summarizable`
devuelve un `Tweet`, pero el código que llama a esta función no lo sabe.

La capacidad de devolver un tipo que solo está especificado por el trait que implementa
es especialmente útil en el contexto de cierres e iteradores, que cubriremos
en el Capítulo 13. Los cierres y los iteradores crean tipos que solo el compilador conoce
o tipos que son muy largos de especificar. La sintaxis `impl Trait` le permite
especificar de manera concisa que una función devuelve algún tipo que implementa el
trait `Iterator` sin necesidad de escribir un tipo muy largo.

Sin embargo, solo puede usar `impl Trait` si está devolviendo un solo tipo. Por
ejemplo, este código que devuelve un `NewsArticle` o un` Tweet` con el
el tipo de retorno especificado como `impl Summary` no funcionaría:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

No se permite devolver un `NewsArticle` o un `Tweet` debido a restricciones
de la implementación de la sintaxis `impl Trait` en el compilador. Cubriremos
cómo escribir una función con este comportamiento en la sección ["Uso de objetos trait que
permiten valores de diferentes tipos”][using-trait-objects-that-allow-for-values-of-different-types]<!--
ignore --> del Capítulo 17.

### Arreglando la Función `largest` con Traits Limitados

Ahora que sabe cómo especificar el comportamiento que desea utilizar utilizando límites de parámetro de tipo genérico,
volvamos al Listado 10-5 para corregir la definición de
la función `largest` que usa un parámetro de tipo genérico. La última vez que intentamos
ejecutar ese código, recibimos este error:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

En el cuerpo de `largest` queríamos comparar dos valores de tipo `T` usando el
operador *mayor que* (`>`). Puesto que ese operador está definido como predeterminado
en el trait de la biblioteca estándar `std::cmp::PartialOrd`, necesitamos especificar
`PartialOrd` en el trait limitado para `T`, por lo que la función `largest` puede trabajar
con slices de cualquier tipo que podamos comparar. No necesitamos traer `PartialOrd`
en el alcance porque está en el preludio. Cambie la declaración de `largest` para que
se parezca a esto:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-fixing-listing-10-05/src/main.rs:here}}
```

Esta vez, cuando compilamos el código, obtenemos un conjunto diferente de errores:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-fixing-listing-10-05/output.txt}}
```

La línea clave en este error es `cannot move out of type [T], a non-copy slice`.
Con nuestras versiones no genéricas de la función `largest`, solo intentábamos
buscar el `i32` o `char` más grande. Como se explica en la sección ["Datos solo de pila:
Copy”][stack-only-data-copy]<!-- ignore --> del Capítulo 4, los tipos como
`i32` y `char` que tienen un tamaño conocido se pueden almacenar en la pila, por lo que
implementan el trait `Copy`. Pero cuando hicimos genérica la función `largest`,
se hizo posible que el parámetro `list` tuviera tipos que no
implementan el trait `Copy`. En consecuencia, no podríamos mover el
valor fuera de `list[0]` dentro de la variable `largest`, lo que da como resultado este
error.

Para llamar a este código solo con aquellos tipos que implementan el trait `Copy`, podemos
agregar `Copy` a los trait limitados de `T`. El Listado 10-15 muestra el código completo de
una función genérica `largest` que se compilará siempre que los tipos de
los valores en el slice que pasamos a la función implementen los traits `PartialOrd`
*y* `Copy`, como lo hacen `i32` y `char`.

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/main.rs}}
```

<span class="caption">Listado 10-15: una definición de la función `largest`
que trabaja en cualquier tipo genérico que implemente traits `PartialOrd` y
`Copy`</span>

Si no queremos restringir la función `largest` a los tipos que implementan
el trait `Copy`, podríamos especificar que `T` tiene el trait limitado `Clone` en lugar
de `Copy`. Entonces podríamos clonar cada valor en el slice cuando queramos que
la función `largest` tenga la propiedad. El uso de la función `clone` significa que estamos
potencialmente haciendo más asignaciones en el montón, en el caso de tipos que poseen datos en el 
montón como "String", y las asignaciones en el montón pueden ser lentas si estamos trabajando con
grandes cantidades de datos.

Otra forma en que podríamos implementar `largest` es que la función devuelva una
referencia a un valor `T` en el slice. Si cambiamos el tipo de retorno a `&T`
en lugar de `T`, cambiando así el cuerpo de la función para devolver una
referencia, no necesitaríamos los traits limitados `Clone` o` Copy` y podríamos
evitar las asignaciones en el montón. ¡Intente implementar estas soluciones alternativas por su cuenta!

### Uso de Traits Limitados para Implementar Métodos Condicionalmente

Al usar un trait limitado con un bloque `impl` que usa parámetros de tipo genérico,
podemos implementar condicionalmente métodos para tipos que implementan el trait especificado.
Por ejemplo, el tipo `Pair <T>` en el Listado 10-16 siempre implementa la
función `new`. Pero `Pair <T>` solo implementa el método `cmp_display` si su
tipo interno `T` implementa el trait `PartialOrd` que permite la comparación *y*
el trait `Display` que permite imprimir.

<span class="filename"> ​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/lib.rs}}
```

<span class="caption">Listado 10-16: Implementar métodos condicionalmente en un
tipo genérico según los límites del trait</span>

También podemos implementar condicionalmente un trait para cualquier tipo que implemente
otro trait. Implementaciones de un trait en cualquier tipo que satisfaga trait limitados
se denominan *implementaciones generales* y se utilizan ampliamente en la
biblioteca estándar de Rust. Por ejemplo, la biblioteca estándar implementa el
trait `ToString` en cualquier tipo que implemente el trait `Display`.
El bloque `impl` en la biblioteca estándar es similar a este código:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

Debido a que la biblioteca estándar tiene esta implementación general, podemos llamar al
método `to_string` definido por el trait `ToString` en cualquier tipo que implemente
el trait `Display`. Por ejemplo, podemos convertir enteros en sus correspondientes
valores de `String` porque los enteros implementan `Display`:

```rust
let s = 3.to_string();
```

Las implementaciones generales aparecen en la documentación del trait en la
Sección “Implementadores”.

Los traits y los traits limitados nos permiten escribir código que utiliza parámetros de tipo genérico para
reducir la duplicación, pero también especificar al compilador que queremos el tipo genérico
para tener un comportamiento particular. El compilador puede usar la
información del trait limitado para comprobar que todos los tipos concretos utilizados con nuestro código proporcionan el
comportamiento correcto. En lenguajes tipados dinámicamente, obtendríamos un error en
tiempo de ejecución si llamamos a un método en un tipo que no definió el método. Pero Rust
mueve estos errores en tiempo de compilación, por lo que nos vemos obligados a solucionar los problemas antes
de que nuestro código pueda incluso ejecutarse. Además, no tenemos que escribir código que
compruebe el comportamiento en tiempo de ejecución porque ya lo hemos comprobado en tiempo de compilación.
Hacer esto mejora el rendimiento sin tener que renunciar a la flexibilidad de los genéricos.

Otro tipo de genérico que ya hemos estado usando se llama *lifetime*.
En lugar de garantizar que un tipo tenga el comportamiento que queremos, lifetime garantiza
que las referencias son válidas siempre que las necesitemos. Veamos cómo hace eso lifetime.

[copia-de-datos-de-pila]: ch04-01-what-is-property.html#datos-solo-en-pila--copiar
[uso-de-objetos-trait-que-permiten-valores-de-diferentes-tipos]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types

