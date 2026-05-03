#!/usr/bin/env node
const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");

const root = __dirname;
const exe = process.platform === "win32" ? "card.exe" : "card";
const candidates = [
  path.join(root, "target", "release", exe),
  path.join(root, "bin", exe),
];

const binary = candidates.find((candidate) => fs.existsSync(candidate));

if (!binary) {
  console.error("Missing native card binary. Run `npm install` or `cargo build --release` first.");
  process.exit(1);
}

const result = spawnSync(binary, process.argv.slice(2), { stdio: "inherit" });

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

process.exit(result.status ?? 0);
