import { AbstractDeserializer, AbstractSerializer, MAX_LIST_LENGTH, SerializerError } from "../base.js";
import { IntPayloadSerializer, IntPayloadDeserializer } from "../int.js";
import { ListSerializer, ListDeserializer } from "../list.js";
import { ListTag } from "./base.js";
/**
@import { SerializationGenerator, DeserializationGenerator, SerializationConfig, DeserializationConfig, TagUnion, Tag, } from "../base.js"
*/

/**
 * A {@linkcode TagList} that stores lists.
*/ export class ListListTag extends ListTag
{
    /**
    @param {readonly (ListTag)[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {readonly ListTag[]}
        @public*/ this.value = value;
    }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }

    /**
    @returns {ArrayIterator<ListTag>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }

    /**
    @returns {unknown}
    @public @override*/ toJSON() { return this.value.map((x) => x.toJSON()) }
}

// MARK: Serializer
/**
@extends {AbstractSerializer<ListListTag, SerializationConfig>}
*/ export class ListListSerializer extends AbstractSerializer
{
    /**
    @param {ListListTag} value
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
            let elementSerializer = new ListSerializer(list[0], config);
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
@extends {AbstractDeserializer<ListListTag, DeserializationConfig>}
*/ export class ListListDeserializer extends AbstractDeserializer
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

        const elementDeserializer = new ListDeserializer(config);

        /**
        @type {ListTag[]}
        */ const list = new Array(length);
        for (let i = 0; i < length; i++)
        {
            if (i > 0)
                elementDeserializer.reset();

            let element;
            while ((element = elementDeserializer.push(yield)) === null);

            list[i] = element;
        }

        return new ListListTag(list);
    }
}