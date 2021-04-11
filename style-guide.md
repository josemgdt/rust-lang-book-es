# Guia de Estilos

## Prosa

* Se prefiere el título en mayúsculas y minúsculas para los títulos de los 
  capítulos/secciones, por ejemplo: `## Generación de un Número Secreto` 
  en lugar de `## Generación de un número secreto`.
* Se prefiere cursiva a comillas simples cuando se llame un término, por 
  ejemplo: `es una  *función asociada* de` en lugar de `es una 'función asociada' de`.
* Cuando en la prosa mencione un método, NO incluya el paréntesis, por ejemplo:
   `read_line` en lugar de `read_line()`.
* Limitar a 80 caracteres.
* Se prefiere no mezclar código y no código en una palabra, por ejemplo: ``Recuerda cuando escribimos
   `use std::io`?`` en lugar de ``¿Recuerdas cuando `use`d `std::io`?`` 

## Código

* Agregue el nombre del archivo antes de los bloques markdown para dejar claro de qué archivo 
   estamos hablando, cuando corresponda.
* Al realizar cambios en el código, aclare qué partes del código cambiaron y cuáles permanecieron                                            
   igual ... no estoy seguro de cómo hacerlo todavía
* Divida las líneas largas según corresponda para mantenerlas por debajo de 80 caracteres si es 
   posible
* Utilice la sintaxis de resaltado de `bash` para bloques de código de salida de línea de comando

## Links

Once all the scripts are done:

* If a link shouldn't be printed, mark it to be ignored
  * This includes all "Chapter XX" intra-book links, which *should* be links
    for the HTML version
* Make intra-book links and stdlib API doc links relative so they work whether
  the book is read offline or on docs.rust-lang.org
* Use markdown links and keep in mind that they will be changed into `text at
  *url*` in print, so word them in a way that it reads well in that format
