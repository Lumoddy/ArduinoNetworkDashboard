import { ArduinoInterface } from "./arduino-interface.js";
import * as NBT from "./nbt.js";

// @ts-ignore allow use in debug console.
window.NBT = NBT;
// @ts-ignore
window.ArduinoInterface = ArduinoInterface;

import "./common.js";
import "./elements/edge-resizer.js";
import "./elements/pin-indicator-switch.js";
import "./elements/pin-multi-switch.js";
import "./elements/pin-toggle-switch.js";