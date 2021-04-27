# Escribir pruebas automatizadas

En su ensayo de 1972 "The Humble Programmer", Edsger W. Dijkstra dijo que
"Las pruebas de programas pueden ser una forma muy eficaz de mostrar la presencia de errores, pero
es desesperadamente inadecuado para mostrar su ausencia". Eso no significa que
¡No debería intentar probar tanto como podamos!

La corrección de nuestros programas está medida en que nuestro código hace lo que pretendemos
que debe hacer. Rust está diseñado con un alto grado de obsesión por la corrección
de los programas, pero la corrección es compleja y no es fácil de probar.
El sistema de tipos de Rust soporta una gran parte de esta carga, pero el sistema de tipos no puede atrapar
todo tipo de incorrecciones. Como tal, Rust incluye soporte para escribir
pruebas de software automatizadas dentro del lenguaje.

Como ejemplo, digamos que escribimos una función llamada `add_two` que suma 2 
al número que se le pase. La declaración de esta función acepta un número entero como
parámetro y devuelve un número entero como resultado. Cuando implementamos y compilamos
esa función, Rust realiza todas las comprobaciones de tipos y comprueba los prestamos que hemos
aprendido hasta ahora para asegurarnos de que, por ejemplo, no estamos pasando un valor `String`
o una referencia no válida a esta función. Pero Rust *no puede* comprobar que
la función hará precisamente lo que pretendemos, que es devolver el parámetro más 2
en lugar de, digamos, el parámetro más 10 o el parámetro menos 50! Ahí es donde
vienen las pruebas.

Podemos escribir pruebas que afirmen, por ejemplo, que cuando pasamos `3` a la
función `add_two`, el valor devuelto es `5`. Podemos ejecutar estas pruebas siempre que
realizemos cambios en nuestro código para asegurarnos de que no se haya producido ningún 
cambió en el comportamiento correcto existente.

La prueba es una habilidad compleja: aunque no podemos cubrir todos los detalles sobre cómo
escribir buenas pruebas en un capítulo, discutiremos la mecánica de las pruebas de Rust.
Hablaremos sobre las anotaciones y macros disponibles, cuando
escribir sus pruebas, el comportamiento predeterminado y las opciones proporcionadas para ejecutar su
pruebas y cómo organizar pruebas en pruebas unitarias y pruebas de integración.

