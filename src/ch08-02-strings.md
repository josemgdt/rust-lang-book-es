## Almacenando Texto Codificado UTF-8 con Strings

Hablamos de cadenas en el Capítulo 4, pero las veremos con más profundidad ahora.
Los nuevos rustáceos comúnmente se atascan en las strings por una combinación de tres
razones: la propensión de Rust a exponer posibles errores, las cadenas son
estructura de datos más complicadas de lo que muchos programadores creen, y
UTF-8. Estos factores se combinan de una manera que puede parecer difícil cuando
se procede de otros lenguajes de programación.

Resulta útil analizar las cadenas en el contexto de las colecciones porque las cadenas
se implementan como una colección de bytes, además de algunos métodos para proporcionar
funcionalidad cuando esos bytes se interpretan como texto. En esta sección, vamos a
hablar sobre las operaciones en `String` que tiene cada tipo de colección, como
crear, actualizar y leer. También discutiremos las formas en las que `String`
es diferente de las otras colecciones, concretamente cómo la indexación en una `String` es
complicada por las diferencias entre cómo las personas y las computadoras interpretan
datos de `String`.

### ¿Qué es una String?

Primero definiremos lo que queremos decir con el término *String*. Rust tiene un solo tipo string,
que es el segmento de cadena `str` que se suele ver en su forma prestada `& str`. 
En el Capítulo 4, hablamos sobre *string slices*,
que son referencias a algunos datos de cadena codificados en UTF-8 almacenados en otro lugar. Las cadenas
literales, por ejemplo, se almacenan en el binario del programa y, por lo tanto, son
string slices.

El tipo `String`, proporcionado por la biblioteca estándar de Rust en lugar de estar
codificado en el lenguaje principal, es un tipo de cadena en código UTF-8 que se puede crecer, mutar y poseer.
Cuando los rustáceos se refieren a "cadenas" en Rust, por lo general se refieren tanto al
tipo `String` y al string slice `&str`.
Aunque esta sección trata principalmente sobre `String`, ambos tipos se utilizan mucho en
la biblioteca estándar de Rust, y tanto `String` como los string slice están codificados en UTF-8.

La biblioteca estándar de Rust también incluye otros tipos de cadenas, como
`OsString`, `OsStr`, `CString` y `CStr`. Las cajas de la biblioteca pueden proporcionar incluso
más opciones para almacenar datos de cadenas. Ve cómo todos esos nombres terminan en "String"
o "Str"? Se refieren a variantes propias y prestadas, al igual que `String` y
`str` que ha visto anteriormente. Estos tipos de cadenas pueden almacenar texto en
codificaciones diferentes o ser representados en la memoria de una manera diferente, por
ejemplo. No discutiremos estos otros tipos de cadenas en este capítulo; ver su
Documentación de la API para obtener más información sobre cómo usarlos y cuándo es apropiado.

### Creando una nueva cadena

Muchas de las operaciones disponibles con `Vec<T>` están disponibles con `String`
también, comenzando con la función `new` para crear una cadena, que se muestra en el Listado
8-11.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

<span class="caption">Listado 8-11: Creación de una nueva `String` vacía</span>

Esta línea crea una nueva cadena vacía llamada `s`, que luego podemos cargar con datos.
A menudo, tendremos algunos datos iniciales que queremos para iniciar la cadena.
Para eso, usamos el método `to_string`, que está disponible en cualquier tipo
que implementa el trait `Display`, como lo hacen los literales de cadena. El listado 8-12 muestra
dos ejemplos.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

<span class="caption">Listado 8-12: Uso del método `to_string` para crear un
`String` de un literal de cadena</span>

Este código crea una cadena que contiene `initial contents`.

También podemos usar la función `String::from` para crear un `String` a partir de una cadena
literal. El código del Listado 8-13 es equivalente al código del Listado 8-12
que usa `to_string`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

<span class="caption">Listado 8-13: Uso de la función `String::from` para crear
una `String` de un literal de cadena</span>

Debido a que las cadenas se utilizan para tantas cosas, podemos utilizar muchos tipos deAPI genéricos diferentes
para cadenas, que nos brindan muchas opciones. Algunos de ellos pueden parecer
redundantes, ¡pero todos tienen su lugar! En este caso, `String::from` y
`to_string` hace lo mismo, por lo que la elección es una cuestión de estilo.

Recuerde que las cadenas están codificadas en UTF-8, por lo que podemos incluir cualquier codificación adecuada
de datos en ellos, como se muestra en el Listado 8-14.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

<span class="caption">Listado 8-14: Almacenamiento de saludos en diferentes idiomas en
cadenas</span>

Todos estos son valores de `String` válidos

### Actualizar una cadena

Un "String" puede aumentar de tamaño y su contenido puede cambiar, al igual que el contenido
de un `Vec<T>`, si inserta más datos en él. Además, puede convenientemente
use el operador `+` o la macro `format!` para concatenar valores de `String`.

#### Agregar a una cadena con `push_str` y `push`

Podemos hacer crecer un `String` usando el método `push_str` para agregar un segmento de string,
como se muestra en el Listado 8-15.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

<span class="caption">Listado 8-15: Agregar un segmento de cadena a una `String`
usando el método `push_str`</span>

Después de estas dos líneas, `s` contendrá `foobar`. El método `push_str` toma un
segmento de cadena porque no necesariamente queremos apropiarnos del
parámetro. Por ejemplo, el código del Listado 8-16 muestra que sería
desafortunado si no pudiéramos usar `s2` después de agregar su contenido a` s1`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

<span class="caption">Listado 8-16: Uso de un segmento de cadena después de agregar su
contenido a una `String`</span>

Si el método `push_str` tomara posesión de `s2`, no podríamos imprimir
su valor en la última línea. Sin embargo, este código funciona como era de esperar.

El método `push` toma un solo carácter como parámetro y lo agrega a la
`String`. El Listado 8-17 muestra el código que agrega la letra `l` a una `String` usando
el método `push`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

<span class="caption">Listado 8-17: Agregar un carácter a un valor de `String`
usando `push`</span>

Como resultado de este código, `s` contendrá `lol`.

#### Concatenacion con el operador `+` o la macro `format!`

A menudo, querrá combinar dos cadenas existentes. Una forma es usar el operador `+`,
como se muestra en el Listado 8-18.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

<span class="caption">Listado 8-18: Uso del operador `+` para combinar dos
Valores de `String` en un nuevo valor de `String`</span>

La cadena `s3` contendrá `¡Hola, mundo!` como resultado de este código. El
motivo por el que `s1` ya no es válido después de la adición y el motivo por el que usamos una
referencia a `s2` tiene que ver con la declaración del método que se llama
cuando usamos el operador `+`. El operador `+` usa el método `add`, cuya
declaración se parece a esto:

```rust,ignore
fn add(self, s: &str) -> String {
```

Esta no es la declaración exacta que se encuentra en la biblioteca estándar: en la
biblioteca estándar, `add` se define usando genéricos. Aquí, estamos mirando la declaración
de `add` con tipos concretos sustituidos por los genéricos, que es lo que
sucede cuando llamamos a este método con valores `String`. Discutiremos los genéricos
en el Capítulo 10. Esta declaración nos da las pistas que necesitamos para comprender los
detalles complicados del operador `+`.

Primero, `s2` tiene un `&`, lo que significa que estamos agregando una *referencia* de la segunda
cadena a la primera debido al parámetro `s` en la función `add`:
solo podemos agregar un `&str` a un `String`; no podemos agregar dos valores de `String`
juntos. Pero espere, el tipo de `&s2` es `&String`, no `&str`, como se especifica en
el segundo parámetro para `add`. Entonces, ¿por qué se compila el Listado 8-18?

La razón por la que podemos usar `&s2` en la llamada a` add` es que el compilador
puede *coarcionar* el argumento `&String` en un `& str`. Cuando llamamos al
método `add`, Rust usa una *coerción deref*, que aquí convierte `&s2` en `&s2[..]`.
Analizaremos la coerción deref con más profundidad en el capítulo 15. Ya que `add` no
no tomar posesión del parámetro `s`, `s2` seguirá siendo una `String` válida
después de esta operación.

En segundo lugar, podemos ver en la declaración que `add` toma posesión de `self`,
porque `self` *no* tiene un `&`. Esto significa que `s1` en el Listado 8-18
se movió a la llamada `add` y ya no será válido después de eso. Así que aunque 
`let s3 = s1 + & s2;` parece que copiará ambas cadenas y creará una nueva, esta
declaración realmente toma posesión de `s1`, agrega una copia del contenido de
`s2`, y luego devuelve la propiedad del resultado. En otras palabras, parece
está haciendo muchas copias, pero no; la implementación es más eficiente
que copiar.

Si necesitamos concatenar varias cadenas, el comportamiento del operador `+`
se vuelve difícil de manejar:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

En este punto, `s` será `tic-tac-toe`. Con todos los caracteres `+` y `"`,
es difícil ver qué está pasando. Para cadenas más complicadas
de combinar, podemos usar la macro `format!`:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

Este código también establece `s` en` tic-tac-toe`. La macro `format!` funciona de la misma forma
como `println!`, pero en lugar de imprimir la salida en la pantalla, devuelve
una `String` con el contenido. La versión del código que usa `format!` Es mucho
más fácil de leer y no se apropia de ninguno de sus parámetros.

### Indexación en cadenas

En muchos otros lenguajes de programación, acceder a caracteres individuales en una
string haciendo referencia a ellos por índice es una operación válida y común. Sin embargo,
si intenta acceder a partes de una `String` utilizando la sintaxis de indexación en Rust,
obtendrá un error. Considere el código no válido del Listado 8-19.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

<span class="caption">Listado 8-19: Intento de usar la sintaxis de indexación con una
cadena</span>

Este código dará como resultado el siguiente error:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

El error y la nota cuentan la historia: las cadenas de Rust no admiten la indexación. Pero
¿Por qué no? Para responder a esa pregunta, debemos analizar cómo Rust almacena cadenas en
memoria.

#### Representacion interna

Un `String` es un contenedor sobre un` Vec<u8>`. Echemos un vistazo a algunos de nuestras
cadenas de ejemplo codificadas en UTF-8 del Listado 8-14. Primero, este:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

En este caso, `len` será 4, lo que significa que el vector que almacena la cadena `Hola`
tiene 4 bytes de longitud. Cada una de estas letras ocupa 1 byte cuando se codifica en UTF-8. Pero
¿qué pasa con la siguiente línea? (tenga en cuenta que esta cadena comienza con la 
letra cirílica mayúscula Ze, no el número arábigo 3.)

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

Cuando se pregunte cuánto mide la cadena, podría responder 12. Sin embargo, la respuesta de Rust es 24:
esa es la cantidad de bytes que se necesitan para codificar `Здравствуйте` en UTF-8, porque
cada valor escalar Unicode en esa cadena ocupa 2 bytes de almacenamiento. Por lo tanto,
un índice en los bytes de la cadena no siempre se correlacionará con un
valor escalar Unicode válido. Para demostrarlo, considere este código de Rust no válido:

```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```

¿Cuál debería ser el valor de `answer`? ¿Debería ser `З`, la primera letra? Cuándo se
codifica en UTF-8, el primer byte de `З` es `208` y el segundo es `151`, por lo que
`answer` debería ser de hecho `208`, pero `208` no es un carácter válido en si
mismo. Devolver `208` probablemente no sea lo que el usuario querría si solicitara la
primera letra de esta cadena; sin embargo, esos son los únicos datos que tiene Rust en
byte index 0. Los usuarios generalmente no quieren que se devuelva el valor del byte, incluso si
la cadena contiene solo letras latinas: si `&"hola"[0]` fueran un código válido que
devolvió el valor del byte, devolvería `104`, no `h`. Para evitar devolver un
valor inesperado y causar errores que podrían no ser descubiertos inmediatamente,
Rust no compila este código en absoluto y evita malentendidos al principio
del proceso de desarrollo.

#### Bytes y valores escalares y grupos de grafemas

Otro punto sobre UTF-8 es que en realidad hay tres formas relevantes de
ver las cadenas desde la perspectiva de Rust: como bytes, valores escalares y
clusters de grafema (lo más parecido a lo que llamaríamos *letras*).

Si miramos la palabra hindi `नमस्ते` escrita en la escritura devanagari, está
almacenada como un vector de valores `u8` que se ve así:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

Eso son 18 bytes y es la forma en que las computadoras almacenan estos datos en última instancia. Si miramos
como valores escalares Unicode, que son el tipo "char" de Rust, esos
bytes se ven así:

`` texto
['न', 'म', 'स', '्', 'त', 'े']
''

Aquí hay seis valores `char`, pero el cuarto y sexto no son letras;
son signos diacríticos que no tienen sentido por sí mismos. Finalmente, si miramos
como grupos de grafemas, obtendríamos lo que una persona llamaría las cuatro letras
que componen la palabra hindi:

```text
["न", "म", "स्", "ते"]
```

Rust proporciona diferentes formas de interpretar los datos de cadena sin procesar que las computadoras
almacenan para que cada programa pueda elegir la interpretación que necesita, sin importar
en qué lenguaje humano están los datos.

Una última razón por la que Rust no nos permite indexar en una `String` para obtener un
carácter es que se espera que las operaciones de indexación siempre tomen un tiempo constante
(O (1)). Pero no es posible garantizar ese rendimiento con una `String`,
porque Rust tendría que revisar el contenido desde el principio hasta el
índice para determinar cuántos caracteres válidos había.

### Slice String

Indexar en una cadena es a menudo una mala idea porque no está claro cuál es el
tipo de retorno de la operación de indexación de cadenas: un valor de byte, un
caracter, un grupo de grafemas o un segmento de cadena. Por lo tanto, Rust le pide que
sea más específico si realmente necesita usar índices para crear Slice String. Para
ser más específico en su indexación e indicar que desea un Slice String,
en lugar de indexar usando `[]` con un solo número, puede usar `[]` con un
rango para crear un segmento de cadena que contenga bytes particulares:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Aquí, `s` será un `&str` que contiene los primeros 4 bytes de la cadena.
Anteriormente, mencionamos que cada uno de estos caracteres tenía 2 bytes, lo que significa
`s` será` Зд`.

¿Qué pasaría si usáramos `&hello[0..1]`? La respuesta: en tiempo de ejecucion, Rust entraría en pánico
de la misma manera que si se accediera a un índice no válido en un vector:

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

Debe usar rangos para crear Slice String con precaución, porque al hacerlo
puede bloquear su programa.

### Metodos para iterar sobre cadenas

Afortunadamente, puede acceder a los elementos de una cadena de otras formas.

Si necesita realizar operaciones en valores escalares Unicode individuales, la mejor
forma de hacerlo es usar el método `chars`. Llamar a `chars` en `नमस्ते`  separa la
salida y devuelve seis valores de tipo `char`, y puede iterar sobre el resultado
para acceder a cada elemento:


```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

Este código imprimirá lo siguiente:


```text
न
म
स
्
त
े
```

El método `bytes` devuelve cada byte sin procesar, que podría ser apropiado para su
dominio:

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

Este código imprimirá los 18 bytes que componen esta `String`:

```text
224
164
// --snip--
165
135
```

Pero asegúrese de recordar que los valores escalares Unicode válidos pueden estar compuestos por más
de 1 byte.

Obtener grupos de grafemas a partir de cadenas es complejo, por lo que esta funcionalidad no es
proporcionada por la biblioteca estándar. Las cajas están disponibles en
[crates.io](https://crates.io/) si esta es la funcionalidad que necesita.

### Las cadenas no son tan simples

Para resumir, las cadenas son complicadas. Diferentes lenguajes de programación tienen
diferentes opciones sobre cómo presentar esta complejidad al programador. Rust
ha elegido hacer que el manejo correcto de los datos de `String` sea el comportamiento predeterminado
para todos los programas de Rust, lo que significa que los programadores deben pensar más en
manejo de datos UTF-8 por adelantado. Esta compensación expone más la complejidad de
cadenas de lo que es evidente en otros lenguajes de programación, pero le impide
tener que manejar errores que involucran caracteres no ASCII más adelante en su
ciclo de vida del desarrollo.

Cambiemos a algo un poco menos complejo: mapas hash.
