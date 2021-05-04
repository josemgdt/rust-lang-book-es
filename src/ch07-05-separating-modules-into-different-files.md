## Separación de Módulos en Diferentes Archivos

Hasta ahora, todos los ejemplos de este capítulo definían varios módulos en un archivo.
Cuando los módulos crecen, es posible que desee mover sus definiciones a archivos separados
para facilitar la navegación por el código.

Por ejemplo, comencemos con el código del Listado 7-17 y mueva el módulo
`fachada` a su propio archivo *src/fachada.rs* cambiando el
archivo crate raíz para que contenga el código que se muestra en el Listado 7-21. En este caso,
el archivo raíz de la caja es *src/lib.rs*, pero este procedimiento también funciona con cajas binarias
cuya caja raíz es *src/main.rs*.

<span class="filename">​​Nombre de archivo: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<span class="caption">Listado 7-21: Declarando el módulo `fachada` cuyo
el cuerpo estará en *src/fachada.rs*</span>

Y *src/fachada.rs* obtiene las definiciones del cuerpo del
módulo `fachada`, como se muestra en el Listado 7-22.


<span class="filename">​​Nombre de archivo: src/fachada.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/fachada.rs}}
```

<span class="caption">Listado 7-22: Definiciones dentro del módulo `fachada`
en *src/fachada.rs*</span>

Usar un punto y coma después de `mod fachada` en lugar de usar un bloque le indica
a Rust que cargue el contenido del módulo desde otro archivo con el mismo nombre que
el módulo. Para continuar con nuestro ejemplo y extraer el módulo `recepcion` a
su propio archivo también, cambiamos *src/fachada.rs* para contener solo la
declaración del módulo `recepcion`:

<span class="filename">​​Nombre de archivo: src/fachada.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/fachada.rs}}
```

Luego creamos un directorio *src/fachada* y un archivo
*src/front_of_house/recepcion.rs* para contener las definiciones hechas en el
módulo `recepcion`:

<span class="filename">​​Nombre de archivo: src/front_of_house/recepcion.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/fachada/recepcion.rs}}
```

El árbol de módulos sigue siendo el mismo y las llamadas a función en `comer_en_restaurant`
funcionarán sin ninguna modificación, a pesar de que las definiciones viven en
diferentes archivos. Esta técnica le permite mover módulos a nuevos archivos a medida que crecen
en tamaño.

Tenga en cuenta que la declaración `pub use crate::fachada::recepcion` en
*src/lib.rs* tampoco ha cambiado, ni `use` tiene ningún impacto en qué archivos
se compilan como parte de la caja. La palabra clave `mod` declara módulos y Rust
busca en un archivo con el mismo nombre que el módulo para el código que entra en
ese módulo.

## Resumen

Rust le permite dividir un paquete en varias cajas y una caja en módulos
para que pueda consultar los elementos definidos en un módulo desde otro módulo. Puede hacer
esto especificando rutas absolutas o relativas. Estas rutas se pueden llevar a
alcance con una declaración `use` para que pueda usar una ruta más corta para múltiples usos del
item en ese alcance. El código del módulo es privado de forma predeterminada, pero puede hacer
definiciones públicas agregando la palabra clave `pub`.

En el próximo capítulo, veremos algunas estructuras de colección de datos en la
biblioteca estándar que puede utilizar en su código perfectamente organizado.
