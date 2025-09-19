import { AbstractDeserializer, AbstractPayloadDeserializer, AbstractPayloadSerializer, AbstractSerializer, MAX_STRING_LENGTH, SerializerError, Tag } from "./base.js";
/**
@import { SerializationGenerator, SerializationConfig, DeserializationConfig } from "./base.js"
*/

// MARK: StringTag
/**
 * A {@linkcode Tag} that stores a string (`UTF-8`).
*/ export class StringTag extends Tag
{
    /**
    @param {string} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {string}
        @private*/ this._value = value;
    }

    /**
    @returns {string}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }

    /**
    @returns {unknown}
    @public @override*/ toJSON() { return this.value }
}

// MARK: Serializer
/**
@extends {AbstractPayloadSerializer<string, SerializationConfig>}
*/ export class StringPayloadSerializer extends AbstractPayloadSerializer
{
    /**
    @param {string} value
    @param {SerializationConfig} [config]
    @returns {SerializationGenerator}
    @protected @override*/ *generator(value, config)
    {
        const bytes = new TextEncoder().encode(value);

        if (bytes.length > MAX_STRING_LENGTH)
            throw new SerializerError(
                "String too long.");

        const dataView = new DataView(new ArrayBuffer(2));

        dataView.setUint16(0, value.length, config?.endian === "little");

        for (let i = 0; i < 2; i++)
            yield dataView.getUint8(i);

        for (let i = 0; i < bytes.length; i++)
            yield bytes[i];

        return undefined;
    }
}

/**
@extends {AbstractSerializer<StringTag, SerializationConfig>}
*/ export class StringSerializer extends AbstractSerializer
{
    /**
    @param {StringTag} value
    @param {SerializationConfig} [config]
    @protected @override*/ generator(value, config)
    {
        return new StringPayloadSerializer(value.value, config);
    }
}

// MARK: Deserializer
/**
@extends {AbstractPayloadDeserializer<string>}
*/ export class StringPayloadDeserializer extends AbstractPayloadDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const dataView = new DataView(new ArrayBuffer(2));

        for (let i = 0; i < 2; i++)
            dataView.setUint8(i, yield);

        const length = dataView.getUint16(0, config?.endian === "little");

        if (length > MAX_STRING_LENGTH)
            throw new SerializerError(
                "String too long.");

        const bytes = new Int8Array(length);

        for (let i = 0; i < length; i++)
            bytes[i] = yield;

        return new TextDecoder().decode(bytes);
    }
}

/**
@extends {AbstractDeserializer<
    StringTag,
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class StringDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const inner = new StringPayloadDeserializer(config);
        let result;
        while ((result = inner.push(yield)) === null);
        return new StringTag(result);
    }
}