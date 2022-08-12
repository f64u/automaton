import init, {
  tickBriansWorld,
  getConfig,
  worldClick,
  firstDrawBrains,
  worldReload,
  blankWorld,
} from "./pkg/automaton_runner.js";

await init();

let config = getConfig();
console.log(config.width, config.height);

const world = document.getElementById("canvas");
const ctx = world.getContext("2d");
ctx.canvas.width = config.width;
ctx.canvas.height = config.height;

console.log(ctx.canvas.width, ctx.canvas.height);

let isPaused = false;

window.addEventListener("keydown", (event) => {
  if (event.key === " ") {
    isPaused = !isPaused;
  } else if (event.key === "r") {
    worldReload();
  } else if (event.key === "b") {
    blankWorld();
  }
});

canvas.addEventListener("click", (event) => {
  const x = event.clientX - canvas.offsetLeft;
  const y = event.clientY - canvas.offsetTop;

  const dx = x / config.pixel_size,
    dy = y / config.pixel_size;

  worldClick(dx, dy);
});

firstDrawBrains();

function render() {
  if (!isPaused) tickBriansWorld();

  setTimeout(() => render(), 100);
}

render();
