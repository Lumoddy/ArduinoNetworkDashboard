import { AbstractDeserializer, AbstractPayloadDeserializer, AbstractPayloadSerializer, AbstractSerializer, Deserializer, DeserializerError, MAX_LIST_LENGTH, PayloadDeserializer, PayloadSerializer, Serializer, SerializerError, Tag } from "./base.js";
/**
@import { SerializationGenerator, DeserializationGenerator, SerializationConfig, DeserializationConfig } from "./base.js"
*/

// MARK: LongArrayTag
/**
 * A {@linkcode Tag} that stores a byte array (`BigInt64Array`).
*/ export class LongArrayTag extends Tag
{
    /**
    @param {BigInt64Array<ArrayBuffer>} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {BigInt64Array<ArrayBuffer>}
        @private*/ this._value = value;
    }

    /**
    @returns {BigInt64Array<ArrayBuffer>}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}

// MARK: Serializer
/**
@extends {AbstractPayloadSerializer<BigInt64Array<ArrayBuffer>, SerializationConfig>}
*/ export class LongArrayPayloadSerializer extends AbstractPayloadSerializer
{
    /**
    @param {BigInt64Array<ArrayBuffer>} value
    @param {SerializationConfig} [config]
    @protected @override*/ *generator(value, config)
    {
        if (value.length > MAX_LIST_LENGTH)
            throw new SerializerError(
                "Array too long.");

        const dataView = new DataView(new ArrayBuffer(8));
        const littleEndian = config?.endian === "little";

        dataView.setUint32(0, value.length, littleEndian);

        for (let i = 0; i < 4; i++)
            yield dataView.getUint8(i);

        for (let i = 0; i < value.length; i++)
        {
            dataView.setBigInt64(0, value[i], littleEndian);

            for (let i = 0; i < 8; i++)
                yield dataView.getUint8(i);
        }

        return undefined;
    }
}

/**
@extends {AbstractSerializer<LongArrayTag, SerializationConfig>}
*/ export class LongArraySerializer extends AbstractSerializer
{
    /**
    @param {LongArrayTag} value
    @param {SerializationConfig} [config]
    @protected @override*/ generator(value, config)
    {
        return new LongArrayPayloadSerializer(value.value, config);
    }
}

// MARK: Deserializer
/**
@extends {AbstractPayloadDeserializer<BigInt64Array<ArrayBuffer>>}
*/ export class LongArrayPayloadDeserializer extends AbstractPayloadDeserializer
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

        const array = new BigInt64Array(length);

        for (let i = 0; i < length; i++)
        {
            for (let i = 0; i < 4; i++)
                dataView.setUint8(i, yield);

            array[i] = dataView.getBigInt64(0, littleEndian);
        }

        return array;
    }
}

/**
@extends {AbstractDeserializer<
    LongArrayTag,
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class LongArrayDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const inner = new LongArrayPayloadDeserializer(config);
        let result;
        while ((result = inner.push(yield)) === null);
        return new LongArrayTag(result);
    }
}