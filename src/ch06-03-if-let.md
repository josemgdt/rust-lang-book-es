## Control de Flujo Conciso con `if let`

La sintaxis `if let` le permite combinar `if` y `let` en una forma menos detallada de
manejar valores que coincidan con un patrón, ignorando el resto. Considere el
programa en el Listado 6-6 que coincide con un valor `Option<u8>` pero solo quiere
ejecutar código si el valor es 3.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

<span class="caption">Listado 6-6: Un `match` que solo se preocupa de ejecutar
código cuando el valor es `Some(3)`</span>

Queremos hacer algo con la coincidencia `Some(3)` pero no hacer nada ni con ningun otro `Some<u8>`,
ni con el valor `None`. Para satisfacer la expresión `match`,
tiene que agregar `_ => ()` después de procesar solo una variante, que es una gran cantidad de
código repetitivo por agregar.

En su lugar, podríamos escribir esto de una manera más corta usando `if let`. El siguiente
código se comporta igual que `match` en el Listado 6-6:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

La sintaxis `if let` toma un patrón y una expresión separados por un signo igual.
Funciona de la misma manera que un `match`, donde la expresión se da al
`match` y el patrón es su primer brazo.

Usar `if let` significa menos escritura, menos sangría y menos código repetitivo.
Sin embargo, pierde la comprobación exhaustiva que impone `match`. Elegir
entre `match` e `if let` depende de lo que esté haciendo en su
situación particular y si ganar concisión es una compensación adecuada a
perder el control exhaustivo.

En otras palabras, puede pensar en `if let` como variante sintáctica para `match` que
ejecuta código cuando el valor coincide con un patrón e ignora todos los demás valores.

Podemos incluir un `else` con un `if let`. El bloque de código que acompaña al
`else` es el mismo que el bloque de código que iría con el caso `_` en la
expresión `match` que es equivalente a `if let` y `else`. Recuerde la
definición de enumeración `Coin` en el Listado 6-4, donde la variante `Quarter` también tenía un
valor de `UsState`. Si quisiéramos contar todas las monedas que no sean de un cuarto, mientras vemos también
el estado de los cuartos, podríamos hacerlo con una expresión `match` como esta:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

O podríamos usar una expresión `if let` y `else` como esta:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

Si tiene una situación en la que su programa tiene una lógica demasiado detallada para
expresar usando `match`, recuerde que `if let` también está en su caja de herramientas Rust.

## Resumen

Ahora hemos cubierto cómo usar enumeraciones para crear tipos personalizados que pueden ser uno de
un conjunto de valores enumerados. Hemos mostrado cómo el tipo `Option<T>` de la biblioteca estándar
le ayuda a utilizar el sistema de tipos para evitar errores. Cuando los valores de enumeración tienen
datos dentro de ellos, puede usar `match` o `if let` para extraer y usar esos
valores, dependiendo de cuántos casos necesite manejar.

Sus programas Rust ahora pueden expresar conceptos en su dominio usando estructuras y
enumeraciones. La creación de tipos personalizados para usar en su API garantiza la seguridad de los tipos;
el compilador se asegurará de que sus funciones obtengan solo valores del tipo que
espera cada función.

Para proporcionar una API bien organizada a sus usuarios que sea sencilla
de usar y solo exponga exactamente lo que sus usuarios necesitarán, pasemos ahora a los
módulos de Rust.

