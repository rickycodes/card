#!/usr/bin/env node
const fs = require("fs");
const path = require("path");

const root = path.join(__dirname, "..");
const source = path.join(root, "target", "wasm32-unknown-unknown", "release", "card.wasm");
const distDir = path.join(root, "dist");
const destination = path.join(distDir, "card.wasm");

if (!fs.existsSync(source)) {
  console.error("Missing release wasm build. Run `cargo +stable build --release --target wasm32-unknown-unknown --lib --no-default-features` first.");
  process.exit(1);
}

fs.mkdirSync(distDir, { recursive: true });
fs.copyFileSync(source, destination);
console.log(`Staged ${path.relative(root, destination)}`);
