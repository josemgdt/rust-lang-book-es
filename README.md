# El Lenguaje de Programación Rust

![Build Status](https://github.com/rust-lang/book/workflows/CI/badge.svg)

Este repositorio contiene las fuentes del libro "El Lenguaje de Programación Rust".

[El libro (en inglés) está disponible en forma impresa en No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust

También puede leer el libro en línea gratuitamente. Consulte el libro según 
los últimos lanzamientos de Rust [estable], [beta] o [nocturno]. Tenga en 
cuenta que los problemas en esas versiones puede que ya se hayan corregido en 
este repositorio, ya que las versiones se actualizan con menos frecuencia.

[estable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nocturno]: https://doc.rust-lang.org/nightly/book/

Consulte [releases] para descargar solo los listados de códigos que aparecen en el libro.

[releases]: https://github.com/rust-lang/book/releases

## Requisitos

La construcción del libro requiere [mdBook], idealmente la misma versión que
rust-lang/rust utiliza en [este archivo][rust-mdbook]. Para conseguirlo:

[mdBook]: https://github.com/rust-lang-nursery/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --vers [num-version]
```

## Creación

Para construir el libro, escriba:

```bash
$ mdbook build
```

La salida estará en el subdirectorio `book`. Para comprobarlo, ábralo en
su navegador web.

_Firefox:_
```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_
```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

Para ejecutar los tests:

```bash
$ mdbook test
```

## Contribuir

¡Nos encantaría tu ayuda! Consulte [CONTRIBUTING.md][contrib] para obtener más 
información sobre los tipos de contribuciones que estamos buscando.

[contrib]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md

### Translations

We'd love help translating the book! See the [Translations] label to join in
efforts that are currently in progress. Open a new issue to start working on
a new language! We're waiting on [mdbook support] for multiple languages
before we merge any in, but feel free to start!

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang-nursery/mdBook/issues/5

## Spellchecking

To scan source files for spelling errors, you can use the `spellcheck.sh`
script available in the `ci` directory. It needs a dictionary of valid words,
which is provided in `ci/dictionary.txt`. If the script produces a false
positive (say, you used word `BTreeMap` which the script considers invalid),
you need to add this word to `ci/dictionary.txt` (keep the sorted order for
consistency).
