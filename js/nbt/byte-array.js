import { AbstractDeserializer, AbstractPayloadDeserializer, AbstractPayloadSerializer, AbstractSerializer, Deserializer, DeserializerError, MAX_LIST_LENGTH, PayloadDeserializer, PayloadSerializer, Serializer, SerializerError, Tag } from "./base.js";
/**
@import { SerializationGenerator, DeserializationGenerator, SerializationConfig, DeserializationConfig } from "./base.js"
*/

// MARK: ByteArrayTag
/**
 * A {@linkcode Tag} that stores a byte array (`Int8Array`).
*/ export class ByteArrayTag extends Tag
{
    /**
    @param {Int8Array<ArrayBuffer>} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {Int8Array<ArrayBuffer>}
        @private*/ this._value = value;
    }

    /**
    @returns {Int8Array<ArrayBuffer>}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}

// MARK: Serializer
/**
@extends {AbstractPayloadSerializer<Int8Array<ArrayBuffer>, SerializationConfig>}
*/ export class ByteArrayPayloadSerializer extends AbstractPayloadSerializer
{
    /**
    @param {Int8Array<ArrayBuffer>} value
    @param {SerializationConfig} [config]

    @protected @override*/ *generator(value, config)
    {
        if (value.length > MAX_LIST_LENGTH)
            throw new SerializerError(
                "Array too long.");

        const dataView = new DataView(new ArrayBuffer(4));

        dataView.setUint32(0, value.length, config?.endian === "little");

        for (let i = 0; i < 4; i++)
            yield dataView.getUint8(i);

        for (let i = 0; i < value.length; i++)
            yield value[i];

        return undefined;
    }
}

/**
@extends {AbstractSerializer<ByteArrayTag, SerializationConfig>}
*/ export class ByteArraySerializer extends AbstractSerializer
{
    /**
    @param {ByteArrayTag} value
    @param {SerializationConfig} [config]
    @protected @override*/ generator(value, config)
    {
        return new ByteArrayPayloadSerializer(value.value, config);
    }
}

// MARK: Deserializer
/**
@extends {AbstractPayloadDeserializer<Int8Array<ArrayBuffer>>}
*/ export class ByteArrayPayloadDeserializer extends AbstractPayloadDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const dataView = new DataView(new ArrayBuffer(4));

        for (let i = 0; i < 4; i++)
            dataView.setUint8(i, yield);

        const length = dataView.getUint32(0, config?.endian === "little");

        if (length > MAX_LIST_LENGTH)
            throw new SerializerError(
                "Array too long.");

        const array = new Int8Array(length);

        for (let i = 0; i < length; i++)
            array[i] = yield;

        return array;
    }
}

/**
@extends {AbstractDeserializer<
    ByteArrayTag,
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class ByteArrayDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const inner = new ByteArrayPayloadDeserializer(config);
        let result;
        while ((result = inner.push(yield)) === null);
        return new ByteArrayTag(result);
    }
}