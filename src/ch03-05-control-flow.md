## Control de Flujo 

Decidir si ejecutar o no algún código dependiendo de si una condición es verdadera o no
y decidir si ejecutar algún código repetidamente mientras una condición es verdadera son
bloques básicos de construcción en la mayoría de los lenguajes de programación. Los constructos más comunes que
le permiten controlar el flujo de ejecución del código en Rust son las expresiones `if` y los
bucles.

### Expresiones `if`

Una expresión `if` le permite ramificar su código dependiendo de condiciones; se
proporciona una condición y luego se dice: "Si se cumple esta condición, ejecutar este bloque
de código. Si no se cumple, no ejecutarlo".

Cree un nuevo proyecto llamado *branches* en su directorio *projects* para explorar
la expresión `if`. En el archivo *src/main.rs*, ingrese lo siguiente:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Todas las expresiones `if` comienzan con la palabra clave` if`, seguida de una
condición. En este caso, la condición comprueba si la variable
`number` tiene un valor menor que 5. El bloque de código que queremos ejecutar si
la condición es verdadera se coloca inmediatamente después de la condición dentro de
llaves. Los bloques de código asociados con las condiciones en las expresiones `if` son
llamados a veces *brazos*, al igual que los brazos en las expresiones de "coincidencia" que
discutimos en la sección ["Comparando la conjetura con el Número secreto”][comparing-the-guess-to-the-secret-number]<!-- ignore --> del Capitulo 2.

Opcionalmente, también podemos incluir una expresión "else"
para darle al programa un bloque de código alternativo a ejecutar cuando
la condición se evalúa como falsa. Si no proporciona una expresión "else" y
la condición es falsa, el programa simplemente saltará el bloque `if` y continuará
al siguiente código.

Intente ejecutar este código; debería ver el siguiente resultado:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Intentemos cambiar el valor de `number` a un valor que haga la condición
`false` para ver qué pasa:


```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Ejecute el programa nuevamente y observe el resultado:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

También vale la pena señalar que la condición en este código *debe* ser un `bool`. Si
la condición no es un "bool", obtendremos un error. Por ejemplo, intente ejecutar el
siguiente código:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

La condición `if` se evalúa con un valor de `3` esta vez, y Rust arroja un
error:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

El error indica que Rust esperaba un "bool" pero obtuvo un número entero. A diferencia de
lenguajes como Ruby y JavaScript, Rust no intentará automáticamente
convertir tipos no booleanos en booleanos. Debe ser explícito y proporcionar siempre
`if` con un booleano como condición. Si queremos que se ejecute el bloque de código `if`
solo cuando un número no es igual a "0", por ejemplo, podemos cambiar la
expresión `if` a lo siguiente:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Ejecutar este código imprimirá `number was something other than zero`.

#### Manejo de Condiciones Multiples en `else if`

Puede tener varias condiciones combinando `if` y` else` en una
expresión `else if`. Por ejemplo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Este programa tiene cuatro caminos posibles que puede tomar. Después de ejecutarlo, debería
ver el siguiente resultado:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Cuando este programa se ejecuta, comprueba cada expresión `if` cada vez y ejecuta
el primer cuerpo de codigo para el que se cumple la condición. Tenga en cuenta que aunque 6 es
divisible por 2, no vemos el resultado `número es divisible por 2`, ni
el texto `número no es divisible por 4, 3 o 2` del bloque` else`.
Eso es porque Rust solo ejecuta el bloque para la primera condición verdadera, y
una vez que encuentra uno, ni siquiera comprueba el resto.

Usar demasiadas expresiones `else if` puede saturar su código, así que si tiene más
de una, es posible que desee refactorizar su código. El capítulo 6 describe una poderosa
construcción de ramificación de Rust llamada `match` para estos casos.

#### Usando `if` en una Instruccion` let`

Como `if` es una expresión, podemos usarla en el lado derecho de una
declaración `let`, como en el Listado 3-2.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

<span class="caption">Listado 3-2: Asignación del resultado de una expresión `if`
a una variable</span>

La variable `number` estará vinculada a un valor basado en el resultado de la
expresión `if`. Ejecute este código para ver qué sucede:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Recuerde que los bloques de código se evalúan hasta la última expresión en ellos, y
los números en sí mismos también son expresiones. En este caso, el valor de
toda la expresión `if` depende del bloque de código que se ejecute. Esto significa que
los valores que tienen el potencial de ser resultados de cada brazo del `if` deben ser
del mismo tipo; en el Listado 3-2, los resultados del brazo `if` y del `else`
eran enteros "i32". Si los tipos no coinciden, como en el siguiente
ejemplo, obtendremos un error:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Cuando intentemos compilar este código, obtendremos un error. Los brazos `if` y `else`
tienen tipos de valor que son incompatibles, y Rust indica exactamente dónde
encuentra el problema en el programa:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

La expresión en el bloque `if` se evalúa como un número entero, y la expresión en
el bloque `else` se evalúa como una cadena. Esto no funcionará porque las variables deben
tener un solo tipo. Rust necesita saber en el momento de la compilación qué tipo de
variable es `number`, definitivamente, por lo que puede verificar en tiempo de compilación que su tipo es
válido en todos los lugares donde usamos `number`. Rust no podría hacer eso si el tipo de
`number` solo se determinó en tiempo de ejecución; el compilador sería más complejo
y ofrecería menos garantías sobre el código si tuviera que realizar un seguimiento de
múltiples tipos hipotéticos para cualquier variable.

### Repeticion con Bucles

Suele ser útil ejecutar un bloque de código más de una vez. Para esta tarea,
Rust proporciona varios *bucles*. Un bucle recorre el código dentro
cuerpo del bucle hasta el final y luego comienza inmediatamente desde el principio. Para
experimentar con bucles, hagamos un nuevo proyecto llamado *loops*.

Rust tiene tres tipos de bucles: `loop`, `while` y `for`. Probemos cada uno.

#### Codigo Repetido con `loop`

La palabra clave `loop` le dice a Rust que ejecute un bloque de código una y otra vez
hasta que se le diga explícitamente que se detenga.

Como ejemplo, cambie el archivo *src/main.rs* en su directorio *loops* para tener algo
como esto:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Cuando ejecutamos este programa, veremos `again!` impreso una y otra vez continuamente
hasta que detengamos el programa manualmente. La mayoría de los terminales admiten un atajo de teclado,
<span class="keystroke">ctrl-c</span>, para interrumpir un programa que está atascado en
un bucle continuo. Probemoslo:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

El símbolo `^C` representa el lugar donde presionó <span class="keystroke">ctrl-c
</span>. Es posible que vea o no la palabra `again!` impresa después de `^C`,
dependiendo de dónde estaba el código en el bucle cuando recibió la señal de interrupción.

Afortunadamente, Rust proporciona otra forma más confiable de salir de un bucle.
Puede colocar la palabra clave `break` dentro del bucle para indicarle al programa cuándo
dejar de ejecutar el bucle. Recuerde que hicimos esto en el juego de adivinanzas en la sección
["Salir después de una suposición correcta"][quitting-after-a-correct-guess]<!-- ignore
--> del Capítulo 2 para salir del programa cuando el usuario ganó el juego al
adivinar el número correcto.

#### Devolucion de Valores desde Bucles

Uno de los usos de un `loop` es reintentar una operación que sabe que puede fallar, como
comprobar si un hilo ha completado su trabajo. Sin embargo, es posible que deba
pasa el resultado de esa operación al resto del código. Para hacer esto, puede
agregar el valor que desea que se devuelva después de la expresión `break` que usa para detener
el lazo; ese valor se devolverá fuera del ciclo para que pueda usarlo, como
se muestra aquí:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Antes del ciclo, declaramos una variable llamada `counter` y la inicializamos a
`0`. Luego declaramos una variable llamada `result` para contener el valor devuelto del
lazo. En cada iteración del ciclo, agregamos `1` a la variable `counter`,
y luego verificamos si el contador es igual a "10". Cuando lo es, usamos la
palabra clave `break` con el valor `counter * 2`. Después del ciclo, usamos un
punto y coma para finalizar la instrucción que asigna el valor a `result`. Finalmente,
imprime el valor en `result`, que en este caso es 20.

#### Bucles Condicionales con `while`

A menudo, es útil que un programa evalúe una condición dentro de un bucle. Mientras
la condición es verdadera, el ciclo se ejecuta. Cuando la condición deja de ser verdadera, el
el programa llama a `break`, deteniendo el bucle. Este tipo de bucle podría implementarse
usando una combinación de `loop`, `if`, `else` y `break`; podría, si lo desea, intentar ahora eso
en un programa.

Sin embargo, este patrón es tan común que Rust tiene una construcción del lenguaje incorporada
para ello, que se llama un bucle `while`. El Listado 3-3 usa `while`; el programa se repite
tres veces, contando hacia atrás cada vez, y luego, después del bucle, imprime
otro mensaje y sale.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<span class="caption">Listado 3-3: Uso de un bucle `while` para ejecutar código mientras
la condición es verdadera</span>

Esta construcción elimina una gran cantidad de anidamiento que sería necesario si utilizara
`loop`, `if`, `else` y `break`, y es más claro. Mientras una condición se mantenga
cierta, el código se ejecuta; de lo contrario, se sale del bucle.

#### Recorrer una Coleccion con `for`

Puede usar la construcción `while` para recorrer los elementos de una colección,
como una matriz. Por ejemplo, veamos el Listado 3-4.

<span class="filename">Nombre de archivo: src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<span class="caption">Listado 3-4: Recorrer cada elemento de una colección
usando un bucle `while`</span>

Aquí, el código cuenta los elementos de la matriz. Empieza en el índice
`0`, y luego se repite hasta que alcanza el índice final en la matriz (es decir,
cuando `index < 5` ya no es cierto). Ejecutar este código imprimirá todos los elementos
en la matriz:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Los cinco valores de la matriz aparecen en la terminal, como se esperaba. Aunque `index`
alcanzará un valor de `5` en algún momento, el ciclo dejará de ejecutarse antes de intentar
obtener un sexto valor de la matriz.

Pero este enfoque es propenso a errores; podríamos hacer que el programa entre en pánico si
la longitud del índice es incorrecta. También es lento, porque el compilador agrega código de tiempo de ejecución
para realizar la verificación condicional en cada elemento en cada iteración
a través del bucle.

Como alternativa más concisa, puede usar un bucle `for` y ejecutar algún código
para cada elemento de una colección. Un bucle "for" se parece al código del Listado 3-5.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<span class="caption">Listado 3-5: Recorrer cada elemento de una colección
usando un bucle `for`</span>

Cuando ejecutamos este código, veremos el mismo resultado que en el Listado 3-4. Y
lo que es más importante, ahora hemos aumentado la seguridad del código y hemos eliminado la
posibilidad de errores que puedan resultar de ir más allá del final de la matriz, o no
ir lo suficientemente lejos y falten algunos elementos.

Por ejemplo, en el código del Listado 3-4, si cambió la definición de la
matriz `a` para tener cuatro elementos pero se olvidó de actualizar la condición `whileindex < 4`, 
el código entraría en pánico. Usando el ciclo `for`, no necesitaría
recordar cambiar cualquier otro código si cambió el número de valores en la matriz.

La seguridad y la concisión de los bucles `for` los convierten en la construccion de bucle más utilizada
en Rust. Incluso en situaciones en las que desea ejecutar algún código
cierto número de veces, como en el ejemplo de la cuenta regresiva que usó un bucle `while`
en el Listado 3-3, la mayoría de los rustáceos usarían un bucle `for`. La forma de hacer eso
sería utilizar un `Range`, que es un tipo proporcionado por la biblioteca estándar
que genera todos los números en secuencia comenzando desde un número y terminando
antes de otro número.

Así es como se vería la cuenta regresiva usando un bucle `for` y otro método del que
todavía no hemos hablado, `rev`, para invertir el rango:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Este código es un poco mejor, ¿no?

## Resumen

¡Hecho! Ese fue un capítulo considerable: aprendiste sobre variables, escalares
y tipos de datos compuestos, funciones, comentarios, expresiones `if` y bucles. Si
desea practicar con los conceptos discutidos en este capítulo, intente construir
programas para hacer lo siguiente:

* Convertir temperaturas entre Fahrenheit y Celsius.
* Genera el n-ésimo número de Fibonacci.
* Imprimir la letra del villancico "Los doce días de Navidad"
  aprovechando la repetición en la canción.

Cuando esté listo para seguir adelante, hablaremos sobre un concepto en Rust que *no*
existe comúnmente en otros lenguajes de programación: la propiedad.

[comparing-the-guess-to-the-secret-number]:ch02-00-guessing-game-tutorial.html#comparando-la-conjetura-con-el-numero-secreto
[quitting-after-a-correct-guess]:ch02-00-guessing-game-tutorial.html#salir-despues-de-una-suposicion-correcta

