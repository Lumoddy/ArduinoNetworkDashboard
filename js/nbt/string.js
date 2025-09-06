import { Tag } from "./base.js";

/**
 * A {@linkcode Tag} that stores a string (`UTF-8`).
*/ export class StringTag extends Tag
{
    /**
    @param {string} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {string}
        @private*/ this._value = value;
    }

    /**
    @returns {number}
    @public @override*/ static typeId() { return 8 }

    /**
    @returns {number}
    @public @override*/ typeId() { return 8 }

    /**
    @returns {string}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}