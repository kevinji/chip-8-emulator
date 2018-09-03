/* @flow */
import {
  KeyState,
  update_key_state as updateKeyState,
} from '../pkg/chip_8_emulator';

const KEY_CODES = [
  'KeyX', 'Digit1', 'Digit2', 'Digit3', 'KeyQ', // 0 - 4
  'KeyE', 'KeyA', 'KeyS', 'KeyD', 'KeyW', // 5 - 9
  'KeyZ', 'KeyC', 'Digit4', 'KeyR', 'KeyF', 'KeyV', // A - F
];

function handleKeyPress(e, pressed) {
  const keyIndex = KEY_CODES.includes(e.code);
  if (keyIndex === -1) {
    return;
  }

  const state = pressed ? KeyState.Down : KeyState.Up;
  updateKeyState(keyIndex, state);
}

// eslint-disable-next-line import/prefer-default-export
export function addListeners(canvas) {
  canvas.addEventListener('keydown', e => handleKeyPress(e, true));
  canvas.addEventListener('keyup', e => handleKeyPress(e, false));
}
