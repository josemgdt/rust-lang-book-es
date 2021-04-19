## Tipos de Datos

Cada valor en Rust es de un determinado *tipo de datos*, lo que le dice a Rust de que tipo
se están especificando los datos para que sepa cómo trabajar con ellos. Veremos
dos subconjuntos de tipos de datos: escalares y compuestos.

Tenga en cuenta que Rust es un lenguaje *tipeado estáticamente*, lo que significa que
debe conocer los tipos de todas las variables en tiempo de compilación. El compilador normalmente
puede inferir qué tipo queremos usar según el valor y cómo lo usamos. En casos en los que
son posibles muchos tipos, como cuando convertimos una `String` a un tipo numérico
usando `parse` en la  sección ["Comparando la Conjetura con el Número Secreto”][comparing-the-guess-to-the-secret-number]<!-- ignore --> en el
Capítulo 2, debemos agregar una anotación de tipo, como esta:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Si no agregamos el tipo de anotación aquí, Rust mostrará el siguiente
error, lo que significa que el compilador necesita más información de nosotros para saber qué
tipo queremos usar:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Verá diferentes tipos de anotaciones para otros tipos de datos.

### Tipos Escalares

Un tipo *escalar* representa un valor único. Rust tiene cuatro tipos escalares principales:
enteros, números de punto flotante, booleanos y caracteres. Puede reconocerlos
de otros lenguajes de programación. Veamos cómo funcionan en Rust.

#### Tipos de Enteros

Un *entero* es un número sin componente fraccionario. Usamos un tipo entero
en el Capítulo 2, el tipo `u32`. Esta declaración de tipo indica que el
valor con el que está asociado debe ser un entero sin signo (los tipos entero con signo
comienzan con `i`, en lugar de `u`) que ocupa 32 bits de espacio. La tabla 3-1 muestra
los tipos enteros incorporados en Rust. Cada variante en las columnas 
*Con signo* y *Sin signo* (por ejemplo, `i16`) se pueden usar para declarar el tipo de valor entero.

<span class="caption">Tabla 3-1: Tipos de números enteros en Rust</span>

|Longitud |Con signo|Sin signo |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

Cada variante puede tener signo o no y tiene un tamaño explícito.
*Con signo* y *Sin signo* se refieren a si es posible que el número sea
negativo, en otras palabras, si el número debe tener un signo
con él (con signo) o si solo será positivo y, por lo tanto, puede ser
representado sin signo. Es como escribir números en papel: cuando
el signo importa, un número se muestra con un signo más o un signo menos; sin emabargo,
cuando es seguro asumir que el número es positivo, se muestra sin signo.
Los números con signos se almacenan mediante la representación de [complemento a dos](https://en.wikipedia.org/wiki/Two%27s_complement).

Cada variante con signo puede almacenar números desde -(2<sup>n - 1</sup>) a 2<sup>n -
1</sup> - 1 inclusive, donde *n* es el número de bits que usa la variante. Por tanto, un
`i8` puede almacenar números desde -(2<sup>7</sup>) a 2<sup>7</sup> - 1, que equivale desde
-128 a 127. Las variantes sin signo pueden almacenar números del 0 a 2<sup>n</sup> - 1,
por lo que un `u8` puede almacenar números del 0 al 2<sup>8</sup> - 1, que equivale desde 0 a 255.

Además, los tipos `isize` y `usize` dependen del tipo de computadora donde
se está ejecutando el programa: 64 bits si está en una arquitectura de 64 bits y 32 bits
si está en una arquitectura de 32 bits.

Puede escribir literales enteros en cualquiera de las formas que se muestran en la Tabla 3-2. Note
que todos los literales numéricos, excepto el literal byte, permiten un sufijo de tipo, como
`57u8` y `_` como separador visual, como `1_000`.

<span class="caption">Tabla 3-2: Literales enteros en Rust</span>

|Literal numéricos | Ejemplo       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` solo) | `b'A'`        |

Entonces, ¿cómo saber qué tipo de número entero usar? Si no está seguro, los 
valores predeterminados de Rust son generalmente buenas opciones, y los tipos enteros predeterminados son `i32`; este
tipo es generalmente el más rápido, incluso en sistemas de 64 bits. La situación principal en
la que usaría `isize` o `usize` es cuando indexe algún tipo de colección.

> ##### Desbordamiento de Enteros
>
> Supongamos que tiene una variable de tipo "u8" que puede contener valores entre 0 y 255.
> Si intenta cambiar la variable a un valor fuera de ese rango, tal
> como 256, se producirá *desbordamiento de enteros*. Rust tiene algunas reglas interesantes
> sobre este comportamiento. Cuando compila en modo de depuración, Rust incluye
> comprobacion de desbordamiento de enteros, que haria que su programa entrase en pánico en tiempo de ejecución si
> ocurre este comportamiento. Rust usa el término `entrar en pánico` cuando un programa sale con
> error; analizaremos los pánicos con más profundidad en la sección [“Errores Irrecuperables con
> `panic!`”][unrecoverable-errors-with-panic]<!-- ignore --> en el Capítulo 9.
>
> Cuando compila en modo de lanzamiento con el indicador `--release`, Rust 
> *no* incluyen comprobaciones de desbordamiento de enteros que provocan pánico. En cambio, si
> se produce un desbordamiento, Rust realiza *envoltura en complemento a dos*. En resumen, valores
> mayores que el valor máximo que el tipo puede contener "vuelven" al mínimo
> de los valores que puede contener el tipo. En el caso de un `u8`, 256 se convierte en 0, 257
> se convierte en 1, y así sucesivamente. El programa no entrará en pánico, pero la variable tendrá un
> valor que probablemente no sea el que se esperaba que tuviera. Depender del
> comportamiento de envoltura del desbordamiento de enteros se considera mala praxis.
>
> Para manejar explícitamente la posibilidad de desbordamiento, puede usar estas familias
> de métodos que proporciona la biblioteca estándar sobre primitivas de tipos numéricos:
>
> - Envuelva en todos los modos con los métodos `wrapping_*`, como `wrapping_add`
> - Devuelva el valor `None` si hay un desbordamiento con los métodos `check_*`
> - Devuelva el valor y un booleano que indica si hubo desbordamiento con
> los métodos `overflowing_*`
> - Sature en los valores mínimo o máximo del valor con métodos `saturating_ e*`

#### Tipos de Coma Flotante

Rust también tiene dos tipos primitivos para *números de punto flotante*, que son
números con puntos decimales. Los tipos de punto flotante de Rust son `f32` y `f64`,
que tienen un tamaño de 32 bits y 64 bits, respectivamente. El tipo predeterminado es `f64`
porque en las CPU modernas tiene aproximadamente la misma velocidad que con `f32` pero son capaces de
más precisión.

A continuación, se muestra un ejemplo que muestra números de punto flotante en acción:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Los números de coma flotante se representan de acuerdo con el estándar IEEE-754.
El tipo `f32` es un flotante de precisión simple y `f64` tiene doble precisión.

#### Operaciones Numéricas

Rust admite las operaciones matemáticas básicas que esperaría para todos los
tipos de números: suma, resta, multiplicación, división y resto.
El siguiente código muestra cómo se usaría cada uno en una declaración "let":

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Cada expresión de estas declaraciones utiliza un operador matemático y evalúa
a un solo valor, que luego se vincula a una variable. El [Apéndice B][appendix_b]<!-- ignore --> contiene una
lista de todos los operadores que proporciona Rust.

#### El Tipo Booleano

Como en la mayoría de los otros lenguajes de programación, un tipo booleano en Rust tiene dos posibles
valores: `verdadero` y `falso`. Los booleanos tienen un tamaño de un byte. El tipo booleano en
Rust se especifica mediante "bool". Por ejemplo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

La forma principal de utilizar valores booleanos es a través de condicionales, como una expresión "if". 
Cubriremos cómo funcionan las expresiones "if" en Rust en la sección ["Control de Flujo”][control-flow]<!-- ignore -->.

#### El Tipo Carácter

Hasta ahora hemos trabajado solo con números, pero Rust también admite letras.
El tipo `char` es el tipo alfabético más primitivo del lenguaje y el siguiente
código muestra una forma de usarlo. (Tenga en cuenta que los literales `char` se especifican con
comillas simples, a diferencia de los literales de cadena, que usan comillas dobles).

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

El tipo "char" de Rust tiene un tamaño de cuatro bytes y representa un valor escalar Unicode,
lo que significa que puede representar mucho más que ASCII. Letras acentuadas,
caracteres chinos, japoneses y coreanos, emojis y espacios de ancho cero son todos
valores `char` válidos en Rust. Los valores escalares Unicode van de `U+0000` a
`U+D7FF` y de `U+E000` a `U+10FFFF` inclusive. Sin embargo, un "carácter" no es
realmente un concepto en Unicode, por lo que su intuición humana de lo que es un "carácter"
puede no coincidir con lo que es un `char` en Rust. Discutiremos este tema en
detalle en ["Almacenando texto codificado UTF-8 con strings"][strings]<!-- ignore -->
en el Capítulo 8.

### Tipos Compuestos

*Los tipos compuestos* pueden agrupar varios valores en un solo tipo. Rust tiene dos
tipos compuestos primitivos: tuplas y matrices.

#### El Tipo Tupla

Una tupla es una forma general de agrupar varios valores de varios
tipos en un tipo compuesto. Las tuplas tienen una longitud fija; una vez declaradas,
no pueden crecer ni encogerse de tamaño.

Creamos una tupla escribiendo una lista de valores separados por comas dentro
paréntesis. Cada posición en la tupla tiene un tipo, y los tipos de
los diferentes valores de la tupla no tienen por qué ser iguales. Hemos agregado
anotaciones opcionales de tipo en este ejemplo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

La variable `tup` se refiere a la tupla completa, porque una tupla se considera un
elemento compuesto único. Para obtener los valores individuales de una tupla, podemos
usar la coincidencia de patrones para desestructurar un valor de tupla, como esto:

<span class="filename">Nombre de archivo: src / main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Este programa crea primero una tupla y la vincula a la variable `tup`. Entonces
usa un patrón con "let" para tomar "tup" y convertirlo en tres
variables, `x`,`y` y `z`. Esto se llama *desestructuración*, porque la tupla única
se rompe en tres partes. Finalmente, el programa imprime el valor de
`y`, que es `6.4`.

Además de desestructurar mediante la coincidencia de patrones, podemos acceder a un elemento
tupla directamente mediante el uso de un punto (`.`) seguido del índice del valor que
quiere acceder. Por ejemplo:

<span class="filename">Nombre de archivo: src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Este programa crea una tupla, `x`, y luego crea nuevas variables para cada
elemento mediante el uso de sus respectivos índices. Como ocurre con la mayoría de los lenguajes de programación,
el primer índice de una tupla es 0.

#### El Tipo Matriz

Otra forma de tener una colección de valores es con una *matriz*. A diferencia de
una tupla, todos los elementos de una matriz deben tener el mismo tipo. Las matrices en Rust son
diferente de las matrices en algunos otros lenguajes porque las matrices en Rust tienen una
longitud fija, como las tuplas.

En Rust, los valores que entran en una matriz se escriben como una lista separada por comas
dentro de corchetes:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Las matrices son útiles cuando desea que sus datos se asignen en la pila en lugar de en
el monton (discutiremos la pila y el monton en el Capítulo 4) o cuando
desea asegurarse de tener siempre un número fijo de elementos. Una matriz no es,
sin embargo, tan flexible como el tipo vector. Un vector es un tipo de colección similar
proporcionada por la biblioteca estándar que *puede* aumentar o reducir su tamaño.
Si no está seguro de si usar una matriz o un vector, probablemente debería usar un
vector. El capítulo 8 analiza los vectores con más detalle.

Un ejemplo de cuándo podría querer usar una matriz en lugar de un vector es en un
programa que necesita saber los nombres de los meses del año. Es muy
poco probable que un programa de este tipo necesite agregar o eliminar meses, por lo que puede usar
una matriz porque sabe que siempre contendrá 12 elementos:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

Escribirá el tipo matriz utilizando corchetes y dentro de
los corchetes incluya el tipo de cada elemento, un punto y coma, y ​​luego el número de
elementos en la matriz, así:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Aquí, `i32` es el tipo de cada elemento. Después del punto y coma, el número `5`
indica que la matriz contiene cinco elementos.

Escribir el tipo matriz de esta manera parece similar a una sintaxis alternativa para
inicializar una matriz; si desea crear una matriz que contenga el mismo
valor para cada elemento, puede especificar el valor inicial, seguido de un
punto y coma, y luego la longitud de la matriz entre corchetes, como se muestra aquí:

```rust
let a = [3; 5];
```

La matriz denominada `a` contendrá `5` elementos que se establecerán en el valor
`3` inicialmente. Esto es lo mismo que escribir `let a = [3, 3, 3, 3, 3];` pero de
forma más concisa.

##### Acceso a Elementos de Matriz

Una matriz es una sola porción de memoria asignada en la pila. Puede acceder a los
elementos de una matriz usando indexación, asi:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

En este ejemplo, la variable denominada `first` obtendrá el valor `1`, porque
ese es el valor en el índice `[0]` en la matriz. La variable denominada `second`
obtendrá el valor `2` del índice `[1]` en la matriz.

##### Acceso no Válido a Elemento de Matriz

¿Qué sucede si intenta acceder a un elemento de una matriz que está más allá del final
de la matriz? Digamos que cambia el ejemplo a lo siguiente, que usa código
similar al juego de adivinanzas del Capítulo 2, para obtener un índice de matriz de usuario:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Este código se compila correctamente. Si ejecuta este código usando `cargo run` e
ingresa 0, 1, 2, 3 o 4, el programa imprimirá el valor correspondiente en
ese índice en la matriz. Si, en cambio, ingresa un número después del final del
matriz, como 10, verá un resultado como este:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console

thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

El programa dio como resultado un error *en tiempo de ejecución* en el momento de usar un
valor en la operación de indexación. El programa salió con un mensaje de error y
no ejecutó la declaración final `println!`. Cuando intenta acceder a un
elemento mediante la indexación, Rust comprobará que el índice que ha especificado es menor
que la longitud de la matriz. Si el índice es mayor o igual que la longitud,
Rust entrará en pánico. Esta verificación debe realizarse en tiempo de ejecución, especialmente en este caso,
porque el compilador no puede saber qué valor ingresará un usuario cuando
ejecute el código más tarde.

Este es el primer ejemplo de los principios de seguridad de Rust en acción. En muchos
lenguajes de bajo nivel, este tipo de comprobación no se realiza y, cuando proporciona un
índice incorrecto, se puede acceder a memoria inválida. Rust te protege contra este
tipo de error al salir inmediatamente en lugar de permitir el acceso a la memoria y
continuar. El capítulo 9 trata más sobre el manejo de errores de Rust.

[comparing-the-guess-to-the-secret-number]:ch02-00-guessing-game-tutorial.html#comparando-la-conjetura-con-el-numero-secreto
[control-flow]: ch03-05-control-flow.html#control-de-flujo
[strings]: ch08-02-strings.html#almacenando-texto-codificado-utf-8-con-strings
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[wrapping]: ../std/num/struct.Wrapping.html
[appendix_b]: appendix-02-operators.md
