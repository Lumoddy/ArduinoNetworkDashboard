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
*/ export class ReadonlyLoud
{
    /**
    @param {T} startingValue
    @protected*/ constructor(startingValue)
    {
        /**
        @type {T}
        @protected*/ this.storedValue = startingValue;
        /**
        @type {LoudListener<T>[]}
        @protected*/ this.listeners = [];
    }

    /**
    @public*/ get value() { return this.get() }

    /**
    @returns {Readonly<T>}
    @public*/ get() { return this.storedValue }

    /**
    @param {LoudListener<T>} listener
    @public*/ addListener(listener)
    {
        this.listeners.push(listener);
    }
    /**
    @param {LoudListener<T>} listener
    @public*/ removeListener(listener)
    {
        const index = this.listeners.indexOf(listener);
        if (index >= 0)
            this.listeners.splice(index, 1);
    }
}

/**
@extends {ReadonlyLoud<T>}
@template T
*/ export class Loud extends ReadonlyLoud
{
    /**
    @param {T} startingValue
    @public*/ constructor(startingValue)
    {
        super(startingValue);
    }

    /**
    @public @override*/ get value() { return this.get() }
    /**
    @public @override*/ set value(value) { this.set(value) }

    /**
    @param {Readonly<T>} value
    @public*/ set(value)
    {
        const oldValue = this.storedValue;
        this.storedValue = value;

        for (const listener of this.listeners)
            listener({
                oldValue: oldValue,
                newValue: value,
            });
    }
}

/**
@template T
@export @typedef {{
    index: number,
    array: readonly T[],
} & ({
    method: "insert",
    oldValues: null,
    newValues: readonly T[],
} | {
    method: "remove",
    oldValues: readonly T[],
    newValues: null,
} | {
    method: "splice" | "replace",
    oldValues: readonly T[],
    newValues: readonly T[],
})} LoudArrayEvent
*/

/**
@template T
@export @typedef {(event: LoudArrayEvent<T>) => void} LoudArrayListener
*/

/**
@template T
*/ export class ReadonlyLoudArray
{
    /**
    @param {T[]} startingElements
    @protected*/ constructor(...startingElements)
    {
        /**
        @type {T[]}
        @protected*/ this.storedArray = startingElements;
        /**
        @type {LoudArrayListener<T>[]}
        @protected*/ this.listeners = [];
    }

    /**
    @public*/ get value() { return this.get() }

    /**
    @returns {readonly T[]}
    @public*/ get() { return this.storedArray }

    /**
    @param {number} index
    @returns {T}
    @public*/ getAt(index) { return this.storedArray[index < 0 ? index + this.storedArray.length : index] }

    /**
    @param {number} index
    @returns {T}
    @public*/ at(index) { return this.getAt(index) }

    /**
    @type {number}
    @public*/ get length() { return this.storedArray.length }

    /**
    @param {(T | ConcatArray<T>)[]} items
    @returns {T[]}
    @public*/ concat(...items) { return this.storedArray.concat(...items) }

    /**
    @param {string} [separator]
    @returns {string}
    @public*/ join(separator) { return this.storedArray.join(separator) }

    /**
    @param {number} start 
    @param {number} end 
    @returns {T[]}
    @public*/ slice(start, end) { return this.storedArray.slice(start, end) }

    /**
    @returns {ArrayIterator<T>}
    @public*/ [Symbol.iterator]() { return this.storedArray[Symbol.iterator]() }

    /**
    @param {LoudArrayListener<T>} listener
    @public*/ addListener(listener)
    {
        this.listeners.push(listener);
    }
    /**
    @param {LoudArrayListener<T>} listener
    @public*/ removeListener(listener)
    {
        const index = this.listeners.indexOf(listener);
        if (index >= 0)
            this.listeners.splice(index, 1);
    }
}

/**
@extends {ReadonlyLoudArray<T>}
@template T
*/ export class LoudArray extends ReadonlyLoudArray
{
    /**
    @param {T[]} startingElements
    @public*/ constructor(...startingElements)
    {
        super(...startingElements);
    }

    /**
    @public @override*/ get value() { return this.get() }
    /**
    @public @override*/ set value(value) { this.set(value) }

    /**
    @param {readonly T[]} elements
    @public*/ set(elements)
    {
        const oldElements = this.storedArray;
        this.storedArray = [...elements];

        for (const listener of this.listeners)
            listener({
                index: 0,
                array: this.storedArray,
                method: oldElements.length === elements.length ? "replace" : "splice",
                oldValues: oldElements,
                newValues: elements,
            });
    }

    /**
    @param {number} index
    @param {T} value
    @public*/ setAt(index, value)
    {
        if (index < 0)
            index += this.storedArray.length;

        if (index < 0)
            return;

        if (index >= this.storedArray.length)
        {
            this.storedArray[index] = value;

            for (const listener of this.listeners)
                listener({
                    index: index,
                    array: this.storedArray,
                    method: "insert",
                    oldValues: null,
                    newValues: [value],
                });
        }
        else
        {
            const oldValue = this.storedArray[index]; 
            this.storedArray[index] = value;

            for (const listener of this.listeners)
                listener({
                    index: index,
                    array: this.storedArray,
                    method: "replace",
                    oldValues: [oldValue],
                    newValues: [value],
                });
        }
    }

    /**
    @returns {T | undefined}
    @public*/ pop()
    {
        if (this.storedArray.length <= 0)
            return undefined;

        const oldValue = /** @type {T} */(this.storedArray.pop());

        for (const listener of this.listeners)
            listener({
                index: this.storedArray.length,
                array: this.storedArray,
                method: "remove",
                oldValues: [oldValue],
                newValues: null,
            });

        return oldValue;
    }

    /**
    @param {T[]} items
    @returns {number}
    @public*/ push(...items)
    {
        const insertIndex = this.storedArray.length;
        const result = this.storedArray.push(...items);

        for (const listener of this.listeners)
            listener({
                index: insertIndex,
                array: this.storedArray,
                method: "insert",
                oldValues: null,
                newValues: items,
            });

        return result;
    }

    /**
    @returns {T[]}
    @public*/ reverse()
    {
        const oldArray = this.storedArray;
        this.storedArray = [...this.storedArray].reverse();

        for (const listener of this.listeners)
            listener({
                index: 0,
                array: this.storedArray,
                method: "replace",
                oldValues: oldArray,
                newValues: this.storedArray,
            });

        return this.storedArray;
    }

    /**
    @returns {T | undefined}
    @public*/ shift()
    {
        if (this.storedArray.length <= 0)
            return undefined;

        const oldValue = /** @type {T} */(this.storedArray.shift());

        for (const listener of this.listeners)
            listener({
                index: 0,
                array: this.storedArray,
                method: "remove",
                oldValues: [oldValue],
                newValues: null,
            });

        return oldValue;
    }

    /**
    @param {(a: T, b: T) => number} [compareFn]
    @returns {this}
    @public*/ sort(compareFn)
    {
        const oldArray = this.storedArray;
        this.storedArray = [...this.storedArray].sort(compareFn);

        for (const listener of this.listeners)
            listener({
                index: 0,
                array: this.storedArray,
                method: "replace",
                oldValues: oldArray,
                newValues: this.storedArray,
            });

        return this;
    }

    /**
    @param {number} start
    @param {number} deleteCount
    @param {T[]} rest
    @returns {T[]}
    @public*/ splice(start, deleteCount = this.storedArray.length - start, ...rest)
    {
        if (start < 0)
            start += this.storedArray.length;

        if (start < 0)
            start = 0;
        else if (start > this.storedArray.length)
            start = this.storedArray.length;

        const removedElements = this.storedArray.splice(start, deleteCount, ...rest);

        for (const listener of this.listeners)
            listener({
                index: start,
                array: this.storedArray,
                method: removedElements.length === rest.length ? "replace" : "splice",
                oldValues: removedElements,
                newValues: rest,
            });

        return removedElements;
    }

    /**
    @param {T[]} items
    @returns {number}
    @public*/ unshift(...items)
    {
        const result = this.storedArray.unshift(...items);

        for (const listener of this.listeners)
            listener({
                index: 0,
                array: this.storedArray,
                method: "insert",
                oldValues: null,
                newValues: items,
            });

        return result;
    }
}