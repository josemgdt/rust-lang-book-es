# Colecciones comunes

La biblioteca estándar de Rust incluye una serie de estructuras de datos muy útiles llamadas
*colecciones*. La mayoría de los otros tipos de datos representan un valor específico, pero
las colecciones pueden contener varios valores. A diferencia de la matriz y la tupla incorporadas,
los datos a los que apuntan estas colecciones se almacenan en el montón, lo que significa que
la cantidad de datos no necesita ser conocida en el momento de la compilación y puede crecer o
encoger a medida que se ejecuta el programa. Cada tipo de colección tiene diferentes capacidades
y costos, y elegir una apropiada para su situación actual es una
habilidad que desarrollará con el tiempo. En este capítulo, analizaremos tres
colecciones que se utilizan con mucha frecuencia en los programas de Rust:

* Un *vector* le permite almacenar un número variable de valores, uno al lado del otro.
* Una *string* es una colección de caracteres. Hemos mencionado el tipo `String`
  anteriormente, pero en este capítulo hablaremos de ello en profundidad.
* Un *mapa hash* le permite asociar un valor con una clave en particular. Es una
  implementación particular de la estructura de datos más general llamada *map*.

Para conocer los otros tipos de colecciones que proporciona la biblioteca estándar,
ver [la documentación][collections].

[collections]: ../std/collections/index.html

También discutiremos cómo crear y actualizar vectores, cadenas y mapas hash y
que hace a cada uno especial.
