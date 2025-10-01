import { deserializeVia, serializerVia } from "../smf/serialization.js";

/**
@export @typedef {
    | "digital-input"
    | "digital-output"
} ArduinoPinMode
*/

/**
*/ export const arduinoPinModeSerializer = /** @type {typeof serializerVia<ArduinoPinMode>} */(
    serializerVia)(function*(tracer, value)
    {
        switch (value)
        {
            case "digital-input":
                yield 0;
                break;
            case "digital-output":
                yield 1;
                break;
            default:
                throw new TypeError();
        }
    });

/**
*/ export const arduinoPinModeDeserializer = /** @type {typeof deserializeVia<ArduinoPinMode>} */(
    deserializeVia)(function*(visitor)
    {
        switch (yield)
        {
            case 0:
                return "digital-input";
            case 1:
                return "digital-output";
            default:
                throw new SyntaxError("Expected pin mode but found arbitrary byte.");
        }
    });
