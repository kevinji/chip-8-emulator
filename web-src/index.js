/* @flow */
import './index.scss';

(async () => {
  const wasm = await import('../pkg/chip_8_emulator');
  const view = await import('./view');
  const keypad = await import('./keypad');

  function main() {
    const canvas = view.init();
    keypad.addListeners(canvas);
    wasm.entry();
  }

  document.addEventListener('DOMContentLoaded', main);
})();
