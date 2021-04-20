## El Operador de Control de Flujo `match`

Rust tiene un operador de control de flujo extremadamente poderoso llamado `match` que permite
comparar un valor con una serie de patrones y luego ejecutar código basado
en el patrón coincidente. Los patrones pueden estar formados por valores literales, variables
nombres, comodines y muchas otras cosas; El capítulo 18 cubre todos los diferentes
tipos de patrones y lo que hacen. El poder de `match` proviene de la
expresividad de los patrones y el hecho de que el compilador confirma que
se manejan todos los posibles casos.

Piense en una expresión `match` como si fuera una máquina clasificadora de monedas; las monedas se deslizan
por una pista con agujeros de varios tamaños a lo largo de ella, y cada moneda cae en
el primer agujero en el que encaja. Del mismo modo, los valores van
a través de cada patrón en `match`, y en el primer patrón el valor "encaja",
el valor cae en el bloque de código asociado que se utilizará durante la ejecución.

Debido a que acabamos de mencionar las monedas, usémoslas como ejemplo usando `match`.
Podemos escribir una función que puede tomar una moneda desconocida y,
de manera similar a la máquina de conteo, determinar qué moneda es y devolver su
valor en centimos, como se muestra en el Listado 6-3.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

<span class="caption">Listado 6-3: Una enumeración y una expresión `match` que tiene
las variantes de la enumeración como sus patrones </span>

Analicemos `match` en la función `value_in_cents`. Primero,
la palabra clave `match` seguida de una expresión, que en este caso es el valor
`coin`. Esto parece muy similar a una expresión usada con `if`, pero hay una
gran diferencia: con `if`, la expresión debe devolver un valor booleano, pero
aquí, puede ser de cualquier tipo. El tipo de "moneda" en este ejemplo es la enumeración `Coin`
que definimos en la línea 1.

A continuación están los brazos de `match`. Un brazo tiene dos partes: un patrón y un código.
El primer brazo aquí tiene un patrón que es el valor `Coin::Penny` y luego el operador `=>`
que separa el patrón y el código a ejecutar. El código en este caso
es solo el valor `1`. Cada brazo está separado del siguiente con una coma.

Cuando se ejecuta la expresión `match`, compara el valor resultante con
el patrón de cada brazo, en orden. Si un patrón coincide con el valor, el código
asociado con ese patrón se ejecuta. Si ese patrón no coincide con el
valor, la ejecución continúa hasta el siguiente brazo, como en una máquina clasificadora de monedas.
Podemos tener tantos brazos como necesitemos: en el Listado 6-3, nuestro `match` tiene cuatro brazos.

El código asociado con cada brazo es una expresión y el valor resultante de
la expresión en el brazo coincidente es el valor que se devuelve para el
expresión completa de `match`.

Los corchetes normalmente no se utilizan si el código del brazo es corto, como
en el Listado 6-3 donde cada brazo solo devuelve un valor. Si desea ejecutar varias
líneas de código en un brazo, puede utilizar llaves. Por ejemplo,
el siguiente código imprimiría "Lucky penny!" cada vez que se llamó al método con
un `Coin::Penny` pero aún así devolvería el último valor del bloque, `1`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### Patrones que se unen a valores

Otra característica útil de los brazos es que se pueden unir a las partes de
valores que coinciden con el patrón. Así es como podemos extraer valores de variantes enum.

Como ejemplo, cambiemos una de nuestras variantes de enum para contener datos en su interior.
Desde 1999 hasta 2008, Estados Unidos acuñó monedas de veinticinco centavos con diferentes
diseños para cada uno de los 50 estados. Ninguna otra moneda tiene este
diseño, por lo que solo los cuartos tienen este valor adicional. Podemos agregar esta información a
nuestro `enum` cambiando la variante` Quarter` para incluir un valor `UsState` almacenado
dentro de él, lo que hemos hecho aquí en el Listado 6-4.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<span class="caption">Listado 6-4: Una enumeración `Coin` en la que la variante` Quarter`
también tiene un valor `UsState`</span>

Imaginemos que un amigo está tratando de coleccionar los 50 cuartos estatales.
Mientras clasificamos nuestro cambio suelto por tipo de moneda, también llamaremos el nombre de
el estado asociado con cada cuarto, por lo que si es uno que nuestro amigo no tiene,
pueden agregarlo a su colección.

En la expresión match para este código, agregamos una variable llamada `state` al
patrón que coincide con los valores de la variante `Coin::Quarter`. Cuando una
`Coin::Quarter` coincide, la variable `state` se vinculará al valor de ese
estado del cuarto. Entonces podemos usar `state` en el código para ese brazo, así:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

Si tuviéramos que llamar a `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin`
sería `Coin:: Quarter(UsState::Alaska)`. Cuando comparamos ese valor con cada uno
de los brazos, ninguno de ellos coincide hasta que lleguemos a `Coin::Quarter(state)`.
En ese punto, el enlace para `state` será el valor` UsState::Alaska`. Podemos
luego usar ese enlace en la expresión `println!`, obteniendo así el
valor de estado de la variante enum `Coin` para` Quarter`.

### Matching con `Option<T>`

En la sección anterior, queríamos obtener el valor interno de `T` del
caso `Some` cuando se usa `Option<T>`; también podemos manejar `Option<T>` usando `match` como
se hizo con la enum `Coin`! En lugar de comparar monedas, compararemos las
variantes de `Option<T>`, pero la forma en que funciona la expresión `match` permanece
igual.

Digamos que queremos escribir una función que tome una `Option<i32>` y, si
hay un valor dentro, agrega 1 a ese valor. Si no hay un valor dentro,
la función debe devolver el valor `None` y no intentar realizar ninguna
operación.

Esta función es muy fácil de escribir, gracias a `match`, y se verá como en el
Listado 6-5.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<span class="caption">Listado 6-5: Una función que usa una expresión `match` en
una `Option<i32>`</span>

Examinemos la primera ejecución de `plus_one` con más detalle. Cuando llamamos
`plus_one(five)`, la variable `x` en el cuerpo de `plus_one` tendrá el
valor `Some(5)`. Luego comparamos eso con cada brazo de match.

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

El valor `Some(5)` no coincide con el patrón `None`, por lo que continuamos con el
siguiente brazo.

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

¿`Some(5)` coincide con `Some(i)`? ¡sí! Tenemos la misma variante. La
`i` se une al valor contenido en `Some`, por lo que `i` toma el valor `5`.
Luego se ejecuta el código en el brazo de coincidencia, por lo que agregamos 1 al valor de `i` y
se crea un nuevo valor `Some` con nuestro total de `6` adentro.

Ahora consideremos la segunda llamada de `plus_one` en el Listado 6-5, donde` x` es
`None`. Entramos en `match` y lo comparamos con el primer brazo.

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

¡Concuerda! No hay ningún valor para agregar, por lo que el programa se detiene y devuelve el
valor `None` en el lado derecho de `=> `. Como el primer brazo coincidió, no se comparan ningún otro
brazo.

La combinación de `match` y enum es útil en muchas situaciones. Verá este
patrón mucho en el código de Rust: `match` contra una enum, vincula una variable a los
datos interiores, y luego ejecuta código basado en él. Es un poco complicado al principio, pero
una vez que se acostumbre, deseará tenerlo en todos los lenguajes. Es un favorito de los usuarios.

### Las coincidencias son exhaustivas

Hay otro aspecto de `match` que debemos discutir. Considere esta versión
de nuestra función `plus_one` que tiene un error y no se compila:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

No manejamos el caso `None`, por lo que este código provocará un error. Afortunadamente, es
un error que Rust sabe atrapar. Si intentamos compilar este código, obtendremos este
error:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust sabe que no cubrimos todos los casos posibles e incluso sabe que
patrón olvidamos! Los matches en Rust son *exhaustivos*: debemos agotar hasta la última
posibilidad para que el código sea válido. Especialmente en el caso de
`Option<T>`, cuando Rust nos impide olvidarnos de manejar explícitamente el
caso `None`, nos protege de asumir que tenemos un valor cuando podríamos
tienen un nulo, lo que hace imposible el error de mil millones de dólares discutido anteriormente.

### El marcador de posición `_`

Rust también tiene un patrón que podemos usar cuando no queremos enumerar todos los posibles
valores. Por ejemplo, un `u8` puede tener valores válidos de 0 a 255. Si solo
nos preocupase los valores 1, 3, 5 y 7, no queremos tener que enumerar 0, 2,
4, 6, 8, 9 hasta 255. Afortunadamente, no tenemos que hacerlo: podemos usar el
patrón especial `_` en su lugar:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-11-underscore-placeholder/src/main.rs:here}}
```

El patrón `_` coincidirá con cualquier valor. Al ponerlo después de nuestros otros brazos, el
`_` coincidirá con todos los casos posibles que no se hayan especificado antes. El `()`
es solo el valor unitario, por lo que no sucederá nada en el caso `_`. Como resultado,
podemos decir que no queremos hacer nada para todos los valores posibles que no
se listan antes del marcador de posición `_`.

Sin embargo, la expresión `match` puede ser un poco prolija en una situación en la que
nos preocupamos de sólo *uno* de los casos. Para esta situación, Rust proporciona "if let".

Puede encontrar más información sobre patrones y coincidencias en el [capítulo 18][ch18-00-patterns].

[ch18-00-patrones]:
ch18-00-patterns.html

