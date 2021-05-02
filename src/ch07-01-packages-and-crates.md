## Paquetes y Cajas

Las primeras partes del sistema de módulos que cubriremos son los paquetes y las cajas. Una
caja es un binario o una biblioteca. *crate root* es un archivo fuente con el que el compilador Rust
comienza y constituye el módulo raíz de su caja (explicaremos los
módulos en profundidad en la sección [“Definición de módulos para controlar el alcance y
privacidad ”][modules]<!-- ignore -->). Un *paquete* es una o más cajas
que proporcionan un conjunto de funcionalidades. Un paquete contiene un archivo *Cargo.toml*
que describe cómo construir esas cajas.

Hay varias reglas que determinan lo que puede contener un paquete. Un paquete *debe* contener
**cero o una** caja de biblioteca, y no más. Puede contener tantas cajas de binarios
como desee, pero debe contener al menos una caja (ya sea de biblioteca o
binario).

Repasemos lo que sucede cuando creamos un paquete. Primero, ingresamos el
comando `cargo new`:

```console
$ cargo new mi-proyecto
     Created binary (application) `mi-proyecto` package
$ ls mi-proyecto
Cargo.toml
src
$ ls mi-proyecto/src
main.rs
```

Cuando ingresamos el comando `cargo new`, Cargo crea un archivo *Cargo.toml*, dándonos un
paquete. En cuanto al contenido de *Cargo.toml*, no se menciona
*src/main.rs* porque Cargo sigue la convención de que *src/main.rs* es el
*crate root* de una caja binaria con el mismo nombre que el paquete. Asimismo, Cargo
sabe que si el directorio del paquete contiene *src/lib.rs*, el paquete contiene
una caja de biblioteca con el mismo nombre que el paquete, y *src/lib.rs* es su
*crate root*. Cargo pasa los archivos *crate root* a `rustc` para construir la biblioteca
o el binario.

Por ahora, tenemos un paquete que solo contiene *src/main.rs*, lo que significa que solo
contiene una caja binaria llamada `mi-proyecto`. Si un paquete contiene *src/main.rs*
y *src/lib.rs*, tiene dos cajas; una biblioteca y un binario, ambos con el mismo
nombre que el paquete. Un paquete puede tener varias cajas binarias colocando archivos
en el directorio *src/bin*; cada archivo será una caja binaria separada.

Una caja agrupará la funcionalidad relacionada en un ámbito para que
sea fácil de compartir entre varios proyectos. Por ejemplo,la
caja `rand` que usamos en el [Capítulo 2][rand]<!-- ignore --> proporciona funcionalidad
para generar números aleatorios. Podemos usar esa funcionalidad en nuestro propio
proyecto al llevar la caja `rand` al alcance de nuestro proyecto.
La funcionalidad proporcionada por la caja "rand" es accesible a través del nombre de caja `rand`.

Mantener la funcionalidad de una caja en su propio alcance aclara si una funcionalidad en particular
se define en nuestra caja o en la caja `rand` y evita
posibles conflictos. Por ejemplo, la caja `rand` proporciona un trait llamado
`Rng`. También podemos definir una `struct` llamada `Rng` en nuestra propia caja. Ya que
la funcionalidad de una caja tiene un espacio de nombres en su propio alcance, cuando agregamos `rand` como una
dependencia, el compilador no se confunde sobre a qué se refiere el nombre `Rng`. En
nuestra caja, se refiere a la `struct Rng` que definimos. Accederíamos al
trait `Rng` de la caja `rand` como `rand::Rng`.

¡Sigamos adelante y hablemos del sistema de módulos!

[modules]: ch07-02-definition-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generando-un-numero-aleatorio

