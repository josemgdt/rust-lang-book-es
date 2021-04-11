# Guia de Estilos

## Prosa

* Se prefiere el título en mayúsculas y minúsculas para los títulos de los 
  capítulos/secciones, por ejemplo: `## Generación de un Número Secreto` 
  en lugar de `## Generación de un número secreto`.
* Se prefiere cursiva a comillas simples cuando se llame un término, por ejemplo: `es una 
  *función asociada* de` en lugar de `es una 'función asociada' de`.
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
  
## Enlaces

Una vez que todos los scripts estén terminados:

* Si un enlace no debe imprimirse, márquelo para ignorarlo
   * Esto incluye todos los enlaces dentro del libro de "Capítulo XX", que *deberían* ser enlaces 
     para la versión HTML
* Haga que los enlaces intra-libro y los enlaces de documentos de la API stdlib sean relativos
  para que funcionen tanto si el libro se lee sin conexión o en docs.rust-lang.org
* Utilice enlaces markdown y tenga en cuenta que se cambiarán a `texto en 
  *url*` en forma impresa, así que redactelos de una manera que se lea bien en ese formato
