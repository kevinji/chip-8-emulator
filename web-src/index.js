/* @flow */

import './index.scss';
import { update_key_state } from './static/chip-8-emulator';
// import { booted } from './static/chip-8-emulator_bg.wasm';

const SCALE = 10;
let canvas;
let ctx;

export function drawPixel(x, y, isFilled) {
  const fill_color = isFilled ? 'rgb(0, 0, 0)' : 'rgb(255, 255, 255)';
  ctx.fillStyle = fill_color;
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
  return () => {
    drawPixel(ctx, 10, 20, true);
    // window.requestAnimationFrame(step);
  };
}

const KEY_CODES = [
  'KeyX', 'Digit1', 'Digit2', 'Digit3', 'KeyQ', // 0 - 4
  'KeyE', 'KeyA', 'KeyS', 'KeyD', 'KeyW', // 5 - 9
  'KeyZ', 'KeyC', 'Digit4', 'KeyR', 'KeyF', 'KeyV', // A - F
];

function handleKeyPress(e: KeyboardEvent, pressed: boolean) {
  const keyIndex = KEY_CODES.includes(e.code);
  if (keyIndex === -1) {
    return;
  }

  update_key_state(keyIndex, pressed);
}

function setupView() {
  canvas = document.getElementById('view');
  ctx = canvas.getContext('2d');
  ctx.scale(SCALE, SCALE);
}

function main() {
  setupView();
  canvas.addEventListener('keydown', (e) => handleKeyPress(e, true));
  canvas.addEventListener('keyup', (e) => handleKeyPress(e, false));
  window.requestAnimationFrame(step(ctx));
}

document.addEventListener('DOMContentLoaded', main);
