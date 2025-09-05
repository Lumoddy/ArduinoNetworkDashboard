import { ListTag } from "./index.js";

/**
 * A {@linkcode TagList} that stores floats (`BigInt64Array[]`).
*/ export class LongArrayListTag extends ListTag
{
    /**
    @param {BigInt64Array<ArrayBuffer>[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {BigInt64Array<ArrayBuffer>[]}
        @public*/ this.value = value;
    }

    /**
    @returns {number}
    @public @override*/ static elementTypeId() { return 12 }

    /**
    @returns {number}
    @public @override*/ elementTypeId() { return 12 }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<BigInt64Array<ArrayBuffer>>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}