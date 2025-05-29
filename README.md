# lusr

Lebensunterhaltssicherungsrechner

## About

Dieser Rechner soll die **aufenthaltsrechtliche** Beratung zur Frage, ob der Lebensunterhalt im Sinne der [§§ 2 Abs. 3, 5 Abs. 1 Nr. 1 AufenthG](https://dejure.org/gesetze/AufenthG/5.html) gesichert werden kann, unterstützen. Dabei kann und soll er die Beratung bestenfalls begleiten, aber keinesfalls ersetzen, da es, je nach konkretem Fall, eine Vielzahl von Besonderheiten zu beachten geben kann, die hier ummöglich alle abgebildet werden können.

## Installation

Der Rechner kann mit Docker oder mit Trunk installiert werden. In beiden Fällen ist es erforderlich, das Repository zunächst zu klonen:

```console
$  git clone https://github.com/dusmarcel/lusr.git
```

Anschließend wechselt man in das entsprechende Verzeichnis:

```console
$ cd lusr
```

### Installation mit Docker

Wenn Docker installiert ist und der daemon läuft, kann der Container einfach mit

```console
$ docker compose up -d
```

erzeugt und gestartet werden. Der Rechner wird sodann über einen Webserver auf http://localhost:8686 bereitgestellt.

### Installation mit Trunk

Da der Rechner vollständig clientseitig ausgeführt wird, sind serverseitig keine besonderen Vorbereitungen erforderlich. Die Installation sollte auf jedem üblichen Webserver möglich sein. Für die Installation wird [Rust](https://www.rust-lang.org/tools/install) benötigt, ferner das Target wasm32-unknown-unknown:

```console
$ rustup target add wasm32-unknown-unknown
```

Trunk muss auch noch installiert werden:

```console
$ cargo install trunk
```

Die benötigten JavaScript- und WASM-Dateien werden erzeugt mit:

```console
$ trunk build --release
```

trunk erzeugt einen Unterordner „dist“. Dessen Inhalt muss jetzt nur noch in das gewünschte Verzeichnis des Websververs verschoben oder kopiert werden.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
