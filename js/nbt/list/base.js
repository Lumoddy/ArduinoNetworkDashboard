import { Tag } from "../base.js";
/**
@import { EmptyListTag } from "./empty.js"
@import { ByteListTag } from "./byte.js"
@import { ShortListTag } from "./short.js"
@import { IntListTag } from "./int.js"
@import { LongListTag } from "./long.js"
@import { FloatListTag } from "./float.js"
@import { DoubleListTag } from "./double.js"
@import { ByteArrayListTag } from "./byte-array.js"
@import { StringListTag } from "./string.js"
@import { ListListTag } from "./list.js"
@import { CompoundListTag } from "./compound.js"
@import { IntArrayListTag } from "./int-array.js"
@import { LongArrayListTag } from "./long-array.js"
*/

/**
 * The base class for the list NBT tags. To get the value of an instance of this
 * class, test what type it is with '`instanceof`' with one of its subclasses.
 * 
 * This is an exhaustive list of types this tag supports:
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
    @protected*/ constructor() { super() }

    /**
    @returns {number}
    @public @override*/ static typeId() { return 9 }

    /**
    @returns {number}
    @public @override*/ typeId() { return 9 }

    /**
    @returns {number}
    @public @virtual*/ static elementTypeId()
    {
        throw new TypeError("Cannot call abstract function.");
    }

    /**
    @returns {number}
    @public @virtual*/ elementTypeId()
    {
        throw new TypeError("Cannot call abstract function.");
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