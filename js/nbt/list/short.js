import { AbstractDeserializer, AbstractSerializer, MAX_LIST_LENGTH, SerializerError } from "../base.js";
import { ListTag } from "./base.js";
import { ShortPayloadDeserializer, ShortPayloadSerializer, ShortTag } from "../short.js";
import { IntPayloadSerializer, IntPayloadDeserializer } from "../int.js";
/**
@import { SerializationGenerator, DeserializationGenerator, SerializationConfig, DeserializationConfig, TagUnion, } from "../base.js"
*/

/**
 * A {@linkcode TagList} that stores shorts (`Uint16[]`).
*/ export class ShortListTag extends ListTag
{
    /**
    @param {readonly (number | ShortTag)[]} value
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
@extends {AbstractSerializer<ShortListTag, SerializationConfig>}
*/ export class ShortListSerializer extends AbstractSerializer
{
    /**
    @param {ShortListTag} value
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
            let elementSerializer = new ShortPayloadSerializer(list[0], config);
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
@extends {AbstractDeserializer<ShortListTag, DeserializationConfig>}
*/ export class ShortListDeserializer extends AbstractDeserializer
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

        const elementDeserializer = new ShortPayloadDeserializer(config);

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

        return new ShortListTag(list);
    }
}