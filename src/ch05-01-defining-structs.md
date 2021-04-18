## Definición y creación de instancias de estructuras

Las estructuras son similares a las tuplas, que se analizaron en el capítulo 3. Al igual que las tuplas,
las piezas de una estructura pueden ser de diferentes tipos. A diferencia de las tuplas, se nombrará
cada dato para que quede claro lo que significan los valores. Como resultado de estos
nombres, las estructuras son más flexibles que las tuplas: no tienes que depender del
orden de los datos para especificar o acceder a los valores de una instancia.

Para definir una estructura, ingresamos la palabra clave `struct` y nombramos la estructura completa.
El nombre de la estructura debe describir la importancia de los datos que se están
agrupando. Luego, entre llaves, definimos los nombres y tipos de
los datos, que llamamos *campos*. Por ejemplo, el Listado 5-1 muestra una
estructura que almacena información sobre una cuenta de usuario.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

<span class="caption">Listado 5-1: Una definición de estructura `User`</span>

Para usar una estructura después de haberla definido, creamos una *instancia* de esa estructura
especificando valores concretos para cada uno de los campos. Creamos una instancia
indicando el nombre de la estructura y luego agregando corchetes que contengan pares de valores `clave:valor`, 
donde las claves son los nombres de los campos y los valores son los
datos que queremos almacenar en esos campos. No tenemos que especificar los campos en
el mismo orden en que los declaramos en la estructura. En otras palabras,
la definición de estructura es como una plantilla general para un tipo, y las instancias llenan
esa plantilla con datos particulares para crear valores del tipo.
Por ejemplo, podemos declarar un usuario en particular como se muestra en el Listado 5-2.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

<span class="caption">Listado 5-2: Creación de una instancia de la estructura `User`</span>

Para obtener un valor específico desde una estructura, podemos usar la notación de puntos. Si quisiéramos
solo la dirección de correo electrónico de este usuario, podríamos usar `user1.email` donde queramos
utilizar este valor. Si la instancia es mutable, podemos cambiar un valor usando
la notación de puntos y la asignación a un campo en particular. El Listado 5-3 muestra cómo
cambiar el valor en el campo `email` de una instancia mutable de `User`.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

<span class="caption">Listado 5-3: Cambiar el valor en el campo `email` de una instancia
`User`</span>

Tenga en cuenta que toda la instancia debe ser mutable; Rust no nos permite marcar
solo ciertos campos como mutables. Como con cualquier expresión, podemos construir una nueva
instancia de la estructura como la última expresión en el cuerpo de la función para
devolver implícitamente esa nueva instancia.

El Listado 5-4 muestra una función `build_user` que devuelve una instancia de `User` con
el correo electrónico y el nombre de usuario proporcionados. El campo `active` obtiene el valor `true`, y
`sign_in_count` obtiene un valor de `1`.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

<span class="caption">Listado 5-4: Una función `build_user` que toma un correo electrónico
y nombre de usuario y devuelve una instancia `User`</span>

Tiene sentido nombrar los parámetros de la función con el mismo nombre que los campos de la estructura,
pero tener que repetir los nombres de los campos `email` y `username` y las
variables es un poco tedioso. Si la estructura tenía más campos, repetir cada nombre
se volvería aún más molesto. Afortunadamente, hay una abreviatura conveniente.

### Uso de la Abreviatura de Inicio de Campo Cuando las Variables y los Campos Tienen el Mismo Nombre

Debido a que los nombres de los parámetros y los nombres de los campos de estructura son exactamente iguales en
el listado 5-4, podemos usar la sintaxis *field init abreviada* para reescribir
`build_user` y se comporte exactamente igual pero no tenga la
repetición de `email` y `username`, como se muestra en el Listado 5-5.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

<span class="caption">Listado 5-5: Una función `build_user` que usa
abreviatura field init porque los parámetros `email` and `username` tienen el mismo nombre que
los campos de estructura</span>

Here, we’re creating a new instance of the `User` struct, which has a field
named `email`. We want to set the `email` field’s value to the value in the
`email` parameter of the `build_user` function. Because the `email` field and
the `email` parameter have the same name, we only need to write `email` rather
than `email: email`.
Aquí, estamos creando una nueva instancia de la estructura `User`, que tiene un campo
llamado `email`. Queremos establecer el valor del campo `email` en el valor del
parámetro `email` de la función `build_user`. Puesto que el campo `email` y
el parámetro `email` tiene el mismo nombre, solo necesitamos escribir `email` en lugar
de `email: email`.

### Creación de Instancias a Partir de Otras Instancias con Sintaxis de Actualización de Estructuras

Suele ser útil crear una nueva instancia de una estructura que utilice la mayor parte de
los valores de una instancia anterior, pero cambiando algunos. Esto se hará 
usando *sintaxis de actualización de estructura*.

Primero, el Listado 5-6 muestra cómo creamos una nueva instancia de `User` en `user2` sin
la sintaxis de actualización. Establecemos nuevos valores para `email` y `username` pero el resto
usa los mismos valores de `user1` que creamos en el Listado 5-2.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

<span class="caption">Listado 5-6: Creación de una nueva instancia de `User` usando algunos de
los valores de `user1`</span>

Usando la sintaxis de actualización de la estructura, podemos lograr el mismo efecto con menos código, como
se muestra en el Listado 5-7. La sintaxis `..` especifica que los campos restantes no
explícitamente establecidos deben tener el mismo valor que los campos en la instancia dada.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

<span class="caption">Listado 5-7: Uso de la sintaxis de actualización de estructura para establecer nuevos
valores de `email` y `username` para una instancia de `User` pero usan el resto de
valores de los campos de la instancia en la variable `user1`</span>

El código del Listado 5-7 también crea una instancia en `user2` que tiene un
valor diferente para `email` y `username` pero tiene los mismos valores para los
campos `active` y `sign_in_count` de `user1`.

### Uso de Estructuras de Tuplas sin Campos con Nombre para Crear Diferentes Tipos

También puede definir estructuras que se parecen a las tuplas, llamadas *estructuras de tupla*. 
Las estructuras de tupla tienen el significado agregado que proporciona el nombre de la estructura, pero
no tienen nombres asociados con sus campos; solo tienen los tipos de los campos. 
Las estructuras de tupla son útiles cuando desea dar a toda la tupla un
nombre y hacer que la tupla sea de un tipo diferente a otras tuplas, y nombrar cada 
campo como en una estructura regular sería redundante.

Para definir una estructura de tupla, comience con la palabra clave `struct` y el nombre de la estructura
seguido de los tipos en la tupla. Por ejemplo, aquí hay definiciones y
usos de dos estructuras de tupla llamadas `Color` y `Point`:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs:here}}
```

Tenga en cuenta que los valores `black` y `origin` son tipos diferentes, porque son
instancias de diferentes estructuras de tupla. Cada estructura que defina es de su propio tipo,
aunque los campos dentro de la estructura tienen los mismos tipos. Por ejemplo, una
función que toma un parámetro de tipo `Color` no puede tomar un `Punto` como
argumento, aunque ambos tipos se componen de tres valores `i32`. Por otro lado,
las instancias de estructura de tupla se comportan como tuplas: puede desestructurarlas en sus
piezas individuales, puede utilizar un `.` seguido del índice para acceder a un
valor individual, etc.

### Estructuras Tipo Unidad sin Campos

¡También puede definir estructuras que no tengan campos! Estas se llaman
*estructuras tipo unidad* porque se comportan de manera similar a `()`, el tipo de unidad.
Las estructuras tipo unidad pueden ser útiles en situaciones en las que necesita implementar una
trait en algún tipo, pero no tiene ningún dato que desee almacenar en el tipo en
sí. Discutiremos los traits en el Capítulo 10.

> ### Propiedad de los Datos de Estructura
>
> En la definición de estructura `User` en el Listado 5-1, usamos el tipo con propiedad `String`
> en lugar del tipo slice de cadena `&str`. Esta es una elección deliberada
> porque queremos que las instancias de esta estructura posean todos sus datos y para
> que los datos sean válidos mientras toda la estructura sea válida.
>
> Es posible que las estructuras almacenen referencias a datos que pertenecen a otra entidad,
> pero para hacerlo requiere el uso de *duraciones o lifetimes*, una característica de Rust que
> discutiremos en el Capítulo 10. Las lifetimes aseguran que los datos referenciados por una estructura
> son válidos mientras lo sea la estructura. Supongamos que intenta almacenar una referencia
> en una estructura sin especificar lifetimes, como aqui, que no funcionará:
>
> <span class="filename">Nombre de archivo: src/main.rs</span>
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct User {
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
>     active: bool,
> }
>
> fn main() {
>     let user1 = User {
>         email: "someone@example.com",
>         username: "someusername123",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
>
> El compilador se quejará de que necesita especificadores de lifetime:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:2:15
>   |
> 2 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 | struct User<'a> {
> 2 |     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:12
>   |
> 3 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 | struct User<'a> {
> 2 |     username: &str,
> 3 |     email: &'a str,
>   |
>
> error: aborting due to 2 previous errors
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs`
>
> To learn more, run the command again with --verbose.
> ```
>
> In Chapter 10, we’ll discuss how to fix these errors so you can store
> references in structs, but for now, we’ll fix errors like these using owned
> types like `String` instead of references like `&str`.
> En el Capítulo 10 analizaremos cómo corregir estos errores para que pueda almacenar
> referencias en estructuras, pero por ahora, corregiremos errores como estos usando tipos con propiedad,
> como `String` en lugar de referencias `&str`.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->
