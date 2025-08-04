import { readFileSync } from "fs";
import { resolve } from "path";
import { WASI } from "wasi";

// Suppress the WASI Warning
process.removeAllListeners("warning").on("warning", (err) => {
  if (err.name !== "ExperimentalWarning" && !err.message.includes("wasi")) {
    console.warn(err);
  }
});

const packageRoot = resolve(import.meta.dirname, "..");
const bytes = readFileSync(resolve(packageRoot, "target/wasm32-wasip1/debug/livraison.wasm"));

const wasi = new WASI({
  env: {
    RUST_BACKTRACE: "1",
    COLOR: "1",
    CLICOLOR_FORCE: "1",
  },

  version: "preview1",
  args: process.argv.slice(2),
  preopens: {
    "/": "/",
  },
});

const imports = {
  wasi_snapshot_preview1: wasi.wasiImport,
};

imports.env = imports.env || {};

Object.assign(imports.env, {
  memoryBase: 0,
  tableBase: 0,
  memory: new WebAssembly.Memory({
    initial: 256,
    maximum: 512,
  }),

  table: new WebAssembly.Table({
    initial: 0,
    maximum: 0,
    element: "anyfunc",
  }),

  _log: Math.log,
});

const wasm = await WebAssembly.compile(bytes);

const instance = await WebAssembly.instantiate(wasm, imports);

wasi.start(instance);
