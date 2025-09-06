import { Tag } from "./base.js";

/**
 * A {@linkcode Tag} that stores a double (`Double32`).
*/ export class DoubleTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = value;
    }

    /**
    @returns {number}
    @public @override*/ static typeId() { return 6 }

    /**
    @returns {number}
    @public @override*/ typeId() { return 6 }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}