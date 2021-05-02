# Enumeraciones y Coincidencia de Patrones

En este capítulo veremos las *enumeraciones*, también conocidas como *enums*.
Las enumeraciones le permiten definir un tipo enumerando sus posibles *variantes*. Primero,
definiremos y usaremos una enumeración para mostrar cómo pueden codificar el significado junto con
los datos. A continuación, exploraremos una enumeración particularmente útil, llamada `Option`, que
expresa que un valor puede ser algo o nada. Despues veremos
cómo la coincidencia de patrones en la expresión `match` facilita la ejecución de diferente
código para diferentes valores de una enumeración. Finalmente, cubriremos cómo la construccion "if let"
es otra forma disponible, conveniente y concisa, para manejar enumeraciones en su código.

Las enumeraciones son una característica en muchos lenguajes, pero sus capacidades difieren en cada uno.
Las enumeraciones de Rust son más similares a *tipos de datos algebraicos* en lenguajes funcionales,
como F#, OCaml y Haskell.
