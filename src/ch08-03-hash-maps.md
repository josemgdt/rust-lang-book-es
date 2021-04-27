## Almacenamiento de claves con valores asociados en mapas hash

La última de nuestras colecciones comunes es el *mapa hash*. El tipo `HashMap<K, V>`
almacena un mapeo de claves de tipo `K` a valores de tipo `V`. Lo hace a través de una
*función hash*, que determina cómo coloca estas claves y valores en
memoria. Muchos lenguajes de programación admiten este tipo de estructura de datos, pero
a menudo usan un nombre diferente, como hash, mapa, objeto, tabla hash,
diccionario, o matriz asociativa, solo por nombrar algunos.

Los mapas hash son útiles cuando desea buscar datos sin usar un índice, ya que
puede hacerlo con vectores, pero usando una clave que puede ser de cualquier tipo. Por ejemplo,
en un juego, puede realizar un seguimiento de la puntuación de cada equipo en un mapa hash en el que
cada clave es el nombre de un equipo y los valores son la puntuación de cada equipo. Dado el nombre de un equipo,
puede recuperar su puntuación.

Repasaremos la API básica de mapas hash en esta sección, pero muchas más ventajas
se esconden en las funciones definidas  por la biblioteca estándar en `HashMap<K, V>`.
Como siempre, consulte la documentación de la biblioteca estándar para obtener más información.

### Creación de un nuevo mapa hash

Puede crear un mapa hash vacío con `new` y agregar elementos con `insert`. En el
Listado 8-20, estamos realizando un seguimiento de las puntuaciones de dos equipos cuyos nombres son
Blue y Yellow. El equipo blue comienza con 10 puntos y el equipo yellow
comienza con 50.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

<span class="caption">Listado 8-20: Creando un nuevo mapa hash e insertando algunos
claves y valores</span>

Tenga en cuenta que primero debemos `use` el` HashMap` de la parte de colecciones de
la biblioteca estándar. De nuestras tres colecciones comunes, esta es la de
de uso menos frecuente, por lo que no se incluye en las funciones incluidas en el alcance
automáticamente en el preludio. Los mapas hash también tienen menos soporte de
biblioteca estándar; no hay una macro incorporada para construirlos, por ejemplo.

Al igual que los vectores, los mapas hash almacenan sus datos en el montón. Este `HashMap` tiene
claves de tipo `String` y valores de tipo `i32`. Como los vectores, los mapas hash son
homogéneos: todas las claves deben tener el mismo tipo y todos los valores
debe tener el mismo tipo.

Otra forma de construir un mapa hash es mediante el uso de iteradores y el método `collect`
en un vector de tuplas, donde cada tupla consta de una clave y su valor.
Entraremos en más detalles sobre los iteradores y sus métodos asociados en
la sección [”Procesamiento de una serie de elementos con iteradores”][iterators]<!-- ignore --> del Capítulo 13.
El método `collect` recopila datos en distintos
tipos de colección, incluido `HashMap`. Por ejemplo, si tuviéramos los nombres de los equipos
y puntuaciones iniciales en dos vectores separados, podríamos usar el método `zip` para
crear un vector de tuplas donde "Azul" se empareja con 10, y así sucesivamente. Entonces nosotros
podriamos usar el método `collect` para convertir ese vector de tuplas en un mapa hash,
como se muestra en el Listado 8-21.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

<span class="caption">Listado 8-21: Creación de un mapa hash a partir de una lista de equipos
y una lista de puntuaciones</span>

La anotación de tipo `HashMap <_, _>` se necesita aquí porque es posible
`collect` en muchas estructuras de datos diferentes y Rust no sabe cuál
quiere a menos que usted la especifique. Para los parámetros de los tipos de clave y valor,
sin embargo, usamos guiones bajos y Rust puede inferir los tipos que el mapa hash
contiene basado en los tipos de datos en los vectores. En el Listado 8-21,
el tipo de clave será `String` y el tipo de valor será `i32`, al igual que los tipos
en el Listado 8-20.

### Mapas hash y propiedad

Para los tipos que implementan el trait `Copy`, como `i32`, los valores se copian
en el mapa hash. Para valores propios como `String`, los valores se moverán y
el mapa hash será el propietario de esos valores, como se muestra en el Listado 8-22.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

<span class="caption">Listado 8-22: muestra que las claves y los valores son propiedad de
el mapa hash una vez que se insertan</span>

No podemos usar las variables `field_name` y `field_value` después de que
se han movido al mapa hash con la llamada a `insert`.

Si insertamos referencias a valores en el mapa hash, los valores no se moverán
en el mapa hash. Los valores a los que apuntan las referencias deben ser válidos
al menos mientras el mapa hash sea válido. Hablaremos más sobre estos problemas en
la sección ["Validación de referencias con Lifetimes”][validating-references-with-lifetimes]<!-- ignore --> en
Capítulo 10.

### Acceso a valores en un mapa hash

Podemos obtener un valor del mapa hash proporcionando su clave al método `get`.
como se muestra en el Listado 8-23.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

<span class="caption">Listado 8-23: Acceso a la puntuación del equipo azul
almacenado en el mapa hash</span>

Aquí, `score` tendrá el valor asociado con el equipo azul, y el
el resultado será `Some(&10)`. El resultado está envuelto en `Some` porque `get`
devuelve una `Option<&V>`; si no hay ningún valor para esa clave en el mapa hash,
`get` devolverá `None`. El programa deberá manejar la `Option` en una
de las formas que cubrimos en el Capítulo 6.

Podemos iterar sobre cada par clave/valor en un mapa hash de manera similar a
los vectores, usando un bucle `for`:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

Este código imprimirá cada par en un orden arbitrario:

```text
Yellow: 50
Blue: 10
```

### Actualización de un mapa hash

Aunque la cantidad de claves y valores se puede aumentar, cada clave solo puede tener un
valor asociado a la vez. Cuando quiera cambiar los datos en un hash
map, debe decidir cómo manejar el caso cuando una clave ya tiene un valor
asignado. Puede reemplazar el valor anterior con el nuevo valor, completamente
sin tener en cuenta el antiguo valor. Podrías mantener el valor anterior e ignorar el nuevo
valor, solo agregando el nuevo valor si la clave *no* ya tiene un valor. O
podría combinar el valor anterior y el nuevo valor. Veamos cómo hacer cada uno
de estos.

#### Sobrescribir un valor

Si insertamos una clave y un valor en un mapa hash y luego insertamos esa misma clave
con un valor diferente, se reemplazará el valor asociado con esa clave.
Aunque el código del Listado 8-24 llama a `insert` dos veces, el mapa hash
solo contienen un par clave/valor porque estamos insertando el valor de la
clave de equipo Blue en ambas ocasiones.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

<span class="caption">Listado 8-24: Reemplazo de un valor almacenado por una
clave</span>

Este código imprimirá `{"Blue": 25}`. El valor original de "10" ha sido
sobrescrito.

#### Insertar un valor solo si la clave no tiene ningún valor

Es común verificar si una clave en particular tiene un valor y, si no lo tiene,
insertarlo. Los mapas hash tienen una API especial para esto llamada `entry`
que toma la clave que desea verificar como parámetro. El valor de retorno del
método `entry` es una enumeración llamada `Entry` que representa un valor que podría o
no existir. Digamos que queremos comprobar si la clave para el equipo yellow
tiene un valor asociado. Si no es así, queremos insertar el valor 50,
y lo mismo para el equipo blue. Usando la API `entry`, el código se ve como en el
Listado 8-25.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

<span class="caption">Listado 8-25: usando el método `entry` para insertar solo si
la clave aún no tiene un valor</span>

El método `or_insert` en `Entry` está definido para devolver una referencia mutable al
valor de la clave `Entry` correspondiente si esa clave existe, y si no,
inserta el parámetro como el nuevo valor para esta clave y devuelve una
referencia mutable al nuevo valor. Esta técnica es mucho más limpia que escribir
la lógica nosotros mismos y, además, juega mejor con el verificador de préstamos.

Al ejecutar el código del Listado 8-25 se imprimirá `{"Yellow": 50,"Blue": 10}`. La
primera llamada a `entry` insertará la clave para el equipo amarillo con el valor
50 porque el equipo amarillo aún no tiene valor. La segunda llamada a
`entry` no cambiará el mapa hash porque el equipo azul ya tiene el
valor 10.

#### Actualización de un valor basado en el valor anterior

Otro caso de uso común de los mapas hash es buscar el valor de una clave y luego
actualícelo según el valor anterior. Por ejemplo, el Listado 8-26 muestra el código que
cuenta cuántas veces aparece cada palabra en algún texto. Usamos un mapa hash con
las palabras como claves e incrementamos el valor para realizar un seguimiento de cuántas veces hemos
visto esa palabra. Si es la primera vez que vemos una palabra, primero insertaremos
el valor 0.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-26/src/main.rs:here}}
```

<span class="caption">Listado 8-26: Contar ocurrencias de palabras usando un hash
map que almacena palabras y conteo</span>

Este código imprimirá `{"mundo": 2,"hola": 1,"maravilloso": 1}`.
El método `or_insert` en realidad devuelve una referencia mutable (`&mut V`) al valor
para esta clave. Aquí almacenamos esa referencia mutable en la variable `count`, por lo que
para asignar ese valor, primero debemos eliminar la referencia de `count` usando el
asterisco (`*`). La referencia mutable sale del alcance al final del
bucle `for`, por lo que todos estos cambios son seguros y están permitidos por las reglas de préstamo.

### Funciones hash

Por defecto, `HashMap` usa una función hash llamada SipHash que puede proporcionar
resistencia a los ataques de denegación de servicio (DoS) que involucran tablas hash [^siphash]. Esto
no es el algoritmo hash más rápido disponible, pero la compensación para mejorar
la seguridad que viene con la caída en el rendimiento vale la pena. Si
descubre que la función hash predeterminada es demasiado lenta para su
propósitos, puede cambiar a otra función especificando un *hasher*.
Un hasher es un tipo que implementa el `BuildHasher`.
Hablaremos sobre los traits y cómo implementarlos en el Capítulo 10. No
necesariamente tienes que implementar tu propio hasher desde cero;
[crates.io](https://crates.io/) tiene bibliotecas compartidas por otros usuarios de Rust que
proporcionan hash que implementan muchos algoritmos de hash comunes.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

## Resumen

Vectores, cadenas y mapas hash proporcionarán una gran cantidad de funcionalidad
necesaria en los programas cuando necesita almacenar, acceder y modificar datos. Aquí están
algunos ejercicios que ahora debería estar preparado para resolver:

* Dada una lista de números enteros, use un vector y devuelva la media (el valor promedio),
  mediana (cuando se ordena, el valor en la posición media) y moda (el
  valor que ocurre con mayor frecuencia; aquí será útil un mapa hash) de la lista.
* Convertir cuerdas a cerdo latino. La primera consonante de cada palabra se mueve a
  se agrega el final de la palabra y "ay", por lo que "first" se convierte en "first-fay". Palabras
  que comienzan con una vocal tienen "hay" agregado al final en su lugar ("manzana" se convierte en
  "Heno de manzana"). ¡Tenga en cuenta los detalles sobre la codificación UTF-8!
* Usando un mapa hash y vectores, cree una interfaz de texto para permitir que un usuario agregue
  nombres de los empleados a un departamento de una empresa. Por ejemplo, "Agrega Sally a
  Ingeniería ”o“ Agregar Amir a Ventas ”. Luego, deje que el usuario recupere una lista de todos
  personas de un departamento o todas las personas de la empresa por departamento, ordenadas
  alfabéticamente.

La documentación de la API de la biblioteca estándar describe métodos para vectores, cadenas,
y mapas hash que serán útiles para estos ejercicios.

Estamos entrando en programas más complejos en los que las operaciones pueden fallar, por lo que es
un momento perfecto para discutir el manejo de errores. ¡Lo haremos a continuación!

[iteradores]: ch13-02-iterators.html
[[validating-references-with-lifetimes]]: ch10-03-life-syntax.html#validando-referencias-con-lifetimes

