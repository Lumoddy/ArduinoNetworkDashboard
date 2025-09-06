import { ListTag } from "./base.js";

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
    @returns {number}
    @public @override*/ static elementTypeId() { return 8 }

    /**
    @returns {number}
    @public @override*/ elementTypeId() { return 8 }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<string>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}