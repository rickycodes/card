## Goal

Write a CLI in Rust that can still be published to `npm` and launched with `npx`.

<img src="demo.gif">

## Building

```sh
npm run build
```

This builds the wasm target and stages the packaged artifact at:

```sh
./dist/card.wasm
```

For local native development, you can still build the Rust CLI directly:

```sh
npm run native:build
```

Which runs cargo directly:

```sh
cargo +stable build --release
```

## Running

```sh
npx rickycodes
```

The default npm experience is now Node + wasm. Exit with `q`, `Esc`, or `Ctrl-C`.

To inspect the ANSI palette helper:

```sh
npx rickycodes colours
```

## Packaging tradeoff

The published package can now run on a Node-only machine because it ships a prebuilt `card.wasm` artifact instead of compiling Rust during `npm install`. Rust tooling is still needed by the publisher when producing that artifact.

## License

MIT
