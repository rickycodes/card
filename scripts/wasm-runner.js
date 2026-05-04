#!/usr/bin/env node
const fs = require("fs");
const path = require("path");
const { spawn } = require("child_process");

const decoder = new TextDecoder();

function colours() {
  for (let x = 1; x < 255; x += 1) {
    process.stdout.write(`\x1b[38;5;${x}m${String(x).padStart(3, " ")} ${"█".repeat(40)}\x1b[0m\n`);
  }
}

function resolveWasmPath() {
  const root = path.join(__dirname, "..");
  const candidates = [
    process.env.CARD_WASM_PATH,
    path.join(root, "dist", "card.wasm"),
    path.join(root, "target", "wasm32-unknown-unknown", "release", "card.wasm"),
    path.join(root, "target", "wasm32-unknown-unknown", "debug", "card.wasm"),
  ].filter(Boolean);

  return candidates.find((candidate) => fs.existsSync(candidate));
}

function openUrl(url) {
  if (process.platform === "darwin") {
    spawn("open", [url], { stdio: ["ignore", "ignore", "ignore"], detached: true }).unref();
  } else if (process.platform === "win32") {
    spawn("cmd", ["/C", "start", "", url], { stdio: ["ignore", "ignore", "ignore"], detached: true }).unref();
  } else {
    spawn("xdg-open", [url], { stdio: ["ignore", "ignore", "ignore"], detached: true }).unref();
  }
}

function terminalSize() {
  return {
    width: Math.max(1, process.stdout.columns || 80),
    height: Math.max(1, process.stdout.rows || 24),
  };
}

function enterTerminalUi() {
  process.stdout.write("\x1b[?1049h\x1b[?25l\x1b[2J\x1b[H");
  if (process.stdin.isTTY) {
    process.stdin.setRawMode(true);
  }
  process.stdin.resume();
}

function exitTerminalUi() {
  process.stdout.write("\x1b[?25h\x1b[?1049l");
  if (process.stdin.isTTY) {
    process.stdin.setRawMode(false);
  }
  process.stdin.pause();
}

async function main() {
  const args = process.argv.slice(2);
  if (args[0] === "colours" || args[0] === "colors") {
    colours();
    return;
  }

  const wasmPath = resolveWasmPath();
  if (!wasmPath) {
    console.error("Missing wasm build. Run `npm run build` or `cargo +stable build --release --target wasm32-unknown-unknown --lib --no-default-features` first.");
    process.exit(1);
  }

  const wasmBytes = fs.readFileSync(wasmPath);
  const { instance } = await WebAssembly.instantiate(wasmBytes, {});
  const { exports } = instance;

  if (!(exports.memory instanceof WebAssembly.Memory)) {
    console.error("Wasm module does not export memory.");
    process.exit(1);
  }

  const app = exports.wasm_app_new();
  let closed = false;
  let ticking = null;
  let lastDialogCount = 0;

  function readUtf8(ptr, len) {
    return decoder.decode(new Uint8Array(exports.memory.buffer, ptr, len));
  }

  function readUrl(index) {
    const ptr = exports.wasm_link_url_ptr(index);
    const len = exports.wasm_link_url_len(index);
    return readUtf8(ptr, len);
  }

  function render() {
    const { width, height } = terminalSize();
    const ptr = exports.wasm_app_render(app, width, height);
    const len = exports.wasm_app_render_len(app);
    const frame = readUtf8(ptr, len);
    process.stdout.write(`\x1b[2J\x1b[H${frame}`);
    lastDialogCount = exports.wasm_app_dialog_count(app);
  }

  function cleanup(code = 0) {
    if (closed) {
      return;
    }
    closed = true;
    if (ticking) {
      clearInterval(ticking);
    }
    process.stdin.off("data", onData);
    process.stdout.off("resize", onResize);
    process.off("SIGINT", onSigInt);
    exitTerminalUi();
    exports.wasm_app_free(app);
    process.exit(code);
  }

  function onResize() {
    render();
  }

  function onSigInt() {
    cleanup(0);
  }

  function onData(buffer) {
    const input = buffer.toString("utf8");

    if (input === "\u0003" || input === "q" || input === "\u001b") {
      cleanup(0);
      return;
    }

    if (input === "\r") {
      const { width, height } = terminalSize();
      const linkIndex = exports.wasm_app_activate(app, width, height, BigInt(Date.now()));
      if (linkIndex >= 0) {
        openUrl(readUrl(linkIndex));
      }
      render();
      return;
    }

    if (input === "\t" || input === "\u001b[B" || input === "\u001b[C") {
      exports.wasm_app_next_button(app);
      render();
      return;
    }

    if (input === "\u001b[Z" || input === "\u001b[A" || input === "\u001b[D") {
      exports.wasm_app_previous_button(app);
      render();
    }
  }

  enterTerminalUi();
  process.stdin.on("data", onData);
  process.stdout.on("resize", onResize);
  process.on("SIGINT", onSigInt);

  ticking = setInterval(() => {
    exports.wasm_app_tick(app, BigInt(Date.now()));
    const dialogCount = exports.wasm_app_dialog_count(app);
    if (dialogCount !== lastDialogCount) {
      render();
    }
  }, 100);

  render();
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
