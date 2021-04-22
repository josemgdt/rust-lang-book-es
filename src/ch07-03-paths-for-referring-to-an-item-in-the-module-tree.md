## Rutas para hacer referencia a un elemento en el arbol del modulo

Para mostrar a Rust dónde encontrar un elemento en un árbol de módulos, usamos una ruta en la misma
forma en que usamos una ruta cuando navegamos por un sistema de archivos. Si queremos llamar a una función,
necesitamos conocer su camino.

Un camino puede tomar dos formas:

* Una *ruta absoluta* que comienza desde la raíz de una caja usando un nombre de caja o un
  literal `crate`.
* Una *ruta relativa* que comienza desde el módulo actual y usa `self`, `super` o
  un identificador en el módulo actual.

Tanto las rutas absolutas como las relativas son seguidas por uno o más identificadores
separados por dos puntos dobles (`::`).

Regresemos al ejemplo del Listado 7-1. ¿Cómo llamamos a la
función `add_to_waitlist`? Esto es lo mismo que preguntar, ¿cuál es el camino de la
función `add_to_waitlist`? En el Listado 7-3 simplificamos nuestro código un poco al
eliminar algunos de los módulos y funciones. Mostraremos dos formas de llamar a la
función `add_to_waitlist` desde una nueva función `eat_at_restaurant` definida en
la caja raiz. La función `eat_at_restaurant` es parte de nuestra API pública de la caja biblioteca,
por lo que la marcamos con la palabra clave `pub`. En la sección [”Exponiendo rutas con
`pub`”]​​[pub] <!-- ignore -->, entraremos en más detalles
sobre `pub`. Tenga en cuenta que este ejemplo no se compilará todavía; explicaremos por qué
en breve.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

<span class="caption">Listado 7-3: Llamar a la función `add_to_waitlist` usando
rutas absolutas y relativas</span>

La primera vez que llamamos a la función `add_to_waitlist` en `eat_at_restaurant`,
usamos una ruta absoluta. La función `add_to_waitlist` se define en el mismo
crate que `eat_at_restaurant`, lo que significa que podemos usar la palabra clave `crate` para
iniciar un camino absoluto.

Después de `crate`, incluimos cada uno de los módulos sucesivos hasta que nos abrimos paso
a `add_to_waitlist`. Puede imaginar un sistema de archivos con la misma estructura y
especificaríamos la ruta `/front_of_house/hosting/add_to_waitlist` para ejecutar el
programa `add_to_waitlist`; usar el nombre de la caja para comenzar desde la raíz de la caja
es como usar `/` para comenzar desde la raíz del sistema de archivos en su shell.

La segunda vez que llamamos a `add_to_waitlist` en `eat_at_restaurant`, usamos un
camino relativo. La ruta comienza con `front_of_house`, el nombre del módulo
definido en el mismo nivel del árbol de módulos que `eat_at_restaurant`. Aquí
el equivalente del sistema de archivos estaría usando la ruta
`front_of_house/hosting/add_to_waitlist`. Comenzar con un nombre significa que el
camino es relativo.

Elegir si usar una ruta relativa o absoluta es una decisión
basada en su proyecto. La decisión debe depender de si es más probable
mover el código de definición de un item por separado o junto con el código que
usa el item. Por ejemplo, si movemos el módulo `front_of_house` y la
función `eat_at_restaurant` en un módulo llamado `customer_experience`,
necesita actualizar la ruta absoluta a `add_to_waitlist`, pero la ruta relativa
todavía sería válida. Sin embargo, si movimos la función `eat_at_restaurant`
por separado en un módulo llamado `dining`, la ruta absoluta a la
la llamada `add_to_waitlist` permanecería igual, pero la ruta relativa debería
ser actualizada. Nuestra preferencia es especificar rutas absolutas porque es más
probable que se muevan definiciones de código y llamadas de elementos de forma independiente entre sí.

¡Intentemos compilar el Listado 7-3 y descubramos por qué no se compilará todavía!
El error que obtenemos se muestra en el Listado 7-4.


```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

<span class="caption">Listado 7-4: Errores del compilador al compilar el código en
Listado 7-3</span>

Los mensajes de error dicen que el módulo `hosting` es privado. En otras palabras,
tenemos las rutas correctas para el módulo `hosting` y la función `add_to_waitlist`,
pero Rust no nos deja usarlos porque no tiene acceso a la secciones privadas.

Los módulos no son útiles solo para organizar su código. También definen el
*límite de privacidad* en Rust; la línea que encapsula los detalles de implementación
del código externo que no puede conocer, llamar ni confiar. Por tanto, si quiere
hacer que un elemento como una función o estructura sea privado, pongalo en un módulo.

La forma en que funciona la privacidad en Rust es que todos los elementos (funciones, métodos, estructuras,
enumeraciones, módulos y constantes) son privados de forma predeterminada. Los elementos de un módulo principal
no puede usar los elementos privados dentro de los módulos secundarios, pero los elementos de los módulos secundarios
puede usar los elementos en sus módulos ancestros. La razón es que los módulos secundarios
envuelven y ocultan sus detalles de implementación, pero los módulos secundarios pueden ver el
contexto en el que se definen. Para continuar con la metáfora del restaurante,
piense en las reglas de privacidad como si fueran la oficina de un restaurante; lo que hay
allí es privado para los clientes del restaurante, pero los gerentes pueden
ver y hacer todo en el restaurante en el que operan.

Se eligió que el sistema de módulos funcionara de esta manera para que el interior de
los detalles de implementación sean los predeterminados. De esa forma, sabrá qué partes del
código interno puede cambiar sin romper el código externo. Pero puede exponer el interior
del código de los módulos secundarios a los módulos ancestros externos mediante el uso de
palabra clave `pub` para hacer público un item.

### Exposicion de rutas con la palabra clave `pub`

Regresemos al error en el Listado 7-4 que nos dijo que el módulo `hosting` es
privado. Queremos que la función `eat_at_restaurant` en el módulo padre tenga
acceso a la función `add_to_waitlist` en el módulo hijo, por lo que marcamos el
módulo `hosting` con la palabra clave `pub`, como se muestra en el Listado 7-5.


<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs}}
```

<span class="caption">Listado 7-5: Declarar el módulo `hosting` como `pub` para
utilízarlo desde `eat_at_restaurant`</span>

Desafortunadamente, el código del Listado 7-5 todavía da como resultado un error, como se muestra en
Listado 7-6.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

<span class="caption">Listado 7-6: Errores del compilador al compilar el código en
Listado 7-5</span>

¿Qué sucedió? Agregar la palabra clave `pub` delante de `mod hosting` hace el
módulo público. Con este cambio, al poder acceder a `front_of_house`, podemos
acceder a `hosting`. Pero el *contenido* de `hosting` sigue siendo privado; haciendo el
el módulo público no hace que su contenido sea público. La palabra clave `pub` en un módulo
solo permite que se refiera a él el código en sus módulos ancestros.

Los errores del Listado 7-6 dicen que la función `add_to_waitlist` es privada.
Las reglas de privacidad se aplican a estructuras, enumeraciones, funciones y métodos, así como
módulos.

Hagamos también pública la función `add_to_waitlist` agregando la
palabra clave `pub` antes de su definición, como en el Listado 7-7.

<span class="filename">Nombre de archivo: src/lib.rs </span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs}}
```

<span class="caption">Listado 7-7: Adición de la palabra clave `pub` a `mod hosting`
y `fn add_to_waitlist` nos permite llamar a la función desde `eat_at_restaurant`</span>

¡Ahora el código se compilará! Veamos la ruta absoluta y relativa y
vuelva a comprobar por qué agregar la palabra clave `pub` nos permite usar estas rutas en
`add_to_waitlist` con respecto a las reglas de privacidad.

En la ruta absoluta, comenzamos con `crate`, la raíz del árbol de módulos de nuestro crate.
Luego, el módulo `front_of_house` se define en la raíz de la caja.
El módulo `front_of_house` no es público, pero como la función `eat_at_restaurant`
se define en el mismo módulo que `front_of_house` (es decir,
`eat_at_restaurant` y `front_of_house` son hermanos), podemos referirnos a
`front_of_house` desde `eat_at_restaurant`. El siguiente es el módulo `hosting` marcado
con `pub`. Podemos acceder al módulo padre de `hosting`, por lo que podemos acceder a
`hosting`. Finalmente, la función `add_to_waitlist` está marcada con` pub` y se
puede acceder a su módulo principal, por lo que esta llamada a función funciona.

En la ruta relativa, la lógica es la misma que la ruta absoluta excepto el
primer paso; en lugar de comenzar desde la raíz de la caja, el camino comienza desde
`front_of_house`. El módulo `front_of_house` se define dentro del mismo módulo
que `eat_at_restaurant`, por lo que la ruta relativa a partir del módulo en el que
`eat_at_restaurant` se define funciona. Entonces, debido a que `hosting` y
`add_to_waitlist` están marcados con `pub`, el resto de la ruta funciona, y esta
llamada a la función es válida!

### Iniciando rutas relativas con `super`

También podemos construir rutas relativas que comiencen en el módulo principal usando
`super` al comienzo del camino. Esto es como iniciar una ruta de sistema de archivos con
la sintaxis `..`. ¿Por qué querríamos hacer esto?

Considere el código del Listado 7-8 que modela la situación en la que un chef
corrige un pedido incorrecto y se lo presenta personalmente al cliente. La
function `fix_incorrect_order` llama a la función `serve_order` especificando
la ruta a `serve_order` comenzando con` super`:

<span class="filename">Nombre de archivo: src/lib.rs </span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

<span class="caption">Listado 7-8: Llamar a una función usando una ruta relativa
comenzando con `super`</span>

La función `fix_incorrect_order` está en el módulo `back_of_house`, por lo que podemos
usar `super` para ir al módulo principal de `back_of_house`, que en este caso
es `crate`, la raíz. A partir de ahí, buscamos `serve_order` y lo encontramos con
exito. Como pensamos que el módulo `back_of_house` y la función` serve_order`
permanecen en la misma relación y se muevan juntos
si decidimos reorganizar el árbol de módulos de la caja. Por lo tanto, usamos
`super`, por lo que tendremos menos lugares para actualizar el código en el futuro si este código
se mueve a un módulo diferente.

### Hacer publicas las estructuras y enumeraciones

También podemos usar `pub` para designar estructuras y enumeraciones como públicas, pero hay
algunos detalles extra. Si usamos `pub` antes de una definición de estructura, hacemos la
struct pública, pero los campos de la estructura seguirán siendo privados. Podemos hacer cada
campo público o no en base a caso por caso. En el Listado 7-9, definimos una
estructura pública `back_of_house::Breakfast` con un campo público `toast` pero un
campo privado `seasonal_fruit`. Esto modela el caso en un restaurante donde
el cliente puede elegir el tipo de pan que viene con la comida, pero el chef
decide qué fruta acompaña la comida en función de la temporada y la
existencias. La fruta disponible cambia rápidamente, por lo que los clientes no pueden elegir la fruta.
o incluso ver qué fruta les serviran.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

<span class="caption">Listado 7-9: una estructura con algunos campos públicos y algunos
campos privados</span>

Debido a que el campo `toast` en la estructura `back_of_house::Breakfast` es público,
en `eat_at_restaurant` podemos escribir y leer en el campo `toast` usando la
notación punto. Tenga en cuenta que no podemos usar el campo `seasonal_fruit` en
`eat_at_restaurant` porque `Breakfast` es privado. Intente descomentar la linea
modificando el valor del campo `seasonal_fruit` para ver qué error obtiene!

Además, tenga en cuenta que debido a que `back_of_house::Breakfast` tiene un campo privado,
la estructura debe proporcionar una función pública asociada que construya una
instancia de `Breakfast` (aquí lo llamamos `summer`). Si `Breakfast` no
tiene una función de este tipo, no podríamos crear una instancia de `Breakfast` en
`eat_at_restaurant` porque no pudimos establecer el valor del campo privado
`Breakfast` en` eat_at_restaurant`.

Por el contrario, si hacemos pública una enumeración, todas sus variantes son públicas.
Solo necesita la palabra clave `pub` antes de la palabra clave `enum`, como se muestra en el Listado 7-10.

<span class="filename">Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

<span class="caption">Listado 7-10: Designar una enumeración como pública hace que todos sus
variantes sean públicas</span>

Debido a que hicimos pública la enumeración `Appetizer`, podemos usar las variantes `Soup` y `Salad`
en `eat_at_restaurant`. Las enumeraciones no son muy útiles a menos que sus variantes
sean públicas; sería molesto tener que anotar todas las variantes de enumeración con
`pub` en todos los casos, por lo que el valor predeterminado para las variantes de enumeración es público. Las estructuras
a menudo son útiles sin que sus campos sean públicos, por lo que los campos de estructura siguen las
regla general de que todo sea privado de forma predeterminada a menos que se anote con `pub`.

Hay una situación más relacionada con "pub" que no hemos cubierto, y es
nuestra última característica del sistema de módulos; la palabra clave `use`. Cubriremos el `use` por sí solo
primero, y luego mostraremos cómo combinar `pub` y `use`.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposicion-de-rutas-con-la-palabra-clave-pub

