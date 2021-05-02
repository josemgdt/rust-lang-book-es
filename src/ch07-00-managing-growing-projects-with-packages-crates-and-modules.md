# Gestión de Proyectos en Crecimiento con Paquetes, Cajas y Módulos

A medida que escribe programas más grandes, será importante organizar su código porque
hacer un seguimiento de todo su programa en su cabeza será imposible.
Agrupar funcionalidades relacionadas y separar código con características distintas
aclarará dónde encontrar código que implemente una función en particular y
dónde ir para cambiar el funcionamiento de una función.

Los programas que hemos escrito hasta ahora han estado en un módulo, en un archivo. Cuando un
proyecto crece, puede organizar el código dividiéndolo en varios módulos de
varios archivos. Un paquete puede contener varias cajas binarias y,
opcionalmente, una caja de biblioteca. A medida que crece un paquete, puede extraer piezas en
cajas separadas, que se convierten en dependencias externas. Este capítulo cubre todas
estas técnicas. Para proyectos muy grandes de un conjunto de paquetes interrelacionados
que evolucionan juntos, Cargo proporciona espacios de trabajo, que cubriremos en la sección
[“Espacios de trabajo Cargo”][workspaces]<!-- ignore --> en el Capítulo 14.

Además de agrupar la funcionalidad, encapsular los detalles de implementación
le permite reutilizar el código en un nivel superior; una vez que haya implementado una operación,
otro código puede llamar a ese código a través de la interfaz pública del código sin saber
cómo funciona la implementación. La forma en que escribe el código define qué partes son
públicas para que lo use otro código y qué partes son detalles de implementación privados,
con el derecho a modificación reservado. Esta es otra forma de limitar la cantidad
de detalles que tiene que tener en la cabeza.

Un concepto relacionado es el alcance; el contexto anidado en el que se escribe el código tiene un
conjunto de nombres que se definen como "en alcance". Al leer, escribir y
compilar código, los programadores y compiladores necesitan saber si un
nombre en un lugar en particular se refiere a una variable, función, estructura, enumeración, módulo,
constante, u otro elemento y lo que ese elemento significa. Puede crear alcances y
cambiar los nombres que están dentro o fuera de un alcance. No puede tener dos elementos con el
mismo nombre en el mismo alcance; hay herramientas disponibles para resolver conflictos de nombres.

Rust tiene una serie de funciones que le permiten administrar la organización de su código,
incluidos qué detalles están expuestos, cuales son privados,
y qué nombres hay en cada alcance de sus programas. Estas características, a veces
colectivamente denominadas *sistema de módulos*, incluyen:

* **Paquetes:** Una función de Cargo que le permite construir, probar y compartir cajas
* **Cajas:** Un árbol de módulos que produce una biblioteca o un ejecutable
* **Módulos** y **use:** Le permiten controlar la organización, el alcance y la
  privacidad de las rutas
* **Rutas:** Una forma de nombrar un elemento, como una estructura, función o módulo

En este capítulo, cubriremos todas estas funciones, discutiremos cómo interactúan y
explicaremos cómo usarlos para administrar el alcance. Al final, debería tener una sólida
comprensión del sistema de módulos y ser capaz de trabajar con alcances como un profesional.

[workspaces]: ch14-03-cargo-workspaces.html
