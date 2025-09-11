import { AbstractDeserializer, AbstractPayloadDeserializer, AbstractPayloadSerializer, AbstractSerializer, Deserializer, DeserializerError, MAX_LIST_LENGTH, PayloadDeserializer, PayloadSerializer, Serializer, SerializerError, Tag } from "./base.js";
/**
@import { SerializationGenerator, DeserializationGenerator, SerializationConfig, DeserializationConfig } from "./base.js"
*/

// MARK: IntArrayTag
/**
 * A {@linkcode Tag} that stores a byte array (`Int32Array`).
*/ export class IntArrayTag extends Tag
{
    /**
    @param {Int32Array<ArrayBuffer>} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {Int32Array<ArrayBuffer>}
        @private*/ this._value = value;
    }

    /**
    @returns {Int32Array<ArrayBuffer>}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}

// MARK: Serializer
/**
@extends {AbstractPayloadSerializer<Int32Array<ArrayBuffer>, SerializationConfig>}
*/ export class IntArrayPayloadSerializer extends AbstractPayloadSerializer
{
    /**
    @param {Int32Array<ArrayBuffer>} value
    @param {SerializationConfig} [config]
    @protected @override*/ *generator(value, config)
    {
        if (value.length > MAX_LIST_LENGTH)
            throw new SerializerError(
                "Array too long.");

        const dataView = new DataView(new ArrayBuffer(4));
        const littleEndian = config?.endian === "little";

        dataView.setUint32(0, value.length, littleEndian);

        for (let i = 0; i < 4; i++)
            yield dataView.getUint8(i);

        for (let i = 0; i < value.length; i++)
        {
            dataView.setInt32(0, value[i], littleEndian);

            for (let i = 0; i < 4; i++)
                yield dataView.getUint8(i);
        }

        return undefined;
    }
}

/**
@extends {AbstractSerializer<IntArrayTag, SerializationConfig>}
*/ export class IntArraySerializer extends AbstractSerializer
{
    /**
    @param {IntArrayTag} value
    @param {SerializationConfig} [config]
    @protected @override*/ generator(value, config)
    {
        return new IntArrayPayloadSerializer(value.value, config);
    }
}

// MARK: Deserializer
/**
@extends {AbstractPayloadDeserializer<Int32Array<ArrayBuffer>>}
*/ export class IntArrayPayloadDeserializer extends AbstractPayloadDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const dataView = new DataView(new ArrayBuffer(4));
        const littleEndian = config?.endian === "little";

        for (let i = 0; i < 4; i++)
            dataView.setUint8(i, yield);

        const length = dataView.getUint32(0, littleEndian);

        if (length > MAX_LIST_LENGTH)
            throw new SerializerError(
                "Array too long.");

        const array = new Int32Array(length);

        for (let i = 0; i < length; i++)
        {
            for (let i = 0; i < 4; i++)
                dataView.setUint8(i, yield);

            array[i] = dataView.getInt32(0, littleEndian);
        }

        return array;
    }
}

/**
@extends {AbstractDeserializer<
    IntArrayTag,
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class IntArrayDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const inner = new IntArrayPayloadDeserializer(config);
        let result;
        while ((result = inner.push(yield)) === null);
        return new IntArrayTag(result);
    }
}