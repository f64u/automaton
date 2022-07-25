import init, { getState, tick } from "./pkg/gameoflife.js";

async function main() {
  await init();

  const state = getState();
  render(state);
}

function render(state) {
  let root = document.getElementById("life");
  root.innerHTML = state;

  tick();

  setTimeout(() => {
    render(getState());
  }, 1);
}

main();
