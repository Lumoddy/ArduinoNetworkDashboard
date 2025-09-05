import { Tag } from "./index.js";

/**
 * A {@linkcode Tag} that stores a long (`BigInt64`).
*/ export class LongTag extends Tag
{
    /**
    @param {bigint} value
    @public*/ constructor(value)
    {
        super();

        /**
        @type {bigint}
        @private*/ this._value = value & 0xFFFFFFFFFFFFFFFFn;
        if ((this._value & 0x8000000000000000n) !== 0n)
            this._value -= 0xFFFFFFFFFFFFFFFFn;
    }

    /**
    @returns {number}
    @public @override*/ static typeId() { return 4 }

    /**
    @returns {number}
    @public @override*/ typeId() { return 4 }

    /**
    @returns {bigint}
    @public*/ get value() { return this._value }
    /**
    @public*/ set value(value)
    {
        this._value = value & 0xFFFFFFFFFFFFFFFFn;
        if ((this._value & 0x8000000000000000n) !== 0n)
            this._value -= 0xFFFFFFFFFFFFFFFFn;
    }
}