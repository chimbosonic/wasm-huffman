[![Repo Size][loc_img]][loc]
[![License][license_img]][license_file]
[![Build][actions]][loc]

# wasm-huffman
[huff-tree-tap](https://github.com/chimbosonic/huff-tree-tap) in WASM
It is deployed [here](https://chimbosonic.com/wasm_huffman/index.html)
## How to install

```sh
npm install
```

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

## What does each file do?

* `Cargo.toml` contains the standard Rust metadata. You put your Rust dependencies in here. You must change this file with your details (name, description, version, authors, categories)

* `package.json` contains the standard npm metadata. You put your JavaScript dependencies in here. You must change this file with your details (author, name, version)

* `webpack.config.js` contains the Webpack configuration. You shouldn't need to change this, unless you have very special needs.

* The `js` folder contains your JavaScript code (`index.js` is used to hook everything into Webpack, you don't need to change it).

* The `src` folder contains your Rust code.

* The `static` folder contains any files that you want copied as-is into the final build. It contains an `index.html` file which loads the `index.js` file.

* The `tests` folder contains your Rust unit tests.

[loc]: https://github.com/chimbosonic/wasm-huffman "Repository"
[loc_img]: https://tokei.rs/b1/github/chimbosonic/wasm-huffman?category=code "Repository Size"
[actions]: https://github.com/chimbosonic/wasm-huffman/actions/workflows/build.yml/badge.svg
[license_file]: https://github.com/chimbosonic/wasm-huffman/blob/master/LICENSE "License File"
[license_img]: https://img.shields.io/github/license/chimbosonic/wasm-huffman "License Display"