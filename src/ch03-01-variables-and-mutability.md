## Variables y Mutabilidad

Como se mencionó en el Capítulo 2, las variables predeterminadas son inmutables. Este es uno de
los muchos "apoyos" que Rust le da para escribir su código de manera que aproveche
la seguridad y facilidad de concurrencia que ofrece Rust. Sin embargo, todavía tiene la
opción de hacer que sus variables sean mutables. Exploremos cómo y por qué Rust
le anima a favorecer la inmutabilidad y por qué a veces no es deseable.

Cuando una variable es inmutable, una vez que un valor está vinculado a un nombre no se 
puede cambiar ese valor. Para ilustrar esto, generemos un nuevo proyecto llamado *variables*
en el directorio *projects* usando `cargo new variables`.

En su nuevo directorio *variables*, abra *src/main.rs* y reemplace su
código con el siguiente, que aún no compilará:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Guarde y ejecute el programa usando `cargo run`. Debería recibir un mensaje 
de error, como se muestra en esta salida:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

Este ejemplo muestra cómo el compilador le ayuda a encontrar errores en sus programas.
Aunque los errores del compilador pueden ser frustrantes, solo se refieren a que su programa
todavía no está haciendo de forma segura lo que desea que haga; eso *no* significa que no se es
un buen programador! Los rustáceos experimentados todavía sufren errores de compilación.

El mensaje de error indica que la causa es que no se puede
asignar dos veces una variable inmutable, porque intentó asignar un segundo
valor a la variable inmutable `x`.

Es importante que obtengamos errores en tiempo de compilación cuando intentamos cambiar un
valor que previamente designamos como inmutable porque esta misma situación
puede provocar errores. Si una parte de nuestro código opera bajo el supuesto de que un
el valor nunca cambiará y otra parte de nuestro código cambia ese valor, es muy
posible que la primera parte del código no haga lo que deberia hacer.
La causa de este tipo de error puede ser difícil de rastrear después del hecho,
especialmente cuando la segunda parte del código cambia el valor solo *a veces*.

En Rust, el compilador garantiza que cuando declara que un valor no cambiará,
realmente no cambiará. Eso significa que cuando lee y escribe código,
no es necesario realizar un seguimiento de cómo y dónde puede cambiar un valor. Su codigo,
por tanto, es más fácil de razonar.

Pero la mutabilidad puede resultar muy útil. Las variables son inmutables solo por defecto; como
se hizo en el Capítulo 2, puede hacerlas mutables agregando `mut` delante del
nombre de la variable. Además de permitir que este valor cambie, `mut` transmite
la intención a los futuros lectores del código indicando que otras partes del código
cambiarán el valor de esta variable.

Por ejemplo, cambiemos *src/main.rs* a lo siguiente:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Cuando ejecutamos el programa, obtenemos esto:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

Se nos permite cambiar el valor al que se une `x`, de `5` a `6`, cuando se utiliza `mut`.
En algunos casos, querrá convertir una variable en mutable porque hace
el código más conveniente de escribir que si solo tuviera variables inmutables.

Hay múltiples asuntos a considerar además de la prevención de
bugs. Por ejemplo, en los casos en los que utiliza grandes estructuras de datos, la mutación
de una instancia en el lugar puede ser más rápido que copiar y devolver instancias 
recién asignadas. Con estructuras de datos más pequeñas, crear nuevas instancias y escribir en
un estilo de programación más funcional puede ser más fácil de razonar, por lo que podría valer la pena un
menor rendimiento para obtener mas claridad.

### Diferencias entre Variables y Constantes

Ser incapaz de cambiar el valor de una variable podría haberle recordado
otro concepto de programación que tienen la mayoría de los otros lenguajes: las *constantes*. Como
variables inmutables, las constantes son valores que están vinculados a un nombre y no tienen
permitido cambiar, pero hay algunas diferencias entre constantes y variables.

Primero, no se permite usar `mut` con constantes. Las constantes no son solo
inmutables de forma predeterminada; *siempre* son inmutables.

Las constantes se declaran usando la palabra clave `const` en lugar de la palabra clave `let`,
y el tipo de valor *debe* anotarse. Estamos a punto de cubrir los tipos y
sus anotaciones en la siguiente sección, ["Tipos de datos"][data-types]<!-- ignore
--> , así que no se preocupe por los detalles ahora mismo. Solo debe saber que siempre debe
anotar el tipo.

Las constantes se pueden declarar en cualquier ámbito, incluido el ámbito global, lo que las hace
útiles para valores que se necesitan conocer en muchas partes del código.

La última diferencia es que las constantes se pueden establecer solo en una expresión constante,
no como el resultado de una llamada a función o cualquier otro valor que solo podría ser
calculado en tiempo de ejecución.

A continuación, se muestra un ejemplo de una declaración de constante donde el nombre de la constante es
"MAX_POINTS" y su valor se establece en 100.000. (La convención de nomenclatura de Rust para
constantes es usar todo en mayúsculas, con guiones bajos entre las palabras y
se pueden insertar guiones bajos en literales numéricos para mejorar la legibilidad):

```rust
const MAX_POINTS: u32 = 100_000;
```

Las constantes son válidas durante todo el tiempo que se ejecuta un programa, dentro del alcance en que
fueron declaradas, lo que las convierte en una opción útil para los valores en su dominio de aplicación
que varias partes del programa pueden necesitar conocer, como el
número máximo de puntos que cualquier jugador de un juego puede ganar o la velocidad
de luz.

Nombrar los valores codificados como constantes que se utilizan en todo el programa es útil para
transmitir el significado de ese valor a los futuros mantenedores del código. También
ayuda a tener solo un lugar en su código que necesitaría cambiar si el
valor codificado debe actualizarse en el futuro.

### Sombreado

Como vimos en el tutorial del juego de adivinanzas en la sección ["Comparando la Conjetura con el
Número secreto ”][comparing-the-guess-to-the-secret-number]<!-- ignore -->
en el Capítulo 2, puede declarar una nueva variable con el mismo nombre que una
variable anterior. Los rustáceos dicen que la primera variable está *sombreada* por la
segunda, lo que significa que el valor de la segunda variable es lo que aparece cuando 
se utiliza la variable. Podemos sombrear una variable usando el mismo nombre de variable
y repitiendo el uso de la palabra clave `let` de la siguiente manera:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Este programa primero une `x` a un valor de `5`. Luego sombrea `x` 
repitiendo `let x =`, tomando el valor original y agregando `1` para que el valor de
`x` sea entonces `6`. La tercera sentencia `let` también sombrea `x`, multiplicando el
valor anterior por `2` para dar a `x` un valor final de `12`. Cuando ejecutamos este
programa, generará lo siguiente:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

El sombreado es diferente de marcar una variable como "mut", porque obtendremos un
error en tiempo de compilación si accidentalmente intentamos reasignar esta variable sin
usar la palabra clave `let`. Al usar `let`, podemos realizar algunas transformaciones
en un valor, pero la variable será inmutable después de que se completen esas transformaciones.

La otra diferencia entre "mut" y el sombreado es que estamos
creando efectivamente una nueva variable cuando usamos la palabra clave `let` nuevamente, y podemos
cambiar el tipo de valor pero reutilizar el mismo nombre. Por ejemplo, digamos que nuestro
programa le pide a un usuario que muestre cuántos espacios quiere entre un texto
ingresando caracteres de espacio, pero realmente queremos almacenar esa entrada como un número:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

Esta construcción está permitida porque la primera variable `spaces` es un tipo cadena
y la segunda variable `spaces`, que es una variable nueva,
tienen el mismo nombre que la primera, pero es un tipo número. La sombra, por lo tanto, nos ahorra
tener que inventar nombres diferentes, como `spaces_str` y
`spaces_num`; en su lugar, podemos reutilizar el nombre de `spaces` más simple. Sin embargo, si
intenta usar `mut` para esto, como se muestra aquí, obtendremos un error en tiempo de compilación:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

El error dice que no podemos mutar el tipo de una variable:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Ahora que hemos explorado cómo funcionan las variables, veamos más tipos de datos que
podemos tener.

[comparing-the-guess-to-the-secret-number]:ch02-00-guessing-game-tutorial.html#comparando-la-conjetura-con-el-número-secreto
[data-types]: ch03-02-data-types.html#tipos-de-datos
