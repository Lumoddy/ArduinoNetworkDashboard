import { ListTag } from "./list/base.js";
import { AbstractDeserializer, AbstractSerializer, DeserializerError } from "./base.js";
import { ByteListTag } from "./list.js";
import { ByteListSerializer } from "./list.js";
import { ShortListDeserializer, ShortListSerializer, ShortListTag } from "./list/short.js";
import { IntListDeserializer, IntListSerializer, IntListTag } from "./list/int.js";
import { LongListDeserializer, LongListSerializer, LongListTag } from "./list/long.js";
import { FloatListDeserializer, FloatListSerializer, FloatListTag } from "./list/float.js";
import { DoubleListDeserializer, DoubleListSerializer, DoubleListTag } from "./list/double.js";
import { ByteArrayListDeserializer, ByteArrayListSerializer, ByteArrayListTag } from "./list/byte-array.js";
import { StringListDeserializer, StringListSerializer, StringListTag } from "./list/string.js";
import { ListListDeserializer, ListListSerializer, ListListTag } from "./list/list.js";
import { CompoundListDeserializer, CompoundListSerializer, CompoundListTag } from "./list/compound.js";
import { IntArrayListDeserializer, IntArrayListSerializer, IntArrayListTag } from "./list/int-array.js";
import { LongArrayListDeserializer, LongArrayListSerializer, LongArrayListTag } from "./list/long-array.js";
import { EmptyListSerializer, EmptyListTag } from "./list/empty.js";
import { ByteListDeserializer } from "./list/byte.js";
/**
@import { SerializationConfig, DeserializationConfig } from "./base.js"
@import { ListTagUnion } from "./list/base.js"
*/

export * from "./list/base.js";
export * from "./list/byte.js";
export * from "./list/short.js";
export * from "./list/int.js";
export * from "./list/long.js";
export * from "./list/float.js";
export * from "./list/double.js";
export * from "./list/byte-array.js";
export * from "./list/string.js";
export * from "./list/compound.js";
export * from "./list/list.js";
export * from "./list/int-array.js";
export * from "./list/long-array.js";

// MARK: Serializer
/**
@extends {AbstractSerializer<ListTag, SerializationConfig>}
*/ export class ListSerializer extends AbstractSerializer
{
    /**
    @param {ListTag} value
    @param {SerializationConfig} [config]
    @protected @override*/ *generator(value, config)
    {
        switch (true)
        {
            case value instanceof EmptyListTag:
                yield 0;
                yield* new EmptyListSerializer(value, config);
                break;
            case value instanceof ByteListTag:
                yield 1;
                yield* new ByteListSerializer(value, config);
                break;
            case value instanceof ShortListTag:
                yield 2;
                yield* new ShortListSerializer(value, config);
                break;
            case value instanceof IntListTag:
                yield 3;
                yield* new IntListSerializer(value, config);
                break;
            case value instanceof LongListTag:
                yield 4;
                yield* new LongListSerializer(value, config);
                break;
            case value instanceof FloatListTag:
                yield 5;
                yield* new FloatListSerializer(value, config);
                break;
            case value instanceof DoubleListTag:
                yield 6;
                yield* new DoubleListSerializer(value, config);
                break;
            case value instanceof ByteArrayListTag:
                yield 7;
                yield* new ByteArrayListSerializer(value, config);
                break;
            case value instanceof StringListTag:
                yield 8;
                yield* new StringListSerializer(value, config);
                break;
            case value instanceof ListListTag:
                yield 9;
                yield* new ListListSerializer(value, config);
                break;
            case value instanceof CompoundListTag:
                yield 10;
                yield* new CompoundListSerializer(value, config);
                break;
            case value instanceof IntArrayListTag:
                yield 11;
                yield* new IntArrayListSerializer(value, config);
                break;
            case value instanceof LongArrayListTag:
                yield 12;
                yield* new LongArrayListSerializer(value, config);
                break;
        }

        return undefined;
    }
}

// MARK: Deserializer
/**
@extends {AbstractDeserializer<
    ListTagUnion,
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class ListDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @returns {Generator<undefined, ListTagUnion, number>}
    @protected @override*/ *generator(config)
    {
        let valueDeserializer;
        switch (yield)
        {
            case 1:
                valueDeserializer = new ByteListDeserializer(config);
                break;
            case 2:
                valueDeserializer = new ShortListDeserializer(config);
                break;
            case 3:
                valueDeserializer = new IntListDeserializer(config);
                break;
            case 4:
                valueDeserializer = new LongListDeserializer(config);
                break;
            case 5:
                valueDeserializer = new FloatListDeserializer(config);
                break;
            case 6:
                valueDeserializer = new DoubleListDeserializer(config);
                break;
            case 7:
                valueDeserializer = new ByteArrayListDeserializer(config);
                break;
            case 8:
                valueDeserializer = new StringListDeserializer(config);
                break;
            case 9:
                valueDeserializer = new ListListDeserializer(config);
                break;
            case 10:
                valueDeserializer = new CompoundListDeserializer(config);
                break;
            case 11:
                valueDeserializer = new IntArrayListDeserializer(config);
                break;
            case 12:
                valueDeserializer = new LongArrayListDeserializer(config);
                break;
            default:
                throw new DeserializerError(
                    "Found invalid type in list.");
        }

        let value;
        while ((value = valueDeserializer.push(yield)) === null);

        return value;
    }
}