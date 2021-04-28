# Introducción

> Nota: Esta es la traducción del libro [The Rust Programming
> Language][nsprust] . La version en Español no está disponible en formato impreso.

[nsprust]: https://nostarch.com/rust

Bienvenido a *El Lenguaje de Programación Rust*, un libro introductorio sobre Rust.
El lenguaje de programación Rust le ayudara a escribir software más rápido y confiable.
La ergonomía de alto nivel y el control de bajo nivel a menudo están en desacuerdo en el 
diseño de lenguajes de programación; Rust desafía ese conflicto. A través del equilibrio 
entre una poderosa capacidad técnica y una grata experiencia durante el desarrollo, Rust 
le da la opción de controlar los detalles de bajo nivel (como el uso de la memoria) sin 
todas las molestias tradicionalmente asociadas con dicho control.

## Para Quién Es Rust

Rust es ideal para muchas personas por diversas razones. Veamos algunos de los 
grupos más importantes.

### Equipos de Desarrolladores

Rust está demostrando ser una herramienta productiva para colaborar entre grandes equipos de
desarrolladores con distintos niveles de conocimientos de programación de sistemas. El código 
de bajo nivel es propenso a una variedad de errores sutiles, que en la mayoría de lenguajes pueden ser
capturados sólo a través de pruebas exhaustivas y la revisión cuidadosa del código por 
desarrolladores experimentados. En Rust, el compilador juega un papel de guardián al negarse a
compilar código con estos errores esquivos, incluidos los errores de concurrencia. Trabajando
junto con el compilador, el equipo puede dedicar su tiempo a centrarse en la lógica del programa
en lugar de perseguir errores.

Rust también brinda herramientas de desarrollo coetáneas al mundo de la programación de sistemas:

* *Cargo*, el administrador de dependencias y la herramienta de compilación, hace que agregar,
  compilar y gestionar dependencias sea sencillo y coherente en todo el ecosistema Rust.
* *Rustfmt* garantiza un estilo de codificación coherente entre los desarrolladores.
* *Rust Language Server* impulsa la integración con entornos de desarrollo integrado (IDE) para 
  completar el código y dar mensajes de error en línea.

Al utilizar estas y otras herramientas en el ecosistema Rust, los desarrolladores pueden ser
productivos al escribir código a nivel de sistemas.

### Estudiantes

Rust es para estudiantes y aquellos que estén interesados en aprender sobre conceptos de sistemas.
Con Rust, muchas personas han aprendido sobre temas como desarrollo de sistemas operativos. La comunidad 
es muy acogedora y presta a responder preguntas de los estudiantes. A través de esfuerzos como este libro, 
los equipos de Rust quieren hacer que los conceptos de sistemas sean más accesibles para más personas, 
especialmente para los nuevos en programación.

### Compañías

Cientos de empresas, grandes y pequeñas, utilizan Rust en la producción de gran variedad de tareas. 
Esas tareas incluyen herramientas de línea de comandos, servicios web, herramientas DevOps,
dispositivos integrados, análisis y transcodificación de audio y vídeo, criptomonedas,
bioinformática, motores de búsqueda, aplicaciones de Internet de las Cosas, "machine learning", 
e incluso partes importantes del navegador web Firefox.

### Desarrolladores de Código Abierto

Rust es para personas que quieren construir el lenguaje de programación Rust, la comunidad,
las herramientas de desarrollo y las bibliotecas. Nos encantaría que contribuyeses al lenguaje Rust.

### Personas que Valoran la Velocidad y la Estabilidad

Rust es para personas que anhelan velocidad y estabilidad en un lenguaje. Por velocidad, nosotros
entendemos la velocidad de los programas que se pueden crear con Rust y la velocidad a la que Rust 
le permite escribirlos. Las comprobaciones del compilador de Rust garantizan la estabilidad
mediante la incorporación de características y la refactorización. Esto contrasta con el frágil
código heredado en lenguajes sin estas comprobaciones, que los desarrolladores suelen no modificar 
por temor. Al esforzarse por lograr abstracciones de coste cero, las características de nivel superior 
se compilan en código de bajo nivel tan rápido como el escrito manualmente; Rust 
se esfuerza por hacer que el código seguro sea también código rápido.

El lenguaje Rust espera ser compatible también con muchos otros usuarios; los mencionados son solo 
algunos de los principales interesados. En general, la mayor ambición de Rust es eliminar las 
concesiones que los programadores han aceptado durante décadas, brindando seguridad *y* productividad, 
velocidad *y* ergonomía. Pruebe Rust y compruebe si sus opciones funcionan para usted.

## Para Quien Es Este Libro

Este libro asume que ha escrito código en otro lenguaje de programación pero no hace ninguna 
suposición sobre cuál. Intentamos hacer el material ampliamente accesible para aquellos con una amplia 
variedad de antecedentes en programación. No dedicaremos mucho tiempo a hablar sobre qué *es* la 
programación o cómo pensar sobre ella. Si es completamente nuevo en programación, estaría mejor leer 
un libro que proporcione específicamente una introducción a la programación.

## Como Usar Este Libro

En general, este libro asume que lo está leyendo en secuencia, desde el principio hasta el final. 
Los capítulos se basan en conceptos de capítulos anteriores y es posible que los capítulos no profundicen 
en los detalles de un tema; normalmente volvemos a visitar el tema en un capítulo posterior.

Encontrará dos tipos de capítulos en este libro: capítulos de conceptos y capítulos de proyectos. En 
los capítulos de conceptos, aprenderá sobre aspectos de Rust. En los capítulos de proyectos crearemos pequeños 
programas, aplicando lo que ha aprendido antes. Los capítulos 2, 12 y 20 son capítulos de proyectos; el resto 
son capítulos de conceptos.
El capítulo 1 explica cómo instalar Rust, cómo escribir un  programa "¡Hola, mundo!",
y cómo utilizar Cargo, el administrador de paquetes y la herramienta de compilación de Rust. 
El capítulo 2 es una introducción práctica al lenguaje Rust. Aquí cubrimos conceptos de alto
nivel, y los capítulos posteriores proporcionarán detalles adicionales. Si quiere ensuciarse las
manos de inmediato, el Capítulo 2 es el lugar para eso. Al principio, incluso
podría querer omitir el Capítulo 3, que cubre características de Rust similares a las de otros 
lenguajes de programación, y dirigirse directamente al Capítulo 4 para aprender sobre el sistema 
de propiedad de Rust. Sin embargo, si usted es un aprendiz particularmente meticuloso, que prefiere aprender 
cada detalle antes de pasar al siguiente, es posible que desee omitir el Capítulo 2 e ir directamente 
al Capítulo 3, volviendo al Capítulo 2 cuando quiera trabajar en un proyecto aplicando los detalles 
que ha aprendido.

El Capítulo 5 discute estructuras y métodos y el Capítulo 6 cubre enumeraciones, expresiones `match` y 
la construcción de control de flujo "if let". Usará estructuras y enumeraciones para hacer tipos 
personalizados en Rust.

En el Capítulo 7, aprenderá sobre el sistema de módulos de Rust y sobre las reglas de privacidad 
para organizar el código y su interfaz de programación de aplicaciones (API) pública. El capítulo 8 analiza 
algunas estructuras comunes de recopilación de datos que proporciona la biblioteca estándar, como vectores, 
cadenas y mapas hash. El Capítulo 9 explora la filosofía y las técnicas de manejo de errores de Rust.

El capítulo 10 profundiza en genéricos, traits y tiempos de vida, que le dan el poder para definir el código 
que se aplica a varios tipos. El capítulo 11 trata sobre las pruebas, que incluso con las garantías de 
seguridad de Rust son necesarias para garantizar que la lógica del programa sea correcta. En el Capítulo 12, 
crearemos nuestra propia implementación de un subconjunto de la funcionalidad de la herramienta de línea 
de comandos `grep` que busca texto dentro de los archivos. Para esto, usaremos muchos de los conceptos 
que discutimos en capítulos anteriores.

El capítulo 13 explora cierres (closures) e iteradores, características de Rust que provienen de lenguajes de 
programación funcionales. En el Capítulo 14, examinaremos Cargo en más profundidad y hablaremos 
sobre las mejores prácticas para compartir bibliotecas con otros.
El capítulo 15 analiza los punteros inteligentes que proporciona la biblioteca estándar y los traits 
que habilitan su funcionalidad.

En el Capítulo 16 veremos diferentes modelos de programación concurrente y hablaremos sobre cómo 
Rust ayuda a programar múltiples subprocesos sin miedo. El capítulo 17 analiza cómo se compara Rust 
con la programación orientada a objetos, principios con los que quizás puede estar familiarizado.

El capítulo 18 es una referencia sobre patrones y coincidencia de patrones, que son poderosas formas 
de expresar ideas a través de los programas Rust. El capítulo 19 contiene una mezcla heterogénea de 
temas avanzados de interés, incluidos Rust inseguro, macros y más sobre vidas, traits, tipos, funciones 
y cierres.

En el Capítulo 20, completaremos un proyecto en el que implementaremos un servidor web multiproceso!

Por último, algunos apéndices contienen información útil sobre el lenguaje en un formato más parecido a una referencia. 
El Apéndice A cubre las palabras clave de Rust, el Apéndice B
cubre los operadores y símbolos de Rust, el Apéndice C cubre los traits derivables
proporcionados por la biblioteca estándar, el Apéndice D cubre algunas herramientas de desarrollo útiles
y el Apéndice E explica las ediciones de Rust.

<span id="ferris"></span>

Una parte importante del proceso de aprendizaje de Rust es aprender a leer mensajes de error que muestra el 
compilador; estos le guiarán en el trabajo sobre el código.
Por ello, proporcionaremos muchos ejemplos que no se compilan junto con el mensaje de error que el compilador 
mostrará en cada situación. Debe saber que si entra y ejecuta un ejemplo aleatorio, ¡puede que no se 
compile! Asegúrese de leer el texto circundante para ver si el ejemplo que está intentando ejecutar está 
destinado a provocar intencionalmente un error. Ferris (nuestra mascota) también lo ayudará a distinguir el código que no 
está destinado a funcionar:

| Ferris                                                                 | Significado                                      |
|------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain"/>    |        ¡Este código no se compila!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain"/>              |        ¡Este código entra en pánico!                    |
| <img src="img/ferris/unsafe.svg" class="ferris-explain"/>              |        Este bloque de código contiene código inseguro.  |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain"/>|        Este código no produce el comportamiento deseado.|


En la mayoría de las situaciones, le llevaremos a la versión correcta de cualquier código que
no se compila.

## Código Fuente

Los archivos fuente a partir de los cuales se genera este libro se pueden encontrar en
[GitHub][book].

[book]: https://github.com/rust-lang/book/tree/master/src

