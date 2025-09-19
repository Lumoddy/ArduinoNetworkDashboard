import { AbstractDeserializer, AbstractPayloadDeserializer, AbstractPayloadSerializer, AbstractSerializer, Tag } from "./base.js";
/**
@import { SerializationConfig, DeserializationConfig } from "./base.js"
*/

// MARK: IntTag
/**
 * A {@linkcode Tag} that stores an int (`Int32`).
*/ export class IntTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = value & 0xFFFFFFFF;
    }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value & 0xFFFFFFFF }

    /**
    @returns {unknown}
    @public @override*/ toJSON() { return this.value }
}

// MARK: Serializer
/**
@extends {AbstractPayloadSerializer<number, SerializationConfig>}
*/ export class IntPayloadSerializer extends AbstractPayloadSerializer
{
    /**
    @param {number} value
    @param {SerializationConfig} [config]
    @protected @override*/ *generator(value, config)
    {
        const dataView = new DataView(new ArrayBuffer(4));

        dataView.setInt32(0, value, config?.endian === "little");

        for (let i = 0; i < 4; i++)
            yield dataView.getUint8(i);

        return undefined;
    }
}

/**
@extends {AbstractSerializer<IntTag, SerializationConfig>}
*/ export class IntSerializer extends AbstractSerializer
{
    /**
    @param {IntTag} value
    @param {SerializationConfig} [config]
    @protected @override*/ generator(value, config)
    {
        return new IntPayloadSerializer(value.value, config);
    }
}

// MARK: Deserializer
/**
@extends {AbstractPayloadDeserializer<number>}
*/ export class IntPayloadDeserializer extends AbstractPayloadDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const dataView = new DataView(new ArrayBuffer(4));

        for (let i = 0; i < 4; i++)
            dataView.setUint8(i, yield);

        return dataView.getInt32(0, config?.endian === "little");
    }
}

/**
@extends {AbstractDeserializer<
    IntTag,
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class IntDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const inner = new IntPayloadDeserializer(config);
        let result;
        while ((result = inner.push(yield)) === null);
        return new IntTag(result);
    }
}