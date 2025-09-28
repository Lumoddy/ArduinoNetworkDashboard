import { AbstractDeserializer, AbstractPayloadDeserializer, AbstractPayloadSerializer, AbstractSerializer, Tag } from "./base.js";
/**
@import { SerializationConfig, DeserializationConfig } from "./base.js"
*/

// MARK: FloatTag
/**
 * A {@linkcode Tag} that stores a float (`Float32`).
*/ export class FloatTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = Number(value);
    }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = Number(value) }

    /**
    @returns {unknown}
    @public @override*/ toJSON() { return this.value }
}

// MARK: Serializer
/**
@extends {AbstractPayloadSerializer<number, SerializationConfig>}
*/ export class FloatPayloadSerializer extends AbstractPayloadSerializer
{
    /**
    @param {number} value
    @param {SerializationConfig} [config]
    @protected @override*/ *generator(value, config)
    {
        const dataView = new DataView(new ArrayBuffer(4));

        dataView.setFloat32(0, value, config?.endian === "little");

        for (let i = 0; i < 4; i++)
            yield dataView.getUint8(i);

        return undefined;
    }
}

/**
@extends {AbstractSerializer<FloatTag, SerializationConfig>}
*/ export class FloatSerializer extends AbstractSerializer
{
    /**
    @param {FloatTag} value
    @param {SerializationConfig} [config]
    @protected @override*/ generator(value, config)
    {
        return new FloatPayloadSerializer(value.value, config);
    }
}

// MARK: Deserializer
/**
@extends {AbstractPayloadDeserializer<number>}
*/ export class FloatPayloadDeserializer extends AbstractPayloadDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const dataView = new DataView(new ArrayBuffer(4));

        for (let i = 0; i < 4; i++)
            dataView.setUint8(i, yield);

        return dataView.getFloat32(0, config?.endian === "little");
    }
}

/**
@extends {AbstractDeserializer<
    FloatTag,
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class FloatDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const inner = new FloatPayloadDeserializer(config);
        let result;
        while ((result = inner.push(yield)) === null);
        return new FloatTag(result);
    }
}