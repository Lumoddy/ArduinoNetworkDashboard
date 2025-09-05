
// Spec according to:
//  - https://github.com/acfoltzer/nbt/blob/master/NBT-spec.txt
//  - https://minecraft.wiki/w/NBT_format

// MARK: Tag
/**
 * The base class for all NBT tags. To get the value of an instance of this
 * class, test what type it is with '`instanceof`' with one of its subclasses.
 * 
 * This is an exhaustive list of types this tag supports is:
 *  - {@linkcode ByteTag Byte} (`Int8`)
 *  - {@linkcode ShortTag Short} (`Int16`)
 *  - {@linkcode IntTag Int} (`Int32`)
 *  - {@linkcode LongTag Long} (`BigInt64`)
 *  - {@linkcode FloatTag Float} (`Float32`)
 *  - {@linkcode DoubleTag Double} (`Float64`)
 *  - {@linkcode ByteArrayTag Byte Array} (`Int8Array`)
 *  - {@linkcode StringTag String} (`UTF-8`)
 *  - {@linkcode ListTag List} (One of the `ListTag` subclasses)
 *  - {@linkcode CompoundTag Compound} (`Map<string, Tag>`)
 *  - {@linkcode IntArrayTag Int Array} (`Int32Array`)
 *  - {@linkcode LongArrayTag Long Array} (`BigInt64Array`)
@abstract*/ export class Tag
{
    /**
    @protected*/ constructor() { this.constructorCheck() }

    /**
    @protected @virtual*/ constructorCheck()
    {
        throw new TypeError("Cannot call constructor of abstract Tag.");
    }

    /**
    @returns {string}
    @public*/ get [Symbol.toStringTag]() { return "NBTTag"; }
}

// MARK: ByteTag
/**
 * A {@linkcode Tag} that stores a byte (`Uint8`).
*/ export class ByteTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = value & 0xFF;
        if ((this._value & 0x80) !== 0)
            this._value -= 0x100;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value)
    {
        this._value = value & 0xFF;
        if ((this._value & 0x80) !== 0)
            this._value -= 0x100;
    }
}

// MARK: ShortTag
/**
 * A {@linkcode Tag} that stores a short (`Uint16`).
*/ export class ShortTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = value & 0xFFFF;
        if ((this._value & 0x8000) !== 0)
            this._value -= 0x10000;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value)
    {
        this._value = value & 0xFFFF;
        if ((this._value & 0x8000) !== 0)
            this._value -= 0x10000;
    }
}

// MARK: IntTag
/**
 * A {@linkcode Tag} that stores an int (`Int32`).
*/ export class IntTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = value & 0xFFFFFFFF;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value & 0xFFFFFFFF }
}

// MARK: LongTag
/**
 * A {@linkcode Tag} that stores a long (`BigInt64`).
*/ export class LongTag extends Tag
{
    /**
    @param {bigint} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {bigint}
        @private*/ this._value = value & 0xFFFFFFFFFFFFFFFFn;
        if ((this._value & 0x8000000000000000n) !== 0n)
            this._value -= 0xFFFFFFFFFFFFFFFFn;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {bigint}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value)
    {
        this._value = value & 0xFFFFFFFFFFFFFFFFn;
        if ((this._value & 0x8000000000000000n) !== 0n)
            this._value -= 0xFFFFFFFFFFFFFFFFn;
    }
}

// MARK: FloatTag
/**
 * A {@linkcode Tag} that stores a float (`Float32`).
*/ export class FloatTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}

// MARK: DoubleTag
/**
 * A {@linkcode Tag} that stores a double (`Double32`).
*/ export class DoubleTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}

// MARK: ByteArrayTag
/**
 * A {@linkcode Tag} that stores a byte array (`Int8Array`).
*/ export class ByteArrayTag extends Tag
{
    /**
    @param {Int8Array} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {Int8Array}
        @private*/ this._value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {Int8Array}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}

// MARK: StringTag
/**
 * A {@linkcode Tag} that stores a string (`UTF-8`).
*/ export class StringTag extends Tag
{
    /**
    @param {string} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {string}
        @private*/ this._value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {string}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}

// MARK: ListTag
/**
 * The base class for the list NBT tags. To get the value of an instance of this
 * class, test what type it is with '`instanceof`' with one of its subclasses.
 * 
 * This is an exhaustive list of types this tag supports is:
 *  - {@linkcode EmptyListTag Empty} (`[]`)
 *  - {@linkcode ByteListTag Byte} (`Int8[]`)
 *  - {@linkcode ShortListTag Short} (`Int16[]`)
 *  - {@linkcode IntListTag Int} (`Int32[]`)
 *  - {@linkcode LongListTag Long} (`BigInt64[]`)
 *  - {@linkcode FloatListTag Float} (`Float32[]`)
 *  - {@linkcode DoubleListTag Double} (`Float64[]`)
 *  - {@linkcode ByteArrayListTag Byte Array} (`Int8Array[]`)
 *  - {@linkcode StringListTag String} (`(UTF-8 string)[]`)
 *  - {@linkcode ListListTag List} (`ListTag[]`)
 *  - {@linkcode CompoundListTag Compound} (`Map<string, Tag>[]`)
 *  - {@linkcode IntArrayListTag Int Array} (`Int32Array[]`)
 *  - {@linkcode LongArrayListTag Long Array} (`BigInt64Array[]`)
@abstract*/ export class ListTag extends Tag
{
    /**
    @protected*/ constructor()
    {
        super();
        this.constructorCheck();
    }

    /**
    @protected @override*/ constructorCheck()
    {
        throw new TypeError("Cannot call constructor of abstract ListTag.");
    }

    /**
    @returns {number}
    @public @abstract*/ get length()
    {
        throw new TypeError("Cannot call abstract length of ListTag.");
    }
    /**
    @public @abstract*/ set length(value)
    {
        throw new TypeError("Cannot call abstract length of ListTag.");
    }
}

// MARK: CompoundTag
/**
 * A {@linkcode Tag} that stores uniquely named tags (`Map<string, Tag>`).
*/ export class CompoundTag extends Tag
{
    /**
    @param {Record<string, Tag> | Map<string, Tag>} [values]
    @public*/ constructor(values)
    {
        super();

        /**
        @type {Map<string, Tag>}
        @private*/ this._value = new Map();

        if (values instanceof Map)
        {
            for (const [key, value] of values)
                this._value.set(key, value);
        }
        else for (const key in values)
            this._value.set(key, values[key]);
    }

    /**
    @protected @override*/ constructorCheck() { }

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
}

// MARK: IntArrayTag
/**
 * A {@linkcode Tag} that stores a byte array (`Int32Array`).
*/ export class IntArrayTag extends Tag
{
    /**
    @param {Int32Array} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {Int32Array}
        @private*/ this._value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {Int32Array}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}

// MARK: LongArrayTag
/**
 * A {@linkcode Tag} that stores a byte array (`BigInt64Array`).
*/ export class LongArrayTag extends Tag
{
    /**
    @param {BigInt64Array} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {BigInt64Array}
        @private*/ this._value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {BigInt64Array}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}

// MARK: EmptyListTag
/**
 * A {@linkcode TagList} that stores bytes (`Uint8[]`).
*/ export class EmptyListTag extends ListTag
{
    /**
    @public*/ constructor()
    {
        super();
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public @override*/ get length() { return 0 }
    /**
    @public @override*/ set length(value) { }
}

// MARK: ByteListTag
/**
 * A {@linkcode TagList} that stores bytes (`Uint8[]`).
*/ export class ByteListTag extends ListTag
{
    /**
    @param {number[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number[]}
        @public*/ this.value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<number>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: ShortListTag
/**
 * A {@linkcode TagList} that stores shorts (`Uint16[]`).
*/ export class ShortListTag extends ListTag
{
    /**
    @param {number[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number[]}
        @public*/ this.value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<number>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: IntListTag
/**
 * A {@linkcode TagList} that stores ints (`Int32[]`).
*/ export class IntListTag extends ListTag
{
    /**
    @param {number[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number[]}
        @public*/ this.value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<number>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: LongListTag
/**
 * A {@linkcode TagList} that stores longs (`BigInt64[]`).
*/ export class LongListTag extends ListTag
{
    /**
    @param {bigint[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {bigint[]}
        @public*/ this.value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<bigint>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: FloatListTag
/**
 * A {@linkcode TagList} that stores floats (`Float32[]`).
*/ export class FloatListTag extends ListTag
{
    /**
    @param {number[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number[]}
        @public*/ this.value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<number>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: DoubleListTag
/**
 * A {@linkcode TagList} that stores floats (`Float64[]`).
*/ export class DoubleListTag extends ListTag
{
    /**
    @param {number[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number[]}
        @public*/ this.value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<number>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: ByteArrayListTag
/**
 * A {@linkcode TagList} that stores floats (`Int8Array[]`).
*/ export class ByteArrayListTag extends ListTag
{
    /**
    @param {Int8Array[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {Int8Array[]}
        @public*/ this.value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<Int8Array>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: StringListTag
/**
 * A {@linkcode TagList} that stores floats (`(UTF-8 string)[]`).
*/ export class StringListTag extends ListTag
{
    /**
    @param {string[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {string[]}
        @public*/ this.value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<string>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: ListListTag
/**
 * A {@linkcode TagList} that stores lists.
*/ export class ListListTag extends ListTag
{
    /**
    @param {ListTag[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {ListTag[]}
        @public*/ this.value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<ListTag>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: CompoundListTag
/**
 * A {@linkcode TagList} that stores compounds (`Map<string, Tag>[]`).
*/ export class CompoundListTag extends ListTag
{
    /**
    @param {CompoundTag[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {CompoundTag[]}
        @public*/ this.value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<CompoundTag>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: IntArrayListTag
/**
 * A {@linkcode TagList} that stores floats (`Int32Array[]`).
*/ export class IntArrayListTag extends ListTag
{
    /**
    @param {Int32Array[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {Int32Array[]}
        @public*/ this.value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<Int32Array>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}

// MARK: LongArrayListTag
/**
 * A {@linkcode TagList} that stores floats (`BigInt64Array[]`).
*/ export class LongArrayListTag extends ListTag
{
    /**
    @param {BigInt64Array[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {BigInt64Array[]}
        @public*/ this.value = value;
    }

    /**
    @protected @override*/ constructorCheck() { }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<BigInt64Array>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}