import init, * as Ragroc from './wasm/ragroc.js';

async function run() {
    await init();

    window.Ragroc = Ragroc;
    
    Ragroc.finish_load();
}

run();

