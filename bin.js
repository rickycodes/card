#!/usr/bin/env node
const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");

const root = __dirname;
const exe = process.platform === "win32" ? "card.exe" : "card";
const packagedWasm = path.join(root, "dist", "card.wasm");
const candidates = [
  path.join(root, "target", "release", exe),
  path.join(root, "bin", exe),
];
const wasmCandidates = [
  packagedWasm,
  path.join(root, "target", "wasm32-unknown-unknown", "release", "card.wasm"),
  path.join(root, "target", "wasm32-unknown-unknown", "debug", "card.wasm"),
];

const binary = candidates.find((candidate) => fs.existsSync(candidate));
const wasm = wasmCandidates.find((candidate) => fs.existsSync(candidate));

if (wasm) {
  const result = spawnSync(process.execPath, [path.join(root, "scripts", "wasm-runner.js"), ...process.argv.slice(2)], {
    stdio: "inherit",
    env: { ...process.env, CARD_WASM_PATH: wasm },
  });

  if (result.error) {
    console.error(result.error.message);
    process.exit(1);
  }

  process.exit(result.status ?? 0);
}

if (binary) {
  const result = spawnSync(binary, process.argv.slice(2), { stdio: "inherit" });

  if (result.error) {
    console.error(result.error.message);
    process.exit(1);
  }

  process.exit(result.status ?? 0);
}

console.error("Missing card build. Run `npm run build` to stage wasm or `npm run native:build` for the native binary.");
process.exit(1);
