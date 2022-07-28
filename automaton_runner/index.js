import init, {
  tickBriansWorld,
  getDimensions,
} from "./pkg/automaton_runner.js";

await init();

let dimensions = getDimensions();

for (let j = 0; j < dimensions[1]; j++) {
  let row = document.createElement("div");
  row.classList.add("row");
  for (let i = 0; i < dimensions[0]; i++) {
    let cell = document.createElement("span");
    row.appendChild(cell);
  }
  world.appendChild(row);
}

function render() {
  tickBriansWorld();

  setTimeout(() => render(), 100);
}

render();
