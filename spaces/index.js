export function setClass(x, y, className) {
  let world = document.getElementById("world");
  let cell = world.children[y].children[x];
  cell.className = className;
}
