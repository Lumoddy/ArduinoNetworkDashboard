import { ListTag } from "./base.js";

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
    @returns {number}
    @public @override*/ static elementTypeId() { return 9 }

    /**
    @returns {number}
    @public @override*/ elementTypeId() { return 9 }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<ListTag>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}