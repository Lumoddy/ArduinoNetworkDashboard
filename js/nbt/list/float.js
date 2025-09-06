import { ListTag } from "./base.js";

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
    @returns {number}
    @public @override*/ static elementTypeId() { return 5 }

    /**
    @returns {number}
    @public @override*/ elementTypeId() { return 5 }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<number>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}