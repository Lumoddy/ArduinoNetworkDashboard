import { Tag } from "./index.js";

/**
 * A {@linkcode Tag} that stores a float (`Float32`).
*/ export class FloatTag extends Tag
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
    @public @override*/ static typeId() { return 5 }

    /**
    @returns {number}
    @public @override*/ typeId() { return 5 }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value) { this._value = value }
}