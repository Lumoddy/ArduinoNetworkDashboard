import { Tag } from "./base.js";

/**
 * A {@linkcode Tag} that stores a short (`Uint16`).
*/ export class ShortTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = value & 0xFFFF;
        if ((this._value & 0x8000) !== 0)
            this._value -= 0x10000;
    }

    /**
    @returns {number}
    @public @override*/ static typeId() { return 2 }

    /**
    @returns {number}
    @public @override*/ typeId() { return 2 }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value)
    {
        this._value = value & 0xFFFF;
        if ((this._value & 0x8000) !== 0)
            this._value -= 0x10000;
    }
}