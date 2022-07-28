export function setPixel(x, y, size, color) {
  const world = document.getElementById("canvas");
  const ctx = world.getContext("2d");
  ctx.fillStyle = color;
  ctx.fillRect(x * size, y * size, size, size);
}
