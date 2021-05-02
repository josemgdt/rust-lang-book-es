## Definiendo una Enumeración

Veamos una situación que podríamos querer expresar en código y veamos por qué las enumeraciones
son útiles y más apropiadas que las estructuras en este caso. Digamos que tenemos que trabajar
con direcciones IP. Actualmente, se utilizan dos estándares principales para las direcciones IP;
versión cuatro y versión seis. Estas son las únicas posibilidades para una dirección IP
con la que se encontrará nuestro programa; podemos *enumerar* todos las posibles
variantes, que es de donde la enumeración recibe su nombre.

Cualquier dirección IP puede ser una dirección de la versión cuatro o de la versión seis, pero no
de las dos al mismo tiempo. Esa propiedad de las direcciones IP hace que la enumeración
sea la estructura apropiada, porque los valores de enumeración solo pueden ser una de sus variantes.
Tanto las direcciones de la versión cuatro como de la seis siguen siendo fundamentalmente
direcciones IP, por lo que deben tratarse como del mismo tipo cuando el código está manejando
situaciones que se aplican a cualquier tipo de dirección IP.

Podemos expresar este concepto en código definiendo una enumeración `IpAddrKind` y
enumerando los posibles tipos que puede tener una dirección IP, `V4` y `V6`. Estas son las
variantes de la enumeración:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

`IpAddrKind` es ahora un tipo de datos personalizado que podemos usar en otras partes de nuestro código.

### Valores de Enum

Podemos crear instancias de cada una de las dos variantes de `IpAddrKind` así:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

Tenga en cuenta que las variantes de la enumeración tienen un nombre de espacio en su identificador, y
se usa dos puntos dobles para separar los dos. La razón por la que esto es útil es que ahora
ambos valores `IpAddrKind::V4` e `IpAddrKind::V6` son del mismo tipo; `IpAddrKind`. 
Asi, podemos definir una función que tome cualquier `IpAddrKind`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

Y podemos llamar a esta función con cualquiera de las variantes:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

Usar enumeraciones tiene más ventajas aún. Pensando más en nuestro tipo de dirección IP,
por el momento no tenemos forma de almacenar *datos* de la dirección IP real; 
solo sabemos qué *tipo* es. Dado que acaba de aprender sobre estructuras en
el Capítulo 5, puede abordar este problema como se muestra en el Listado 6-1.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

<span class="caption">Listado 6-1: Almacenamiento de los datos y la variante `IpAddrKind` de
una dirección IP usando una `struct`</span>

Aquí, hemos definido una estructura `IpAddr` que tiene dos campos: un campo `kind`, que
es de tipo `IpAddrKind` (la enumeración que definimos anteriormente) y un campo `address`,
de tipo `String`. Tenemos dos instancias de esta estructura. El primero, `home`, tiene
el valor `IpAddrKind::V4` como su `tipo` con los datos de dirección `127.0.0.1` asociados.
La segunda instancia, `loopback`, tiene la otra variante de
`IpAddrKind` con su valor `kind`, `V6`, y tiene la dirección `::1` asociada con
ella. Usamos una estructura para agrupar los valores `kind` y` address`, por lo que
ahora la variante está asociada con el valor.

Por otro lado, podemos representar el mismo concepto de una manera más concisa usando solo una enumeración,
en lugar de una enumeración dentro de una estructura, colocando datos directamente en cada
variante enum. Esta nueva definición de la enumeración `IpAddr` dice que las variantes tanto` V4` como `V6`
tendrán valores `String` asociados:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

Adjuntamos datos a cada variante de la enumeración directamente, por lo que no es necesario una
estructura extra.

Hay otra ventaja de usar una enumeración en lugar de una estructura; cada variante
puede tener diferentes tipos y cantidades de datos asociados. Las direcciones IP versión cuatro
siempre tendrán cuatro componentes numéricos que tendrán valores
entre 0 y 255. Si quisiéramos almacenar direcciones `V4` como cuatro valores` u8` pero
expresar las direcciones "V6" como un valor `String`, no podríamos hacerlo con
una estructura. Enums maneja este caso con facilidad:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

Hemos mostrado varias formas diferentes de definir estructuras de datos para almacenar la versión
cuatro y seis de direcciones IP. Sin embargo, resulta que querer almacenar
direcciones IP y codificar de qué tipo son es tan común que [la biblioteca estándar
tiene una definición que podemos usar!][IpAddr]<!-- ignore -->. Veamos cómo
la biblioteca estándar define `IpAddr`; tiene la enumeración exacta y las variantes
que hemos definido y utilizado, pero incorpora los datos de la dirección dentro de las variantes en
la forma de dos estructuras diferentes, que se definen de manera diferente para cada
variante:

[IpAddr]: ../std/net/enum.IpAddr.html

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Este código ilustra que puede poner cualquier tipo de datos dentro de una variante de enumeración;
cadenas, tipos numéricos o estructuras, por ejemplo. Incluso puede incluir otro
enum! Además, los tipos de bibliotecas estándar no suelen ser mucho más complicados que
lo que se le ocurran.

Tenga en cuenta que aunque la biblioteca estándar contiene una definición para `IpAddr`,
todavía podemos crear y usar nuestra propia definición sin conflicto porque
no hemos traído la definición de la biblioteca estándar a nuestro alcance. Hablaremos
más sobre cómo incluir tipos en el alcance en el Capítulo 7.

Veamos otro ejemplo de enumeración en el Listado 6-2; este tiene una amplia
variedad de tipos incrustados en sus variantes.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

<span class="caption">Listado 6-2: Una enumeración `Message` cuyas variantes almacenan
diferentes cantidades y tipos de valores</span>

Esta enumeración tiene cuatro variantes con diferentes tipos:

* `Quit` no tiene ningún dato asociado.
* `Move` incluye una estructura anónima en su interior.
* `Write` incluye una sola `String`.
* `ChangeColor` incluye tres valores de tipo `i32`.

Definir una enumeración con variantes como las del Listado 6-2 es similar a
definir diferentes tipos de definiciones de estructura, excepto que la enumeración no usa la
palabra clave `struct` y todas las variantes se agrupan bajo el
tipo `Message`. Las siguientes estructuras podrían contener los mismos datos que los de las variantes
de la enumeración anterior:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

Pero si usamos las diferentes estructuras, cada una de las cuales tiene su propio tipo,
no podría definir tan fácilmente una función para tomar cualquiera de estos tipos de mensajes como
podríamos hacer con la enumeración `Message` definida en el Listado 6-2, que es un solo tipo.

Hay una similitud más entre enumeraciones y estructuras; así como podemos
definir métodos en estructuras usando `impl`, también podemos definir métodos en
enums. Aquí hay un método llamado `call` que podríamos definir en nuestra enumeración `Message`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

El cuerpo del método usaría `self` para obtener el valor con el que llamamos
al método. En este ejemplo, creamos una variable `m` que tiene el valor
`Message::Write(String::from("hola"))`, y eso es lo que será `self` en el
cuerpo del método `call` cuando se ejecute `m.call()`.

Veamos otra enumeración en la biblioteca estándar que es muy común y
útil; `Option`.

### La Enumeración `Option` y sus Ventajas Sobre los Valores Nulos

En la sección anterior vimos cómo la enumeración `IpAddr` nos permite usar el
sistema de tipos de Rust para codificar más información que solo los datos en nuestro programa.
Esta sección explora `Option`, que es otra enumeración definida
por la biblioteca estándar. El tipo `Option` se utiliza en muchos lugares porque
codifica un escenario muy común en el que un valor podría ser algo o
podría no ser nada. Expresar este concepto en términos del sistema de tipos significa que
el compilador puede comprobar si ha manejado todos los casos que debería estar manejando;
esta funcionalidad puede prevenir errores que son extremadamente comunes en otros
lenguajes de programación.

En el diseño de lenguajes de programación a menudo se piensa en términos de las características que
incluye, pero las funciones que excluye también son importantes. Rust no tiene la
característica "nulo" que tienen muchos otros idiomas. *Null* es un valor que significa que
no hay ningún valor. En lenguajes con nulo, las variables siempre pueden estar en uno de
dos estados: nulo o no nulo.

En su presentación de 2009 "Referencias nulas: el error del billón de dólares", Tony
Hoare, el inventor de null, dijo esto:

> Yo lo llamo mi error del billón de dólares. En ese momento, estaba diseñando el primer
> sistema de tipos completo para referencias en un lenguaje orientado a objetos. Mi
> objetivo era garantizar que todo uso de referencias fuera absolutamente seguro, con
> comprobación realizada automáticamente por el compilador. Pero no pude resistir la
> tentación de poner una referencia nula, simplemente porque era muy fácil
> implementarla. Esto ha llevado a innumerables errores, vulnerabilidades y caidas de sistemas,
> que probablemente han causado millones de dólares de dolor y daños en
> los últimos cuarenta años.

El problema con los valores nulos es que si intenta utilizar un valor nulo como
valor no nulo, obtendrá un error de algún tipo. Ya que esta propiedad, nulo o no nulo,
es omnipresente, es extremadamente fácil cometer este tipo de error.

Sin embargo, el concepto que null está tratando de expresar sigue siendo útil; un
null es un valor que actualmente no es válido o está ausente por algún motivo.

El problema no es realmente el concepto, sino la implementación particular.
Como tal, Rust no tiene nulos, pero tiene una enumeración
que puede codificar el concepto de un valor presente o ausente. Esta enumeración es
`Option<T>`, y está [definida por la biblioteca estándar][option]<!-- ignore -->
como sigue:

[option]: ../std/option/enum.Option.html

```rust
enum Option<T> {
    Some(T),
    None,
}
```

La enumeración `Option<T>` es tan útil que incluso se incluye en el preludio;
no es necesario incluirlo en el alcance de forma explícita. Además, también están sus variantes;
puede usar `Some` y` None` directamente sin el prefijo `Option::`. La
enum `Option<T>` sigue siendo una enumeración regular, y `Some(T)` y `None` son
variantes del tipo `Option<T>`.

La sintaxis `<T>` es una característica de Rust de la que aún no hemos hablado. Es un
parámetro de tipo genérico, y cubriremos los genéricos con más detalle en el Capítulo 10.
Por ahora, todo lo que necesita saber es que `<T>` significa que la variante `Some` de
la enumeración `Option` puede contener un dato de cualquier tipo. A continuación se muestran algunos ejemplos
usando valores de `Option` para contener tipos de números y de cadenas:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

Si usamos `None` en lugar de `Some`, debemos decirle a Rust qué tipo de
`Option<T>` tenemos, porque el compilador no puede inferir el tipo que la variante `Some`
puede contener mirando solo un valor `None`.

Cuando tenemos un valor `Some`, sabemos que hay un valor presente y el valor se
mantiene dentro de `Some`. Cuando tenemos un valor `None`, en cierto sentido significa
lo mismo que nulo; no tenemos un valor válido. Entonces, ¿por qué tener
`Option<T>` es mejor que tener null?

En resumen, porque `Option<T>` y `T` (donde `T` puede ser de cualquier tipo) son diferentes
tipos, el compilador no nos permitirá usar un valor de `Option<T>` como si fuera
definitivamente un valor válido. Por ejemplo, este código no se compilará porque está
tratando de agregar un `i8` a una ` Option<i8>`:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

Si ejecutamos este código, obtenemos un mensaje de error como este:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

¡Caramba! De hecho, este mensaje de error significa que Rust no sabe cómo
sumar un `i8` y un `Option<i8> `, porque son tipos diferentes. Cuando nosotros
tenemos un valor de un tipo como `i8` en Rust, el compilador se asegurará de que
siempre tiene un valor válido. Podemos proceder con confianza sin tener que comprobar
para null antes de usar ese valor. Solo cuando tenemos una `Option<i8>` (o
cualquier tipo de valor con el que estemos trabajando) tenemos que preocuparnos posiblemente
de no tener un valor, y el compilador se asegurará de que manejemos ese caso antes
de utilizar el valor.

En otras palabras, usted debe convertir un `Option<T>` en `T` antes de poder
realizar operaciones `T` con él. Generalmente, esto ayuda a detectar uno de los
problemas comunes con null; suponer que algo no es nulo cuando en realidad lo
es.

No tener que preocuparse por asumir incorrectamente un valor no nulo le ayuda a ser
más confiado con su código. Para tener un valor que posiblemente pueda ser
null, debe participar explícitamente haciendo que el tipo de ese valor sea `Option<T>`.
Luego, cuando usa ese valor, debe manejar explícitamente el caso
cuando el valor es nulo. Dondequiera que un valor tenga un tipo que no sea un
`Option<T>`, *puede* asumir con seguridad que el valor no es nulo. Esto fue una
decisión de diseño deliberada de Rust para limitar la omnipresencia de null y aumentar
la seguridad de su código.

Entonces, ¿cómo se obtiene el valor `T` de una variante `Some` cuando se tiene un valor
de tipo `Option<T>` para que pueda usar ese valor? La enumeración `Option<T>` tiene un gran
número de métodos que son útiles en una gran variedad de situaciones; puede revisar
en [su documentación][docs]<!-- ignore -->. Familiarizárse con
los métodos en `Option<T>` serán extremadamente útil en su viaje con Rust.

[documentos]: ../std/option/enum.Option.html

En general, para utilizar un valor de `Option<T>`, deseará tener un código que
manejará cada variante. Necesitará algún código que se ejecute solo cuando tenga un
valor `Some(T)`, y este código puede usar la `T` interna. También necesitará
otro código a ejecutar si tiene un valor `None` y ese código no tiene un
valor `T` disponible. La expresión `match` es una construcción de control de flujo que 
hace justo eso cuando se usa con enumeraciones; ejecutará un código diferente dependiendo de qué
variante de la enumeración se tiene, y ese código puede usar los datos dentro de la coincidencia
de valor.

