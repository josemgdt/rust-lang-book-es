## Sintaxis de Metodos

*Los métodos* son similares a las funciones; se declaran con la palabra clave `fn`, 
su nombre pueden tener parámetros y un valor de retorno y contienen algun
código que se ejecuta cuando se les llama desde otro lugar. Sin embargo, los métodos son
diferente de las funciones en que se definen dentro del contexto de una estructura
(o una enumeración o un objeto trait, que cubrimos en los Capítulos 6 y 17,
respectivamente), y su primer parámetro es siempre `self`, que representa la
instancia de la estructura en la que se está llamando al método.

### Definicion de Metodos

Cambiemos la función `area` que tiene una instancia `Rectangle` como parámetro
y en su lugar creemos un método `area` definido en la estructura `Rectangle`, como se muestra
en el Listado 5-13.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<span class="caption">Listado 5-13: Definición del método `area` en la
estructura `Rectangle`</span>

Para definir la función dentro del contexto de `Rectangle`, comenzamos un bloque `impl`
(implementación). Luego movemos la función `area` dentro de los corchetes de `impl`
 y cambiamos el primer (y en este caso, el único) parámetro para que sea
`self` en la declaracion y en todas partes dentro del cuerpo. En `main`, donde
llamamos a la función `area` y pasamos `rect1` como argumento, podemos
usas *sintaxis de método* para llamar al método `area` en nuestra instancia de `Rectangle`.
La sintaxis de método va después de una instancia; agregamos un punto, seguido del nombre del método,
paréntesis y cualquier argumento.

En la definicion de `area`, usamos `&self` en lugar de `rectangle: &Rectangle`
porque Rust sabe que el tipo de `self` es `Rectangle` debido a que este método está
dentro del contexto `impl Rectangle`. Tenga en cuenta que todavía necesitamos usar `&`
antes de `self`, tal como hicimos en `&Rectangle`. Los métodos pueden apropiarse de
`self`, pedir prestado `self` inmutablemente como lo hemos hecho aquí, o tomar prestado `self` mutable,
al igual que cualquier otro parámetro.

Hemos elegido `&self` aquí por la misma razón que usamos `&Rectangle` en la
versión de función; no queremos apropiarnos, solo queremos leer los
datos en la estructura, no escribirlos. Si quisiéramos cambiar la instancia desde la que
hemos llamado al método como parte de lo que hace el propio método, usaremos `&mut self`
como primer parámetro. Tener un método que se apropie de la
instancia usando solo `self` como primer parámetro es poco común; esta técnica se usa
generalmente cuando el método transforma `self` en otra cosa y queremos
evitar que el llamante utilice la instancia original después de la transformación.

El principal beneficio de usar métodos en lugar de funciones, además de usar
sintaxis de método y no tener que repetir el tipo de `self` en cada definicion de método,
es la organización. Hemos puesto todas las cosas que podemos hacer con una
instancia de un tipo en un bloque `impl` en lugar de hacer que futuros usuarios de nuestro
de código tengan que buscar las capacidades de `Rectangle` en varios lugares de la biblioteca que
proveemos.

> ### Donde esta el operador `->`?
>
> En C y C ++, se usan dos operadores diferentes para llamar a los métodos; usted usará
> `.` si está llamando a un método directamente en el objeto y `->` si está
> llamando al método con un puntero al objeto y es necesario eliminar primero la referencia al
> puntero. En otras palabras, si `objeto` es un puntero,
> `objeto->algo()` es similar a `(*objeto).algo()`.
>
> Rust no tiene un equivalente al operador `->`; en cambio, Rust tiene una
> función llamada *referenciación y desreferenciación automáticas*. Las llamadas a métodos son
> uno de los pocos lugares en Rust que tiene este comportamiento.
>
> Así es como funciona; cuando llama a un método con `object.something()`, Rust
> agrega automáticamente `&`, `&mut`, o `*` para que `object` coincida con la declaracion del
> método. En otras palabras, lo siguiente es lo mismo:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> El primero se ve mucho más limpio. Este comportamiento de referencia automático funciona
> porque los métodos tienen un receptor claro: el tipo `self`. Dado el receptor
> y el nombre de un método, Rust puede determinar definitivamente si el método es
> leer (`&self`), mutar (` &mut self`) o consumir (`self`). El hecho de
> que Rust hace implícito el préstamo para los receptores de métodos es una gran parte de
> hacer que la propiedad sea ergonómica en la práctica.

### Metodos con mas Parametros

Practiquemos el uso de métodos implementando un segundo método en la
estructura `Rectangle`. Esta vez, queremos que una instancia de `Rectangle` tome otra instancia
de `Rectangle` y devuelva `true` si el segundo `Rectangle` puede caber completamente
dentro de `self`; de lo contrario, debería devolver `false`. Es decir, queremos poder
escribir el programa que se muestra en el Listado 5-14, una vez que hayamos definido el 
método `can_hold`.


<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<span class="caption">Listing 5-14: Listado 5-14: Uso del método `can_hold` aún no escrito</span>

Y la salida esperada sería similar a la siguiente, porque ambas dimensiones
de `rect2` son más pequeñas que las dimensiones de `rect1` pero `rect3` es más ancho que
`rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```
Sabemos que queremos definir un método, por lo que estará dentro del bloque `impl Rectangle`.
El nombre del método será `can_hold`, y tomará un préstamo inmutable
de otro `Rectangle` como parámetro. Podemos decir de cuál tipo será el
parámetro mirando el código que llama al método:
`rect1.can_hold(&rect2)` pasado en `&rect2`, que es un préstamo inmutable para
`rect2`, una instancia de `Rectangle`. Esto tiene sentido porque solo necesitamos
leer `rect2` (en lugar de escribir, lo que significaría que necesitaríamos un préstamo mutable),
y queremos que `main` conserve la propiedad de `rect2` para que podamos usarlo de nuevo después
de la llamada al método `can_hold`. El valor de retorno de `can_hold` será un
booleano, y la implementación comprobará si el ancho y el alto de
`self` son mayores que el ancho y la altura del otro `Rectangle`,
respectivamente. Agreguemos el nuevo método `can_hold` al bloque `impl` de
Listado 5-13, que se muestra en el Listado 5-15.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<span class="caption">Listado 5-15: Implementación del método `can_hold` en
`Rectangle` que toma otra instancia de `Rectangle` como parámetro </span>

Cuando ejecutamos este código con la función `main` en el Listado 5-14, obtendremos nuestra
salida deseada. Los métodos pueden tomar múltiples parámetros que agregamos a la
declaracion después del parámetro `self`, y esos parámetros funcionan como
parámetros en funciones.

### Funciones asociadas

Otra característica útil de los bloques `impl` es que podemos definir
funciones dentro de bloques `impl` que *no* toman `self` como parámetro. Estas
se denominan *funciones asociadas* porque están asociadas con la estructura.
Siguen siendo funciones, no métodos, porque no tienen una instancia de
la estructura con la que trabajar. Ya usó la función asociada `String::from`.

Las funciones asociadas se utilizan a menudo para constructores que devolverán una nueva
instancia de la estructura. Por ejemplo, podríamos proporcionar una función asociada
que tendría un parámetro de dimensión y lo usaría como ancho y alto,
lo que facilita la creación de un `Rectangle` cuadrado en lugar de tener que
especifique el mismo valor dos veces:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

Para llamar a esta función asociada, usamos la sintaxis `::` con el nombre de la estructura;
`let sq = Rectangle::square(3);` es un ejemplo. Esta función tiene el espacio de nombres por
la estructura: la sintaxis `::` se usa tanto para funciones asociadas como para
espacios de nombres creados por módulos. Discutiremos los módulos en el Capítulo 7.

### Bloques `impl` Multiples

Cada estructura puede tener bloques `impl` múltiples. Por ejemplo, el Listado
5-15 es equivalente al código que se muestra en el Listado 5-16, que tiene cada método
en su propio bloque `impl`.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<span class="caption">Listado 5-16: Reescritura del Listado 5-15 usando múltiples bloques `impl`</span>

No hay razón para separar estos métodos en varios bloques `impl` aquí,
pero esta es una sintaxis válida. Veremos un caso en el que varios bloques `impl` son
útiles en el Capítulo 10, donde discutimos tipos y traits genéricos.

## Resumen

Las estructuras le permiten crear tipos personalizados que son significativos para su dominio.
Utilizando estructuras, puede mantener los datos asociados conectados entre sí
y nombrar cada pieza para que su código sea claro. Los métodos le permiten especificar el
comportamiento que tienen las instancias de sus estructuras, y las funciones asociadas le permiten
funcionalidad de espacio de nombres que es particular para su estructura sin tener una
instancia disponible.

Pero las estructuras no son la única forma de crear tipos personalizados: vayamos a
la función de enumeración de Rust para agregar otra herramienta.
