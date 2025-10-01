import { deserializeVia } from "../smf/serialization.js";
import { arduinoPinIdDeserializer } from "./pin-id.js";
import { arduinoPinModeDeserializer } from "./pin-mode.js";
/**
@import { ArduinoPinId } from "./pin-id.js"
@import { ArduinoPinMode } from "./pin-mode.js"
@import { ArduinoConfig } from "./interface.js"
*/

/**
@export @typedef {
    | { type: "+get-config", config: ArduinoConfig }
    | { type: "+get-pin-ok", pin: ArduinoPinId, isHigh: boolean }
    | { type: "+get-pin-error", pin: ArduinoPinId, message: string }
    | { type: "+set-pin-ok", pin: ArduinoPinId }
    | { type: "+set-pin-error", pin: ArduinoPinId, message: string }
    | { type: "+get-pin-mode-ok", pin: ArduinoPinId, mode: ArduinoPinMode }
    | { type: "+get-pin-mode-error", pin: ArduinoPinId, message: string }
    | { type: "+set-pin-mode-ok", pin: ArduinoPinId }
    | { type: "+set-pin-mode-error", pin: ArduinoPinId, message: string }
    | { type: "pin-changed", pin: ArduinoPinId, isHigh: boolean }
    | { type: "error", error: "invalid-control-byte", byteIndex: number, byte: number }
    | { type: "error", error: "collect-overflow", byteIndex: number, capacity: number }
    | { type: "error", error: "invalid-syntax", byteIndex: number, found: number, expected: string }
    | { type: "error", error: "invalid-value", byteIndex: number, found: string, expected: string }
    | { type: "error", error: "timed-out", byteIndex: number }
} ArduinoResponse
*/

const stringDeserializer = /**
    @type {typeof deserializeVia<string>} */(
    deserializeVia)(function*(visitor)
    {
        /**
        @type {number[]}
        */ const bytes = [];

        while (yield* visitor.isNext())
        {
            bytes.push(yield);
        }

        return new TextDecoder().decode(new Uint8Array(bytes))
    });

const pinConfigDeserializer = /**
    @type {typeof deserializeVia<ArduinoConfig["pins"][number]>} */(
    deserializeVia)(function*(visitor)
    {
        const id = yield* visitor.value(arduinoPinIdDeserializer);
        const name = yield* visitor.value(stringDeserializer);

        let supportsDigitalInput = false;
        let supportsDigitalOutput = false;

        while (yield* visitor.isNext())
        {
            const mode = yield* visitor.value(stringDeserializer);
            switch (mode)
            {
                case "digital-input":
                {
                    if (supportsDigitalInput)
                        console.warn(new Error(
                            "Duplicate pin mode from Arduino"));
                    else
                        supportsDigitalInput = true;

                    break;
                }
                case "digital-output":
                {
                    if (supportsDigitalOutput)
                        console.warn(new Error(
                            "Duplicate pin mode from Arduino"));
                    else
                        supportsDigitalOutput = true;

                    break;
                }
                default:
                {
                    console.warn(new Error(
                        `Unknown pin mode "${mode}" from Arduino`));
                    break;
                }
            }
        }

        if (!supportsDigitalInput)
            throw new Error(
                "Found pin that does not support digital input from Arduino.");

        if (!supportsDigitalOutput)
            throw new Error(
                "Found pin that does not support digital output from Arduino.");

        return (
        {
            id,
            name,
            supportsDigitalInput,
            supportsDigitalOutput,
        });
    });

const pinConfigArrayDeserializer = /**
    @type {typeof deserializeVia<ArduinoConfig["pins"]>} */(
    deserializeVia)(function*(visitor)
    {
        /**
        @type {ArduinoConfig["pins"][number][]}
        */ const pins = [];

        while (yield* visitor.isNext())
        {
            pins.push(yield* visitor.value(pinConfigDeserializer));
        }

        return pins;
    });

/**
*/ export const arduinoResponseDeserializer = /** @type {typeof deserializeVia<ArduinoResponse>} */(
    deserializeVia)(function*(visitor)
    {
        switch (yield* visitor.uint8())
        {
            case 1:
                return (
                {
                    type: "+get-config",
                    config:
                    {
                        model: yield* visitor.value(stringDeserializer),
                        pins: yield* visitor.value(pinConfigArrayDeserializer),
                    },
                });
            case 2:
                return (
                {
                    type: "+get-pin-ok",
                    pin: yield* visitor.value(arduinoPinIdDeserializer),
                    isHigh: yield* visitor.boolean(),
                });
            case 3:
                return (
                {
                    type: "+get-pin-error",
                    pin: yield* visitor.value(arduinoPinIdDeserializer),
                    message: yield* visitor.value(stringDeserializer),
                });
            case 4:
                return (
                {
                    type: "+set-pin-ok",
                    pin: yield* visitor.value(arduinoPinIdDeserializer),
                });
            case 5:
                return (
                {
                    type: "+set-pin-error",
                    pin: yield* visitor.value(arduinoPinIdDeserializer),
                    message: yield* visitor.value(stringDeserializer),
                });
            case 6:
                return (
                {
                    type: "+get-pin-mode-ok",
                    pin: yield* visitor.value(arduinoPinIdDeserializer),
                    mode: yield* visitor.value(arduinoPinModeDeserializer),
                });
            case 7:
                return (
                {
                    type: "+get-pin-mode-error",
                    pin: yield* visitor.value(arduinoPinIdDeserializer),
                    message: yield* visitor.value(stringDeserializer),
                });
            case 8:
                return (
                {
                    type: "+set-pin-mode-ok",
                    pin: yield* visitor.value(arduinoPinIdDeserializer),
                });
            case 9:
                return (
                {
                    type: "+set-pin-mode-error",
                    pin: yield* visitor.value(arduinoPinIdDeserializer),
                    message: yield* visitor.value(stringDeserializer),
                });
            case 10:
                return (
                {
                    type: "pin-changed",
                    pin: yield* visitor.value(arduinoPinIdDeserializer),
                    isHigh: yield* visitor.boolean(),
                });
            case 11:
                return (
                {
                    type: "error",
                    error: "invalid-control-byte",
                    byteIndex: yield* visitor.uint16(),
                    byte: yield* visitor.uint8(),
                });
            case 12:
                return (
                {
                    type: "error",
                    error: "collect-overflow",
                    byteIndex: yield* visitor.uint16(),
                    capacity: yield* visitor.uint16(),
                });
            case 13:
                return (
                {
                    type: "error",
                    error: "invalid-syntax",
                    byteIndex: yield* visitor.uint16(),
                    found: yield* visitor.uint8(),
                    expected: yield* visitor.value(stringDeserializer),
                });
            case 14:
                return (
                {
                    type: "error",
                    error: "invalid-value",
                    byteIndex: yield* visitor.uint16(),
                    found: yield* visitor.value(stringDeserializer),
                    expected: yield* visitor.value(stringDeserializer),
                });
            case 15:
                return (
                {
                    type: "error",
                    error: "timed-out",
                    byteIndex: yield* visitor.uint16(),
                });
            default:
                throw new SyntaxError("Expected response type but found arbitrary byte.");
        }
    });