import { Tag } from "./index.js";

/**
 * A {@linkcode Tag} that stores a byte array (`Int8Array`).
*/ export class ByteArrayTag extends Tag
{
    /**
    @param {Int8Array<ArrayBuffer>} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {Int8Array<ArrayBuffer>}
        @private*/ this._value = value;
    }

    /**
    @returns {number}
    @public @override*/ static typeId() { return 7 }

    /**
    @returns {number}
    @public @override*/ typeId() { return 7 }

    /**
    @returns {Int8Array<ArrayBuffer>}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}