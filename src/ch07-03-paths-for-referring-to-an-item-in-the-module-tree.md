## Rutas para Hacer Referencia a un Elemento en el Árbol de Módulos

Para mostrar a Rust dónde encontrar un elemento en un árbol de módulos, usamos una ruta en la misma
forma en que usamos una ruta cuando navegamos por un sistema de archivos. Si queremos llamar a una función,
necesitamos conocer su ruta.

Una ruta puede tomar dos formas:

* *ruta absoluta* que comienza desde la raíz de una caja usando un nombre de caja o un
  literal `crate`.
* *ruta relativa* que comienza desde el módulo actual y usa `self`, `super` o
  un identificador en el módulo actual.

Tanto las rutas absolutas como las relativas son seguidas por uno o más identificadores
separados por dos puntos dobles (`::`).

Regresemos al ejemplo del Listado 7-1. ¿Cómo llamamos a la
función `poner_en_espera`? Esto es lo mismo que preguntar, ¿cuál es el camino de la
función `poner_en_espera`? En el Listado 7-3 simplificamos nuestro código un poco al
eliminar algunos de los módulos y funciones. Mostraremos dos formas de llamar a la
función `poner_en_espera` desde una nueva función `comer_en_restaurant` definida en
la caja raiz. La función `comer_en_restaurant` es parte de nuestra API pública de la caja biblioteca,
por lo que la marcamos con la palabra clave `pub`. En la 
sección [”Exposición de rutas con la palabra clave `pub`”][pub]<!-- ignore -->, entraremos en más detalles
sobre `pub`. Tenga en cuenta que este ejemplo no se compilará todavía; explicaremos por qué
en breve.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

<span class="caption">Listado 7-3: Llamar a la función `poner_en_espera` usando
rutas absolutas y relativas</span>

La primera vez que llamamos a la función `poner_en_espera` en `comer_en_restaurant`,
usamos una ruta absoluta. La función `poner_en_espera` se define en el mismo
crate que `comer_en_restaurant`, lo que significa que podemos usar la palabra clave `crate` para
iniciar un camino absoluto.

Después de `crate`, incluimos cada uno de los módulos sucesivos hasta que nos abrimos paso
a `poner_en_espera`. Puede imaginar un sistema de archivos con la misma estructura y
especificaríamos la ruta `/fachada/recepcion/poner_en_espera` para ejecutar el
programa `poner_en_espera`; usar el nombre de la caja para comenzar desde la raíz de la caja
es como usar `/` para comenzar desde la raíz del sistema de archivos en su shell.

La segunda vez que llamamos a `poner_en_espera` en `comer_en_restaurant`, usamos un
camino relativo. La ruta comienza con `fachada`, el nombre del módulo
definido en el mismo nivel del árbol de módulos que `comer_en_restaurant`. Aquí
el equivalente del sistema de archivos estaría usando la ruta
`fachada/recepcion/poner_en_espera`. Comenzar con un nombre significa que el
camino es relativo.

Elegir si usar una ruta relativa o absoluta es una decisión
basada en su proyecto. La decisión debe depender de si es más probable
mover el código de definición de un item por separado o junto con el código que
usa el item. Por ejemplo, si movemos el módulo `fachada` y la
función `comer_en_restaurant` en un módulo llamado `experiencia_cliente`,
necesita actualizar la ruta absoluta a `poner_en_espera`, pero la ruta relativa
todavía sería válida. Sin embargo, si movimos la función `comer_en_restaurant`
por separado en un módulo llamado `comida`, la ruta absoluta a la
la llamada `poner_en_espera` permanecería igual, pero la ruta relativa debería
ser actualizada. Nuestra preferencia es especificar rutas absolutas porque es más
probable que se muevan definiciones de código y llamadas de elementos de forma independiente entre sí.

¡Intentemos compilar el Listado 7-3 y descubramos por qué no se compilará todavía!
El error que obtenemos se muestra en el Listado 7-4.


```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

<span class="caption">Listado 7-4: Errores del compilador al compilar el código en
Listado 7-3</span>

Los mensajes de error dicen que el módulo `recepcion` es privado. En otras palabras,
tenemos las rutas correctas para el módulo `recepcion` y la función `poner_en_espera`,
pero Rust no nos deja usarlos porque no tiene acceso a la secciones privadas.

Los módulos no son útiles solo para organizar su código. También definen el
*límite de privacidad* en Rust; la línea que encapsula los detalles de implementación
del código externo que no puede conocer, llamar ni confiar. Por tanto, si quiere
hacer que un elemento como una función o estructura sea privado, pongalo en un módulo.

La forma en que funciona la privacidad en Rust es que todos los elementos (funciones, métodos, estructuras,
enumeraciones, módulos y constantes) son privados de forma predeterminada. Los elementos de un módulo principal
no puede usar los elementos privados dentro de los módulos secundarios, pero los elementos de los módulos secundarios
puede usar los elementos en sus módulos superiores. La razón es que los módulos secundarios
envuelven y ocultan sus detalles de implementación, pero los módulos secundarios pueden ver el
contexto en el que se definen. Para continuar con la metáfora del restaurante,
piense en las reglas de privacidad como si fueran la oficina del restaurante; lo que hay
allí es privado para los clientes del restaurante, pero los gerentes pueden
ver y hacer todo en el restaurante en el que operan.

Se eligió que el sistema de módulos funcionara de esta manera para que el interior de
los detalles de implementación sean los predeterminados. De esa forma, sabrá qué partes del
código interno puede cambiar sin romper el código externo. Pero puede exponer el interior
del código de los módulos secundarios a los módulos superiores externos mediante el uso de
palabra clave `pub` para hacer público un item.

### Exposición de Rutas con la Palabra Clave `pub`

Regresemos al error en el Listado 7-4 que nos dijo que el módulo `recepcion` es
privado. Queremos que la función `comer_en_restaurant` en el módulo padre tenga
acceso a la función `poner_en_espera` en el módulo hijo, por lo que marcamos el
módulo `recepcion` con la palabra clave `pub`, como se muestra en el Listado 7-5.


<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs}}
```

<span class="caption">Listado 7-5: Declarar el módulo `recepcion` como `pub` para
utilízarlo desde `comer_en_restaurant`</span>

Desafortunadamente, el código del Listado 7-5 todavía da como resultado un error, como se muestra en
Listado 7-6.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

<span class="caption">Listado 7-6: Errores del compilador al compilar el código en
Listado 7-5</span>

¿Qué sucedió? Agregar la palabra clave `pub` delante de `mod recepcion` hace el
módulo público. Con este cambio, al poder acceder a `fachada`, podemos
acceder a `recepcion`. Pero el *contenido* de `recepcion` sigue siendo privado; hacer
el módulo público no hace que su contenido sea público. La palabra clave `pub` en un módulo
solo permite que se refiera a él el código en sus módulos superiores.

Los errores del Listado 7-6 dicen que la función `poner_en_espera` es privada.
Las reglas de privacidad se aplican a estructuras, enumeraciones, funciones y métodos, así como
a módulos.

Hagamos también pública la función `poner_en_espera` agregando la
palabra clave `pub` antes de su definición, como en el Listado 7-7.

<span class="filename">Nombre de archivo: src/lib.rs </span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs}}
```

<span class="caption">Listado 7-7: Adición de la palabra clave `pub` a `mod recepcion`
y `fn poner_en_espera` nos permite llamar a la función desde `comer_en_restaurant`</span>

¡Ahora el código se compilará! Veamos la ruta absoluta y relativa y
vuelva a comprobar por qué agregar la palabra clave `pub` nos permite usar estas rutas en
`poner_en_espera` con respecto a las reglas de privacidad.

En la ruta absoluta comenzamos con `crate`, la raíz del árbol de módulos de nuestro crate.
Luego, el módulo `fachada` se define en la raíz de la caja.
El módulo `fachada` no es público, pero como la función `comer_en_restaurant`
se define en el mismo módulo que `fachada` (es decir,
`comer_en_restaurant` y `fachada` son hermanos), podemos referirnos a
`fachada` desde `comer_en_restaurant`. El siguiente es el módulo `recepcion` marcado
con `pub`. Podemos acceder al módulo padre de `recepcion`, por lo que podemos acceder a
`recepcion`. Finalmente, la función `poner_en_espera` está marcada con` pub` y se
puede acceder a su módulo principal, por lo que esta llamada a función funciona.

En la ruta relativa, la lógica es la misma que la ruta absoluta excepto el
primer paso; en lugar de comenzar desde la raíz de la caja, el camino comienza desde
`fachada`. El módulo `fachada` se define dentro del mismo módulo
que `comer_en_restaurant`, por lo que la ruta relativa a partir del módulo en el que
`comer_en_restaurant` se define funciona. Entonces, debido a que `recepcion` y
`poner_en_espera` están marcados con `pub`, el resto de la ruta funciona, y esta
llamada a la función es válida!

### Iniciando rutas relativas con `super`

También podemos construir rutas relativas que comiencen en el módulo principal usando
`super` al comienzo del path. Esto es como iniciar una ruta de sistema de archivos con
la sintaxis `..`. ¿Por qué querríamos hacer esto?

Considere el código del Listado 7-8 que modela la situación en la que un chef
corrige un pedido incorrecto y se lo presenta personalmente al cliente. La
function `correjir_comanda` llama a la función `servir_comanda` especificando
la ruta a `servir_comanda` comenzando con `super`:

<span class="filename">Nombre de archivo: src/lib.rs </span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

<span class="caption">Listado 7-8: Llamar a una función usando una ruta relativa
comenzando con `super`</span>

La función `correjir_comanda` está en el módulo `parte_de_atras`, por lo que podemos
usar `super` para ir al módulo principal de `parte_de_atras`, que en este caso
es `crate`, la raíz. A partir de ahí, buscamos `servir_comanda` y lo encontramos con
exito. Como pensamos que el módulo `parte_de_atras` y la función` servir_comanda`
permanecen en la misma relación y se muevan juntos
si decidimos reorganizar el árbol de módulos de la caja. Por lo tanto, usamos
`super`, por lo que tendremos menos lugares para actualizar el código en el futuro si este código
se mueve a un módulo diferente.

### Hacer publicas las estructuras y enumeraciones

También podemos usar `pub` para designar estructuras y enumeraciones como públicas, pero hay
algunos detalles extra. Si usamos `pub` antes de una definición de estructura, hacemos la
struct pública, pero los campos de la estructura seguirán siendo privados. Podemos hacer cada
campo público o no en base a caso por caso. En el Listado 7-9, definimos una
estructura pública `parte_de_atras::Desayuno` con un campo público `tostada` pero un
campo privado `fruta_de_temporada`. Esto modela el caso en un restaurante donde
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

Debido a que el campo `tostada` en la estructura `parte_de_atras::Desayuno` es público,
en `comer_en_restaurant` podemos escribir y leer en el campo `tostada` usando la
notación punto. Tenga en cuenta que no podemos usar el campo `fruta_de_temporada` en
`comer_en_restaurant` porque `Desayuno` es privado. Intente descomentar la linea
modificando el valor del campo `fruta_de_temporada` para ver qué error obtiene!

Además, tenga en cuenta que debido a que `parte_de_atras::Desayuno` tiene un campo privado,
la estructura debe proporcionar una función pública asociada que construya una
instancia de `Desayuno` (aquí lo llamamos `verano`). Si `Desayuno` no
tiene una función de este tipo, no podríamos crear una instancia de `Desayuno` en
`comer_en_restaurant` porque no pudimos establecer el valor del campo privado
`Desayuno` en `comer_en_restaurant`.

Por el contrario, si hacemos pública una enumeración, todas sus variantes son públicas.
Solo necesita la palabra clave `pub` antes de la palabra clave `enum`, como se muestra en el Listado 7-10.

<span class="filename">Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

<span class="caption">Listado 7-10: Designar una enumeración como pública hace que todos sus
variantes sean públicas</span>

Debido a que hicimos pública la enumeración `Aperitivo`, podemos usar las variantes `Sopa` y `Ensalada`
en `comer_en_restaurant`. Las enumeraciones no son muy útiles a menos que sus variantes
sean públicas; sería molesto tener que anotar todas las variantes de enumeración con
`pub` en todos los casos, por lo que el valor predeterminado para las variantes de enumeración es público. Las estructuras
a menudo son útiles sin que sus campos sean públicos, por lo que los campos de estructura siguen las
regla general de que todo sea privado de forma predeterminada a menos que se anote con `pub`.

Hay una situación más relacionada con `pub` que no hemos cubierto, y es
nuestra última característica del sistema de módulos; la palabra clave `use`. Cubriremos `use` por sí solo
primero, y luego mostraremos cómo combinar `pub` y `use`.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposición-de-rutas-con-la-palabra-clave-pub

