import { ListTag } from "./index.js";

/**
 * A {@linkcode TagList} that stores floats (`Int8Array[]`).
*/ export class ByteArrayListTag extends ListTag
{
    /**
    @param {Int8Array<ArrayBuffer>[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {Int8Array<ArrayBuffer>[]}
        @public*/ this.value = value;
    }

    /**
    @returns {number}
    @public @override*/ static elementTypeId() { return 7 }

    /**
    @returns {number}
    @public @override*/ elementTypeId() { return 7 }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<Int8Array>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}