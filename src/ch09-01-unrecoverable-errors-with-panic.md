## Errores Irrecuperables con `panic!`

A veces, suceden cosas malas en su código y no hay nada que pueda hacer al respecto.
En estos casos, Rust tiene la macro `panic!`. Cuando la macro `panic!` se
ejecuta, su programa imprimirá un mensaje de fallo, se despliega y limpia la pila
y luego sale. Esto ocurre con mayor frecuencia cuando un bug de algún tipo
se ha detectado y el programador no tiene claro cómo manejar el error.

> ### Desenrollar la Pila o Cancelar en Respuesta a un Pánico
>
> De forma predeterminada, cuando se produce un pánico, el programa comienza a *desenrollarse*, lo que
> significa que Rust regresa por la pila y limpia los datos de cada función que
> encuentra. Pero esta caminata de regreso y limpieza es mucho trabajo.
> La alternativa es *abortar* inmediatamente, lo que finaliza el programa sin
> limpieza. La memoria que estaba usando el programa deberá limpiarse
> por el sistema operativo. Si en su proyecto necesita hacer el
> binario lo más pequeño posible, puede cambiar de desenrollar a abortar en un
> pánico agregando `panic = 'abort'` a las secciones correspondientes de `[profile]` en
> su archivo *Cargo.toml*. Por ejemplo, si desea abortar por pánico en
> modo producción, agregue esto:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

Intentemos llamar a `panic!` en un programa simple:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

Cuando ejecute el programa, verá algo como esto:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

La llamada a `panic!` Provoca el mensaje de error contenido en las dos últimas líneas.
La primera línea muestra nuestro mensaje de pánico y el lugar en nuestro código fuente donde
se produjo: *src/main.rs:2:5* indica que es la segunda línea,
quinto carácter de nuestro archivo *src/main.rs*.

En este caso, la línea indicada es parte de nuestro código, y si vamos a esa
línea, vemos la llamada de macro `panic!`. En otros casos, la llamada
estará en el código que llama nuestro código, y el nombre de archivo y el número de línea informado por
el mensaje de error será el código de otra persona, donde se llamó a la macro `panic!`,
no la línea de nuestro código que eventualmente llevó a la llamada `panic!`.
Podemos usar el seguimiento de las funciones de las que provino la llamada `panic!` para investigar
la parte de nuestro código que está causando el problema. Discutiremos qué es
el rastreo a continuación.

### Usando un Rastreo de `panic!`

Veamos otro ejemplo para ver cómo llega una llamada a `panic!`
de una biblioteca debido a un error en nuestro código en lugar de desde nuestra llamada a
la macro directamente. El Listado 9-1 tiene algún código que intenta acceder a un
elemento por índice en un vector.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

<span class="caption">Listado 9-1: Intentando acceder a un elemento más allá del
final de un vector, lo que provocará una llamada a `panic!`</span>

Aquí, estamos intentando acceder al elemento 100 de nuestro vector (que está en
index 99 porque la indexación comienza en cero), pero solo tiene 3 elementos. En esta
situación, Rust entrará en pánico. Se supone que el uso de `[]` devuelve un elemento, pero si
pasa un índice no válido, no hay ningún elemento que Rust pueda devolver que
fuese correcto.

En C, intentar leer más allá del final de una estructura de datos no tiene comportamiento definido.
Puede obtener cualquier cosa que esté en la ubicación de la memoria que
corresponderia a ese elemento en la estructura de datos, aunque esa memoria
no pertenece a esa estructura. Esto se denomina *sobrelectura del búfer* y puede
conducir a vulnerabilidades de seguridad si un atacante puede manipular el índice
de tal manera que lean datos que no deben y que se almacenen después
la estructura de datos.

Para proteger su programa de este tipo de vulnerabilidad, si intenta leer un
elemento en un índice que no existe, Rust detendrá la ejecución y se negará a
seguir. Probémoslo y veamos:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

Este error apunta a la línea 4 de nuestro `main.rs` donde intentamos acceder al índice
99. La siguiente línea de la nota nos dice que podemos establecer la variable de entorno
`RUST_BACKTRACE` para obtener un seguimiento de lo que sucedió exactamente para causar el
error. Un *backtrace* es una lista de todas las funciones que se han llamado hasta
llegar a este punto. Los backtraces en Rust funcionan como en otros lenguajes;
la clave para leer el rastreo es comenzar desde arriba y leer hasta que vea
archivos sobreescritos. Ese es el lugar donde se originó el problema. Las lineas encima de
las líneas que mencionan sus archivos son código que su código llamó; las líneas de abajo
son código que llamó a su código. Estas líneas pueden incluir código básico de Rust,
código de biblioteca estándar o cajas que esté utilizando. Intentemos conseguir un
rastreo estableciendo la variable de entorno `RUST_BACKTRACE` en cualquier valor
excepto 0. El Listado 9-2 muestra una salida similar a la que veria:

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
   1: core::panicking::panic_fmt
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
   2: core::panicking::panic_bounds_check
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
   5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
   6: panic::main
             at ./src/main.rs:4
   7: core::ops::function::FnOnce::call_once
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

<span class="caption">Listado 9-2: El rastreo generado por una llamada a `panic!` 
se muestra cuando se establece la variable de entorno `RUST_BACKTRACE`</span>

¡Eso es mucha salida! La salida exacta que ve puede ser diferente según su sistema operativo 
y la versión de Rust. Para obtener rastreos con esta información, los símbolos de depuración 
deben estar habilitados. Los símbolos de depuración están habilitados de forma predeterminada 
cuando se usa `cargo build` o `cargo run` sin el indicador `--release`, como tenemos aquí.

En la salida del Listado 9-2, la línea 6 del backtrace apunta a la línea en
nuestro proyecto que está causando el problema: línea 4 de *src/main.rs*. 
Si no queremos que nuestro programa entre en pánico, la ubicación apuntada por la primera línea 
que menciona un archivo que escribimos es donde debemos comenzar a investigar. En el Listado 9-1, 
donde deliberadamente escribimos código que entraría en pánico para demostrar cómo usar backtraces, 
la forma de solucionar el pánico es no solicitar un elemento en el índice 99 de un vector que solo 
contiene 3 elementos. Cuando su código entre en pánico en el futuro, deberá averiguar qué acción 
está tomando el código, con qué valores causa el pánico y qué debería hacer el código en su lugar.

Volveremos a `¡panic!` y cuando deberíamos y no deberíamos usarlo para manejar condiciones de error 
en la sección [“¡ `panic!` O NO `panic!`”][to-panic-or-not-to-panic]<!-- ignore -->  más adelante en este capítulo. 
A continuación, veremos cómo recuperarse de un error usando `Result`.

[to-panic-or-not-to-panic]:ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic
