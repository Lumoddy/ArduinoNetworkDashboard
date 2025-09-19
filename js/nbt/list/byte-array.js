import { AbstractDeserializer, AbstractSerializer, MAX_LIST_LENGTH, SerializerError } from "../base.js";
import { ByteArrayPayloadDeserializer, ByteArrayPayloadSerializer, ByteArrayTag } from "../byte-array.js";
import { IntPayloadDeserializer, IntPayloadSerializer } from "../int.js";
import { ListTag } from "./base.js";
/**
@import { SerializationGenerator, SerializationConfig, DeserializationConfig } from "../base.js"
*/

/**
 * A {@linkcode TagList} that stores floats (`Int8Array[]`).
*/ export class ByteArrayListTag extends ListTag
{
    /**
    @param {readonly (Int8Array<ArrayBuffer> | ByteArrayTag)[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {readonly Int8Array<ArrayBuffer>[]}
        @public*/ this.value = value.map(
            (v) => v instanceof Int8Array ? v : v.value);
    }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }

    /**
    @returns {ArrayIterator<Int8Array>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }

    /**
    @returns {unknown}
    @public @override*/ toJSON() { return this.value.map((x) => [...x]) }
}

// MARK: Serializer
/**
@extends {AbstractSerializer<ByteArrayListTag, SerializationConfig>}
*/ export class ByteArrayListSerializer extends AbstractSerializer
{
    /**
    @param {ByteArrayListTag} value
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
            let elementSerializer = new ByteArrayPayloadSerializer(list[0], config);
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
@extends {AbstractDeserializer<ByteArrayListTag, DeserializationConfig>}
*/ export class ByteArrayListDeserializer extends AbstractDeserializer
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

        const elementDeserializer = new ByteArrayPayloadDeserializer(config);

        /**
        @type {Int8Array<ArrayBuffer>[]}
        */ const list = new Array(length);
        for (let i = 0; i < length; i++)
        {
            if (i > 0)
                elementDeserializer.reset();

            let element;
            while ((element = elementDeserializer.push(yield)) === null);

            list[i] = element;
        }

        return new ByteArrayListTag(list);
    }
}