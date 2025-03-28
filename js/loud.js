/**
@template T
@export @typedef {{
    oldValue: T,
    newValue: T,
}} LoudEvent
*/

/**
@template T
@export @typedef {(event: LoudEvent<T>) => void} LoudListener
*/

/**
@template T
*/ export class Loud
{
    /**
    @param {T} startingValue
    @public*/ constructor(startingValue)
    {
        /**
        @type {T}
        @private*/ this._value = startingValue;
        /**
        @type {LoudListener<T>[]}
        @private*/ this._listeners = [];
    }

    /**
    @returns {Readonly<T>}
    @public*/ get() { return this._value }
    /**
    @param {Readonly<T>} value
    @public*/ set(value)
    {
        const oldValue = this._value;
        this._value = value;

        for (const listener of this._listeners)
        {
            listener({
                oldValue: oldValue,
                newValue: value,
            });
        }
    }

    /**
    @param {LoudListener<T>} listener
    @public*/ addListener(listener)
    {
        this._listeners.push(listener);
    }
    /**
    @param {LoudListener<T>} listener
    @public*/ removeListener(listener)
    {
        const index = this._listeners.indexOf(listener);
        if (index >= 0)
            this._listeners.splice(index, 1);
    }
}