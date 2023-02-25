import init, { bootstrap } from './pkg/ascii_d.js';

async function run() {
    await init();
    bootstrap();
}

run();
