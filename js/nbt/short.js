import { AbstractDeserializer, AbstractPayloadDeserializer, AbstractPayloadSerializer, AbstractSerializer, Tag } from "./base.js";
/**
@import { SerializationConfig, DeserializationConfig } from "./base.js"
*/

// MARK: ShortTag
/**
 * A {@linkcode Tag} that stores a short (`Uint16`).
*/ export class ShortTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = value & 0xFFFF;
        if ((this._value & 0x8000) !== 0)
            this._value -= 0x8000;
    }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value)
    {
        this._value = value & 0xFFFF;
        if ((this._value & 0x8000) !== 0)
            this._value -= 0x8000;
    }

    /**
    @returns {unknown}
    @public @override*/ toJSON() { return this.value }
}

// MARK: Serializer
/**
@extends {AbstractPayloadSerializer<number, SerializationConfig>}
*/ export class ShortPayloadSerializer extends AbstractPayloadSerializer
{
    /**
    @param {number} value
    @param {SerializationConfig} [config]
    @protected @override*/ *generator(value, config)
    {
        const dataView = new DataView(new ArrayBuffer(2));

        dataView.setInt16(0, value, config?.endian === "little");

        for (let i = 0; i < 2; i++)
            yield dataView.getUint8(i);

        return undefined;
    }
}

/**
@extends {AbstractSerializer<ShortTag, SerializationConfig>}
*/ export class ShortSerializer extends AbstractSerializer
{
    /**
    @param {ShortTag} value
    @param {SerializationConfig} [config]
    @protected @override*/ generator(value, config)
    {
        return new ShortPayloadSerializer(value.value, config);
    }
}

// MARK: Deserializer
/**
@extends {AbstractPayloadDeserializer<number>}
*/ export class ShortPayloadDeserializer extends AbstractPayloadDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const dataView = new DataView(new ArrayBuffer(2));

        for (let i = 0; i < 2; i++)
            dataView.setUint8(i, yield);

        return dataView.getInt16(0, config?.endian === "little");
    }
}

/**
@extends {AbstractDeserializer<
    ShortTag,
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class ShortDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const inner = new ShortPayloadDeserializer(config);
        let result;
        while ((result = inner.push(yield)) === null);
        return new ShortTag(result);
    }
}