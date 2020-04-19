import init, { run_app } from './pkg/wasm_yew.js';
async function main() {
   await init('/pkg/wasm_yew_bg.wasm');
   run_app();
}
main();