## Validando Referencias con Lifetimes

Un detalle que no discutimos en la sección ["Referencias y
Préstamo”] [referencias-y-préstamo]<!-- ignore -->  en el Capítulo 4 es
que cada referencia en Rust tiene una *vida útil*, que es el alcance para el cual
esa referencia es válida. La mayoría de las veces, las vidas son implícitas e
inferidas, al igual que la mayoría de las veces se infieren los tipos. Debemos anotar
los tipos cuando son posibles varios. De manera similar, debemos anotar las
vidas de las referencias cuando podrían estar relacionadas en diferentes
formas. Rust requiere que anotemos las relaciones utilizando parámetros de ciclo de vida genérico
para garantizar que las referencias reales utilizadas en tiempo de ejecución seán
válidas.

El concepto de vida útil es algo diferente al de las herramientas de otros lenguajes de programación,
haciendo posiblemente de la vida la característica más distintiva de Rust. Aunque
no cubriremos la vida útil en su totalidad en este capítulo, discutiremos las
formas comunes en las que puede encontrar sintaxis de vidas para que pueda familiarizarse con
los conceptos.

### Prevención de Referencias Colgantes con Vidas Útiles

El objetivo principal de la vida es evitar referencias colgantes, que provocan que
un programa haga referencia a datos distintos de los datos a los que se pretende hacer referencia.
Considere el programa del Listado 10-17, que tiene un alcance externo y uno interno.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs:here}}
```

<span class="caption">Listado 10-17: un intento de utilizar una referencia cuyo valor
ha salido del alcance</span>

> Nota: Los ejemplos de los listados 10-17, 10-18 y 10-24 declaran variables
> sin darles un valor inicial, por lo que el nombre de la variable existe en el
> alcance externo. A primera vista, esto podría parecer estar en conflicto con que Rust
> no tiene valores nulos. Sin embargo, si intentamos usar una variable antes de darle
> un valor, obtendremos un error en tiempo de compilación, que muestra que Rust
> no permite valores nulos.

El alcance externo declara una variable llamada `r` sin valor inicial, y el
interno declara una variable llamada `x` con el valor inicial de 5. Dentro
el alcance interno, intentamos establecer el valor de `r` como una referencia a `x`. Luego
el alcance interno termina e intentamos imprimir el valor en `r`. Este código no
compila porque el valor al que se refiere `r` ha salido del alcance antes de
intentar usarlo. Aquí está el mensaje de error:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/output.txt}}
```

La variable `x` no "vive lo suficiente". La razón es que `x` estará fuera
del alcance cuando el alcance interno termina en la línea 7. Pero `r` sigue siendo válido para el
alcance externo; debido a que su alcance es mayor, decimos que "vive más tiempo". Si
Rust permitiera que este código funcionara, `r` estaría haciendo referencia a la memoria que se
desasigno cuando `x` salió del alcance, y cualquier cosa que intentamos hacer con `r`
no funcionaría correctamente. Entonces, ¿cómo determina Rust que este código no es válido?
Utiliza un verificador de préstamos.

### El Verificador de Préstamos

El compilador de Rust tiene un *verificador de préstamos* que compara los alcances para determinar
si todos los préstamos son válidos. El Listado 10-18 muestra el mismo código que el Listado
10-17 pero con anotaciones que muestran la vida útil de las variables.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs:here}}
```

<span class="caption">Listado 10-18: anotaciones de la vida útil de `r` y
`x`, llamadas `'a` y `'b`, respectivamente</span>

Aquí, hemos anotado la vida útil de `r` con `'a` y la de `x`
con `'b`. Como puede ver, el bloque interior `'b` es mucho más pequeño que el exterior
`'a`. En tiempo de compilación, Rust compara el tamaño de las dos
vidas y ve que `r` tiene una vida útil de `'a` pero que se refiere a la memoria
con una vida útil `'b`. El programa se rechaza porque `'b` es más corto que
`'a`; el sujeto de la referencia no vive tanto como la referencia.

El Listado 10-19 corrige el código para que no tenga una referencia colgante y
compila sin errores.

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs:here}}
```

<span class="caption">Listado 10-19: una referencia válida porque los datos tienen una
vida útil más larga que la referencia</span>

Aquí, `x` tiene el tiempo de vida `'b`, que en este caso es mayor que `'a`. Esto
significa que `r` puede hacer referencia a `x` porque Rust sabe que la referencia en `r`
siempre será válida mientras que `x` sea válido.

Ahora que sabe dónde está la vida útil de las referencias y cómo analiza Rust
las vidas para garantizar que las referencias siempre sean válidas, exploremos
la vida útil de los parámetros y los valores de retorno en el contexto de las funciones.

### Vida Util Genérica en Funciones

Escribamos una función que devuelva el más largo de dos slices de cadena. Esta
función tomará dos slicees de cadena y devolverá uno. Después de 
implementar la función `longest`, el código del Listado 10-20 debería imprimir
`la cadena más larga es abcd`.

<span class="filename">​​Nombre de archivo: src / main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs}}
```

<span class="caption">Listado 10-20: Una función `main` que llama a la
función `longest` para encontrar el más largo de dos slices de cadena</span>

Tenga en cuenta que queremos que la función tome strings de cadena, que son referencias,
porque no queremos que la función `longest` se apropie de su
parámetros. Consulte la sección ["String Slices como Parámetros ”][string-slices-as-parameters]<!-- ignore --> en 
el Capítulo 4 para obtener más información sobre por qué los parámetros que usamos en el Listado 10-20 son los
los que necesitamos.

Si intentamos implementar la función `longest` como se muestra en el Listado 10-21,
no se compilará.

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

<span class="caption">Listado 10-21: una implementación de la función `longest`
que devuelve el más largo de dos slices de cadena, pero aún no compila</span>

En cambio, obtenemos el siguiente error que habla de vidas útiles:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/output.txt}}
```

El texto de ayuda revela que el tipo de retorno necesita un parámetro genérico de vida
en él porque Rust no puede saber si la referencia que se devuelve se refiere a
`x` o `y`. En realidad, tampoco lo sabemos, porque el bloque `if` en el cuerpo
de esta función devuelve una referencia a `x` y el bloque `else` devuelve una
referencia a `y`!

Cuando definimos esta función, no conocemos los valores concretos que
pasar a esta función, por lo que no sabemos si se ejecutará el caso `if` o el
caso `else`. Tampoco conocemos la vida útil concreta de las
referencias que se pasarán, por lo que no podemos mirar los alcances como lo hicimos en
los listados 10-18 y 10-19 para determinar si la referencia que devolvemos será
siempre será válida. El verificador de préstamos tampoco puede determinar esto, porque
no sabe cómo se relacionan las duraciones de `x` e `y` con la vida útil del
valor de retorno. Para corregir este error, agregaremos parámetros genéricos de vida que
definen la relación entre las referencias para que el verificador de préstamos pueda
realizar su análisis.

### Sintaxis de Anotación de Vida

Las anotaciones de vida no cambian la duración de las referencias.
Ya que las funciones pueden aceptar cualquier tipo cuando la declaracion especifica un
parámetro de tipo genérico, las funciones pueden aceptar referencias con cualquier duración especificando un
parámetro genérico de vida. Las anotaciones de vida describen las relaciones de
la duración de múltiples referencias entre sí sin afectar las vidas.

Las anotaciones de vida tienen una sintaxis ligeramente inusual: los nombres de los parámetros de vida
deben comenzar con un apóstrofo (`'`) y generalmente son todos en minúsculas y
muy cortos, como tipos genéricos. La mayoría de la gente usa el nombre `'a`. Colocamos
anotaciones de parámetros de vida después del `&` de una referencia, usando un espacio para
separar la anotación del tipo de referencia.

A continuación se muestran algunos ejemplos: una referencia a un `i32` sin un parámetro de duración, una
referencia a un `i32` que tiene un parámetro de vida llamado `'a`, y una
referencia mutable a un `i32` que también tiene el tiempo de vida `'a`.

```rust,ignore
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

Una anotación de vida por sí sola no tiene mucho significado, porque
las anotaciones están destinadas a decirle a Rust cómo los parámetros de vida genéricos de múltiples
referencias se relacionan entre sí. Por ejemplo, digamos que tenemos una función con
el parámetro `first` que es una referencia a un `i32` con una vida útil `'a`.
La función también tiene otro parámetro llamado `second` que es otra referencia a
un `i32` que también tiene el tiempo de vida `'a`. Las anotaciones de vida indican
que las referencias `first` y `second` deben vivir tanto tiempo como esa vida genérica.

### Anotaciones de Vida en Declaraciones de Funciones

Examinemos ahora las anotaciones de vida en el contexto de la función `longest`.
Al igual que con los parámetros de tipo genérico, debemos declarar los parámetros de vida útil genérica
dentro de corchetes angulares entre el nombre de la función y la lista de parámetros.
La restricción que queremos expresar en esta declaracion es que todas 
las referencias en los parámetros y el valor de retorno deben tener la misma duración.
Daremos el nombre de la duración `'a` y luego la agregaremos a cada referencia, como se muestra en
Listado 10-22.

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

<span class="caption">Listado 10-22: La definición de función `longest`
especificando que todas las referencias en la declaración deben tener la misma vida
`'a`</span>

Este código debe compilarse y producir el resultado que queremos cuando lo usemos con la
función `main` en el Listado 10-20.

La declaracion de la función ahora le dice a Rust que durante un tiempo de vida `'a`, la función
toma dos parámetros, los cuales son slices de cadena que viven al menos tanto
que la vida `'a`. La declaración de la función también le dice a Rust que
el slice devuelto por la función vivirá al menos mientras dure la vida `'a`.
En la práctica, significa que la vida útil de la referencia devuelta por
la función `longest` es la misma que la menor de las duraciones de las
referencias pasadas. Estas restricciones son las que queremos que Rust aplique.
Recuerde, cuando especificamos los parámetros de duración en esta declaración de función,
no cambiaremos la duración de los valores pasados ​​o devueltos. En su lugar,
especificamos que el verificador de préstamos debe rechazar cualquier valor que no
se adhiera a estas limitaciones. Tenga en cuenta que la función `longest` no necesita
saber exactamente cuánto tiempo vivirán `x` e `y`, solo que pueden
ser sustituidos por `'a` que satisfará esta declaracion.

Al anotar vidas útiles en funciones, las anotaciones van en la declaración de la función,
no en el cuerpo de la función. Rust puede analizar el código dentro de la
funcion sin ninguna ayuda. Sin embargo, cuando una función tiene referencias hacia o desde
código fuera de esa función, se vuelve casi imposible para Rust averiguar
la vida útil de los parámetros o los valores de retorno por sí mismos. Las vidas
pueden ser diferentes cada vez que se llama a la función. Es por eso que necesitamos
anotar las vidas útiles manualmente.

Cuando pasamos referencias concretas a `longest`, la vida útil concreta que es
sustituido por `'a` es la parte del alcance de `x` que se superpone con el
alcance de `y`. En otras palabras, la vida útil genérica `'a` obtendrá la
duración concreta que es igual a la menor de las duraciones de `x` e `y`. Puesto que
hemos anotado la referencia devuelta con el mismo parámetro de duración `'a`,
la referencia devuelta también será válida para la longitud del menor de las
vidas de `x` e `y`.

Veamos cómo las anotaciones de vida restringen la función `longest`
pasando referencias que tienen distintas duraciones concretas. El listado 10-23 es
un ejemplo sencillo.

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

<span class="caption">Listado 10-23: Uso de la función `longest` con
referencias a valores de `String` que tienen diferentes duraciones concretas</span>

En este ejemplo, `string1` es válido hasta el final del alcance externo,` string2`
es válido hasta el final del ámbito interno, y `result` hace referencia a algo
que es válido hasta el final del ámbito interno. Ejecute este código y verá
que el verificador de préstamos lo aprueba; compilará e imprimirá
`The longest string is long string is long`.

A continuación, intentemos un ejemplo que muestre que la vida útil de la referencia en
`result` debe ser el menor tiempo de vida de los dos argumentos. Moveremos la
declaración de la variable `result` fuera del ámbito interno pero dejaremos la
asignación del valor a la variable `result` dentro del alcance con
`string2`. Luego, moveremos el `println!` que usa `result` fuera del alcance interior,
una vez finalizado el alcance interno. El código del Listado 10-24 no
compila.

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs:here}}
```

<span class="caption">Listado 10-24: Intentando usar `result` después de que `string2`
ha salido del alcance</span>

Cuando intentemos compilar este código, obtendremos este error:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/output.txt}}
```

El error muestra que para que `result` sea válido para la declaración `println!`,
`string2` debería ser válido hasta el final del alcance externo. Rust sabe
esto porque anotamos la vida útil de los parámetros de la función y devolvemos
valores usando el mismo parámetro de vida útil `'a`.

Podemos mirar este código y ver que `string1` es más largo que
`string2` y, por lo tanto, `result` contendrá una referencia a `string1`.
Debido a que `string1` aún no ha salido del alcance, una referencia a `string1`
seguirá siendo válido para la declaración `println!`. Sin embargo, el compilador no puede ver
que la referencia es válida en este caso. Le hemos dicho a Rust que la vida útil de
la referencia devuelta por la función `longest` es la misma que la más pequeña de
la duración de las referencias transmitidas. Por lo tanto, el verificador de préstamos
no permitirá la posibilidad de que el código del Listado 10-24 tenga una referencia no válida.

Intente diseñar más experimentos que varíen los valores y la vida útil de las
referencias pasadas a la función `longest` y cómo se utiliza la referencia devuelta.
Haga hipótesis sobre si sus experimentos pasarán o no el
verificador de prestamos antes de compilar; a continuación, compruebe si está en lo cierto.

### Pensar en Términos de Vidas

La forma en que necesita especificar los parámetros de vida útil depende de lo que
la función está haciendo. Por ejemplo, si cambiamos la implementación de la
función `longest` para devolver siempre el primer parámetro en lugar del slice más largo,
no necesitaríamos especificar una duración en el parámetro `y`.
El siguiente código se compilará :

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

En este ejemplo, hemos especificado un parámetro de duración `'a` para el parámetro
`x` y el tipo de retorno, pero no para el parámetro `y`, porque la vida útil de
`y` no tiene ninguna relación con el tiempo de vida de `x` o el valor de retorno.

Al devolver una referencia de una función, el parámetro de duración del
tipo retornado debe coincidir con el parámetro de duración de uno de los parámetros. Si
la referencia devuelta *no* se refiere a uno de los parámetros, debe referirse
a un valor creado dentro de esta función, que sería una referencia colgante
porque el valor saldrá del alcance al final de la función. Considere
este intento de implementación de la función `longest` que no se compilará:

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

Aquí, aunque hemos especificado un parámetro de vida `'a` para el tipo devuelto,
esta implementación no se compilará porque el tiempo de vida del valor de retorno
no está relacionado con el tiempo de vida de los parámetros en absoluto. Aquí está el
mensaje de error que obtenemos:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

El problema es que `result` queda fuera de alcance y se limpia al final
de la función `longest`. También estamos intentando devolver una referencia a `result`
desde la función. No hay forma de que podamos especificar parámetros de vida que
cambiaría la referencia colgante y Rust no nos permitirá crear una
referencia. En este caso, la mejor solución sería devolver un tipo de datos propio
en lugar de una referencia, por lo que la función de llamada es responsable de
limpiar el valor.

En última instancia, la sintaxis de vida trata de conectar la vida de varios
parámetros y valores de retorno de funciones. Una vez que estén conectados, Rust tiene
suficiente información para permitir operaciones seguras en la memoria y no permitir operaciones que
crearían punteros colgantes o violarían la seguridad de la memoria.

### Anotaciones de Vida en Definiciones de Estructuras

Hasta ahora, solo hemos definido estructuras para contener sus tipos propios. Es posible para
las estructuras contener referencias, pero en ese caso tendríamos que agregar una anotación de vida útil
en cada referencia en la definición de la estructura. El listado 10-25 tiene una
estructura llamada `ImportantExcerpt` que contiene un slice de cadena.

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs}}
```

<span class="caption">Listado 10-25: una estructura que contiene una referencia, por lo que su
definición necesita una anotación de vida</span>

Esta estructura tiene un campo, `part`, que contiene un slice de cadena, que es una
referencia. Al igual que con los tipos de datos genéricos, declaramos el nombre del parámetro genérico
de vida dentro de corchetes angulares después del nombre de la estructura para que podamos
utilizar el parámetro de duración en el cuerpo de la definición de estructura. Esta
anotación significa que una instancia de `ImportantExcerpt` no puede sobrevivir a la referencia
que tiene en su campo `part`.

La función `main` crea aquí una instancia de la estructura` ImportantExcerpt`
que contiene una referencia a la primera oración de la `String` propiedad de la
variable `novel`. Los datos en `novel` existen antes de crear la instancia `ImportantExcerpt`.
Además, `novel` no sale del alcance hasta después de que
`ImportantExcerpt` queda fuera de alcance, por lo que la referencia en la
instancia de `ImportantExcerpt` es válida.

### Eleccion de Tiempos de Vida

Ha aprendido que cada referencia tiene una vida útil y que debe especificar
parámetros de duración para funciones o estructuras que utilizan referencias. Sin embargo, en el
Capítulo 4 teníamos una función en el Listado 4-9, que se muestra nuevamente en el Listado
10-26, compilado sin anotaciones de vida.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-26/src/main.rs:here}}
```

<span class="caption">Listado 10-26: una función que definimos en el Listado 4-9 que
compila sin anotaciones de vida, aunque los tipos del parámetro y el retorno
son referencias</span>

La razón por la que esta función se compila sin anotaciones de vida es histórica:
en las primeras versiones (anteriores a 1.0) de Rust, este código no se habría compilado porque
cada referencia necesitaba una vida útil explícita. En ese momento, la definición de la función
se habría escrito así:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Después de escribir mucho código de Rust, el equipo descubrió que los programadores de Rust
estaban ingresando las mismas anotaciones de vida una y otra vez en situaciones particulares.
Estas situaciones eran predecibles y siguian algunos patrones deterministas.
Los desarrolladores programaron estos patrones en el código del compilador para que
el verificador de préstamos pudiese inferir la vida útil en estas situaciones y no se
necesitasen anotaciones explícitas.

Este fragmento de la historia de Rust es relevante porque es posible que surjan más
patrones deterministas y se agregen al compilador. En el futuro,
es posible que se requieran incluso menos anotaciones de vida.

Los patrones programados en el análisis de referencias de Rust se denominan
*reglas de eleccion de tiempos de vida*. Estas no son reglas que los programadores deben seguir; son
un conjunto de casos particulares que el compilador considerará, y si su código
se ajusta a estos casos, no es necesario que escriba las vidas de forma explícita.

Las reglas de elisión no proporcionan una inferencia completa. Si Rust aplica de forma determinista
las reglas, pero todavía hay ambigüedad en cuanto a las vidas que tienen
las referencias, el compilador no adivinará cuál es la vida útil del resto de
las referencias. En este caso, en lugar de adivinar, el compilador dará
un error que puede resolver agregando las anotaciones de vida que
especifican cómo se relacionan las referencias entre sí.

La vida útil de los parámetros de función o método se denomina *vida útil de entrada*, y
la duración de los valores devueltos se denomina *vida útil de salida*.

El compilador usa tres reglas para averiguar qué tiempos de vida tienen las referencias cuando
no hay anotaciones explícitas. La primera regla se aplica a la vida útil de las entradas,
y la segunda y tercera reglas se aplican a la vida útil de las salidas. Si el compilador aplica
las tres reglas y todavía hay referencias para las que no se puede
averiguar la vida útil, se detendrá con un error. Estas reglas se aplican
a las definiciones `fn` así como a los bloques `impl`.

La primera regla es que cada parámetro que es una referencia tiene su propio parámetro de vida.
En otras palabras, una función con un parámetro obtiene un parámetro de vida útil:
`fn foo<'a>(x: &'a i32)`; una función con dos parámetros obtiene dos
parámetros de duración separados: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; y sucesivamente.

La segunda regla es que si hay exactamente un parámetro de vida útil de entrada,
la vida útil se asigna a todos los parámetros de vida útil de salida: `fn foo<'a>(x: &'a i32)
-> &'a i32`.

La tercera regla es que si hay varios parámetros de vida útil de entrada, pero uno de
ellos son `&self` o` &mut self` porque es un método, la vida de `self`
se asigna a todos los parámetros de vida útil de la salida. Esta tercera regla hace de los métodos
mucho más agradable de leer y escribir porque se necesitan menos símbolos.

Pongamonos en el punto de vista del compilador. Aplicaremos estas reglas para averiguar
la duración de las referencias en la declaración de la función `first_word`
en el Listado 10-26. La declaración comienza sin tiempos de vida asociados
con las referencias:

```rust,ignore
fn first_word(s: &str) -> &str {
```

Luego, el compilador aplica la primera regla, que especifica que cada parámetro
obtiene su propia vida. La llamaremos `'a` como de costumbre, por lo que ahora la declaración es
esto:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```

La segunda regla se aplica porque hay exactamente una vida útil de entrada. La segunda
regla especifica que la vida útil de un parámetro de entrada se asigna a
la vida útil de la salida, por lo que la declaración ahora es esta:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Ahora todas las referencias en esta declaración de función tienen una vida útil, y el
compilador puede continuar su análisis sin necesidad de que el programador realice anotaciones.

Veamos otro ejemplo, esta vez usando la función `longest` que no tenía
parámetros de vida cuando comenzamos a trabajar con ella en el Listado 10-21:

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```

Apliquemos la primera regla: cada parámetro tiene su propia duración. Esta vez 
tenemos dos parámetros en lugar de uno, por lo que tenemos dos vidas:

```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Puede ver que la segunda regla no se aplica porque hay más de una
vida útil de entrada. La tercera regla tampoco se aplica, porque `longest` es una
función en lugar de un método, por lo que ninguno de los parámetros es `self`. Después
de las tres reglas, todavía no hemos descubierto cuál es la vida del tipo del retorno.
Es por eso que obtuvimos un error al intentar compilar el código en el
listado 10-21: el compilador trabajó con las reglas de elite de vida, pero aún así
no pudo averiguar todas las vidas de las referencias en la declaración.

Debido a que la tercera regla realmente solo se aplica a las declaraciones de métodos, veremos a continuación
vidas en ese contexto para ver por qué la tercera regla significa que frecuentemente no tenemos que
anotar la vida útil en las declaraciones de métodos.

### Anotaciones de Vidas en Definiciones de Métodos

Cuando implementamos métodos en una estructura con tiempos de vida, usamos la misma sintaxis que
la de los parámetros de tipo genérico que se muestran en el Listado 10-11. Donde declaramos y
usamos los parámetros de duración depende de si están relacionados con campos de la estructura
o con los parámetros del método y los valores de retorno.

Los nombres de vida para los campos de estructura siempre deben declararse después de la 
palabra clave `impl` y se usan después del nombre de la estructura, porque esas vidas son parte
del tipo estructura.

En las declaraciones de métodos dentro del bloque `impl`, las referencias pueden estar vinculadas a la
vida útil de las referencias en los campos de la estructura, o pueden ser independientes.
Además, las reglas de elite de vida a menudo hacen que las anotaciones de vida
no sean necesarias en las declaraciones de métodos. Veamos algunos ejemplos usando la
estructura denominada `ImportantExcerpt` que definimos en el Listado 10-25.

Primero, usaremos un método llamado `level` cuyo único parámetro es una referencia a
`self` y cuyo valor de retorno es un `i32`, que no es una referencia a nada:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

La declaración del parámetro de duración después de `impl` y su uso después del nombre del tipo
son obligatorios, pero no estamos obligados a anotar la vida útil de la referencia
a `self` debido a la primera regla de elite.

A continuación, se muestra un ejemplo en el que se aplica la tercera regla de elite de vida:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

Hay dos vidas útiles de entrada, por lo que Rust aplica la primera regla de elite de vida
y da tanto a `&self` como a `notice` sus propias vidas. Entonces, ya que
uno de los parámetros es `&self`, el tipo de retorno obtiene la duración de `&self`,
y se han contabilizado todas las vidas.

### La Vida Estática

Una vida especial que debemos discutir es la `estática`, lo que significa que esta
referencia *puede* vivir durante toda la duración del programa. Toda cadena
literal tienen vida útil `estática`, que podemos anotar de la siguiente manera:

```rust
let s: &'static str = "I have a static lifetime.";
```

El texto de esta cadena se almacena directamente en el binario del programa, que
está siempre disponible. Por lo tanto, la vida útil de todos los literales de cadena es
`'static`.

Es posible que vea sugerencias para usar la vida útil `'static` en los mensajes de error. Pero
antes de especificar `'static` como el tiempo de vida de una referencia, piense en
si la referencia realmente vive toda la vida de su
programa o no. Podría considerar si quiere que viva tanto tiempo, incluso
si pudiera. La mayoría de las veces, el problema surge al intentar crear una
referencia colgante o una falta de coincidencia de las vidas disponibles. En tales casos,
la solución es arreglar esos problemas, no especificar la vida útil `'static`.

## Parámetros de Tipo Genérico, Traits Ligados y Tiempos de Vida Juntos

Veamos brevemente la sintaxis para especificar parámetros de tipo genérico, traits
ligados y vidas útiles, ¡todo en una sola función!

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

Esta es la función `longest` del Listado 10-22 que devuelve el más largo de dos slices de cadena.
Pero ahora tiene un parámetro adicional llamado `ann` del tipo genérico
`T`, que se puede completar con cualquier tipo que implemente el trait `Display`
como se especifica en la cláusula `where`. Este parámetro adicional se imprimirá
antes de que la función compare las longitudes de los slices de cadena, por lo que el
El trait ligado `Display` es necesario. Debido a que las vidas son un tipo de genérico,
las declaraciones del parámetro de vida útil `'a` y el parámetro de tipo genérico
`T` va en la misma lista dentro de los corchetes angulares después del nombre de la función.

## Resumen

¡Cubrimos mucho en este capítulo! Ahora que conoces el tipo genérico
parámetros, rasgos y límites de rasgos, y parámetros genéricos de por vida, estás
listo para escribir código sin repetición que funciona en muchas situaciones diferentes.
Los parámetros de tipo genérico le permiten aplicar el código a diferentes tipos. Rasgos y
Los límites de los rasgos garantizan que, aunque los tipos sean genéricos, tendrán la
comportamiento que necesita el código. Aprendió a usar anotaciones de por vida para asegurarse
que este código flexible no tendrá referencias colgantes. Y todo esto
El análisis se realiza en tiempo de compilación, lo que no afecta el rendimiento del tiempo de ejecución.

Lo crea o no, hay mucho más que aprender sobre los temas que discutimos en
este capítulo: El Capítulo 17 trata sobre los objetos de rasgo, que son otra forma de utilizar
rasgos. También hay escenarios más complejos que involucran anotaciones de por vida.
que solo necesitarás en escenarios muy avanzados; para esos, deberías leer
la [Referencia de Rust][referencia]. Pero a continuación, aprenderá a escribir pruebas en
Rust para que pueda asegurarse de que su código funcione como debería.

[referencias-y-préstamos]:ch04-02-referencias-y-préstamos.html#referencias-y-préstamos
[segmentos-de-cadena-como-parámetros]:ch04-03-slices.html # string-slices-as-parameters
[referencia]: ../reference/index.html

