import { AbstractDeserializer, AbstractPayloadDeserializer, AbstractPayloadSerializer, AbstractSerializer, Deserializer, DeserializerError, PayloadDeserializer, PayloadSerializer, Serializer, Tag } from "./base.js";
/**
@import { SerializationGenerator, DeserializationGenerator, SerializationConfig, DeserializationConfig } from "./base.js"
*/

// MARK: DoubleTag
/**
 * A {@linkcode Tag} that stores a double (`Double32`).
*/ export class DoubleTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = value;
    }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}

// MARK: Serializer
/**
@extends {AbstractPayloadSerializer<number, SerializationConfig>}
*/ export class DoublePayloadSerializer extends AbstractPayloadSerializer
{
    /**
    @param {number} value
    @param {SerializationConfig} [config]
    @protected @override*/ *generator(value, config)
    {
        const dataView = new DataView(new ArrayBuffer(8));

        dataView.setFloat64(0, value, config?.endian === "little");

        for (let i = 0; i < 8; i++)
            yield dataView.getUint8(i);

        return undefined;
    }
}

/**
@extends {AbstractSerializer<DoubleTag, SerializationConfig>}
*/ export class DoubleSerializer extends AbstractSerializer
{
    /**
    @param {DoubleTag} value
    @param {SerializationConfig} [config]
    @protected @override*/ generator(value, config)
    {
        return new DoublePayloadSerializer(value.value, config);
    }
}

// MARK: Deserializer
/**
@extends {AbstractPayloadDeserializer<number>}
*/ export class DoublePayloadDeserializer extends AbstractPayloadDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const dataView = new DataView(new ArrayBuffer(8));

        for (let i = 0; i < 8; i++)
            dataView.setUint8(i, yield);

        return dataView.getFloat64(0, config?.endian === "little");
    }
}

/**
@extends {AbstractDeserializer<
    DoubleTag,
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class DoubleDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const inner = new DoublePayloadDeserializer(config);
        let result;
        while ((result = inner.push(yield)) === null);
        return new DoubleTag(result);
    }
}