/**
@import { ByteTag } from "./byte.js"
@import { ShortTag } from "./short.js"
@import { IntTag } from "./int.js"
@import { LongTag } from "./long.js"
@import { FloatTag } from "./float.js"
@import { DoubleTag } from "./double.js"
@import { ByteArrayTag } from "./byte-array.js"
@import { StringTag } from "./string.js"
@import { ListTag, ListTagUnion } from "./list.js"
@import { CompoundTag } from "./compound.js"
@import { IntArrayTag } from "./int-array.js"
@import { LongArrayTag } from "./long-array.js"
*/

// MARK: Tag
/**
 * The base class for all NBT tags. To get the value of an instance of this
 * class, test what type it is with '`instanceof`' with one of its subclasses.
 * 
 * This is an exhaustive list of types this tag supports:
 *  - {@linkcode ByteTag Byte} (`Int8`)
 *  - {@linkcode ShortTag Short} (`Int16`)
 *  - {@linkcode IntTag Int} (`Int32`)
 *  - {@linkcode LongTag Long} (`BigInt64`)
 *  - {@linkcode FloatTag Float} (`Float32`)
 *  - {@linkcode DoubleTag Double} (`Float64`)
 *  - {@linkcode ByteArrayTag Byte Array} (`Int8Array`)
 *  - {@linkcode StringTag String} (`UTF-8 string`)
 *  - {@linkcode ListTag List} (One of the `ListTag` subclasses)
 *  - {@linkcode CompoundTag Compound} (`Map<string, Tag>`)
 *  - {@linkcode IntArrayTag Int Array} (`Int32Array`)
 *  - {@linkcode LongArrayTag Long Array} (`BigInt64Array`)
@abstract*/ export class Tag
{
    /**
    @protected*/ constructor() { }

    /**
    @returns {string}
    @public*/ get [Symbol.toStringTag]() { return "NBTTag"; }

    /**
    @returns {unknown}
    @public @virtual*/ toJSON() { return undefined }

    /**
    @returns {this}
    @public*/ toNBT() { return this }
}

/**
@export @typedef {
    | ByteTag
    | ShortTag
    | IntTag
    | LongTag
    | FloatTag
    | DoubleTag
    | ByteArrayTag
    | StringTag
    | ListTagUnion
    | CompoundTag
    | IntArrayTag
    | LongArrayTag
} TagUnion
*/

/**
*/ export class SerializerError extends Error { }

/**
*/ export class DeserializerError extends Error { }

/**
*/ export const MAX_LIST_LENGTH = 1000;

/**
*/ export const MAX_STRING_LENGTH = 1000;

/**
@export @typedef {
{
    next(): IteratorResult<number, undefined>
    return?(value?: undefined): IteratorResult<number, undefined>,
    throw?(e?: any): IteratorResult<number, undefined>,
    [Symbol.iterator](): SerializationGenerator,
}
} SerializationGenerator
*/

/**
@template T
@export @typedef {
{
    next(...args: [] | [byte: number]): IteratorResult<undefined, T>,
    return?(value?: T): IteratorResult<undefined, T>,
    throw?(e?: any): IteratorResult<undefined, T>,
}
} DeserializationGenerator
*/

/**
@export @typedef {
{
    readonly endian?: "little" | "big",
}
} SerializationConfig
*/

/**
@export @typedef {
{
    readonly endian?: "little" | "big",
}
} DeserializationConfig
*/

// MARK: PayloadSerializer
/**
@abstract*/ export class PayloadSerializer
{
    /**
    @protected*/ constructor() { }

    /**
    @returns {number?}
    @public @abstract*/ pop()
    {
        throw new TypeError("Cannot call abstract function.");
    }

    /**
    @param {never} value
    @public @abstract*/ reset(value)
    {
        throw new TypeError("Cannot call abstract function.");
    }

    /**
    @returns {Iterator<number, unknown, undefined>}
    @public @abstract*/ [Symbol.iterator]()
    {
        throw new TypeError("Cannot call abstract function.");
    }
}

/**
@template {{}} T
@template {{}} [C = {}]
@abstract*/ export class AbstractPayloadSerializer extends PayloadSerializer
{
    /**
    @param {T} value
    @param {C} [config]
    @public*/ constructor(value, config)
    {
        super();

        /**
        @type {C | undefined}
        @private*/ this._config = config;

        /**
        @type {SerializationGenerator}
        @private*/ this._iterator = this.generator(value, this._config);
    }

    /**
    @returns {number?}
    @public @override*/ pop() { return this.next().value ?? null }

    /**
    @param {T} value
    @public @override*/ reset(value) { this._iterator = this.generator(value, this._config) }

    /**
    @param {T} value
    @param {C} [config]
    @returns {SerializationGenerator}
    @protected @abstract*/ generator(value, config)
    {
        throw new TypeError("Cannot call abstract function.");
    }

    /**
    @returns {IteratorResult<number, undefined>}
    @public*/ next() { return this._iterator.next() }

    /**
    @returns {SerializationGenerator}
    @public @override*/ [Symbol.iterator]() { return this }
}

// MARK: Serializer
/**
@abstract*/ export class Serializer
{
    /**
    @protected*/ constructor() { }

    /**
    @returns {number?}
    @public @abstract*/ pop()
    {
        throw new TypeError("Cannot call abstract function.");
    }

    /**
    @param {never} tag
    @public @abstract*/ reset(tag)
    {
        throw new TypeError("Cannot call abstract function.");
    }

    /**
    @returns {Iterator<number, unknown, undefined>}
    @public @abstract*/ [Symbol.iterator]()
    {
        throw new TypeError("Cannot call abstract function.");
    }
}

/**
@template {Tag} T
@template {{}} [C = {}]
@abstract*/ export class AbstractSerializer extends Serializer
{
    /**
    @param {T} value
    @param {C} [config]
    @public*/ constructor(value, config)
    {
        super();

        /**
        @type {C | undefined}
        @private*/ this._config = config;

        /**
        @type {SerializationGenerator}
        @private*/ this._iterator = this.generator(value, this._config);
    }

    /**
    @returns {number?}
    @public @override*/ pop() { return this.next().value ?? null }

    /**
    @param {T} value
    @public @override*/ reset(value) { this._iterator = this.generator(value, this._config) }

    /**
    @param {T} value
    @param {C} [config]
    @returns {SerializationGenerator}
    @protected @abstract*/ generator(value, config)
    {
        throw new TypeError("Cannot call abstract function.");
    }

    /**
    @returns {IteratorResult<number, undefined>}
    @public*/ next() { return this._iterator.next() }

    /**
    @returns {SerializationGenerator}
    @public @override*/ [Symbol.iterator]() { return this }
}

// MARK: PayloadDeserializer
/**
@abstract*/ export class PayloadDeserializer
{
    /**
    @protected*/ constructor() { }

    /**
    @param {number} byte
    @returns {{}?}
    @public @abstract*/ push(byte)
    {
        throw new TypeError("Cannot call abstract function.");
    }

    /**
    @public @abstract*/ reset()
    {
        throw new TypeError("Cannot call abstract function.");
    }
}

/**
@template {{}} T
@template {{}} [C = {}]
@abstract*/ export class AbstractPayloadDeserializer extends PayloadDeserializer
{
    /**
    @param {C} [config]
    @public*/ constructor(config)
    {
        super();

        /**
        @type {C | undefined}
        @private*/ this._config = config;

        /**
        @type {DeserializationGenerator<T>}
        @private*/ this._iterator = this.generator(this._config);
        this._iterator.next();
    }

    /**
    @param {number} byte
    @returns {T?}
    @public @override*/ push(byte)
    {
        const { done, value } = this._iterator.next(byte);

        if (done && value === undefined)
            throw new DeserializerError(
                "Cannot push to deserializer after its completion.");

        return value ?? null;
    }

    /**
    @public @override*/ reset()
    {
        this._iterator = this.generator(this._config);
        this._iterator.next();
    }

    /**
    @param {C} [config]
    @returns {DeserializationGenerator<T>}
    @protected @abstract*/ generator(config)
    {
        throw new TypeError("Cannot call abstract function.");
    }
}

// MARK: Deserializer
/**
@abstract*/ export class Deserializer
{
    /**
    @protected*/ constructor() { }

    /**
    @param {number} byte
    @returns {Tag?}
    @public @abstract*/ push(byte)
    {
        throw new TypeError("Cannot call abstract function.");
    }

    /**
    @public @abstract*/ reset()
    {
        throw new TypeError("Cannot call abstract function.");
    }
}

/**
@template {Tag} T
@template {{}} [C = {}]
@abstract*/ export class AbstractDeserializer extends Deserializer
{
    /**
    @param {C} [config]
    @public*/ constructor(config)
    {
        super();

        /**
        @type {C | undefined}
        @private*/ this._config = config;

        /**
        @type {DeserializationGenerator<T>}
        @private*/ this._iterator = this.generator(this._config);
        this._iterator.next();
    }

    /**
    @param {number} byte
    @returns {T?}
    @public @override*/ push(byte)
    {
        const { done, value } = this._iterator.next(byte);

        if (done && value === undefined)
            throw new DeserializerError(
                "Cannot push to deserializer after its completion.");

        return value ?? null;
    }

    /**
    @public @override*/ reset()
    {
        this._iterator = this.generator(this._config);
        this._iterator.next();
    }

    /**
    @param {C} [config]
    @returns {DeserializationGenerator<T>}
    @protected @abstract*/ generator(config)
    {
        throw new TypeError("Cannot call abstract function.");
    }
}