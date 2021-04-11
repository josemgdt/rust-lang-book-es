## ¡Hola Mundo!

Ahora que has instalado Rust, escribe tu primer programa. Es
tradicional al aprender un nuevo idioma escribir un pequeño programa que imprima
el texto "¡Hola, mundo!" en la pantalla, ¡así que aquí haremos lo mismo!

> Nota: Este libro presume una familiaridad básica con la línea de comandos. Rust no tiene
> demandas específicas sobre edición, herramientas o dónde guardas tu código, por lo que
> si prefieres utilizar un entorno de desarrollo integrado (IDE) en lugar de
> la línea de comandos, no dudes en utilizar tu IDE favorito. Muchos IDE tienen ahora algun
> grado de soporte de Rust; consulte la documentación del IDE para obtener más detalles. Recientemente,
> el equipo de Rust se ha centrado en permitir un buen soporte IDE y el progreso
> ¡se ha hecho rápidamente en ese frente!

### Creación de un Directorio de Proyectos

Empezarás por crear un directorio para almacenar tu código Rust. No importa
donde guardes el código, pero para los ejercicios y proyectos de este libro
sugerimos crear un directorio *projects* en tu directorio personal y mantener todos
los proyectos allí.

Abre una terminal e ingresa los siguientes comandos para crear un directorio *projects*
y un directorio para el  proyecto "¡Hola, mundo!" dentro de el.

Para Linux, macOS y PowerShell en Windows, ingresa esto:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Para CMD de Windows:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

### Escribir y Ejecutar un Programa Rust

A continuación, crea un nuevo archivo fuente y llámalo *main.rs*. Los archivos Rust siempre terminan con
la extensión *.rs*. Si usas más de una palabra en el nombre de archivo, usa
un subrayado para separarlas. Por ejemplo, usa *hello_world.rs* en lugar de
*helloworld.rs*.

Ahora abre el archivo *main.rs* que acabas de crear e ingresa el código del Listado 1-1.

<span class = "filename">Nombre de archivo: main.rs</span>

```rust
fn main() {
    println!("Hola, mundo!");
}
```

<span class="caption"> Listado 1-1: Un programa que imprime `¡Hola, mundo!`</span>

Guarda el archivo y vuelve a la ventana del terminal. En Linux o macOS, ingresa
los siguientes comandos para compilar y ejecutar el archivo:


```console
$ rustc main.rs
$ ./main
¡Hola, Mundo!
```

En Windows, ingrese el comando `.\main.exe` en lugar de `./main`:

```powershell
> rustc main.rs
> .\main.exe
¡Hola, Mundo!
```

Independientemente del sistema operativo, la cadena "¡Hola, mundo!" debería imprimirse en
la terminal. Si no ves este resultado, consulta la
[“Solución de problemas”][troubleshooting]<!-- ignore --> parte de la sección
Instalación para obtener ayuda.

Si se imprimió "¡Hola, mundo!", ¡Felicitaciones! Has escrito oficialmente un
programa Rust. Eso te convierte en un programador Rust, ¡bienvenido!

### Anatomía de un Programa Rust

Repasemos en detalle lo que acaba de suceder en tu programa "¡Hola, mundo!".
Aquí está la primera pieza del rompecabezas:

```rust
fn main() {

}
```

Estas líneas definen una función en Rust. La función `main` es especial: es
siempre el primer código que se ejecuta en cada programa ejecutable de Rust. La primera
linea declara una función llamada `main` que no tiene parámetros y no devuelve
nada. Si hubiera parámetros, irían entre los paréntesis, `()`.

Además, ten en cuenta que el cuerpo de la función está envuelto entre llaves, `{}`. Rust
los requiere alrededor de todos los cuerpos de función. Es un buen estilo colocar la apertura
de corchete en la misma línea que la declaración de función, agregando un espacio entre ellos.

Si deseas ceñirte a un estilo estándar en todos los proyectos de Rust, puedes utilizar una
herramienta de formateo automática llamada `rustfmt` para formatear tu código en un estilo 
particular. El equipo de Rust ha incluido esta herramienta con la distribución estándar de Rust
como `rustc`, por lo que ya debería estar instalado en tu computadora. Comprueba la
documentación en línea para obtener más detalles.

Dentro de la función `main` se encuentra el siguiente código:

```rust
    println!("Hola, mundo!");
```

Esta línea hace todo el trabajo en este pequeño programa: imprime texto en la
pantalla. Hay cuatro detalles importantes a tener en cuenta aquí.

Primero, el estilo Rust es sangrar con cuatro espacios, no una tabulación.

En segundo lugar, `println!` llama a una macro de Rust. Si se llama como función,
se ingresaría como `println` (sin el `!`). Analizaremos las macros de Rust en
más detalles en el Capítulo 19. Por ahora, solo necesitas saber que usar un `!`
significa que estás llamando a una macro en lugar de a una función normal.

En tercer lugar, verás la cadena "¡Hola, mundo!" `. Pasamos esta cadena como argumento
a `println!`, y la cadena se imprime en la pantalla.

Cuarto, terminamos la línea con un punto y coma (`;`), que indica que esta
expresión ha terminado y la siguiente está lista para comenzar. La mayoría de las líneas de código Rust
terminar con un punto y coma.

### La Compilación y la Ejecución son Pasos Separados

Acabas de ejecutar un programa recién creado, así que examinemos cada paso del
proceso.

Antes de ejecutar un programa de Rust, debes compilarlo usando el compilador de Rust
ingresando el comando `rustc` y pasándole el nombre del archivo fuente, como
esto:

```console
$ rustc main.rs
```

Si tienes experiencia en C o C++, notarás que esto es similar a `gcc`
o "clang". Después de compilar con éxito, Rust genera un ejecutable binario.

En Linux, macOS y PowerShell en Windows, puedes ver el ejecutable
ingresando el comando `ls` en la shell. En Linux y macOS, verás dos
archivos. Con PowerShell en Windows, verás los mismos tres archivos que
verías usando CMD.

```console
$ ls
main  main.rs
```

Con CMD en Windows, debes ingresar lo siguiente:

```cmd
> dir /B %= la opción / B dice que solo se muestren los nombres de los archivos =%
main.exe
main.pdb
main.rs
```

Esto muestra el archivo de código fuente con la extensión *.rs*, el archivo ejecutable
(*main.exe* en Windows, y *main* en todas las demás plataformas) y, cuando se usa
Windows, un archivo que contiene información de depuración con la extensión *.pdb*.
Desde aquí, ejecuta el archivo *main* o *main.exe*, así:

```console
$ ./main # o .\main.exe en Windows
```

Si *main.rs* fuera tu programa "¡Hola, mundo!", esta línea imprimiría `Hola,
mundo!' a tu terminal.

Si estás más familiarizado con un lenguaje dinámico, como Ruby, Python o
JavaScript, es posible que no esté acostumbrado a compilar y ejecutar un programa como
pasos separados. Rust es un lenguaje *compilado con anticipación*, lo que significa que puedes
compilar un programa y dale el ejecutable a otras personas, que pueden ejecutarlo
incluso sin tener Rust instalado. Si le das a alguien un *.rb*, *.py* o
*.js*, deben tener una implementación de Ruby, Python o JavaScript (respectivamente)
instalada. Pero en esos lenguajes, solo necesitas un comando para
compilar y ejecutar el programa. Son concesiones mutuas en el diseño del lenguaje.

Compilar solo con `rustc` está bien para programas simples, pero cuando tu proyecto
crece, querrás administrar todas las opciones y facilitar el intercambio de
código. A continuación, le presentaremos la herramienta Cargo, que lo ayudará a escribir
programas de Rust en el mundo real.

[troubleshooting]: ch01-01-installation.html#troubleshooting
