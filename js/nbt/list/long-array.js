import { AbstractDeserializer, AbstractSerializer, MAX_LIST_LENGTH, SerializerError } from "../base.js";
import { ListTag } from "./base.js";
import { LongArrayPayloadDeserializer, LongArrayPayloadSerializer, LongArrayTag } from "../long-array.js";
import { IntPayloadSerializer, IntPayloadDeserializer } from "../int.js";
/**
@import { SerializationGenerator, DeserializationGenerator, SerializationConfig, DeserializationConfig, TagUnion } from "../base.js"
*/

/**
 * A {@linkcode TagList} that stores floats (`BigInt64Array[]`).
*/ export class LongArrayListTag extends ListTag
{
    /**
    @param {readonly (BigInt64Array<ArrayBuffer> | LongArrayTag)[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {readonly BigInt64Array<ArrayBuffer>[]}
        @public*/ this.value = value.map(
            (v) => v instanceof BigInt64Array ? v : v.value);
    }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }

    /**
    @returns {ArrayIterator<BigInt64Array<ArrayBuffer>>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: Serializer
/**
@extends {AbstractSerializer<LongArrayListTag, SerializationConfig>}
*/ export class LongArrayListSerializer extends AbstractSerializer
{
    /**
    @param {LongArrayListTag} value
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
            let elementSerializer = new LongArrayPayloadSerializer(list[0], config);
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
@extends {AbstractDeserializer<LongArrayListTag, DeserializationConfig>}
*/ export class LongArrayListDeserializer extends AbstractDeserializer
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

        const elementDeserializer = new LongArrayPayloadDeserializer(config);

        /**
        @type {BigInt64Array<ArrayBuffer>[]}
        */ const list = new Array(length);
        for (let i = 0; i < length; i++)
        {
            if (i > 0)
                elementDeserializer.reset();

            let element;
            while ((element = elementDeserializer.push(yield)) === null);

            list[i] = element;
        }

        return new LongArrayListTag(list);
    }
}