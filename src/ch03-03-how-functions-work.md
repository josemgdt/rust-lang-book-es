## Funciones

Las funciones estan omnipresentes en el código de Rust. Ya ha visto una de las
funciones más importantes en el lenguaje: la función `main`, que es el punto de entrada
de muchos programas. También ha visto la palabra clave `fn`, que le permite
declarar nuevas funciones.

El código Rust usa *caso snake* como estilo convencional para nombres de función y variables.
En el *caso snake*, todas las letras son minúsculas y las palabras se separan con guiones bajos.
Aquí hay un programa que contiene una definición de función de ejemplo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Las definiciones de funciones en Rust comienzan con `fn` y tienen un par de paréntesis
después del nombre de la función. Las llaves le dicen al compilador donde comienza el
el cuerpo de la función y donde termina.

Podemos llamar a cualquier función que hayamos definido ingresando su nombre seguido de un par
de paréntesis. Debido a que `another_function` está definida en el programa, se puede
llamar desde dentro de la función `main`. Tenga en cuenta que definimos `another_function`
*después de* la función `main` en el código fuente; podríamos haberlo definido antes
también. A Rust no le importa dónde defina sus funciones, solo si se han
definido en alguna parte.

Comencemos un nuevo proyecto binario llamado *functions* para explorar más funciones.
Coloque el ejemplo `another_function` en *src/main.rs* y ejecútelo.
Debería ver el siguiente resultado:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Las líneas se ejecutan en el orden en que aparecen en la función `main`.
Primero, se imprime el mensaje "¡Hola, mundo!", y luego se llama a `another_function`
y se imprime su mensaje.

### Parametros de Funcion

Las funciones también se pueden definir para tener *parámetros*, que son variables especiales
que forman parte de la declaracion de una función. Cuando una función tiene parámetros,
puede proporcionarle valores concretos a esos parámetros. Técnicamente,
los valores concretos se llaman *argumentos*, pero en una conversación informal, la gente tiende
utilizar las palabras *parámetro* y *argumento* indistintamente para
variables en la definición de una función o los valores concretos pasados ​​cuando
se llama a una función.

La siguiente versión reescrita de `another_function` muestra como se ven los parámetros
en Rust:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Intente ejecutar este programa; debería obtener el siguiente resultado:


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

La declaración de `another_function` tiene un parámetro llamado `x`. El tipo de
`x` se especifica como `i32`. Cuando se pasa `5` a `another_function`,
la macro `println!` pone `5` donde estaban el par de llaves en el formato
string.

En las declaraciones de funciones, *debe* declarar el tipo de cada parámetro. Esto es
una decisión deliberada en el diseño de Rust; al requerir anotaciones de tipo en
definiciones de función significa que el compilador casi nunca necesita que las use
en otro lugar en el código para averiguar a qué se refiere.

Cuando desee que una función tenga varios parámetros, separe las 
declaraciones de parámetros con comas, como aqui:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Este ejemplo crea una función con dos parámetros, ambos son tipos `i32`.
La función imprime los valores de ambos parámetros. Tenga en cuenta que
no es necesario que todos los parámetros de función sean del mismo tipo, casualmente lo son
en este ejemplo.

Intentemos ejecutar este código. Reemplace el archivo *src/main.rs* del programa actual 
en sus proyecto *functions* con el ejemplo anterior y ejecútelo usando `cargo run`:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Como llamamos a la función con `5` como valor para `x` y `6`
como valor de `y`, las dos cadenas se imprimen con estos valores.

### Cuerpos de Funciones que Contienen Sentencias y Expresiones

Los cuerpos de funciones se componen de una serie de declaraciones que, opcionalmente, terminan en una
expresión. Hasta ahora, solo cubrimos funciones sin una expresión final,
pero ha visto una expresión como parte de una declaración. Puesto que Rust es un
lenguaje basado en expresiones, esta es una distinción importante de entender.
Otros lenguajes no tienen las mismas distinciones, así que veamos qué
son declaraciones y expresiones y cómo sus diferencias afectan a los cuerpos de
las funciones.

De hecho, ya usamos declaraciones y expresiones. Las *Declaraciones* son
instrucciones que realizan alguna acción y no devuelven un valor. Las *Expresiones*
evaluan algo a un valor resultante. Veamos algunos ejemplos.

Crear una variable y asignarle un valor con la palabra clave `let` es una
declaración. En el Listado 3-1, `let y = 6;` es una declaración.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

<span class="caption">Listado 3-1: Una declaración de función `main` que contiene una declaración</span>

Las definiciones de funciones también son declaraciones; todo el ejemplo anterior es una
declaración en sí misma.

Las declaraciones no devuelven valores. Por lo tanto, no puede asignar una instrucción "let"
a otra variable, como intenta hacer el siguiente código, que dará un error:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Cuando ejecute este programa, el error que dará se verá así:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

La instrucción `let y = 6` no devuelve un valor, por lo que no hay nada que enlazar a
`x`. Esto es diferente de lo que sucede en otros lenguajes, como
C ó Ruby, donde la asignación devuelve el valor de la asignación. En esos
lenguajes, puede escribir `x = y = 6` y hacer que tanto` x` como `y` tengan el valor
`6`; ese no es el caso en Rust.

Las expresiones evalúan algo y constituyen la mayor parte del resto del código que
escribirá en Rust. Considere una operación matemática simple, como `5 + 6`, que
es una expresión que se evalúa con el valor `11`. Las expresiones pueden ser parte de
declaraciones; en el Listado 3-1, el `6` en la declaración `let y = 6;` es una
expresión que se evalúa con el valor `6`. Llamar a una función es una
expresión. Llamar a una macro es una expresión. El bloque que usamos para crear
new scopes, `{}`, es una expresión, por ejemplo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

Esta expresión:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

es un bloque que, en este caso, se evalúa como `4`. Ese valor se vincula a `y`
como parte de la declaración "let". Tenga en cuenta la línea `x + 1` sin punto y coma en
el final, que es diferente a la mayoría de las líneas que has visto hasta ahora. Las expresiones
no incluye punto y coma final. Si agrega un punto y coma al final de una
expresión, la convierte en una declaración, que luego no devolverá un valor.
Tenga esto en cuenta a medida que explore a continuación los valores de retorno de funciones y las expresiones.

### Funciones con Retorno de Valores

Las funciones pueden devolver valores al código que las llama. No nombramos
valores devueltos, pero declaramos su tipo tras una flecha (`->`). En Rust, el
valor retornado de la función es sinónimo del valor de la expresión final en
el bloque del cuerpo de la función. Puede regresar anticipadamente de una función
usando la palabra clave `return` y especificando un valor, pero la mayoría de las funciones devuelven
la última expresión implícitamente. A continuación, se muestra un ejemplo de una función que devuelve un
valor:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

No hay llamadas a funciones, macros o incluso declaraciones `let` en la
función `five`; solo el número `5` por sí mismo. Esa es una función perfectamente válida en
Rust. Tenga en cuenta que el tipo de retorno de la función también se especifica, `-> i32`. Intente
ejecutar este código; la salida debería verse así:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

El `5` en `five` es el valor de retorno de la función, por lo que el tipo de retorno
es `i32`. Examinemos esto con más detalle. Hay dos partes importantes:

Primero, la línea `let x = five ();` muestra que estamos usando el valor de retorno de una
función para inicializar una variable. Debido a que la función `five` devuelve un `5`,
esa línea es la misma que la siguiente:

```rust
let x = 5;
```

En segundo lugar, la función `five` no tiene parámetros y define el tipo de
valor de retorno, pero el cuerpo de la función es un `5` solitario sin punto y coma
porque es una expresión cuyo valor queremos devolver.

Veamos otro ejemplo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Al ejecutar este código, se imprimirá `The value of x is: 6`. Pero si colocamos un
punto y coma al final de la línea que contiene `x + 1`, cambiándola de un
expresión a una declaración, obtendremos un error.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

La compilación de este código produce un error, como el siguiente:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

El mensaje de error principal, “mismatched types,” revela el problema principal con este
código. La definición de la función `plus_one` dice que devolverá un
`i32`, pero las declaraciones no evalúan un valor, que se expresa mediante `()`,
una tupla vacía. Por lo tanto, no se devuelve nada, lo que contradice la definición de función
y da como resultado un error. En esta salida, Rust proporciona un mensaje que
posiblemente ayude a rectificar este problema: sugiere eliminar el punto y coma, que
arreglaría el error.
