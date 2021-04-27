# Tipos Genéricos, Traits y Lifetimes

Cada lenguaje de programación tiene herramientas para manejar eficazmente la duplicación.
de conceptos. En Rust, una de estas herramientas es *generics*. Los genéricos son
sustitutos abstractos para tipos concretos u otras propiedades. Cuando escribimos código,
se puede expresar el comportamiento de los genéricos o cómo se relacionan con otros genéricos
sin saber qué habrá en su lugar al compilar y ejecutar el código.

Similar a la forma en que una función toma parámetros con valores desconocidos para ejecutar el
mismo código con múltiples valores concretos, las funciones pueden tomar parámetros de algun
tipo genérico en lugar de un tipo concreto, como `i32` o `String`. De hecho, hemos
usado ya genéricos ​​en el Capítulo 6 con `Option<T>`, en el Capítulo 8 con `Vec<T>`
y `HashMap<K, V>`, y en el Capítulo 9 con `Result<T, E>`. En este capítulo,
exploraremos cómo definir sus propios tipos, funciones y métodos con genéricos.

Primero, revisaremos cómo extraer una función para reducir la duplicación de código. Luego
usaremos la misma técnica para hacer una función genérica a partir de dos funciones que
difieren solo en los tipos de sus parámetros. También explicaremos cómo usar
tipos genéricos en las definiciones de estructura y enumeración.

Mas tarde aprenderá a usar *traits* para definir el comportamiento de una manera genérica.
Puede combinar traits con tipos genéricos para restringir un tipo genérico a solo
aquellos tipos que tienen un comportamiento particular, y no para cualquier tipo.

Por último, analizaremos *lifetimes*, una variedad de genéricos que dan la
información del compilador sobre cómo las referencias se relacionan entre sí. Las vidas permiten
tomar prestados valores en muchas situaciones y, al mismo tiempo, permiten que el compilador
compruebe que las referencias sean válidas.

## Eliminar duplicaciones extrayendo una función

Antes de sumergirnos en la sintaxis de los genéricos, veamos primero cómo eliminar
duplicación que no implica tipos genéricos mediante la extracción de una función. Luego,
aplicaremos esta técnica para extraer una función genérica. De la misma manera que
reconoce el código duplicado para extraerlo en una función, comenzará a
reconocer el código duplicado que puede utilizar genéricos.

Considere un programa corto que encuentre el número más grande en una lista, como se muestra en
Listado 10-1.

<span class="filename">​​Nombre de archivo:src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```

<span class="caption">Listado 10-1: Código para encontrar el número más grande en una lista
de números</span>

Este código almacena una lista de enteros en la variable `number_list` y coloca
el primer número de la lista en una variable denominada `largest`. Entonces itera
a través de todos los números de la lista, y si el número actual es mayor que
el número almacenado en `largest`, reemplaza el número en esa variable.
Sin embargo, si el número actual es menor o igual al mayor número visto
hasta ahora, la variable no cambia y el código pasa al siguiente número
en la lista. Después de considerar todos los números de la lista, `largest` debería
contener el número más grande, que en este caso es 100.

Para encontrar el número más grande en dos listas de números diferentes, podemos duplicar
el código en el Listado 10-1 y usar la misma lógica en dos lugares diferentes en el
programa, como se muestra en el Listado 10-2.

<span class="filename">​​Nombre de archivo:src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```

<span class="caption">Listado 10-2: Código para encontrar el número más grande en *dos*
listas de números</span>

Aunque este código funciona, la duplicación de código es tediosa y propensa a errores. Nosotros también
tenemos que actualizar el código en varios lugares cuando queremos cambiarlo.

Para eliminar esta duplicación, podemos crear una abstracción definiendo una
función que opera en cualquier lista de enteros que se le de en un parámetro. Esta
solución hace que nuestro código sea más claro y nos permite expresar el concepto de encontrar
el número más grande en una lista de forma abstracta.

En el Listado 10-3, extrajimos el código que encuentra el número más grande en una
función denominada `largest`. A diferencia del código del Listado 10-1, que puede encontrar el
mayor número en una sola lista en particular, este programa puede encontrar el mayor
número en dos listas diferentes.

<span class="filename">​​Nombre de archivo:src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```

<span class="caption">Listado 10-3: código abstraído para encontrar el número más grande
en dos listas</span>

La función `largest` tiene un parámetro llamado `list`, que representa cualquier
segmento concreto de valores `i32` que podríamos pasar a la función.
Como resultado, cuando llamamos a la función, el código se ejecuta en los valores específicos que
pasamos. No se preocupe por la sintaxis del bucle `for` por ahora. No estamos
haciendo referencia a una referencia a un `i32` aquí; estamos emparejando patrones y
desestructurando cada `&i32` que obtiene el bucle `for` para que `item` sea un
`i32` dentro del cuerpo del bucle. Cubriremos la coincidencia de patrones en detalle en el 
[Capítulo 18][ch18]<!-- ignore -->.

En resumen, estos son los pasos para cambiar el código del Listado 10-2 a
Listado 10-3:

1. Identifique el código duplicado.
2. Extraiga el código duplicado en el cuerpo de la función y especifique las
   entradas y valores de retorno de ese código en la declaración de la función.
3. Actualice las dos instancias de código duplicado para llamar a la función en su lugar.

A continuación, usaremos estos mismos pasos con genéricos para reducir la duplicación de código en
diferentes caminos. De la misma manera que el cuerpo funcional puede operar en un
`list` abstracto en lugar de valores específicos, los genéricos permiten que el código opere en
tipos abstractos.

Por ejemplo, digamos que tenemos dos funciones: una que encuentra el elemento más grande en un
slice de valores `i32` y uno que encuentre el elemento más grande en un slice de valores `char`.
¿Cómo eliminaríamos esa duplicación? ¡Vamos a averiguarlo!

[ch18]: ch18-00-patterns.html
