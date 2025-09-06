/**
@import { ByteTag } from "./byte.js"
@import { ShortTag } from "./short.js"
@import { IntTag } from "./int.js"
@import { LongTag } from "./long.js"
@import { FloatTag } from "./float.js"
@import { DoubleTag } from "./double.js"
@import { ByteArrayTag } from "./byte-array.js"
@import { StringTag } from "./string.js"
@import { ListTag } from "./list.js"
@import { CompoundTag } from "./compound.js"
@import { IntArrayTag } from "./int-array.js"
@import { LongArrayTag } from "./long-array.js"
*/

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
    @returns {number}
    @protected @abstract*/ static typeId()
    {
        throw new TypeError("Cannot call abstract function.");
    }

    /**
    @returns {number}
    @protected @abstract*/ typeId()
    {
        throw new TypeError("Cannot call abstract function.");
    }

    /**
    @returns {string}
    @public*/ get [Symbol.toStringTag]() { return "NBTTag"; }
}