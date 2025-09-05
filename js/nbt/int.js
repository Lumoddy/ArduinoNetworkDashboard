import { Tag } from "./index.js";

/**
 * A {@linkcode Tag} that stores an int (`Int32`).
*/ export class IntTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = value & 0xFFFFFFFF;
    }

    /**
    @returns {number}
    @public @override*/ static typeId() { return 3 }

    /**
    @returns {number}
    @public @override*/ typeId() { return 3 }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value & 0xFFFFFFFF }
}