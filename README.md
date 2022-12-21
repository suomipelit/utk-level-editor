# Level editor for Ultimate Tapan Kaikki (TK321)

A feature complete level editor written in Rust for Ultimate Tapan Kaikki (TK321) DOS game classic. See also the [opensourced game itself](https://github.com/suomipelit/ultimatetapankaikki).

This editor was written from scratch by Ultimate Tapan Kaikki fans because the source code of the original DOS era editor has been lost.
It's heavily inspired by the original editor but does not aim to be a carbon copy. Hopefully most of the differences can be considered as improvements.

![Cover image](./media/cover.png)

The editor runs on both desktop (using SDL2) and web browser (compiled to WASM from Rust).

👉👉 [Try the web version here](https://suomipelit.github.io/utk-level-editor-web) 👈👈

Press F1 to see the help screen. To get inspiration, download [original game level files](https://github.com/suomipelit/ultimatetapankaikki/tree/master/LEVS) and open them in the editor by pressing F3.

## Running from source

### Desktop

* Install Rust toolchain
* Install SDL2 and SDL2_image development libraries
* Run `cargo run --release`

### Web

* Install Rust toolchain
* Optional: Install [binaryen](https://github.com/WebAssembly/binaryen), which is used to optimize the WASM output size
* Run `cd web; ./build.sh`
* Serve files from `dist/` directory, e.g. `python3 -m http.server -d dist`, and open `index.html` in your browser
