## Instalación

El primer paso es instalar Rust. Descargaremos Rust a través de "rustup", una
herramienta de línea de comandos para administrar versiones de Rust y herramientas asociadas. Necesitará
una conexión a Internet para la descarga.

> Nota: Si prefiere no utilizar "rustup" por alguna razón, consulte la [Página
> de Instalación de Rust](https://www.rust-lang.org/tools/install) para otras opciones.

Los siguientes pasos instalan la última versión estable del compilador de Rust.
Las garantías de estabilidad de Rust aseguran que todos los ejemplos del libro que
compile continuará compilando con las versiones más recientes de Rust. La salida podría
diferir ligeramente entre versiones, porque Rust a menudo mejora los mensajes de error
y advertencias. En otras palabras, cualquier versión más nueva y estable de Rust que instale
usando estos pasos debería funcionar como se esperaba con el contenido de este libro.

> ### Notación de Línea de Comando
>
> En este capítulo y a lo largo del libro, mostraremos algunos comandos utilizados en el
> terminal. Todas las líneas que debe ingresar en una terminal comienzan con "$".
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
su contraseña. Si la instalación es exitosa, aparecerá la siguiente línea:

```text
Rust is installed now. Great!
```

Además, necesitará algún tipo de enlazador. Es probable que ya haya uno
instalado, pero cuando intente compilar un programa Rust y obtenga errores que indican
que el enlazador no se pudo ejecutar, eso significa que no hay un vinculador instalado en el
sistema y deberá instalar uno manualmente. Los compiladores C generalmente vienen con
el enlazador correcto. Consulte la documentación de su plataforma para saber cómo instalar un
compilador C. Además, algunos paquetes comunes de Rust dependen del código C y necesitarán un
compilador C. Por lo tanto, podría valer la pena instalar uno ahora.

### Instalación de `rustup` en Windows

En Windows, vaya a [https://www.rust-lang.org/tools/install][install] y siga
las instrucciones para instalar Rust. En algún momento de la instalación,
recibirá un mensaje que explica que también necesitará las herramientas de compilación de C++ para
Visual Studio 2013 o posterior. La forma más sencilla de adquirir las herramientas de compilación es
instalar las [Herramientas de compilación para Visual Studio 2019][visualstudio]. Cuando se pregunte cuál
cargas de trabajo debe instalar, asegúrar la seleccion de "Herramientas de compilación de C++" y de que
se incluyen el SDK de Windows 10 y los componentes del paquete del idioma inglés.

[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://visualstudio.microsoft.com/visual-cpp-build-tools/

El resto de este libro utiliza comandos que funcionan tanto en *cmd.exe* como en PowerShell.
Si hay diferencias específicas, le explicaremos cuál usar.

### Actualización y Desinstalación

Después de haber instalado Rust a través de "rustup", la actualización a la última versión es
fácil. Desde su shell, ejecuta el siguiente script de actualización:

```console
$ rustup update
```

Para desinstalar Rust y `rustup`, ejecute el siguiente script de desinstalación desde su
shell:

```console
$ rustup self uninstall
```

### Solucion de Problemas

Para verificar si tiene Rust instalado correctamente, abra un shell e ingrese esta
línea:


```console
$ rustc --version
```

Debería ver el número de versión, el hash de confirmación y la fecha de confirmación de la última
versión estable que se ha lanzado en el siguiente formato:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Si ve esta información, ¡ha instalado Rust correctamente! Si no
la ve y está en Windows, verifique que Rust esté en la
variable de sistema "%PATH%". Si todo es correcto y Rust aún no funciona, hay
varios lugares donde puede obtener ayuda. El más fácil es el canal #beginners en
el [Discord oficial de Rust][discord]. Allí podrá charlar con otros rustáceos
(un apodo tonto que nos llamamos a nosotros mismos) que pueden ayudarle. Otros grandes
recursos incluyen [el foro de usuarios][users] y [Stack Overflow][stackoverflow].

[discord]: https://discord.gg/rust-lang
[users]: https://users.rust-lang.org/
[stackoverflow]: https://stackoverflow.com/questions/tagged/rust

### Documentación Local

La instalación de Rust también incluye una copia de la documentación localmente, por lo que
puedes leerlo sin conexión. Ejecutae `rustup doc` para abrir la documentación local en
tu navegador.

Cada vez que la biblioteca estándar proporciona un tipo o función y no está
seguro de lo que hace o cómo usarlo, use la interfaz de programación de la aplicación
(API) para averiguarlo.
