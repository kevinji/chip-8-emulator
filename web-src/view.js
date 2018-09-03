/* @flow */
const SCALE = 10;
let canvas;
let ctx;

export function drawPixel(x, y, isFilled) {
  const fillColor = isFilled ? 'rgb(0, 0, 0)' : 'rgb(255, 255, 255)';
  ctx.fillStyle = fillColor;
  ctx.fillRect(x, y, 1, 1);
}

export function clear() {
  // Save transformation matrix.
  ctx.save();

  // Use the identity matrix while clearing the canvas.
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  // Restore transformation matrix.
  ctx.restore();
}

function step() {
  drawPixel(ctx, 10, 20, true);
  // window.requestAnimationFrame(step);
}

export function init() {
  canvas = document.getElementById('view');
  ctx = canvas.getContext('2d');
  ctx.scale(SCALE, SCALE);

  window.requestAnimationFrame(step);
  return canvas;
}
