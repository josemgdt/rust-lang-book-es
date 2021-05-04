## Definicion de Módulos para Controlar Alcance y Privacidad

En esta sección hablaremos sobre los módulos y otras partes del sistema de módulos;
las *rutas* que le permiten nombrar elementos, la palabra clave `use` que trae una
ruta dentro del alcance, y la palabra clave `pub` para hacer públicos los elementos. También discutiremos
la palabra clave `as`, paquetes externos y el operador glob. Por ahora, vamos
centrarnos en los módulos!

Los *módulos* nos permiten organizar el código dentro de una caja en grupos para facilitar la lectura y
una reutilización fácil. Los módulos también controlan la *privacidad* de los elementos, es decir, si un
elemento puede ser utilizado por código externo (*es público*) o es una implementación interna
de detalles y no está disponible para uso exterior (*es privado*).

Como ejemplo, escribamos una caja de biblioteca que proporcione la funcionalidad de un
restaurante. Definiremos las declaraciones de funciones pero dejaremos sus cuerpos
vacíos para concentrarnos en la organización del código, en lugar de implementar realmente
un restaurante en código.

En la industria de los restaurantes, algunas zonas de un restaurante se denominan
la *fachada o frente de la casa* y otras son la *parte de atras*. En el frente es donde se acomodan
los clientes; aquí es donde los recepcionistas acomodan a los clientes y los camareros reciben las comandas,
los pagos, y preparan las bebidas. En la parte trasera, los chefs y cocineros
trabajan en la cocina, los lavaplatos limpian y los gerentes hacen el trabajo administrativo.

Para estructurar nuestra caja de la misma manera que funciona un restaurante real, podemos
organizar las funciones en módulos anidados. Creemos una nueva biblioteca llamada
`restaurant` ejecutando `cargo new --lib restaurant`; luego, ponga el código del
Listado 7-1 en *src/lib.rs* para definir algunos módulos y definiciones de funciones.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

<span class="caption">Listado 7-1: El módulo `fachada` que contiene otros
módulos que luego contendrán funciones</span>

Definimos un módulo comenzando con la palabra clave `mod` y luego especificamos el
nombre del módulo (en este caso, `fachada`) y colocamos llaves
alrededor del cuerpo del módulo. Dentro de los módulos podemos tener otros módulos, como en
este caso con los módulos `recepcion` y `servicio`. Los módulos también pueden contener
definiciones de otros elementos, como estructuras, enumeraciones, constantes, traits o - como
en el Listado 7-1 - funciones.

Al usar módulos, podemos agrupar definiciones relacionadas y nombrar el por qué
están relacionadas. A los programadores que utilizan este código les resultará más fácil encontrar
las definiciones que quieran usar porque podían navegar por el código basándose
en los grupos en lugar de tener que leer todas las definiciones.
Los programadores que agregan nuevas funciones a este código sabrían dónde colocarlas
para mantener el programa organizado.

Anteriormente mencionamos que *src/main.rs* y *src/lib.rs* se llaman cajas
raíces. La razón de su nombre es que el contenido de cualquiera de estos dos
archivos forman un módulo llamado `crate` en la raíz de la estructura de módulos de la caja,
conocido como *árbol de módulos*.

El Listado 7-2 muestra el árbol de módulos para la estructura del Listado 7-1.

```text
crate
 └── fachada
     ├── recepcion
     │   ├── poner_en_espera
     │   └── adjudicar_mesa
     └── servicio
         ├── comanda
         ├── servir_comanda
         └── cobrar
```

<span class="caption">Listado 7-2: el árbol de módulos para el código en el Listado
7-1</span>

Este árbol muestra cómo algunos de los módulos se anidan unos dentro de otros (por ejemplo,
`recepcion` se anida dentro de `fachada`). El árbol también muestra que algunos módulos
son *hermanos* entre sí, lo que significa que están definidos en el mismo módulo
(`recepcion` y `servicio` se definen en `fachada`). Para continuar la
metáfora familiar, si el módulo A está contenido dentro del módulo B, decimos que el módulo A
es *hijo* del módulo B, que por tanto es el *padre* del módulo A.
Observe que todo el árbol de módulos tiene sus raíces en el módulo implícito denominado
`crate`.

El árbol de módulos puede recordarle el árbol de directorios del sistema de archivos en su
ordenador; ¡Esta es una comparación muy acertada! Al igual que los directorios en un sistema de archivos,
el árbol de módulos utiliza módulos para organizar su código. Y al igual que los archivos en un directorio,
necesitamos una forma de encontrar nuestros módulos.

