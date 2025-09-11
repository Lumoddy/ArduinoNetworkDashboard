import { AbstractDeserializer, AbstractPayloadDeserializer, AbstractPayloadSerializer, AbstractSerializer, Deserializer, DeserializerError, PayloadDeserializer, PayloadSerializer, Serializer, Tag } from "./base.js";
/**
@import { SerializationGenerator, DeserializationGenerator, SerializationConfig, DeserializationConfig } from "./base.js"
*/

// MARK: ByteTag
/**
 * A {@linkcode Tag} that stores a byte (`Uint8`).
*/ export class ByteTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = value & 0xFF;
        if ((this._value & 0x80) !== 0)
            this._value -= 0x100;
    }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value)
    {
        this._value = value & 0xFF;
        if ((this._value & 0x80) !== 0)
            this._value -= 0x80;
    }
}

// MARK: Serializer
/**
@extends {AbstractPayloadSerializer<number, SerializationConfig>}
*/ export class BytePayloadSerializer extends AbstractPayloadSerializer
{
    /**
    @param {number} value
    @param {SerializationConfig} [config]
    @protected @override*/ *generator(value, config)
    {
        const dataView = new DataView(new ArrayBuffer(1));

        dataView.setInt8(0, value);

        yield dataView.getUint8(0);

        return undefined;
    }
}

/**
@extends {AbstractSerializer<ByteTag, SerializationConfig>}
*/ export class ByteSerializer extends AbstractSerializer
{
    /**
    @param {ByteTag} value
    @param {SerializationConfig} [config]
    @protected @override*/ generator(value, config)
    {
        return new BytePayloadSerializer(value.value, config);
    }
}

// MARK: Deserializer
/**
@extends {AbstractPayloadDeserializer<number>}
*/ export class BytePayloadDeserializer extends AbstractPayloadDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const dataView = new DataView(new ArrayBuffer(1));

        console.log("b(")
        dataView.setUint8(0, yield);
        console.log(")b")

        return dataView.getInt8(0);
    }
}

/**
@extends {AbstractDeserializer<
    ByteTag,
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class ByteDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const inner = new BytePayloadDeserializer(config);
        let result;
        while ((result = inner.push(yield)) === null);
        return new ByteTag(result);
    }
}