import { Tag } from "./base.js";

/**
 * A {@linkcode Tag} that stores a byte array (`BigInt64Array`).
*/ export class LongArrayTag extends Tag
{
    /**
    @param {BigInt64Array<ArrayBuffer>} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {BigInt64Array<ArrayBuffer>}
        @private*/ this._value = value;
    }

    /**
    @returns {number}
    @public @override*/ static typeId() { return 12 }

    /**
    @returns {number}
    @public @override*/ typeId() { return 12 }

    /**
    @returns {BigInt64Array<ArrayBuffer>}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}