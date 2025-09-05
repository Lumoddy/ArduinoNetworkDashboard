import { ListTag } from "./index.js";

/**
 * A {@linkcode TagList} that stores floats (`Int32Array[]`).
*/ export class IntArrayListTag extends ListTag
{
    /**
    @param {Int32Array<ArrayBuffer>[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {Int32Array<ArrayBuffer>[]}
        @public*/ this.value = value;
    }

    /**
    @returns {number}
    @public @override*/ static elementTypeId() { return 11 }

    /**
    @returns {number}
    @public @override*/ elementTypeId() { return 11 }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<Int32Array<ArrayBuffer>>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}