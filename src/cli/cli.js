import { readFileSync } from "fs";
import { resolve } from "path";
import { WASI } from "wasi";

const packageRoot = resolve(import.meta.dirname, "../..");
const importObject = {
  wasi_snapshot_preview1: WASI.defaultBindings,
};

const bytes = readFileSync(resolve(packageRoot, "target/wasm32-wasip1/debug/livraison.wasm"));

// const bytes = readFileSync("target/wasm32-wasip1/debug/test-wasm.wasm");

const wasi = new WASI({
  env: {
    RUST_BACKTRACE: "1",
    COLOR: "1",
    CLICOLOR_FORCE: "1",
  },

  // This option is mandatory.

  version: "preview1",
  args: process.argv.slice(2),
  preopens: {
    "/": "/",
  },
});

// const module = new WebAssembly.Module(new Uint8Array(bytes));

// const instance = new WebAssembly.Instance(module, importObject);

(async () => {
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
})();
