import { AbstractDeserializer, AbstractPayloadDeserializer, AbstractPayloadSerializer, AbstractSerializer, DeserializerError, SerializerError, Tag } from "./base.js";
import { ByteArrayDeserializer, ByteArraySerializer, ByteArrayTag } from "./byte-array.js";
import { ByteDeserializer, ByteSerializer, ByteTag } from "./byte.js";
import { DoubleDeserializer, DoubleSerializer, DoubleTag } from "./double.js";
import { FloatDeserializer, FloatSerializer, FloatTag } from "./float.js";
import { IntArrayDeserializer, IntArraySerializer, IntArrayTag } from "./int-array.js";
import { IntDeserializer, IntSerializer, IntTag } from "./int.js";
import { ListDeserializer, ListSerializer, ListTag } from "./list.js";
import { LongArrayDeserializer, LongArraySerializer, LongArrayTag } from "./long-array.js";
import { LongDeserializer, LongSerializer, LongTag } from "./long.js";
import { ShortDeserializer, ShortSerializer, ShortTag } from "./short.js";
import { StringDeserializer, StringPayloadDeserializer, StringPayloadSerializer, StringSerializer, StringTag } from "./string.js";
/**
@import { SerializationConfig, DeserializationConfig, TagUnion } from "./base.js"
*/

// MARK: CompoundTag
/**
 * A {@linkcode Tag} that stores uniquely named tags (`Map<string, Tag>`).
*/ export class CompoundTag extends Tag
{
    /**
    @param {Iterable<[string, { readonly toNBT: () => Tag }]>} [values]
    @public*/ constructor(values)
    {
        super();

        /**
        @type {Map<string, Tag>}
        @private*/ this._value = new Map();

        if (values !== undefined)
        {
            for (const [key, value] of values)
                this._value.set(String(key), value.toNBT());
        }
    }

    /**
    @public*/ clear() { return this._value.clear() }

    /**
    @param {string} key
    @returns {boolean}
    @public*/ delete(key) { return this._value.delete(key) }

    /**
    @param {(value: Tag, key: string, map: Map<string, Tag>) => void} callbackfn
    @param {any} [thisArg]
    @public*/ forEach(callbackfn, thisArg)
    {
        return this._value.forEach(callbackfn, thisArg);
    }

    /**
    @param {string} key
    @returns {Tag | undefined}
    @public*/ get(key) { return this._value.get(key) }

    /**
    @param {string} key
    @returns {boolean}
    @public*/ has(key) { return this._value.has(key) }

    /**
    @param {string} key
    @param {Tag} value
    @public*/ set(key, value) { return this._value.set(key, value) }

    /**
    @returns {number}
    @public @readonly*/ get size() { return this._value.size }

    /**
    @returns {MapIterator<[string, Tag]>}
    @public*/ [Symbol.iterator]() { return this._value[Symbol.iterator]() }

    /**
    @returns {MapIterator<[string, Tag]>}
    @public*/ entries() { return this._value.entries() }

    /**
    @returns {MapIterator<string>}
    @public*/ keys() { return this._value.keys() }

    /**
    @returns {MapIterator<Tag>}
    @public*/ values() { return this._value.values() }

    /**
    @returns {unknown}
    @public @override*/ toJSON()
    {
        /**
        @type {Record<string, Tag>}
        */ const result = {};
        for (const [name, tag] of this._value)
            result[name] = tag;

        return result;
    }
}

// MARK: Serializer
/**
@extends {AbstractPayloadSerializer<[name: string, tag: Tag], SerializationConfig>}
*/ export class EntrySerializer extends AbstractPayloadSerializer
{
    /**
    @param {[name: string, tag: Tag]} entry
    @param {SerializationConfig} [config]
    @protected @override*/ *generator(entry, config)
    {
        const [name, tag] = entry;
        const nameSerializer = new StringPayloadSerializer(name, config);
        switch (true)
        {
            case tag instanceof ByteTag:
                yield 1;
                yield* nameSerializer;
                yield* new ByteSerializer(tag, config);
                break;
            case tag instanceof ShortTag:
                yield 2;
                yield* nameSerializer;
                yield* new ShortSerializer(tag, config);
                break;
            case tag instanceof IntTag:
                yield 3;
                yield* nameSerializer;
                yield* new IntSerializer(tag, config);
                break;
            case tag instanceof LongTag:
                yield 4;
                yield* nameSerializer;
                yield* new LongSerializer(tag, config);
                break;
            case tag instanceof FloatTag:
                yield 5;
                yield* nameSerializer;
                yield* new FloatSerializer(tag, config);
                break;
            case tag instanceof DoubleTag:
                yield 6;
                yield* nameSerializer;
                yield* new DoubleSerializer(tag, config);
                break;
            case tag instanceof ByteArrayTag:
                yield 7;
                yield* nameSerializer;
                yield* new ByteArraySerializer(tag, config);
                break;
            case tag instanceof StringTag:
                yield 8;
                yield* nameSerializer;
                yield* new StringSerializer(tag, config);
                break;
            case tag instanceof ListTag:
                yield 9;
                yield* nameSerializer;
                yield* new ListSerializer(tag, config);
                break;
            case tag instanceof CompoundTag:
                yield 10;
                yield* nameSerializer;
                yield* new CompoundSerializer(tag, config);
                break;
            case tag instanceof IntArrayTag:
                yield 11;
                yield* nameSerializer;
                yield* new IntArraySerializer(tag, config);
                break;
            case tag instanceof LongArrayTag:
                yield 12;
                yield* nameSerializer;
                yield* new LongArraySerializer(tag, config);
                break;
            default:
                throw new SerializerError("Unknown tag type.");
        }

        return undefined;
    }
}

/**
@extends {AbstractSerializer<CompoundTag, SerializationConfig>}
*/ export class CompoundSerializer extends AbstractSerializer
{
    /**
    @param {CompoundTag} value
    @param {SerializationConfig} [config]
    @protected @override*/ *generator(value, config)
    {
        for (const entry of value)
            yield* new EntrySerializer(entry, config);

        yield 0;
        return undefined;
    }
}

// MARK: Deserializer
/**
@extends {AbstractPayloadDeserializer<
    [name: string, tag: TagUnion] | [name: "", end: null],
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class EntryOrEndDeserializer extends AbstractPayloadDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @returns {Generator<undefined, [name: string, tag: TagUnion] | [name: "", end: null], number>}
    @protected @override*/ *generator(config)
    {
        const nameDeserializer = new StringPayloadDeserializer(config);
        let valueDeserializer;
        switch (yield)
        {
            case 0:
                return ["", null];
            case 1:
                valueDeserializer = new ByteDeserializer(config);
                break;
            case 2:
                valueDeserializer = new ShortDeserializer(config);
                break;
            case 3:
                valueDeserializer = new IntDeserializer(config);
                break;
            case 4:
                valueDeserializer = new LongDeserializer(config);
                break;
            case 5:
                valueDeserializer = new FloatDeserializer(config);
                break;
            case 6:
                valueDeserializer = new DoubleDeserializer(config);
                break;
            case 7:
                valueDeserializer = new ByteArrayDeserializer(config);
                break;
            case 8:
                valueDeserializer = new StringDeserializer(config);
                break;
            case 9:
                valueDeserializer = new ListDeserializer(config);
                break;
            case 10:
                valueDeserializer = new CompoundDeserializer(config);
                break;
            case 11:
                valueDeserializer = new IntArrayDeserializer(config);
                break;
            case 12:
                valueDeserializer = new LongArrayDeserializer(config);
                break;
            default:
                throw new DeserializerError(
                    "Found invalid type in entry.");
        }

        let name;
        while ((name = nameDeserializer.push(yield)) === null);

        let value;
        while ((value = valueDeserializer.push(yield)) === null);

        return [name, value];
    }
}

/**
@extends {AbstractPayloadDeserializer<
    [name: string, tag: TagUnion],
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class EntryDeserializer extends AbstractPayloadDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @returns {Generator<undefined, [name: string, tag: TagUnion], number>}
    @protected @override*/ *generator(config)
    {
        const nameDeserializer = new StringPayloadDeserializer(config);
        let valueDeserializer;
        switch (yield)
        {
            case 1:
                valueDeserializer = new ByteDeserializer(config);
                break;
            case 2:
                valueDeserializer = new ShortDeserializer(config);
                break;
            case 3:
                valueDeserializer = new IntDeserializer(config);
                break;
            case 4:
                valueDeserializer = new LongDeserializer(config);
                break;
            case 5:
                valueDeserializer = new FloatDeserializer(config);
                break;
            case 6:
                valueDeserializer = new DoubleDeserializer(config);
                break;
            case 7:
                valueDeserializer = new ByteArrayDeserializer(config);
                break;
            case 8:
                valueDeserializer = new StringDeserializer(config);
                break;
            case 9:
                valueDeserializer = new ListDeserializer(config);
                break;
            case 10:
                valueDeserializer = new CompoundDeserializer(config);
                break;
            case 11:
                valueDeserializer = new IntArrayDeserializer(config);
                break;
            case 12:
                valueDeserializer = new LongArrayDeserializer(config);
                break;
            default:
                throw new DeserializerError(
                    "Found invalid type in entry.");
        }

        let name;
        while ((name = nameDeserializer.push(yield)) === null);

        let value;
        while ((value = valueDeserializer.push(yield)) === null);

        return [name, value];
    }
}

/**
@extends {AbstractDeserializer<
    CompoundTag,
    {
        readonly endian?: "little" | "big",
    }>}
*/ export class CompoundDeserializer extends AbstractDeserializer
{
    /**
    @param {DeserializationConfig} [config]
    @protected @override*/ *generator(config)
    {
        const result = new CompoundTag();

        while (true)
        {
            const entryDeserializer = new EntryOrEndDeserializer(config);
            let entry;
            while ((entry = entryDeserializer.push(yield)) === null);

            if (entry[1] === null)
                return result;

            result.set(entry[0], entry[1]);
        }
    }
}