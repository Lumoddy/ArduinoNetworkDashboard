import { AbstractDeserializer, AbstractSerializer, MAX_LIST_LENGTH, SerializerError } from "../base.js";
import { ListTag } from "./base.js";
import { FloatPayloadDeserializer, FloatPayloadSerializer, FloatTag } from "../float.js";
import { IntPayloadSerializer, IntPayloadDeserializer } from "../int.js";
/**
@import { SerializationGenerator, DeserializationGenerator, SerializationConfig, DeserializationConfig, TagUnion } from "../base.js"
*/

/**
 * A {@linkcode TagList} that stores floats (`Float32[]`).
*/ export class FloatListTag extends ListTag
{
    /**
    @param {readonly (number | FloatTag)[]} value
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
}

// MARK: Serializer
/**
@extends {AbstractSerializer<FloatListTag, SerializationConfig>}
*/ export class FloatListSerializer extends AbstractSerializer
{
    /**
    @param {FloatListTag} value
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
            let elementSerializer = new FloatPayloadSerializer(list[0], config);
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
@extends {AbstractDeserializer<FloatListTag, DeserializationConfig>}
*/ export class FloatListDeserializer extends AbstractDeserializer
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

        const elementDeserializer = new FloatPayloadDeserializer(config);

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

        return new FloatListTag(list);
    }
}