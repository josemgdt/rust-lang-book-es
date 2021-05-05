# Manejo de Errores

El compromiso de Rust con la confiabilidad se extiende al manejo de errores. Los errores son un hecho
de vida en software, por lo que Rust tiene una serie de características para manejar situaciones
en las que algo sale mal. En muchos casos, Rust requiere que se reconozca
la posibilidad de un error y realizar alguna acción antes de que se compile el código.
Este requisito hace que su programa sea más sólido al garantizar que
descubrir errores y manejarlos adecuadamente antes de implementar su código
para producción!

Rust agrupa los errores en dos categorías principales: *recuperables* e *irrecuperables*.
Para un error recuperable, como un error de archivo no encontrado, es
razonable informar del problema al usuario y reintentar la operación.
Los errores irrecuperables son siempre síntomas de bugs, como intentar acceder a un
ubicación más allá del final de una matriz.

La mayoría de los lenguajes no distinguen entre estos dos tipos de errores y manejan
ambos de la misma forma, utilizando mecanismos como las excepciones. Rust no tiene
excepciones. En cambio, tiene el tipo `Result<T, E>` para errores recuperables y
la macro `panic!` que detiene la ejecución cuando el programa encuentra un
error irrecuperable. Este capítulo trata primero sobre cómo llamar `panic!` y luego habla
acerca de devolver valores `Result<T, E>`. Además, exploraremos las consideraciones al 
decidir si intentar recuperarse de un error o detener la ejecución.

