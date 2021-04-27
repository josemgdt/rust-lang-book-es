## Cómo escribir pruebas

Las pruebas son funciones de Rust que verifican que el código a probar funciona en
la manera esperada. Los cuerpos de funciones de prueba típicamente realizan estos tres
comportamientos:

1. Configurar los datos o el estado necesarios.
2. Ejecutar el código que desea probar.
3. Afirmar que los resultados son los que se esperan.

Veamos las funciones que Rust proporciona específicamente para escribir pruebas que
realizan estas acciones, que incluyen el atributo `test`, algunas macros y el
Atributo `should_panic`.

### La anatomía de una función de prueba

En su forma más simple, una prueba en Rust es una función que se anota con el
atributo `test`. Los atributos son metadatos sobre partes del código de Rust; un ejemplo es
el atributo `derive` que usamos con las estructuras en el Capítulo 5. Para cambiar una función
en una función de prueba, agregue `#[test]` en la línea antes de `fn`. Cuando ejecuta su
pruebas con el comando `cargo test`, Rust crea un binario de ejecución de pruebas que ejecuta
las funciones anotadas con el atributo `test` e informa sobre si cada
función de prueba pasa o falla.

Cuando hacemos un nuevo proyecto de biblioteca con Cargo, se genera automáticamente un módulo 
de prueba con una función de prueba en él. Este módulo le ayuda a comenzar a
escribir sus pruebas para que no tenga que buscar la estructura y sintaxis exactas
de funciones de prueba cada vez que inicia un nuevo proyecto. Puede agregar tantas
funciones de prueba adicionales y tantos módulos de prueba como desee!

Exploraremos algunos aspectos de cómo funcionan las pruebas experimentando con la plantilla de
prueba generada para nosotros sin probar ningún código. Entonces escribiremos algunas
pruebas del mundo real que llaman a un código que hemos escrito y afirman que
el comportamiento es correcto.

Creemos un nuevo proyecto de biblioteca llamado `adder`:

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

El contenido del archivo *src/lib.rs* en su biblioteca `adder` debería verse como el
Listado 11-1.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

<span class="caption">Listado 11-1: El módulo de prueba y la función generados
automáticamente por `cargo new`</span>

Por ahora, ignoremos las dos líneas superiores y centrémonos en la función para ver cómo
obra. Tenga en cuenta la anotación `#[test]` antes de la línea `fn`: este atributo
indica que se trata de una función de prueba, por lo que el corredor de pruebas sabe cómo tratar esta
funcionar como una prueba. También podríamos tener funciones que no sean de prueba en el módulo `tests`
para ayudar a configurar escenarios comunes o realizar operaciones comunes, por lo que necesitamos
indicar qué funciones son pruebas utilizando el atributo `#[test]`.

El cuerpo de la función usa la macro `assert_eq!` Para afirmar que 2 + 2 es igual a 4.
Esta afirmación sirve como ejemplo del formato de una prueba típica. Corramos esta prueba
para ver que pasa.

El comando `cargo test` ejecuta todas las pruebas en nuestro proyecto, como se muestra en el Listado
11-2.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

<span class="caption">Listado 11-2: El resultado de ejecutar automáticamente la
prueba generada</span>

Cargo compiló y ejecutó la prueba. Después de las líneas `Compiling`, `Finished` y
`Running` está la línea `running 1 test`. La siguiente línea muestra el nombre
de la función de prueba generada, llamada `it_works`, y el resultado de ejecutar
esa prueba, `ok`. El resumen general de la ejecución de las pruebas aparece a continuación.
El texto `test result: ok.` significa que todas las pruebas pasaron, y la parte que
dice `1 passed; 0 failed` suma el número de pruebas que pasaron o fallaron.

Debido a que no tenemos ninguna prueba que hayamos marcado como ignorada, el resumen muestra 
`0 ignored`. Tampoco hemos filtrado las pruebas que se están ejecutando, por lo que el final del
resumen muestra `0 filtered out`. Hablaremos sobre ignorar y filtrar
pruebas en la siguiente sección, ["Control de cómo se realizan las 
pruebas”][controlando-cómo-se-ejecutan-las-pruebas]<!-- ignore -->

La estadística `0 measured` es para pruebas de referencia que miden el rendimiento.
Las pruebas de referencia están, en el momento de escribir este artículo, solo disponibles en Rust nocturno. Ver
[la documentación sobre las pruebas comparativas][bench] para obtener más información.

[bench]: ../unstable-book/library-features/test.html

La siguiente parte del resultado de la prueba, que comienza con `Doc-tests adder`, es para
los resultados de las pruebas de documentación. No tenemos pruebas de documentación
aún, pero Rust puede compilar cualquier ejemplo de código que aparezca en nuestra
documentación API. ¡Esta función nos ayuda a mantener nuestros documentos y nuestro código sincronizados!
Discutiremos cómo escribir pruebas de documentación en la sección ["Comentarios de documentación como
Pruebas”][doc-comments]<!-- ignore --> del Capítulo 14. Por ahora,
ignore la salida de `Doc-tests`.

Cambiemos el nombre de nuestra prueba para ver cómo cambia el resultado de la prueba.
Cambie la función `it_works` a un nombre diferente, como `exploration`:

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

A continuación, vuelva a ejecutar `cargo test`. La salida ahora muestra `exploration` en lugar de
`it_works`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

¡Agreguemos otra prueba, pero esta vez haremos una prueba que falla! Las pruebas fallan
cuando algo en la función de prueba entra en pánico. Cada prueba se ejecuta en un hilo nuevo,
y cuando el hilo principal ve que un hilo de prueba ha muerto, la prueba se marca
como fallida. Hablamos sobre la forma más sencilla de causar pánico en el Capítulo 9,
que es llamar a la macro `¡pánico!`. Ingrese la nueva prueba, `another`, por lo que su
archivo *src/lib.rs* se parecerá al Listado 11-3.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs:here}}
```

<span class="caption">Listado 11-3: Agregar una segunda prueba que fallará porque
llamamos a la macro `panic!`</span>

Ejecute las pruebas de nuevo usando `cargo test`. La salida debería verse como el Listado
11-4, que muestra que nuestra prueba `exploration` pasó y `another` falló.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

<span class="caption">Listado 11-4: Resultados de la prueba cuando una prueba pasa y otra
prueba falla</span>

En lugar de `ok`, la línea `test tests::another` muestra `FAILED`. 
Aparecen dos nuevas secciones entre los resultados individuales y el resumen: la primera
sección muestra la razón detallada de cada fallo de la prueba. En este caso,
`another` falló porque entró en pánico en `Make this test fail`, lo que sucedió
en la línea 10 del archivo *src/lib.rs*. La siguiente sección enumera solo los nombres de
todas las pruebas fallidas, lo cual es útil cuando hay muchas pruebas y mucha
salida detallada de la prueba que falla. Podemos usar el nombre de una prueba fallida para ejecutar solo
esa prueba para depurarlo más fácilmente; hablaremos más sobre las formas de ejecutar pruebas en
la sección ["Control de cómo se ejecutan las pruebas"][controlando-cómo-se-ejecutan-las-pruebas]<!-- ignore
-->

La línea de resumen se muestra al final: en general, el resultado de nuestra prueba es `FAILED`.
Tuvimos una prueba aprobada y una prueba fallida.

Ahora que ha visto cómo se ven los resultados de la prueba en diferentes escenarios,
veamos algunas macros distintas de `panic!` que son útiles en las pruebas.

### Comprobación de resultados con la macro `assert!`

La macro `assert!`, proporcionada por la biblioteca estándar, es útil cuando desea
asegurarse de que alguna condición en una prueba se evalúe como `verdadera`. Le damos a
la macro `assert!` un argumento que se evalúa como un booleano. Si el valor es
`true`, `assert!` no hace nada y la prueba pasa. Si el valor es `false`,
la macro `assert!` llama a la macro `panic!`, lo que hace que la prueba falle.
El uso de la macro `assert!` Nos ayuda a comprobar que nuestro código funciona en la
manera que pretendemos.

En el Capítulo 5, Listado 5-15, usamos una estructura `Rectangle` y un método `can_hold`,
que se repiten aquí en el Listado 11-5. Pongamos este código en
*src/lib.rs* y escribiremos algunas pruebas usando la macro `assert!`.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs:here}}
```

<span class="caption">Listado 11-5: Uso de la estructura `Rectangle` y su
método `can_hold` del Capítulo 5</span>

El método `can_hold` devuelve un booleano, lo que significa que es un caso de uso perfecto
para la macro `assert!`. En el Listado 11-6, escribimos una prueba sobre el
método `can_hold` mediante la creación de una instancia de `Rectangle` que tiene un ancho de 8 y
una altura de 7 y afirmando que puede contener otra instancia de `Rectangle` que
tiene una anchura de 5 y una altura de 1.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

<span class="caption">Listado 11-6: una prueba para `can_hold` que verifica si un
un rectángulo más grande puede contener un rectángulo más pequeño</span>

Tenga en cuenta que hemos agregado una nueva línea dentro del módulo `tests`; `use super::*;`.
El módulo `tests` es un módulo regular que sigue las reglas de visibilidad habituales
que cubrimos en el Capítulo 7 en la sección ["Rutas para hacer referencia a un elemento en el módulo
Árbol ”] [rutas-para-hacer-referencia-a-un-elemento-en-el-árbol-del-módulo]<!-- ignore -->.
Debido a que el módulo `tests` es un módulo interno, necesitamos traer el
código bajo prueba en el módulo externo en el alcance del módulo interno. Usamos
un glob aquí para que todo lo que definamos en el módulo externo esté disponible para este
módulo `tests`.

Hemos nombrado a nuestra prueba `larger_can_hold_smaller` y hemos creado las dos
instancias de `Rectangle` que necesitamos. Luego llamamos a la macro `assert!` y
pasamos el resultado de llamar a `larger.can_hold(&smaller)`. Esta expresión
se supone que devuelve `verdadero`, por lo que nuestra prueba debería pasar. ¡Vamos a averiguarlo!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

¡Pasa! Agreguemos otra prueba, esta vez afirmando que un
rectángulo menor no puede contener un rectángulo más grande:

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

Debido a que el resultado correcto de la función `can_hold` en este caso es `false`,
necesitamos negar ese resultado antes de pasarlo a la macro `assert!`. Como
resultado, nuestra prueba pasará si `can_hold` devuelve` false`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

¡Dos pruebas que pasan! Ahora veamos qué sucede con los resultados de nuestras pruebas cuando
introducimos un error en nuestro código. Cambiemos la implementación del método `can_hold`
reemplazando el signo "mayor que" con un signo "menor que" cuando
comparamos los anchos:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

La ejecución de las pruebas ahora produce lo siguiente:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

¡Nuestras pruebas detectaron el error! Porque `larger.width` es 8 y `smaller.width` es
5, la comparación de los anchos en `can_hold` ahora devuelve `false`: 8 no es
menos de 5.

### Prueba de igualdad con las macros `assert_eq!` y `assert_ne!`

Una forma común de probar la funcionalidad es comparar el resultado del código en
prueba con el valor que se espera que devuelva el código para asegurar que sean iguales.
Podrá hacer esto usando la macro `assert!` y pasándole una expresión usando el operador
`==`. Sin embargo, esta es una prueba tan común que la biblioteca estándar
proporciona un par de macros, `assert_eq!` y `assert_ne!`, para realizar esta prueba
más convenientemente. Estas macros comparan dos argumentos a favor de la igualdad o
desigualdad, respectivamente. También imprimirán los dos valores si la aserción
falla, lo que hace que sea más fácil ver *por qué* falló la prueba; a la inversa,
la macro `assert!` solo indica que obtuvo un valor `false` para la expresión `==`,
no los valores que conducen al valor `false`.

En el Listado 11-7, escribimos una función llamada `add_two` que agrega `2` a su
parámetro y devuelve el resultado. Luego probamos esta función usando la
macro `assert_eq!`.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

<span class="caption">Listado 11-7: Prueba de la función `add_two` usando la
macro `assert_eq!`</span>

¡Comprobemos que pasa!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

El primer argumento que le dimos a la macro `assert_eq!`, `4`, es igual al
resultado de llamar a `add_two(2)`. La línea para esta prueba es `testtests::it_adds_two ... ok`, 
y el texto `ok` indica que nuestra prueba pasó.

Introduzcamos un error en nuestro código para ver cómo se ve cuando se realiza una prueba
utiliza `assert_eq!` que falla. Cambie la implementación de la función `add_two` y
en su lugar agregue `3`:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

Ejecute las pruebas nuevamente:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

¡Nuestra prueba detectó el error! La prueba `it_adds_two` falló, mostrando el mensaje
`` assertion failed: `(left == right)` `` y mostrando que `left` era `4` y
`right` era `5`. Este mensaje es útil y nos ayuda a comenzar a depurar: significa
el argumento `left` para` assert_eq! `era `4` pero el argumento `right`, donde
tenía `add_two(2)`, era `5`.

Tenga en cuenta que en algunos lenguajes y marcos de prueba, los parámetros de
las funciones que afirman que dos valores son iguales se denominan "esperadas" y "reales",
y el orden en el que especificamos los argumentos es importante. Sin embargo, en Rust,
se llaman `left` y `right`, y el orden en el que especificamos el valor
que esperamos y el valor que produce el código bajo prueba no importa. Nosotros
podría escribir la aserción en esta prueba como `assert_eq!(add_two(2), 4)`, que
daría como resultado un mensaje de error que mostraria `` assertion failed: `(left == right)` `` 
y que `left` era `5` y `right` era `4`.

La macro `assert_ne!` pasará si los dos valores que le damos no son iguales y
fallará si son iguales. Esta macro es más útil para los casos en los que no estamos seguros
qué valor *será*, pero sabemos definitivamente cuál  *no* será el valor si nuestro
código está funcionando como pretendemos. Por ejemplo, si estamos probando una función que
se garantiza que cambiará su entrada de alguna manera, pero la forma en que la entrada
se cambia depende del día de la semana en que realizamos nuestras pruebas, la mejor cosa
a afirmar podría ser que la salida de la función no es igual a la entrada.

Debajo de la superficie, las macros `assert_eq!` y `assert_ne!` usan los operadores
`==` y `!=`, respectivamente. Cuando las afirmaciones fallan, estas macros imprimen sus
argumentos que utiliza el formato de depuración, lo que significa que los valores que se comparan deben
implementar los traits `PartialEq` y `Debug`. Todos los tipos primitivos y la mayoría
de los tipos de bibliotecas estándar implementan estos traits. Para las estructuras y enumeraciones
que defina, deberá implementar `PartialEq` para afirmar que los valores de
esos tipos son iguales o no iguales. Deberá implementar `Debug` para imprimir
los valores cuando falla la aserción. Debido a que ambos traits son derivables,
como se menciona en el Listado 5-12 en el Capítulo 5, esto suele ser tan sencillo
como agregar la anotación `#[derive(PartialEq, Debug)]` a su definición de estructura o enumeración.
Consulte el Apéndice C, [“Traits derivables”][derivable-traits]<!-- ignore -->
para obtener más detalles sobre estos y otros traits derivables.

### Adición de mensajes de error personalizados

También puede agregar un mensaje personalizado para que se imprima con el mensaje de fallo como
argumentos opcionales para las macros `assert!`, `assert_eq!` y `assert_ne!`. Cualquier
argumento especificado después del único argumento requerido para `assert!` o los dos
argumentos requeridos para `assert_eq!` y `assert_ne!` se pasan a la
macro `format!` (discutido en el Capítulo 8 en la sección [“Concatenación con elOperador `+`
o la Macro `formato!`”][concatenation-with-the--operator-or-the-format-macro]<!-- ignore -->) ,
para que pueda pasar una cadena de formato que contenga marcadores de posición `{}` y
valores para ir en esos marcadores de posición. Los mensajes personalizados son útiles para documentar
lo que significa una afirmación; cuando falla una prueba, tendrá una mejor idea de que
problema hay en el código.

Por ejemplo, supongamos que tenemos una función que saluda a las personas por su nombre y nosotros
queremos probar que el nombre que pasamos a la función aparece en la salida:

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

Los requisitos para este programa aún no se han acordado y estamos
bastante seguros de que el texto `Hola` al comienzo del saludo cambiará. Nosotros
decidimos que no queremos tener que actualizar la prueba cuando cambien los requisitos,
así que en lugar de verificar la igualdad exacta con el valor devuelto por la
función `greeting`, simplemente afirmaremos que la salida contiene el texto del
parámetro de entrada.

Introduzcamos un error en este código cambiando `greeting` para que no incluya
`name` para ver cómo se ve este error de prueba:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

La ejecución de esta prueba produce lo siguiente:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

Este resultado solo indica que la aserción falló y en qué línea
la afirmación está activada. Un mensaje de fallo más útil en este caso imprimiría el
valor que obtuvimos de la función `greeting`. Cambiemos la función de prueba,
dándole un mensaje de fallo personalizado hecho a partir de una cadena de formato con un marcador de posición
completado con el valor real que obtuvimos de la función `greeting`:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

Ahora, cuando ejecutemos la prueba, obtendremos un mensaje de error más informativo:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

Podemos ver el valor que obtuvimos realmente en la salida de prueba, lo que nos ayudaría
a depurar lo que sucedió en lugar de lo que esperábamos que sucediera.

### Comprobación de pánico con `should_panic`

Además de comprobar que nuestro código devuelve los valores correctos que esperamos,
también es importante comprobar que maneja las condiciones de error como se espera.
Por ejemplo, considere el tipo `Guess` que creamos en el Capítulo 9,
Listado 9-10. Otro código que usa `Guess` depende de la garantía de que
las instancias de `Guess` contendrán solo valores entre 1 y 100. Podemos escribir una prueba que
asegura que intentar crear una instancia de `Guess` con un valor fuera de ese rango
produce pánico.

Hacemos esto agregando otro atributo, `should_panic`, a nuestra función de prueba.
Este atributo hace un pase de prueba si el código dentro de la función entra en pánico;
la prueba fallará si el código dentro de la función no entra en pánico.

El Listado 11-8 muestra una prueba que verifica que las condiciones de error de `Guess::new`
suceden cuando esperamos que sucedan.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

<span class="caption">Listado 11-8: Prueba de que una condición provocará un
`panic!`</span>

Colocamos el atributo `#[should_panic]` después del atributo `#[test]` y
antes de la función de prueba a la que se aplica. Veamos el resultado cuando esta prueba
pasa:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

¡Se ve bien! Ahora introduzcamos un error en nuestro código eliminando la condición
que la función `new` entrará en pánico si el valor es mayor que 100:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

Cuando ejecutamos la prueba en el Listado 11-8, fallará:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

No recibimos un mensaje muy útil en este caso, pero cuando miramos la función de prueba,
vemos que está anotado con `#[should_panic]`. El fallo que tenemos
significa que el código en la función de prueba no causó pánico.

Las pruebas que usan `should_panic` pueden ser imprecisas porque solo indican que
el código ha provocado cierto pánico. Una prueba `should_panic` pasaría incluso si
el pánico es por una razón diferente a la que esperábamos que sucediera. Para
hacer que las pruebas `should_panic` sean más precisas, podemos agregar un parámetro `expected` opcional
al atributo `should_panic`. El arnés de prueba se asegurará de que
el mensaje de error contiene el texto proporcionado. Por ejemplo, considere el
código modificado para `Guess` en el Listado 11-9 donde la función` new` entra en pánico con
diferentes mensajes dependiendo de si el valor es demasiado pequeño o demasiado grande.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

<span class="caption">Listado 11-9: Prueba de que una condición provocará una
`panic!` con un mensaje de pánico en particular</span>

Esta prueba pasará porque el valor que pusimos en el parámetro `expected` del atributo `should_panic`
es una subcadena del mensaje que la función `Guess::new`
con pánico. Podríamos haber especificado todo el mensaje de pánico que
esperar, que en este caso sería `Guess value must be less than or equal to 100, got 200.` 
Lo que elija especificar en el parámetro esperado para
`should_panic` depende de cuánto del mensaje de pánico es único o dinámico
y qué tan precisa desea que sea su prueba. En este caso, una subcadena del
El mensaje de pánico es suficiente para garantizar que el código de la función de prueba se ejecute
el caso `else if value > 100`.

Para ver qué sucede cuando una prueba `should_panic` con un mensaje `expected`
falla, introduzcamos de nuevo un error en nuestro código intercambiando los cuerpos de los bloques
`if value < 1` y `else if value > 100`:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

Esta vez, cuando ejecutamos la prueba `should_panic`, fallará:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

El mensaje de falla indica que esta prueba realmente entró en pánico como esperábamos,
pero el mensaje de pánico no incluía la cadena esperada `'Guess value must be less than or equal to 100'`. 
El mensaje de pánico que recibimos en este caso fue
`Guess value must be greater than or equal to 1, got 200.` Ahora podemos empezar a
averiguar dónde está nuestro error!

### Uso de `Result<T, E>` en pruebas

Hasta ahora, hemos escrito pruebas que entran en pánico cuando fallan. También podemos escribir pruebas
que usan `Result<T, E>`! Aquí está la prueba del Listado 11-1, reescrita para usar
`Result<T, E>` y devuelve un `Err` en lugar de entrar en pánico:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs}}
```

La función `it_works` ahora tiene un tipo de retorno, `Result<(), String>` en el
cuerpo de la función, en lugar de llamar a la macro `assert_eq!`, devolvemos
`Ok(())` cuando la prueba pasa y un `Err` con una `String` adentro cuando la prueba
falla.

Escribir pruebas para que devuelvan un `Result<T, E>` le permite usar la pregunta
Marcar al operador en el cuerpo de las pruebas, que puede ser una forma conveniente de escribir
pruebas que deberían fallar si alguna operación dentro de ellas devuelve una variante `Err`.

No puede usar la anotación `#[should_panic]` en pruebas que usan `Result <T, E>`. 
En su lugar, debe devolver un valor `Err` directamente cuando la prueba debería
fallar.

Ahora que conoce varias formas de escribir pruebas, veamos lo que está sucediendo
cuando ejecutamos nuestras pruebas y exploramos las diferentes opciones que podemos usar con `cargo test`.

[concatenación-con-el-operador-o-la-macro-de-formato]:ch08-02-strings.html#concatenación-con-el-operador-o-la-macro-de-formato
[controlando-cómo-se-ejecutan-las-pruebas]:ch11-02-running-tests.html#controlando-cómo-se-ejecutan-las-pruebas
[rasgos-derivables]: apéndice-03-rasgos-derivables.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests
[rutas-para-hacer-referencia-a-un-elemento-en-el-árbol-de-módulos]: ch07-03-rutas-para-hacer-referencia-a-un-elemento-en-el-árbol-de-módulos.html

