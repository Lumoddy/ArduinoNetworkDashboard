import { AbstractDeserializer, AbstractSerializer, MAX_LIST_LENGTH, SerializerError } from "../base.js";
import { ListTag } from "./base.js";
import { LongPayloadDeserializer, LongPayloadSerializer, LongTag } from "../long.js";
import { IntPayloadSerializer, IntPayloadDeserializer } from "../int.js";
/**
@import { SerializationGenerator, DeserializationGenerator, SerializationConfig, DeserializationConfig, TagUnion, } from "../base.js"
*/

/**
 * A {@linkcode TagList} that stores longs (`BigInt64[]`).
*/ export class LongListTag extends ListTag
{
    /**
    @param {readonly (bigint | LongTag)[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {readonly bigint[]}
        @public*/ this.value = value.map(
            (v) => typeof v === "bigint" ? v : v.value);
    }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }

    /**
    @returns {ArrayIterator<bigint>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: Serializer
/**
@extends {AbstractSerializer<LongListTag, SerializationConfig>}
*/ export class LongListSerializer extends AbstractSerializer
{
    /**
    @param {LongListTag} value
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
            let elementSerializer = new LongPayloadSerializer(list[0], config);
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
@extends {AbstractDeserializer<LongListTag, DeserializationConfig>}
*/ export class LongListDeserializer extends AbstractDeserializer
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

        const elementDeserializer = new LongPayloadDeserializer(config);

        /**
        @type {bigint[]}
        */ const list = new Array(length);
        for (let i = 0; i < length; i++)
        {
            if (i > 0)
                elementDeserializer.reset();

            let element;
            while ((element = elementDeserializer.push(yield)) === null);

            list[i] = element;
        }

        return new LongListTag(list);
    }
}