import { readFileSync } from "fs";
import { resolve } from "path";
import { WASI } from "wasi";

// Suppress the WASI Warning
process.removeAllListeners("warning").on("warning", (err) => {
  if (err.name !== "ExperimentalWarning" && !err.message.includes("wasi")) {
    // eslint-disable-next-line no-console
    console.warn(err);
  }
});

const packageRoot = resolve(import.meta.dirname, "..");
const bytes = readFileSync(resolve(packageRoot, "target/wasm32-wasip1/release/livraison.wasm"));

const cwd = process.cwd();
const wasi = new WASI({
  env: {
    RUST_BACKTRACE: "1",
    COLOR: "1",
    CLICOLOR_FORCE: "1",
  },

  version: "preview1",
  args: ["livraison.js", ...process.argv.slice(2)],
  preopens: {
    "/": cwd,
  },
});

const wasm = await WebAssembly.compile(bytes);

const instance = await WebAssembly.instantiate(wasm, wasi.getImportObject() as any);

wasi.start(instance);
