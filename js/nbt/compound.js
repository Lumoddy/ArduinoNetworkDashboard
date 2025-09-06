import { Tag } from "./base.js";

/**
 * A {@linkcode Tag} that stores uniquely named tags (`Map<string, Tag>`).
*/ export class CompoundTag extends Tag
{
    /**
    @param {Record<string, Tag> | Map<string, Tag>} [values]
    @public*/ constructor(values)
    {
        super();

        /**
        @type {Map<string, Tag>}
        @private*/ this._value = new Map();

        if (values instanceof Map)
        {
            for (const [key, value] of values)
                this._value.set(key, value);
        }
        else for (const key in values)
            this._value.set(key, values[key]);
    }

    /**
    @returns {number}
    @public @override*/ static typeId() { return 10 }

    /**
    @returns {number}
    @public @override*/ typeId() { return 10 }

    /**
    @public*/ clear() { return this._value.clear() }

    /**
    @param {string} key
    @returns {boolean}
    @public*/ delete(key) { return this._value.delete(key) }

    /**
    @param {(value: Tag, key: string, map: Map<string, Tag>) => void} callbackfn
    @param {any} [thisArg]
    @public*/ forEach(callbackfn, thisArg)
    {
        return this._value.forEach(callbackfn, thisArg);
    }

    /**
    @param {string} key
    @returns {Tag | undefined}
    @public*/ get(key) { return this._value.get(key) }

    /**
    @param {string} key
    @returns {boolean}
    @public*/ has(key) { return this._value.has(key) }

    /**
    @param {string} key
    @param {Tag} value
    @public*/ set(key, value) { return this._value.set(key, value) }

    /**
    @returns {number}
    @public @readonly*/ get size() { return this._value.size }

    /**
    @returns {MapIterator<[string, Tag]>}
    @public*/ [Symbol.iterator]() { return this._value[Symbol.iterator]() }

    /**
    @returns {MapIterator<[string, Tag]>}
    @public*/ entries() { return this._value.entries() }

    /**
    @returns {MapIterator<string>}
    @public*/ keys() { return this._value.keys() }

    /**
    @returns {MapIterator<Tag>}
    @public*/ values() { return this._value.values() }
}