import { AbstractDeserializer, AbstractSerializer, MAX_LIST_LENGTH, SerializerError } from "../base.js";
import { ListTag } from "./base.js";
import { IntArrayPayloadDeserializer, IntArrayPayloadSerializer, IntArrayTag } from "../int-array.js";
import { IntPayloadSerializer, IntPayloadDeserializer } from "../int.js";
/**
@import { SerializationGenerator, DeserializationGenerator, SerializationConfig, DeserializationConfig, TagUnion } from "../base.js"
*/

/**
 * A {@linkcode TagList} that stores floats (`Int32Array[]`).
*/ export class IntArrayListTag extends ListTag
{
    /**
    @param {readonly (Int32Array<ArrayBuffer> | IntArrayTag)[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {readonly Int32Array<ArrayBuffer>[]}
        @public*/ this.value = value.map(
            (v) => v instanceof Int32Array ? v : v.value);
    }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }

    /**
    @returns {ArrayIterator<Int32Array<ArrayBuffer>>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }

    /**
    @returns {unknown}
    @public @override*/ toJSON() { return this.value.map((x) => [...x]) }
}

// MARK: Serializer
/**
@extends {AbstractSerializer<IntArrayListTag, SerializationConfig>}
*/ export class IntArrayListSerializer extends AbstractSerializer
{
    /**
    @param {IntArrayListTag} value
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
            let elementSerializer = new IntArrayPayloadSerializer(list[0], config);
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
@extends {AbstractDeserializer<IntArrayListTag, DeserializationConfig>}
*/ export class IntArrayListDeserializer extends AbstractDeserializer
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

        const elementDeserializer = new IntArrayPayloadDeserializer(config);

        /**
        @type {Int32Array<ArrayBuffer>[]}
        */ const list = new Array(length);
        for (let i = 0; i < length; i++)
        {
            if (i > 0)
                elementDeserializer.reset();

            let element;
            while ((element = elementDeserializer.push(yield)) === null);

            list[i] = element;
        }

        return new IntArrayListTag(list);
    }
}