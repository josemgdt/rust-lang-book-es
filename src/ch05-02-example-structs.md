## Un programa de ejemplo que usa estructuras

Para entender cuándo podríamos necesitar estructuras, escribamos un programa que
calcula el área de un rectángulo. Empezaremos con variables individuales y luego
refactorizaremos el programa hasta que usemos estructuras.

Hagamos un nuevo proyecto binario con Cargo llamado *rectangles* que tomará
el ancho y alto de un rectángulo, especificado en píxeles, y calculará el área
del rectángulo. El Listado 5-8 muestra un programa corto con una forma de hacerlo
en *src/main.rs* de nuestro proyecto.

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

<span class="caption">Listado 5-8: Calcular el área de un rectángulo
especificado por ancho y alto en variables separadas</span>

Ahora, ejecutemos este programa usando `cargo run`:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

Aunque el Listado 5-8 funciona y calcula el área del rectángulo
llamando a la función `area` con cada dimensión, podemos hacerlo mejor. La anchura
y la altura están relacionadas entre sí porque juntas describen un rectángulo.

El problema con este código es evidente en la declaracion de `area`:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

Se supone que la función `area` calcula el área de un rectángulo, pero la
función que escribimos tiene dos parámetros. Los parámetros están relacionados, pero eso
no esta expresado en ninguna parte de nuestro programa. Sería más legible y más
manejable agrupar ancho y alto juntos. Ya hemos hablado de una forma con la que
podríamos hacer eso en la sección [“El Tipo Tupla”][the-tuple-type]<!-- ignore -->
del Capítulo 3; mediante el uso de tuplas.

### Refactorización con tuplas

El listado 5-9 muestra otra versión de nuestro programa que usa tuplas.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

<span class="caption">Listado 5-9: Especificando el ancho y alto del
rectángulo con una tupla</span>

De alguna manera, este programa es mejor. Las tuplas nos permiten agregar algo de estructuración y
ahora estamos pasando un solo argumento. Pero por otro lado, esta versión es menos
clara; las tuplas no nombran sus elementos, por lo que nuestro cálculo se ha vuelto más
confuso porque tenemos que indexar las partes de la tupla.

No importa el orden de ancho y alto para el cálculo del área, pero
si queremos dibujar el rectángulo en la pantalla, ¡si importaría! Tendríamos
para tener en cuenta que `width` es el índice de tupla `0` y `height` es el índice `1`.
Si alguien más trabajara en este código, tendrían que saber esto
y tenerlo en cuenta también. Sería fácil olvidarlo o mezclar estos
valores y causar errores, porque no hemos transmitido el significado de nuestros datos en
nuestro código.

### Refactorización con estructuras: agregando más significado

Usamos estructuras para agregar significado al etiquetar los datos. Podemos transformar la tupla
que estamos usando en un tipo de datos con un nombre unico, así como nombres para las
partes, como se muestra en el Listado 5-10.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

<span class="caption">Listado 5-10: Definición de una estructura `Rectangle`</span>

Aquí definimos una estructura y la llamamos `Rectangle`. Dentro de las llaves
definimos los campos `width` y `height`, los cuales tienen
tipo `u32`. Luego, en `main`, creamos una instancia particular de` Rectangle`
que tiene un ancho de 30 y una altura de 50.

Nuestra función `area` ahora está definida con un parámetro, que hemos llamado
`rectangle`, cuyo tipo es un préstamo inmutable de una instancia de estructura` Rectangle`.
Como se mencionó en el Capítulo 4, queremos tomar prestada la estructura en lugar de
tomar posesión de ella. De esta manera, `main` conserva su propiedad y puede continuar
usando `rect1`, que es la razón por la que usamos el `&` en la declaracion de la función y
donde llamamos a la función.

La función `area` accede a los campos `width` y `height` de la instancia `Rectangle`.
Nuestra declaracion de función para `área` ahora dice exactamente lo que queremos decir:
calcular el área de `Rectangle`, usando sus campos `width` y `height`. Esto
muestra que el ancho y la altura están relacionados entre sí, y da
nombres descriptivos a los valores en lugar de utilizar los valores de índice de tupla "0"
y "1". Esto es una victoria en pro de claridad.

### Añadiendo Funcionalidad Util con Traits Derivados

Sería bueno poder imprimir una instancia de `Rectangle` mientras estamos
depurando nuestro programa y ver los valores de todos sus campos. El Listado 5-11 lo intenta
usando la macro `println!` como hemos la usado en capítulos anteriores. Esto, sin embargo, no
trabaja.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

<span class="caption">Listado 5-11: Intentando imprimir una instancia `Rectangle`</span>

Cuando compilamos este código, obtenemos un error con este mensaje principal:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

La macro `println!` Puede realizar muchos tipos de formateo y, por defecto,
los corchetes le dicen a la macro `println!` que use el formato conocido como `Display`; salida prevista
para consumo directo del usuario final. Los tipos primitivos que hemos visto hasta ahora
implementan `Display` de forma predeterminada, porque solo hay una forma en la que desea mostrar
un `1` o cualquier otro tipo primitivo al usuario. Pero con estructuras, la forma en la que
`println!` debería formatear la salida es menos clara porque hay más
posibilidades de visualización: ¿Quieres comas o no? ¿Quieres imprimir los
corchetes? ¿Deberían mostrarse todos los campos? Debido a esta ambigüedad, Rust
no intenta adivinar lo que queremos y las estructuras no tienen una
implementación de `Display`.

Si continuamos leyendo los errores, encontraremos esta nota útil:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

¡Vamos a intentarlo! La llamada a la macro `println!` se verá ahora como `println!("rect1 is{:?}", rect1);`. 
Poner el especificador `:?` dentro de las llaves indica a `println!` que queremos usar un formato de 
salida llamado `Debug`. El trait `Debug` nos permite imprimir nuestra estructura de una manera que sea útil 
para los desarrolladores y ver su valor mientras depuramos nuestro código.

Compile el código con este cambio. ¡Vaya! Seguimos recibiendo un error:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

Pero nuevamente, el compilador nos da una nota útil:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

Rust *does* include functionality to print out debugging information, but we
have to explicitly opt in to make that functionality available for our struct.
To do that, we add the annotation `#[derive(Debug)]` just before the struct
definition, as shown in Listing 5-12.
Rust *tiene* incluida funcionalidad para imprimir información de depuración, pero nosotros
tenemos que escojer explícitamente para que esa funcionalidad esté disponible para nuestra estructura.
Para hacer eso, agregamos la anotación `#[derive (Debug)]` justo antes de la definición de la 
estructura, como se muestra en el Listado 5-12.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

<span class="caption">Listado 5-12: Adición de la anotación para derivar el trait `Debug`
e imprimir la instancia `Rectangle` usando el formato debug</span>

Ahora, cuando ejecutamos el programa, no obtendremos ningún error y veremos la
siguiente salida:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

Nice! It’s not the prettiest output, but it shows the values of all the fields
for this instance, which would definitely help during debugging. When we have
larger structs, it’s useful to have output that’s a bit easier to read; in
those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string.
When we use the `{:#?}` style in the example, the output will look like this:


¡Fantastico! No es el resultado más bonito, pero muestra los valores de todos los campos
para este caso, lo que definitivamente ayudará durante la depuración. Cuando tenemos
estructuras más grandes, es útil tener resultados que sean un poco más fáciles de leer; en
esos casos, podemos usar `{:#?}` en lugar de `{:?}` en la cadena `println!`.
Cuando usamos el estilo `{:#?}` en el ejemplo, la salida se verá así:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

Our `area` function is very specific: it only computes the area of rectangles.
It would be helpful to tie this behavior more closely to our 
struct, because it won’t work with any other type. Let’s look at how we can
continue to refactor this code by turning the `area` function into an `area`
*method* defined on our `Rectangle` type.

Rust nos ha proporcionado una serie de traits para que los usemos con la anotación `derive`
que pueden agregar un comportamiento útil a nuestros tipos personalizados. Esos traits y sus
comportamientos se enumeran en el [Apéndice C][app-c]<!-- ignore -->. Cubriremos cómo
implementar estos traits con un comportamiento personalizado, así como también cómo crear sus propios
traits en el Capítulo 10.

Nuestra función `area` es muy específica: solo calcula el área de rectángulos.
Sería útil vincular este comportamiento más estrechamente a nuestra estructura `Rectangle`,
porque no funcionará con ningún otro tipo. Veamos cómo podemos
continuar refactorizando este código convirtiendo la función `area` en un *método* `area`
definido en nuestro tipo `Rectangle`.

[the-tuple-type]: ch03-02-data-types.html#el-tipo-tupla
[app-c]: appendix-03-derivable-traits.md
