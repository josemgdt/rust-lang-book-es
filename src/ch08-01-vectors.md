## Almacenamiento de listas de valores con vectores

El primer tipo de colección que veremos es `Vec<T>`, también conocido como *vector*.
Los vectores le permiten almacenar más de un valor en una sola estructura de datos que
pone todos los valores uno al lado del otro en la memoria. Los vectores solo pueden almacenar valores
del mismo tipo. Son útiles cuando tiene una lista de elementos, como las
líneas de texto en un archivo o los precios de los artículos en un carrito de compras.

### Creacion de un nuevo vector

Para crear un nuevo vector vacío, podemos llamar a la función `Vec::new`, como se muestra en
Listado 8-1.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

<span class="caption">Listado 8-1: Creación de un nuevo vector vacío para contener valores
de tipo `i32`</span>

Tenga en cuenta que agregamos una anotación de tipo aquí. Ya que no estamos insertando ningun
valores en este vector, Rust no sabe qué tipo de elementos pretendemos
almacenar. Éste es un punto importante. Los vectores se implementan usando genéricos;
cubriremos cómo usar genéricos con sus propios tipos en el Capítulo 10. Por ahora,
saber que el tipo `Vec<T>` proporcionado por la biblioteca estándar puede contener cualquier tipo,
y cuando un vector específico contiene un tipo específico, el tipo se especifica dentro
paréntesis angulares. En el Listado 8-1, le hemos dicho a Rust que el `Vec<T>` en `v`
contienen elementos del tipo `i32`.

En un código más realista, Rust a menudo puede inferir el tipo de valor que desea
almacenar una vez que inserte valores, por lo que rara vez necesita hacer este tipo de anotación.
Es más común crear un `Vec<T>` que tenga valores iniciales y Rust
proporciona la macro `vec!` para mayor comodidad. La macro creará un nuevo vector
que tiene los valores que le da. El Listado 8-2 crea un nuevo `Vec<i32>` que
contiene los valores `1`, `2`, y `3`. El tipo de número entero es `i32` porque es
el tipo de entero predeterminado, como discutimos en la sección ["Tipos de datos"][data-types]<!--
ignore --> del Capítulo 3.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

<span class="caption">Listado 8-2: Creando un nuevo vector que contiene
valores</span>

Como hemos proporcionado valores iniciales de `i32`, Rust puede inferir que el tipo de `v`
es `Vec<i32>`, y la anotación de tipo no es necesaria. A continuación, veremos cómo
para modificar un vector.

### Actualizacion de un vector

Para crear un vector y luego agregarle elementos, podemos usar el método `push`,
como se muestra en el Listado 8-3..

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

<span class="caption">Listado 8-3: Uso del método `push` para agregar valores a un
vector</span>

Como con cualquier variable, si queremos poder cambiar su valor, necesitamos
hacerlo mutable usando la palabra clave `mut`, como se discutió en el Capítulo 3. Los números
que colocamos dentro son todos del tipo `i32`, y Rust lo infiere de los datos, por lo que
no necesitamos la anotación `Vec<i32>`.

### Eliminar un vector elimina sus elementos

Como cualquier otra `struct`, un vector libera memoria cuando sale del alcance, como
en el Listado 8-4.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

<span class="caption">Listado 8-4: Dónde el vector y sus elementos
son eliminados</span>

Cuando se elimina el vector, todos sus contenidos también se eliminan, lo que significa
que esos enteros que contiene se limpiarán. Esto puede parecer un
punto sencillo, pero puede volverse un poco más complicado cuando se comienza a
introducir referencias a los elementos del vector. ¡Abordemos eso a continuación!

### Lectura de elementos de vectores

Ahora que sabe cómo crear, actualizar y destruir vectores, saber cómo
leer su contenido es el siguiente paso. Hay dos formas de hacer referencia a un
valor almacenado en un vector. En los ejemplos, hemos anotado los tipos de
valores que se devuelven desde estas funciones para mayor claridad.

El Listado 8-5 muestra ambos métodos para acceder a un valor en un vector, ya sea con
sintaxis de indexación o con el método `get`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

<span class="caption">Listado 8-5: Uso de la sintaxis de indexación o el método `get` para
acceder a un elemento en un vector</span>

Tenga en cuenta dos detalles aquí. Primero, usamos el valor de índice `2` para obtener el tercer
elemento; los vectores están indexados por números, comenzando en cero. En segundo lugar, las dos formas
para obtener el tercer elemento es mediante el uso de `&` y `[]`, que nos da una referencia,
o usando el método `get` con el índice pasado como argumento, lo que nos da
una `Option<&T>`.

Rust tiene dos formas de hacer referencia a un elemento para que pueda elegir cómo se comporta el programa
cuando intenta utilizar un valor de índice que el vector no tiene.
Como ejemplo, veamos qué hará un programa si tiene un vector
que contiene cinco elementos e intenta acceder a un elemento en el índice 100, como
se muestra en el Listado 8-6.

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

<span class="caption">Listado 8-6: Intentando acceder al elemento en el índice
100 en un vector que contiene cinco elementos</span>

Cuando ejecutamos este código, el primer método `[]` hará que el programa entre en pánico
porque hace referencia a un elemento inexistente. Este método se utiliza mejor cuando
desea que su programa se bloquee si hay un intento de acceder a un elemento más allá del
final del vector.

Cuando al método `get` se le pasa un índice que está fuera del vector, devuelve
`None` sin entrar en pánico. Usaría este método si el acceso a un elemento
más allá del rango del vector ocurre ocasionalmente en circunstancias normales.
Su código entonces tendrá la lógica para manejar esto, ya sea `Some(&element)` o
`None`, como se discutió en el Capítulo 6. Por ejemplo, el índice podría provenir de
una persona que ingresa un número. Si ingresan accidentalmente un número, también
grande y el programa obtiene un valor `None`, podría decirle al usuario cuántos
elementos están en el vector actual y dar otra oportunidad de ingresar un valor válido.
¡Eso sería mejor que bloquear el programa debido a un error tipográfico!

Cuando el programa tiene una referencia válida, el verificador de préstamos hace cumplir la
reglas de propiedad y préstamos (cubiertas en el Capítulo 4) para asegurar que esta referencia
y cualquier otra referencia al contenido del vector sigue siendo válida. Recuerde la
regla que establece que no puede tener referencias mutables e inmutables en el mismo
alcance. Esa regla se aplica en el Listado 8-7, donde tenemos una referencia inmutable
al primer elemento de un vector e intente agregar un elemento al final, que no
funciona si también intentamos hacer referencia a ese elemento más adelante en la función:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

<span class="caption">Listado 8-7: Intentando agregar un elemento a un vector
mientras hay una referencia a un item</span>

Compilar este código resultará en este error:

```console
{{#include ../listings/ch08-common-collections/listing-08-07/output.txt}}
```

El código del Listado 8-7 podría parecer que debería funcionar: ¿por qué una referencia
al primer elemento le importa qué cambie el final del vector? Este
error se debe a la forma en que funcionan los vectores; agregar un nuevo elemento al final del
vector puede requerir la asignación de nueva memoria y la copia de los elementos antiguos en el
nuevo espacio si no hay suficiente para poner todos los elementos uno al lado de
otro donde está el vector actualmente. En ese caso, la referencia al primer
elemento estaría apuntando a la memoria desasignada. Las reglas de prestamo prohiben
programas con esa situación.

> Nota: Para obtener más información sobre la implementación del tipo `Vec<T>`, consulte [“El
> Rustonomicon”][nomicon].

### Iterando los valores en un vector

Si queremos acceder a cada elemento en un vector, uno a la vez, podemos iterar a través de
todos los elementos en lugar de utilizar índices para accederlos. El listado
8-8 muestra cómo usar un bucle `for` para obtener referencias inmutables a cada elemento
en un vector de valores `i32` e imprimirlos.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

<span class="caption">Listado 8-8: Imprimiendo cada elemento en un vector
iterando sobre los elementos usando un bucle `for`</span>

We can also iterate over mutable references to each element in a mutable vector
in order to make changes to all the elements. The `for` loop in Listing 8-9
will add `50` to each element.
También podemos iterar sobre referencias mutables a cada elemento en un vector mutable
para realizar cambios en todos los elementos. El bucle `for` en el Listado 8-9
agregará `50` a cada elemento.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

<span class="caption">Listado 8-9: Iterando sobre referencias mutables a
elementos en un vector</span>

To change the value that the mutable reference refers to, we have to use the
dereference operator (`*`) to get to the value in `i` before we can use the
`+=` operator. We’ll talk more about the dereference operator in the
[“Following the Pointer to the Value with the Dereference Operator”][deref]
section of Chapter 15.
Para cambiar el valor al que se refiere la referencia mutable, tenemos que usar el
operador de desreferencia (`*`) para obtener el valor en `i` antes de que podamos usar el
operador `+=`. Hablaremos más sobre el operador de desreferencia en la sección
["Seguir el puntero al valor con el operador de desreferencia"][deref]
 del Capítulo 15.

### Uso de una enumeracion para almacenar varios tipos

Al comienzo de este capítulo, dijimos que los vectores solo pueden almacenar valores
que son del mismo tipo. Esto puede resultar inconveniente; definitivamente hay
casos con necesidad de almacenar una lista de elementos de diferentes tipos. Afortunadamente,
las variantes de una enumeración se definen bajo el mismo tipo de enumeración, por lo que cuando necesitamos
almacenar elementos de un tipo diferente en un vector, podemos definir y usar una enumeración!

Por ejemplo, digamos que queremos obtener valores de una fila en una hoja de cálculo en la que
algunas de las columnas de la fila contienen números enteros, algunos números de punto flotante,
y algunas cadenas. Podemos definir una enumeración cuyas variantes contendrán los diferentes
tipos de valor, y luego todas las variantes de enumeración se considerarán del mismo tipo:
el de la enumeración. Entonces podemos crear un vector que contenga esa enumeración y así,
en última instancia, tiene diferentes tipos. Demostramos esto en el Listado 8-10.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

<span class="caption">Listado 8-10: Definición de una `enum` para almacenar valores de
diferentes tipos en un vector</span>

Rust necesita saber qué tipos estarán en el vector en tiempo de compilación para saber
exactamente cuánta memoria en el montón se necesitará para almacenar cada elemento.
La ventaja secundaria es que podemos ser explícitos sobre qué tipos están permitidos en
este vector. Si Rust permitiera que un vector contenga cualquier tipo, habría posibilidad
de que uno o más de los tipos causarían errores con las operaciones realizadas
en los elementos del vector. Usar una enumeración más una expresión `match` significa
que Rust se asegurará en el momento de la compilación de que se manejen todos los casos posibles, como
discutimos en el Capítulo 6.

Cuando escribe un programa, si no conoce el conjunto exhaustivo de tipos
el programa los obtendrá en tiempo de ejecución para almacenar en un vector, la técnica de enumeración no
trabaja. En su lugar, puede usar un objeto trait, que cubriremos en el Capítulo 17.

Ahora que hemos analizado algunas de las formas más comunes de utilizar vectores, asegúrese de
revisar [la documentación de la API][vec-api] para conocer todos los métodos útiles definidos en
`Vec<T>` por la biblioteca estándar. Por ejemplo, además de `push`, el método `pop`
elimina y devuelve el último elemento. Pasemos al siguiente tipo de colección: `String`!

[data-types]: ch03-02-data-types.html#tipos-de-datos
[nomicon]: ../nomicon/vec.html
[vec-api]: ../std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
