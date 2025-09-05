import { ListTag } from "./index.js";

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
    @returns {number}
    @public @override*/ static elementTypeId() { return 1 }

    /**
    @returns {number}
    @public @override*/ elementTypeId() { return 1 }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<number>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}