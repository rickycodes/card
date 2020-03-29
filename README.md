## Goal

Write a CLI using rustlang targetting wasm so we can run on node and deploy with npx/npm

## Building

```
cargo build --target=wasm32-unknown-emscripten --release
```

This moves files into:
```
./target/wasm32-unknown-emscripten/release/
```
Copy generated `card.js` and `card.wasm` into the root so the [bin](bin.js) can read it:

```
mv ./target/wasm32-unknown-emscripten/release/card.* .
```

Done!

Once published the binary can be executed with npx:
```
npx rickycodes
```
Scripts are also discoverable in [package.json](package.json#L9)
