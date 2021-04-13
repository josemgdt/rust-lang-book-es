## Comentarios

Todos los programadores se esfuerzan por hacer que su código sea fácil de entender, pero a veces
se justifica una explicación adicional. En estos casos, los programadores dejan notas o
*comentarios*, en su código fuente que el compilador ignorará pero a la gente
que lee el código fuente pueden resultarle útiles.

Esto es un comentario simple:

```rust
// hello, world
```

En Rust, el estilo de comentario idiomático es comenzar un comentario con dos barras, y el
comentario continúa hasta el final de la línea. Para comentarios que se extienden más allá de una
única línea, deberá incluir `//` en cada línea, así:

```rust
// Estamos haciendo algo complicado aquí, lo suficiente para que necesitemos
// ¡Varias líneas de comentarios! ¡Uf! Con suerte, este comentario
// explica lo que está pasando.
```

Los comentarios también se pueden colocar al final de las líneas que contienen código:

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

Pero lo que verá más a menudo, es este formato, con el comentario en una
línea separada sobre el código que está comentando:

<span class="filename">​​Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

Rust también tiene otro tipo de comentario, comentarios de documentación, que
discutiremos en la sección "Publicar una caja en Crates.io" del Capítulo 14.

