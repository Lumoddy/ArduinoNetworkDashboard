import { serializerVia } from "../smf/serialization.js";
import { arduinoPinIdSerializer } from "./pin-id.js";
import { arduinoPinModeSerializer } from "./pin-mode.js";
/**
@import { ArduinoPinId } from "./pin-id.js"
@import { ArduinoPinMode } from "./pin-mode.js"
*/

/**
@export @typedef {
    | { readonly type: "get-config" }
    | { readonly type: "get-pin", readonly pin: ArduinoPinId }
    | { readonly type: "set-pin", readonly pin: ArduinoPinId, readonly isHigh: boolean }
    | { readonly type: "get-pin-mode", readonly pin: ArduinoPinId }
    | { readonly type: "set-pin-mode", readonly pin: ArduinoPinId, readonly mode: ArduinoPinMode }
} ArduinoRequest
*/

/**
*/ export const arduinoRequestSerializer = /** @type {typeof serializerVia<ArduinoRequest>} */(
    serializerVia)(function*(tracer, value)
    {
        switch (value.type)
        {
            case "get-config":
                yield 1;
                break;
            case "get-pin":
                yield 2;
                yield* tracer.value(value.pin, arduinoPinIdSerializer);
                break;
            case "set-pin":
                yield 3;
                yield* tracer.value(value.pin, arduinoPinIdSerializer);
                yield* tracer.boolean(value.isHigh);
                break;
            case "get-pin-mode":
                yield 4;
                yield* tracer.value(value.pin, arduinoPinIdSerializer);
                break;
            case "set-pin-mode":
                yield 5;
                yield* tracer.value(value.pin, arduinoPinIdSerializer);
                yield* tracer.value(value.mode, arduinoPinModeSerializer);
                break;
            default:
                throw new TypeError();
        }
    });