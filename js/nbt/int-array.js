import { Tag } from "./index.js";

/**
 * A {@linkcode Tag} that stores a byte array (`Int32Array`).
*/ export class IntArrayTag extends Tag
{
    /**
    @param {Int32Array<ArrayBuffer>} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {Int32Array<ArrayBuffer>}
        @private*/ this._value = value;
    }

    /**
    @returns {number}
    @public @override*/ static typeId() { return 11 }

    /**
    @returns {number}
    @public @override*/ typeId() { return 11 }

    /**
    @returns {Int32Array<ArrayBuffer>}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}