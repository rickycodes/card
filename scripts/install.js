const path = require("path");
const { spawnSync } = require("child_process");

if (process.env.CARD_SKIP_BUILD === "1") {
  process.exit(0);
}

const result = spawnSync("cargo", ["build", "--release"], {
  cwd: path.join(__dirname, ".."),
  stdio: "inherit",
});

if (result.error) {
  console.error("Failed to start cargo build:", result.error.message);
  process.exit(1);
}

process.exit(result.status ?? 0);
