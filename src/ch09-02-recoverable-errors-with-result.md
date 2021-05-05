## Errores recuperables con `Result`

La mayoría de los errores no son lo suficientemente graves como para requerir que el programa se detenga por completo.
A veces, cuando una función falla, es por una razón que usted puede
interpretar y responder. Por ejemplo, si intenta abrir un archivo y
la operación falla porque el archivo no existe, es posible que desee crear el
archivo en lugar de finalizar el proceso.

Recuerde de [“Manejo de Fallos Potenciales con el Tipo `Result`][handle_failure]<!-- ignore --> en el Capítulo 2 
que la enumeración `Result` está definida para tener dos variantes, "Ok" y "Err", de la siguiente manera:

[handle_failure]: ch02-00-guessing-game-tutorial.html#manejo-de-fallos-potenciales-con-el-tipo-result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` y `E` son parámetros de tipo genérico; discutiremos los genéricos con más
detalle en el Capítulo 10. Lo que necesita saber ahora es que `T` representa
el tipo de valor que se devolverá en caso de éxito dentro de la variante `Ok`,
y `E` representa el tipo de error que se devolverá en un
caso de fallo dentro de la variante `Err`. Puesto que `Result` tiene estos parámetros de tipo genérico,
podemos usar el tipo `Result` y las funciones que la biblioteca estándar
ha definido en él en muchas situaciones diferentes donde el valor de éxito
y el valor de error que queremos devolver pueden diferir.

Llamemos a una función que devuelve un valor de `Result` porque la función podría
fallar. En el Listado 9-3 intentamos abrir un archivo.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<span class="caption">Listado 9-3: Abrir un archivo</span>

¿Cómo sabemos que `File::open` devuelve un `Result`? Podríamos mirar en la 
[documentación de la API de la biblioteca estándar](../std/index.html)<!-- ignore -->, o podríamos preguntar
al compilador! Si le damos a `f` una anotación de tipo que sabemos que *no* es el tipo de la devolución
de la función y luego intentamos compilar el código, el compilador le dirá
que los tipos no coinciden. El mensaje de error nos dirá cuál *es*
el tipo de `f`. ¡Vamos a intentarlo! Sabemos que el tipo de retorno de `File::open`
no es del tipo `u32`, así que cambiemos la instrucción `let f` a esto:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/src/main.rs:here}}
```

Intentar compilar ahora nos da el siguiente resultado:

```console
{{#include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/output.txt}}
```

Esto nos dice que el tipo de retorno de la función `File::open` es un `Result <T, E>`.
El parámetro genérico `T` se ha completado aquí con el tipo del
valor, `std::fs::File`, que es un identificador de archivo. El tipo de `E` utilizado en el
el valor de error es `std::io::Error`.

Este tipo de retorno significa que la llamada a `File::open` podría tener éxito y devolver un
handler de archivo desde el que podemos leer o escribir. La llamada a la función también puede fallar.
Por ejemplo, es posible que el archivo no exista o que no tengamos permiso para
accederlo. La función `File::open` necesita tener una forma de decirnos
si tuvo éxito o no y al mismo tiempo nos da el handler de archivo
o la información de error. Esta información es exactamente lo que la enumeración `Result`
transmite.

En el caso de que `File::open` tenga éxito, el valor de la variable` f` será
una instancia de `Ok` que contiene un manejador de archivo. En el caso de que falle,
el valor en `f` será una instancia de `Err` que contiene más información
sobre el tipo de error que ocurrió.

Necesitamos agregar al código en el Listado 9-3 para tomar diferentes acciones dependiendo
del valor que devuelve `File::open`. El Listado 9-4 muestra una forma de manejar
`Result` usando una herramienta básica, la expresión` match` que discutimos en
Capítulo 6.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<span class="caption">Listado 9-4: Uso de una expresión `match` para manejar las
variantes de `Result` que podrían devolverse</span>

Tenga en cuenta que, al igual que la enumeración `Option`, la enumeración `Result` y sus variantes se han
traído al alcance por el preludio, por lo que no es necesario especificar `Result::`
antes de las variantes `Ok` y` Err` en los brazos `Match`.

Aquí le decimos a Rust que cuando el resultado sea "Ok", devuelva el valor interno del "archivo".
fuera de la variante `Ok`, y luego asignamos ese valor de identificador de archivo a la
variable `f`. Después de la "coincidencia", podemos usar el identificador de archivo para leer o
escritura.

El otro brazo del `match` maneja el caso donde obtenemos un valor` Err` de
`File::open`. En este ejemplo, hemos elegido llamar a la macro `panic!`. Si
no hay ningún archivo llamado *hello.txt* en nuestro directorio actual y ejecutamos este
código, veremos la siguiente salida de la macro `panic!`:


```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

Como de costumbre, esta salida nos dice exactamente qué ha fallado.

### Matching en Diferentes Errores

El código del Listado 9-4 entrará en `panic!` sin importar por qué falló `File::open`.
Lo que queremos hacer en su lugar es tomar diferentes acciones por diferentes razones de fallo;
si `File::open` falló porque el archivo no existe, queremos crear el
archivo y devolver un handler al nuevo archivo. Si `File::open` falló en algún
otro motivo, por ejemplo, porque no teníamos permiso para abrir el archivo,
queremos que el código entre en pánico de la misma manera que lo hizo en el Listado 9-4. Vea
el Listado 9-5, que agrega una expresión interna `match`.

<span class="filename">Nombre de archivo: src/main.rs</span>

<!-- ignore esta prueba porque, de lo contrario, crea hello.txt que causa otros
pruebas para fallar -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<span class="caption">Listado 9-5: Manejo de diferentes tipos de errores en
diferentes formas</span>

El tipo de valor que `File::open` devuelve dentro de la variante `Err` es
`io::Error`, que es una estructura proporcionada por la biblioteca estándar. Esta estructura
tiene un método `kind` que podemos llamar para obtener un valor `io::ErrorKind`. La enumeración
`io::ErrorKind` está proporcionado por la biblioteca estándar y tiene variantes
representando los diferentes tipos de errores que pueden resultar de una
operación `io`. La variante que queremos usar es `ErrorKind::NotFound`, que indica
que el archivo que intentamos abrir aún no existe. Así que tenemos match en `f`, pero
también tiene un match en `error.kind()`.

La condición que queremos verificar en el match interno es si el valor devuelto
por `error.kind()` es la variante `NotFound` de la enumeración` ErrorKind`. Si esto es asi,
intentamos crear el archivo con `File::create`. Sin embargo, debido a que `File::create`
también podría fallar, necesitamos un segundo brazo en la expresión interna de `match`. Cuando
no se puede crear el archivo, se imprime un mensaje de error diferente. El segundo brazo de
`match` externo permanece igual, por lo que el programa entra en pánico ante cualquier error además
del error de archivo faltante.

¡Eso es mucho `match`! La expresión `match` es muy útil pero también muy
muy primitiva. En el Capítulo 13, aprenderá sobre cierres. El tipo `Result<T, E>`
tiene muchos métodos que aceptan un cierre y se implementan usando
xpresiones `match`. El uso de esos métodos hará que su código sea más conciso. Los
rustaceos más experimentados podrían escribir este código en lugar del Listado 9-5:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-03-closures/src/main.rs}}
```

Aunque este código tiene el mismo comportamiento que el Listado 9-5, no contiene ninguna expresion
`match` y es más limpio de leer. Regrese a este ejemplo después
del Capítulo 13, y busque el método `unsrap_or_else` en la documentación de la biblioteca estándar.
Muchos más de estos métodos pueden limpiar anidamientos grandes de expresiones `match` cuando se trata de errores.

### Atajos Para Pánico en Caso de Error: `unwrap` y `expect`

El uso de `match` funciona bastante bien, pero puede resultar demasiado detallado y no siempre
comunicar bien la intención. El tipo `Result <T, E>` tiene muchos métodos auxiliares
definido en él para realizar diversas tareas. Uno de esos métodos, llamado `unwrap`, es un
método de acceso directo que se implementa como la expresión `match` que escribimos en
Listado 9-4. Si el valor de `Result` es la variante `Ok`, `unwrap` devolverá
el valor dentro de `Ok`. Si `Result` es la variante `Err`, `unwrap`
llama a la macro `panic!` por nosotros. Aquí hay un ejemplo de `unwrap` en acción:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

Si ejecutamos este código sin un archivo *hello.txt*, veremos un mensaje de error de
la llamada `panic!` que hace el método `unwrap`:

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

El otro método, `expect`, que es similar a `unwrap`, nos permite también elegir el
mensaje de error de pánico. El uso de `expect` en lugar de `unwrap` y proporcionar un buen
mensaje de error pueden transmitir su intención y es más fácil rastrear la fuente de un
pánico. La sintaxis de `expect` se ve así:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

Usamos `expect` de la misma manera que `unwrap`: para devolver el handler del archivo o llamar
a la macro `panic!`. El mensaje de error usado por `wait` en su llamada a `panic!`
será el parámetro que pasamos a `expect`, en lugar del mensaje `panic!` predeterminado
que utiliza `unwrap`. Así es como se ve:

```text
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

Debido a que este mensaje de error comienza con el texto que especificamos, `No se pudo abrir hello.txt`, 
será más fácil encontrar de qué parte del código procede este mensaje de error.
Si usamos `unwrap` en varios lugares, puede llevar más tiempo
averiguar exactamente qué `unwrap` está causando el pánico porque todas
las llamadas de pánico `unwrap` imprimen el mismo mensaje.

### Propagación de Errores

Cuando escribe una función cuya implementación llama a algo que podría
falla, en lugar de manejar el error dentro de esta función, puede devolver el
error al código de llamada para que pueda decidir qué hacer. Esto se conoce como
* propaga * el error y le da más control al código de llamada, donde hay
podría haber más información o lógica que dicta cómo debe ser el error
manejado de lo que tiene disponible en el contexto de su código.

Por ejemplo, el Listado 9-6 muestra una función que lee un nombre de usuario de un archivo. Si
el archivo no existe o no se puede leer, esta función devolverá esos errores
al código que llamó a esta función.

<span class="filename">Nombre de archivo: src/main.rs</span>

<! - Deliberadamente no usar rustdoc_include aquí; la función `main` en el
archivo de pánico. Queremos incluirlo con fines de experimentación del lector, pero
no quiero incluirlo con fines de prueba de rustdoc. ->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<span class="caption">Listado 9-6: una función que devuelve errores al
código de llamada usando `match`</span>

Esta función se puede escribir de una manera mucho más corta, pero vamos a empezar por
hacerlo de forma manual para explorar el manejo de errores; al final,
mostraremos el camino más corto. Veamos primero el tipo de retorno de la función:
`Result<String, io::Error>`. Esto significa que la función devuelve un valor del
tipo `Result <T, E>` donde se ha completado el parámetro genérico `T`
con el tipo concreto `String` y el tipo genérico `E` se ha completado
con el tipo concreto `io::Error`. Si esta función tiene éxito sin ningún
problema, el código que llama a esta función recibirá un valor `Ok` que
contiene una `String` con el nombre de usuario que esta función lee del archivo. Si esto
función encuentra algún problema, el código que llama a esta función
recibe un valor `Err` que contiene una instancia de `io::Error` que contiene
más información sobre los problemas. Elegimos `io::Error` como
el tipo devuelto de esta función porque ese es el tipo de error
devuelto por las dos operaciones que llamamos en este cuerpo de función
que pueden fallar: la función `File::open` y el método `read_to_string`.

El cuerpo de la función comienza llamando a la función `File::open`. Entonces nosotros
manejar el valor de `Result` devuelto con un `match` similar al `match` en el
Listado 9-4, solo que en lugar de llamar a `panic!` en el caso de `Err`, regresamos
antes de esta función y pasamos el valor de error de `File::open` al
código de llamada como valor de error de esta función. Si `File::open` tiene éxito, almacenamos
el handler de archivo en la variable `f` y continuamos.

Luego creamos una nueva `String` en la variable `s` y llamamos a `read_to_string`
en el handler de archivo en `f` para leer el contenido del archivo en `s`.
El método `read_to_string` también devuelve un `Result` porque podría fallar, incluso
aunque `File::open` tuviese éxito. Entonces necesitamos otra `match` para manejar ese
`Result`: si `read_to_string` tiene éxito, entonces nuestra función ha tenido éxito y
devuelve el nombre de usuario del archivo que ahora está en `s` envuelto en un `Ok`. Si
`read_to_string` falla, devolvemos el valor de error de la misma manera que se
devolvió el valor de error en la `match` que manejó el valor de retorno de
`File::open`. Sin embargo, no es necesario que digamos explícitamente `return`, porque esta
es la última expresión de la función.

El código que llama a este código se encargará de obtener un valor `Ok`
que contiene un nombre de usuario o un valor `Err` que contiene un `io::Error`. Nosotros
no sabemos qué hará el código de llamada con esos valores. Si el código de llamada
obtiene un valor `Err`, podría llamar a `panic! `y bloquear el programa, usar un
nombre de usuario predeterminado, o buscar el nombre de usuario en otro lugar que no sea un archivo, por
ejemplo. No tenemos suficiente información sobre lo que realmente está tratando de hacer el código de llamada,
por lo que propagamos toda la información de éxito o error hacia arriba para
manejarla apropiadamente.

Este patrón de propagación de errores es tan común en Rust, que se proporciona el
operador interrogación `?` para hacer esto más fácil.

#### Un Atajo Para Propagar Errores: el Operador `?`

El Listado 9-7 muestra una implementación de `read_username_from_file` que tiene la
misma funcionalidad que tenía en el Listado 9-6, pero esta implementación usa el
operador `?`.

<span class="filename">Nombre de archivo: src/main.rs</span>

<! - Deliberadamente no usar rustdoc_include aquí; la función `main` en el
archivo de pánico. Queremos incluirlo con fines de experimentación del lector, pero
no quiero incluirlo con fines de prueba de rustdoc. ->


```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<span class="caption">Listado 9-7: una función que devuelve errores al
código de llamada usando el operador `?`</span>

El `?` colocado después de un valor de `Result` se define para funcionar casi de la misma manera
como las expresiones `match` que definimos para manejar los valores de` Result` en el Listado
9-6. Si el valor del `Result` es un `Ok`, el valor dentro de `Ok`
se devuelve de esta expresión y el programa continuará. Si el valor
es un `Err`, el `Err` se devolverá de la función completa, como si usasemos
la palabra clave `return` para que el valor de error se propague al código de llamada.

Hay una diferencia entre lo que hace la expresión `match` del Listado 9-6
y lo qué hace el operador `?`; valores de error que tienen el operador `?` en la llamada
pasan por la función `from`, definida en el trait `From` en la
biblioteca estándar, que se utiliza para convertir errores de un tipo a otro.
Cuando el operador `?` llama a la función `from`, el tipo de error recibido es
convertido en el tipo de error definido en el tipo de retorno de la función actual.
Esto es útil cuando una función devuelve un tipo de error para representar todas
las formas en que una función puede fallar, incluso si las partes pueden fallar por muchas
razones. Siempre que cada tipo de error implemente la función `from` para definir cómo
convertirse al tipo de error devuelto, el operador `?` se encarga de la
conversión automáticamente.

En el contexto del Listado 9-7, el `?` al final de la llamada `File::open`
devuelve el valor dentro de un `Ok` a la variable `f`. Si ocurre un error, el
operador `?` regresará antes de toda la función y dará cualquier valor de error `Err`
al código de llamada. Lo mismo se aplica al `?` al final de la
llamada `read_to_string`.

El operador `?` Elimina una gran cantidad de texto repetitivo y hace que esta función sea
de implementación más sencilla. Incluso podríamos acortar aún más este código encadenando
llamadas a métodos inmediatamente después de `?`, como se muestra en el Listado 9-8.

<span class="filename">Nombre de archivo: src/main.rs</span>

<! - Deliberadamente no usar rustdoc_include aquí; la función `main` en el
archivo de pánico. Queremos incluirlo con fines de experimentación del lector, pero
no quiero incluirlo con fines de prueba de rustdoc. ->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<span class="caption">Listado 9-8: Encadenamiento de llamadas a métodos después del operador `?`</span>

Hemos movido la creación de la nueva `String` en `s` al comienzo de la
función; esa parte no ha cambiado. En lugar de crear una variable `f`, hemos
encadenado la llamada a `read_to_string` directamente en el resultado de
`File::open("hello.txt")?`. Todavía tenemos un `?` al final de la
llamada `read_to_string`, y todavía devolvemos un valor `Ok` que contiene el
nombre de usuario en `s` cuando `File::open` y `read_to_string` tienen éxito en lugar de
devolver errores. La funcionalidad es nuevamente la misma que en el Listado 9-6 y
Listado 9-7; esta es solo una forma diferente y más ergonómica de escribirlo.

Hablando de diferentes formas de escribir esta función, el Listado 9-9 muestra que
hay una manera de hacer esto aún más corta.

<span class="filename">Nombre de archivo: src/main.rs </span>

<! - Deliberadamente no usar rustdoc_include aquí; la función `main` en el
archivo de pánico. Queremos incluirlo con fines de experimentación del lector, pero
no quiero incluirlo con fines de prueba de rustdoc. ->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<span class="caption">Listado 9-9: usando `fs::read_to_string` en lugar de
abriendo y luego leyendo el archivo </span>

Leer un archivo en una cadena es una operación bastante común, por lo que Rust proporciona la
conveniente función `fs::read_to_string` que abre el archivo, crea un nuevo
`String`, lee el contenido del archivo, coloca el contenido en esa` String`,
y lo devuelve. Por supuesto, usar `fs::read_to_string` no nos da la
oportunidad de explicar todo el manejo de errores, así que lo hicimos de la manera más larga
primero.

#### El Operador `?` Se Puede Utilizar en Funciones que Devuelven `Result`

El operador `?` Se puede utilizar en funciones que tienen un tipo `Result` de retorno,
porque está definido para funcionar de la misma manera que la expresión `match`
que definimos en el Listado 9-6. La parte de `match` que requiere un
tipo de retorno `Result` es `return Err(e)`, por lo que el tipo de retorno de la función
tiene que ser un `Result` para ser compatible con este `return`.

Veamos qué sucede si usamos el operador `?` en la función `main`,
que recordará tiene un tipo de retorno de `()`:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-06-question-mark-in-main/src/main.rs}}
```

Cuando compilamos este código, obtenemos el siguiente mensaje de error:

```console
{{#include ../listings/ch09-error-handling/no-listing-06-question-mark-in-main/output.txt}}
```

Este error indica que solo podemos utilizar el operador `?` en una
función que devuelve `Result` u `Option` u otro tipo que implemente
`std::ops::Try`. Cuando escribe código en una función
que no devuelve uno de estos tipos y desea utilizar `?` cuando llama a otras
funciones que devuelven `Result<T, E>`, tiene dos opciones para solucionar este problema.
Una técnica es cambiar el tipo de retorno de su función para que sea `Result<T, E> `si no tiene 
restricciones que lo impidan. La otra técnica es utilizar
un `match` o uno de los métodos `Result<T, E> `para manejar el `Result<T, E> `en
cualquier forma que sea apropiada.

La función `main` es especial, y existen restricciones sobre lo que devuelve.
Un tipo de retorno válido para main es `()`, y convenientemente, otro
tipo de retorno válido es `Result<T, E>`, como se muestra aquí:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-07-main-returning-result/src/main.rs}}
```

El tipo `Box<dyn Error>` se denomina objeto de trait, del que hablaremos en la sección
["Uso de objetos trait que permiten valores de diferentes Tipos ”][trait-objects]<!-- ignore -->  en el Capítulo 17. Por ahora, puede leer `Box<dyn Error>` para significar "cualquier tipo de error". Usando `?` en un `main`
se permite este tipo de retorno.

Ahora que hemos discutido los detalles de llamar a `panic!` o devolver `Result`,
regresemos al tema de cómo decidir cuál es apropiado usar y en qué casos.

[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types

