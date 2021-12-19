import init, { wasm_main } from "./ascii_d.js";

async function run() {
  await init();
  wasm_main();
}

run();