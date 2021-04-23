## Separacion de módulos en diferentes archivos

Hasta ahora, todos los ejemplos de este capítulo definían varios módulos en un archivo.
Cuando los módulos crecen, es posible que desee mover sus definiciones a archivos separados
para facilitar la navegación por el código.

Por ejemplo, comencemos con el código del Listado 7-17 y mueva el módulo
`front_of_house` a su propio archivo *src/front_of_house.rs* cambiando el
archivo crate raíz para que contenga el código que se muestra en el Listado 7-21. En este caso,
el archivo raíz de la caja es *src/lib.rs*, pero este procedimiento también funciona con cajas binarias
cuyo archivo caja raíz es *src/main.rs*.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<span class="caption">Listado 7-21: Declarando el módulo `front_of_house` cuyo
el cuerpo estará en *src/front_of_house.rs*</span>

Y *src/front_of_house.rs* obtiene las definiciones del cuerpo del
módulo `front_of_house`, como se muestra en el Listado 7-22.


<span class="filename">​​Nombre de archivo: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

<span class="caption">Listado 7-22: Definiciones dentro del módulo `front_of_house`
en *src/front_of_house.rs*</span>

Usar un punto y coma después de `mod front_of_house` en lugar de usar un bloque le indica
a Rust que cargue el contenido del módulo desde otro archivo con el mismo nombre que
el módulo. Para continuar con nuestro ejemplo y extraer el módulo `hosting` a
su propio archivo también, cambiamos *src/front_of_house.rs* para contener solo la
declaración del módulo `hosting`:

<span class="filename">​​Nombre de archivo: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

Luego creamos un directorio *src/front_of_house* y un archivo
*src/front_of_house/hosting.rs* para contener las definiciones hechas en el
módulo `hosting`:

<span class="filename">​​Nombre de archivo: src/front_of_house/hosting.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

El árbol de módulos sigue siendo el mismo y las llamadas a función en `eat_at_restaurant`
funcionarán sin ninguna modificación, a pesar de que las definiciones viven en
diferentes archivos. Esta técnica le permite mover módulos a nuevos archivos a medida que crecen
en tamaño.

Tenga en cuenta que la declaración `pub use crate::front_of_house::hosting` en
*src/lib.rs* tampoco ha cambiado, ni `use` tiene ningún impacto en qué archivos
se compilan como parte de la caja. La palabra clave `mod` declara módulos y Rust
busca en un archivo con el mismo nombre que el módulo para el código que entra en
ese módulo.

## Resumen

Rust le permite dividir un paquete en varias cajas y una caja en módulos
para que pueda consultar los elementos definidos en un módulo desde otro módulo. Puede hacer
esto especificando rutas absolutas o relativas. Estos caminos se pueden llevar a
alcance con una declaración `use` para que pueda usar una ruta más corta para múltiples usos del
item en ese alcance. El código del módulo es privado de forma predeterminada, pero puede hacer
definiciones públicas agregando la palabra clave `pub`.

En el próximo capítulo, veremos algunas estructuras de colección de datos en la
biblioteca estándar que puede utilizar en su código perfectamente organizado.
