import { AbstractDeserializer, AbstractSerializer, MAX_LIST_LENGTH, SerializerError } from "../base.js";
import { ListTag } from "./base.js";
import { CompoundDeserializer, CompoundSerializer, CompoundTag } from "../compound.js";
import { IntPayloadSerializer, IntPayloadDeserializer } from "../int.js";
/**
@import { SerializationGenerator, SerializationConfig, DeserializationConfig, Tag } from "../base.js"
*/

/**
 * A {@linkcode TagList} that stores compounds (`Map<string, Tag>[]`).
*/ export class CompoundListTag extends ListTag
{
    /**
    @param {readonly (Map<string, Tag> | Record<string, Tag> | CompoundTag)[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {readonly CompoundTag[]}
        @public*/ this.value = value.map(
            (v) => v instanceof CompoundTag ? v : new CompoundTag(v));
    }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }

    /**
    @returns {ArrayIterator<CompoundTag>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: Serializer
/**
@extends {AbstractSerializer<CompoundListTag, SerializationConfig>}
*/ export class CompoundListSerializer extends AbstractSerializer
{
    /**
    @param {CompoundListTag} value
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
            let elementSerializer = new CompoundSerializer(list[0], config);
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
@extends {AbstractDeserializer<CompoundListTag, DeserializationConfig>}
*/ export class CompoundListDeserializer extends AbstractDeserializer
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

        const elementDeserializer = new CompoundDeserializer(config);

        /**
        @type {CompoundTag[]}
        */ const list = new Array(length);
        for (let i = 0; i < length; i++)
        {
            if (i > 0)
                elementDeserializer.reset();

            let element;
            while ((element = elementDeserializer.push(yield)) === null);

            list[i] = element;
        }

        return new CompoundListTag(list);
    }
}