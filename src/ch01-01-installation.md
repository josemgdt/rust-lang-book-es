## Instalación

El primer paso es instalar Rust. Descargaremos Rust a través de "rustup", una
herramienta de línea de comandos para administrar versiones de Rust y herramientas asociadas. Necesitarás
una conexión a Internet para la descarga.

> Nota: Si prefiere no utilizar "rustup" por alguna razón, consulte la [Página
> de instalación de Rust](https://www.rust-lang.org/tools/install) para otras opciones.

Los siguientes pasos instalan la última versión estable del compilador de Rust.
Las garantías de estabilidad de Rust aseguran que todos los ejemplos del libro que
compile continuará compilando con las versiones más recientes de Rust. La salida podría
diferir ligeramente entre versiones, porque Rust a menudo mejora los mensajes de error
y advertencias. En otras palabras, cualquier versión más nueva y estable de Rust que instale
usando estos pasos debería funcionar como se esperaba con el contenido de este libro.

> ### Notación de Línea de Comando
>
> En este capítulo y a lo largo del libro, mostraremos algunos comandos utilizados en el
> terminal. Todas las líneas que debes ingresar en una terminal comienzan con "$".
> No es necesario escribir el carácter `$`; indica el inicio de cada
> comando. Las líneas que no comienzan con "$" suelen mostrar el resultado del
> comando anterior. Además, los ejemplos específicos de PowerShell usarán `>`
> en lugar de `$`.

### Instalación de `rustup` en Linux o macOS

Si está usando Linux o macOS, abra una terminal e ingrese el siguiente comando:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

El comando descarga un script e inicia la instalación de `rustup`
, que instala la última versión estable de Rust. Es posible que se solicite
tu contraseña. Si la instalación es exitosa, aparecerá la siguiente línea:

```text
Rust is installed now. Great!
```

Además, necesitará algún tipo de vinculador. Es probable que ya haya uno
instalado, pero cuando intentas compilar un programa Rust y obtienes errores que indican
que el vinculador no se pudo ejecutar, eso significa que no hay un vinculador instalado en el
sistema y deberás instalar uno manualmente. Los compiladores C generalmente vienen con
el enlazador correcto. Consulta la documentación de tu plataforma para saber cómo instalar un
compilador C. Además, algunos paquetes comunes de Rust dependen del código C y necesitarán un
compilador C. Por lo tanto, podría valer la pena instalar uno ahora.

### Instalación de `rustup` en Windows

En Windows, vaya a [https://www.rust-lang.org/tools/install][install] y siga
las instrucciones para instalar Rust. En algún momento de la instalación,
recibiras un mensaje que explica que también necesitarás las herramientas de compilación de C ++ para
Visual Studio 2013 o posterior. La forma más sencilla de adquirir las herramientas de compilación es
instalar [Herramientas de compilación para Visual Studio 2019][visualstudio]. Cuando te pregunte cuál
cargas de trabajo debe instalar, asegúrar la seleccion de "Herramientas de compilación de C ++" y de que
se incluyen el SDK de Windows 10 y los componentes del paquete de idioma inglés.

[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://visualstudio.microsoft.com/visual-cpp-build-tools/

El resto de este libro utiliza comandos que funcionan tanto en *cmd.exe* como en PowerShell.
Si hay diferencias específicas, te explicaremos cuál usar.

### Updating and Uninstalling

After you’ve installed Rust via `rustup`, updating to the latest version is
easy. From your shell, run the following update script:

```console
$ rustup update
```

To uninstall Rust and `rustup`, run the following uninstall script from your
shell:

```console
$ rustup self uninstall
```

### Troubleshooting

To check whether you have Rust installed correctly, open a shell and enter this
line:

```console
$ rustc --version
```

You should see the version number, commit hash, and commit date for the latest
stable version that has been released in the following format:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this information, you have installed Rust successfully! If you don’t
see this information and you’re on Windows, check that Rust is in your `%PATH%`
system variable. If that’s all correct and Rust still isn’t working, there are
a number of places you can get help. The easiest is the #beginners channel on
[the official Rust Discord][discord]. There, you can chat with other Rustaceans
(a silly nickname we call ourselves) who can help you out. Other great
resources include [the Users forum][users] and [Stack Overflow][stackoverflow].

[discord]: https://discord.gg/rust-lang
[users]: https://users.rust-lang.org/
[stackoverflow]: https://stackoverflow.com/questions/tagged/rust

### Local Documentation

The installation of Rust also includes a copy of the documentation locally, so
you can read it offline. Run `rustup doc` to open the local documentation in
your browser.

Any time a type or function is provided by the standard library and you’re not
sure what it does or how to use it, use the application programming interface
(API) documentation to find out!
