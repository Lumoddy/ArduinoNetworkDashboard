import { AbstractDeserializer, AbstractSerializer, MAX_LIST_LENGTH, SerializerError } from "../base.js";
import { IntPayloadSerializer, IntPayloadDeserializer } from "../int.js";
import { ListTag } from "./base.js";
/**
@import { SerializationGenerator, DeserializationGenerator, SerializationConfig, DeserializationConfig, TagUnion } from "../base.js"
*/

/**
 * A {@linkcode TagList} that stores bytes (`Uint8[]`).
*/ export class EmptyListTag extends ListTag
{
    /**
    @param {readonly []} [value]
    @public*/ constructor(value)
    {
        super();
    }

    /**
    @returns {number}
    @public @override*/ get length() { return 0 }

    /**
    @returns {ArrayIterator<never>}
    @public*/ [Symbol.iterator]() { return [][Symbol.iterator]() }

    /**
    @returns {unknown}
    @public @override*/ toJSON() { return [] }
}

// MARK: Serializer
/**
@extends {AbstractSerializer<EmptyListTag, SerializationConfig>}
*/ export class EmptyListSerializer extends AbstractSerializer
{
    /**
    @param {EmptyListTag} value
    @param {SerializationConfig} [config]
    @returns {SerializationGenerator}
    @protected @override*/ *generator(value, config)
    {
        yield* new IntPayloadSerializer(0, config);
    }
}

// MARK: Deserializer
/**
@extends {AbstractDeserializer<EmptyListTag, DeserializationConfig>}
*/ export class EmptyListDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const lengthDeserializer = new IntPayloadDeserializer(config);
        let length;
        while ((length = lengthDeserializer.push(yield)) === null);

        if (length > 0)
            throw new SerializerError(
                "Empty list type was not found to be empty.");

        return new EmptyListTag();
    }
}