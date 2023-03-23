import init, { bootstrap } from './pkg/ascii_d.js';

window.readFromClipboard = async () => {
  const permission = await navigator.permissions.query({
    name: "clipboard-read",
  });
  if (permission.state === "denied") {
    throw new Error("Not allowed to read clipboard.");
  }
  const content = await navigator.clipboard.readText();
  return content;
};

window.writeToClipboard = (content) => {
  navigator.clipboard.writeText(content);
}

async function run() {
    await init();
    bootstrap();
}

run();
