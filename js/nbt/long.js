import { AbstractDeserializer, AbstractPayloadDeserializer, AbstractPayloadSerializer, AbstractSerializer, Deserializer, DeserializerError, PayloadDeserializer, PayloadSerializer, Serializer, Tag } from "./base.js";
/**
@import { SerializationGenerator, DeserializationGenerator, SerializationConfig, DeserializationConfig } from "./base.js"
*/

// MARK: LongTag
/**
 * A {@linkcode Tag} that stores a long (`BigLong64`).
*/ export class LongTag extends Tag
{
    /**
    @param {bigint} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {bigint}
        @private*/ this._value = value & 0xFFFFFFFFFFFFFFFFn;
        if ((this._value & 0x8000000000000000n) !== 0n)
            this._value -= 0x8000000000000000n;
    }

    /**
    @returns {bigint}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value)
    {
        this._value = value & 0xFFFFFFFFFFFFFFFFn;
        if ((this._value & 0x8000000000000000n) !== 0n)
            this._value -= 0x8000000000000000n;
    }
}

// MARK: Serializer
/**
@extends {AbstractPayloadSerializer<bigint, SerializationConfig>}
*/ export class LongPayloadSerializer extends AbstractPayloadSerializer
{
    /**
    @param {bigint} value
    @param {SerializationConfig} [config]
    @protected @override*/ *generator(value, config)
    {
        const dataView = new DataView(new ArrayBuffer(8));

        dataView.setBigInt64(0, value, config?.endian === "little");

        for (let i = 0; i < 8; i++)
            yield dataView.getUint8(i);

        return undefined;
    }
}

/**
@extends {AbstractSerializer<LongTag, SerializationConfig>}
*/ export class LongSerializer extends AbstractSerializer
{
    /**
    @param {LongTag} value
    @param {SerializationConfig} [config]
    @protected @override*/ generator(value, config)
    {
        return new LongPayloadSerializer(value.value, config);
    }
}

// MARK: Deserializer
/**
@extends {AbstractPayloadDeserializer<bigint>}
*/ export class LongPayloadDeserializer extends AbstractPayloadDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const dataView = new DataView(new ArrayBuffer(8));

        for (let i = 0; i < 8; i++)
            dataView.setUint8(i, yield);

        return dataView.getBigInt64(0, config?.endian === "little");
    }
}

/**
@extends {AbstractDeserializer<
    LongTag,
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class LongDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const inner = new LongPayloadDeserializer(config);
        let result;
        while ((result = inner.push(yield)) === null);
        return new LongTag(result);
    }
}