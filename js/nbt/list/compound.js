import { CompoundTag } from "../compound.js";
import { ListTag } from "./base.js";

/**
 * A {@linkcode TagList} that stores compounds (`Map<string, Tag>[]`).
*/ export class CompoundListTag extends ListTag
{
    /**
    @param {CompoundTag[]} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {CompoundTag[]}
        @public*/ this.value = value;
    }

    /**
    @returns {number}
    @public @override*/ static elementTypeId() { return 10 }

    /**
    @returns {number}
    @public @override*/ elementTypeId() { return 10 }

    /**
    @returns {number}
    @public @override*/ get length() { return this.value.length }
    /**
    @public @override*/ set length(value) { this.value.length = value }

    /**
    @returns {ArrayIterator<CompoundTag>}
    @public*/ [Symbol.iterator]() { return this.value[Symbol.iterator]() }
}