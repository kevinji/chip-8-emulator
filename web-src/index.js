/* @flow */
import './index.scss';

import { entry } from '../pkg/chip_8_emulator';
import * as view from './view';
import * as keypad from './keypad';

function main() {
  const canvas = view.init();
  keypad.addListeners(canvas);
  entry();
}

document.addEventListener('DOMContentLoaded', main);
