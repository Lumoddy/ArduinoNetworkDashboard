import { ListTag } from "./index.js";

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
    @returns {number}
    @public @override*/ static elementTypeId() { return 4 }

    /**
    @returns {number}
    @public @override*/ elementTypeId() { return 4 }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<bigint>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}