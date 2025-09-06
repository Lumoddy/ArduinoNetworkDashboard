import { Tag } from "./base.js";

/**
 * A {@linkcode Tag} that stores a byte (`Uint8`).
*/ export class ByteTag extends Tag
{
    /**
    @param {number} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {number}
        @private*/ this._value = value & 0xFF;
        if ((this._value & 0x80) !== 0)
            this._value -= 0x100;
    }

    /**
    @returns {number}
    @public @override*/ static typeId() { return 1 }

    /**
    @returns {number}
    @public @override*/ typeId() { return 1 }

    /**
    @returns {number}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value)
    {
        this._value = value & 0xFF;
        if ((this._value & 0x80) !== 0)
            this._value -= 0x100;
    }
}