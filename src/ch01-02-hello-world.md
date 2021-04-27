## ¡Hola Mundo!

Ahora que ha instalado Rust, escriba su primer programa. Al aprender un nuevo lenguaje es
tradicional escribir un pequeño programa que imprima
el texto "¡Hola, mundo!" en la pantalla, ¡así que aquí haremos lo mismo!

> Nota: Este libro presume familiaridad básica con la línea de comandos. Rust no tiene
> demandas específicas sobre edición, herramientas o dónde guardar el código, por lo que
> si prefiere utilizar un entorno de desarrollo integrado (IDE) en lugar de
> la línea de comandos, no dude en utilizar su IDE favorito. Muchos IDE tienen ahora algun
> grado de soporte para Rust; consulte la documentación del IDE para obtener más detalles. Recientemente,
> el equipo de Rust se ha centrado en permitir un buen soporte IDE y
> ¡se han hecho progresos rápidamente en ese frente!

### Creación de un Directorio de Proyectos

Empezaremos por crear un directorio para almacenar su código Rust. No importa
donde guarde el código, pero para los ejercicios y proyectos de este libro
sugerimos crear un directorio *projects* en su directorio personal y mantener todos
los proyectos allí.

Abra una terminal e ingrese los siguientes comandos para crear el directorio *projects*
y un directorio para el proyecto "¡Hola, mundo!" dentro de el.

Para Linux, macOS y PowerShell en Windows, ingresar esto:

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

A continuación, cree un nuevo archivo fuente y llámelo *main.rs*. Los archivos Rust siempre terminan con
la extensión *.rs*. Si usa más de una palabra en el nombre de archivo, use
un subrayado para separarlas. Por ejemplo, use *hello_world.rs* en lugar de
*helloworld.rs*.

Ahora abra el archivo *main.rs* que acaba de crear e ingrese el código del Listado 1-1.

<span class = "filename">Nombre de archivo: main.rs</span>

```rust
fn main() {
    println!("Hola, mundo!");
}
```

<span class="caption"> Listado 1-1: Un programa que imprime `¡Hola, mundo!`</span>

Guarde el archivo y vuelva a la ventana del terminal. En Linux o macOS, ingrese
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
la terminal. Si no ve este resultado, consulte [“Solución de problemas”][troubleshooting]<!-- ignore --> de la sección
Instalación para obtener ayuda.

Si se imprimió "¡Hola, mundo!", ¡Felicitaciones! Oficialmente ha escrito un
programa Rust. Eso le convierte en un programador Rust, ¡bienvenido!

### Anatomia de un Programa Rust

Repasemos en detalle lo que acaba de suceder en su programa "¡Hola, mundo!".
Esta es la primera pieza del rompecabezas:

```rust
fn main() {

}
```

Estas líneas definen una función en Rust. La función `main` es especial; es
siempre el primer código que se ejecuta en cada ejecutable Rust. La primera
linea declara una función llamada `main` que no tiene parámetros y no devuelve
nada. Si hubiera parámetros, irían entre los paréntesis, `()`.

Además, tenga en cuenta que el cuerpo de la función está envuelto entre llaves, `{}`. Rust
las requiere alrededor de todos los cuerpos de función. Es un buen estilo colocar la apertura
de corchete en la misma línea que la declaración de la función, agregando un espacio entre ellos.

Si desea ceñirse a un estilo estándar en todos los proyectos de Rust, puedes utilizar una
herramienta de formateo automática llamada `rustfmt` para formatear su código en un estilo 
particular. El equipo de Rust ha incluido esta herramienta con la distribución estándar de Rust
`rustc`, por lo que ya debería estar instalado en su computadora. Compruebe la
documentación en línea para obtener más detalles.

Dentro de la función `main` se encuentra el siguiente código:

```rust
    println!("Hola, mundo!");
```

Esta línea hace todo el trabajo en este pequeño programa; imprime texto en la
pantalla. Hay cuatro detalles importantes a tener en cuenta aquí.

Primero, el estilo Rust es sangrar con cuatro espacios, no tabulaciones.

En segundo lugar, `println!` llama a una macro Rust. Si se llama como función,
se ingresaría como `println` (sin el `!`). Analizaremos las macros Rust en
más detalle en el Capítulo 19. Por ahora, solo necesita saber que usar un `!`
significa que está llamando a una macro en lugar de a una función normal.

En tercer lugar, verá la cadena `"¡Hola, mundo!"`. Pasamos esta cadena como argumento
a `println!`, y la cadena se imprime en la pantalla.

Cuarto, terminamos la línea con un punto y coma (`;`), que indica que esta
expresión ha terminado y la siguiente está lista para comenzar. La mayoría de las líneas de código Rust
terminan con un punto y coma.

### La Compilación y la Ejecución son Pasos Separados

Acaba de ejecutar un programa recién creado, así que examinemos cada paso del
proceso.

Antes de ejecutar un programa Rust, debe compilarlo usando el compilador Rust
ingresando el comando `rustc` y pasándole el nombre del archivo fuente, como
esto:

```console
$ rustc main.rs
```

Si tiene experiencia en C o C++, notará que esto es similar a `gcc`
o `clang`. Después de compilar con éxito, Rust genera un binario ejecutable.

En Linux, macOS y PowerShell en Windows, puede ver el ejecutable
ingresando el comando `ls` en la shell. En Linux y macOS, verá dos
archivos. Con PowerShell en Windows, verá los mismos tres archivos que
vería usando CMD.

```console
$ ls
main  main.rs
```

Con CMD en Windows, debe ingresar lo siguiente:

```cmd
> dir /B %= la opción /B dice que solo se muestren los nombres de los archivos =%
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

Si *main.rs* fuera su programa "¡Hola, mundo!", esta línea imprimiría `Hola,
mundo!' en su terminal.

Si está más familiarizado con un lenguaje dinámico, como Ruby, Python o
JavaScript, es posible que no esté acostumbrado a compilar y ejecutar un programa como
pasos separados. Rust es un lenguaje *compilado con anticipación*, lo que significa que puede
compilar un programa y dale el ejecutable a otras personas, que pueden ejecutarlo
incluso sin tener Rust instalado. Si le da a alguien un *.rb*, *.py* o
*.js*, deben tener una implementación de Ruby, Python o JavaScript (respectivamente)
instalada. Pero en esos lenguajes, solo necesita un comando para
compilar y ejecutar el programa. Son concesiones mutuas en el diseño del lenguaje.

Compilar solo con `rustc` está bien para programas simples, pero cuando su proyecto
crece, querrá administrar todas las opciones y facilitar el intercambio de
código. A continuación, le presentaremos la herramienta Cargo, que lo ayudará a escribir
programas de Rust en el mundo real.

[troubleshooting]: ch01-01-installation.html#solucion-de-problemas

