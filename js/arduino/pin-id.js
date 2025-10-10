import { deserializeVia, serializerVia } from "../smf/serialization.js";

/**
*/ export const arduinoPinIdSerializer = /** @type {typeof serializerVia<number>} */(
    serializerVia)(function*(tracer, value)
    {
        yield* tracer.uint8(value);
    });

/**
*/ export const arduinoPinIdDeserializer = /** @type {typeof deserializeVia<number>} */(
    deserializeVia)(function*(visitor)
    {
        return yield* visitor.uint8();
    });