import { deserializeVia, serializerVia } from "../smf/serialization.js";

/**
@export @typedef {number} ArduinoPinId
*/

/**
*/ export const arduinoPinIdSerializer = /** @type {typeof serializerVia<ArduinoPinId>} */(
    serializerVia)(function*(tracer, value)
    {
        yield* tracer.uint8(value);
    });

/**
*/ export const arduinoPinIdDeserializer = /** @type {typeof deserializeVia<ArduinoPinId>} */(
    deserializeVia)(function*(visitor)
    {
        return yield* visitor.uint8();
    });