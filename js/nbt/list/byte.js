import { AbstractDeserializer, AbstractSerializer, MAX_LIST_LENGTH, SerializerError } from "../base.js";
import { BytePayloadDeserializer, BytePayloadSerializer, ByteTag } from "../byte.js";
import { IntPayloadDeserializer, IntPayloadSerializer } from "../int.js";
import { ListTag } from "./base.js";
/**
@import { SerializationGenerator, SerializationConfig, DeserializationConfig } from "../base.js"
*/

/**
 * A {@linkcode TagList} that stores bytes (`Uint8[]`).
*/ export class ByteListTag extends ListTag
{
    /**
    @param {readonly (number | ByteTag)[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {readonly number[]}
        @public*/ this.value = value.map(
            (v) => typeof v === "number" ? v : v.value);
    }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }

    /**
    @returns {ArrayIterator<number>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }

    /**
    @returns {unknown}
    @public @override*/ toJSON() { return this.value }
}

// MARK: Serializer
/**
@extends {AbstractSerializer<ByteListTag, SerializationConfig>}
*/ export class ByteListSerializer extends AbstractSerializer
{
    /**
    @param {ByteListTag} value
    @param {SerializationConfig} [config]
    @returns {SerializationGenerator}
    @protected @override*/ *generator(value, config)
    {
        const list = value.value;

        if (list.length > MAX_LIST_LENGTH)
            throw new SerializerError(
                "Array too long.");

        yield* new IntPayloadSerializer(list.length, config);

        if (list.length > 0)
        {
            let elementSerializer = new BytePayloadSerializer(list[0], config);
            yield* elementSerializer;

            for (let i = 1; i < list.length; i++)
            {
                elementSerializer.reset(list[i]);
                yield* elementSerializer;
            }
        }
    }
}

// MARK: Deserializer
/**
@extends {AbstractDeserializer<ByteListTag, DeserializationConfig>}
*/ export class ByteListDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const lengthDeserializer = new IntPayloadDeserializer(config);
        let length;
        while ((length = lengthDeserializer.push(yield)) === null);

        if (length > MAX_LIST_LENGTH)
            throw new SerializerError(
                "Array too long.");

        const elementDeserializer = new BytePayloadDeserializer(config);

        /**
        @type {number[]}
        */ const list = new Array(length);
        for (let i = 0; i < length; i++)
        {
            if (i > 0)
                elementDeserializer.reset();

            let element;
            while ((element = elementDeserializer.push(yield)) === null);

            list[i] = element;
        }

        return new ByteListTag(list);
    }
}