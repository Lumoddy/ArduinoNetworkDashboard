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
    @public*/ get value() { return this.get() }
    /**
    @public*/ set value(value) { this.set(value) }

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
            listener({
                oldValue: oldValue,
                newValue: value,
            });
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
*/ export class LoudArray
{
    /**
    @param {T[]} startingElements
    @public*/ constructor(...startingElements)
    {
        /**
        @type {T[]}
        @private*/ this._array = startingElements;
        /**
        @type {LoudArrayListener<T>[]}
        @private*/ this._listeners = [];
    }
    
    /**
    @public*/ get value() { return this.get() }
    /**
    @public*/ set value(value) { this.set(value) }

    /**
    @returns {readonly T[]}
    @public*/ get() { return this._array }
    /**
    @param {readonly T[]} elements
    @public*/ set(elements)
    {
        const oldElements = this._array;
        this._array = [...elements];

        for (const listener of this._listeners)
            listener({
                index: 0,
                array: this._array,
                method: oldElements.length === elements.length ? "replace" : "splice",
                oldValues: oldElements,
                newValues: elements,
            });
    }
    
    /**
    @param {number} index
    @returns {T}
    @public*/ getAt(index) { return this._array[index < 0 ? index + this._array.length : index] }
    /**
    @param {number} index
    @param {T} value
    @public*/ setAt(index, value)
    {
        if (index < 0)
            index += this._array.length;

        if (index < 0)
            return;

        if (index >= this._array.length)
        {
            this._array[index] = value;
    
            for (const listener of this._listeners)
                listener({
                    index: index,
                    array: this._array,
                    method: "insert",
                    oldValues: null,
                    newValues: [value],
                });
        }
        else
        {
            const oldValue = this._array[index]; 
            this._array[index] = value;
    
            for (const listener of this._listeners)
                listener({
                    index: index,
                    array: this._array,
                    method: "replace",
                    oldValues: [oldValue],
                    newValues: [value],
                });
        }
    }

    /**
    @param {number} index
    @returns {T}
    @public*/ at(index) { return this.getAt(index) }

    /**
    @type {number}
    @public*/ get length() { return this._array.length }

    /**
    @returns {T | undefined}
    @public*/ pop()
    {
        if (this._array.length <= 0)
            return undefined;

        const oldValue = /** @type {T} */(this._array.pop());

        for (const listener of this._listeners)
            listener({
                index: this._array.length,
                array: this._array,
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
        const insertIndex = this._array.length;
        const result = this._array.push(...items);

        for (const listener of this._listeners)
            listener({
                index: insertIndex,
                array: this._array,
                method: "insert",
                oldValues: null,
                newValues: items,
            });

        return result;
    }

    /**
    @param {(T | ConcatArray<T>)[]} items
    @returns {T[]}
    @public*/ concat(...items) { return this._array.concat(...items) }

    /**
    @param {string} [separator]
    @returns {string}
    @public*/ join(separator) { return this._array.join(separator) }

    /**
    @returns {T[]}
    @public*/ reverse()
    {
        const oldArray = this._array;
        this._array = [...this._array].reverse();

        for (const listener of this._listeners)
            listener({
                index: 0,
                array: this._array,
                method: "replace",
                oldValues: oldArray,
                newValues: this._array,
            });

        return this._array;
    }

    /**
    @returns {T | undefined}
    @public*/ shift()
    {
        if (this._array.length <= 0)
            return undefined;

        const oldValue = /** @type {T} */(this._array.shift());

        for (const listener of this._listeners)
            listener({
                index: 0,
                array: this._array,
                method: "remove",
                oldValues: [oldValue],
                newValues: null,
            });

        return oldValue;
    }

    /**
    @param {number} start 
    @param {number} end 
    @returns {T[]}
    @public*/ slice(start, end) { return this._array.slice(start, end) }

    /**
    @param {(a: T, b: T) => number} [compareFn]
    @returns {this}
    @public*/ sort(compareFn)
    {
        const oldArray = this._array;
        this._array = [...this._array].sort(compareFn);

        for (const listener of this._listeners)
            listener({
                index: 0,
                array: this._array,
                method: "replace",
                oldValues: oldArray,
                newValues: this._array,
            });

        return this;
    }

    /**
    @param {number} start
    @param {number} deleteCount
    @param {T[]} rest
    @returns {T[]}
    @public*/ splice(start, deleteCount = this._array.length - start, ...rest)
    {
        if (start < 0)
            start += this._array.length;

        if (start < 0)
            start = 0;
        else if (start > this._array.length)
            start = this._array.length;

        const removedElements = this._array.splice(start, deleteCount, ...rest);
        
        for (const listener of this._listeners)
            listener({
                index: start,
                array: this._array,
                method: "splice",
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
        const result = this._array.unshift(...items);

        for (const listener of this._listeners)
            listener({
                index: 0,
                array: this._array,
                method: "insert",
                oldValues: null,
                newValues: items,
            });

        return result;
    }

    /**
    @returns {ArrayIterator<T>}
    @public*/ [Symbol.iterator]() { return this._array[Symbol.iterator]() }

    /**
    @param {LoudArrayListener<T>} listener
    @public*/ addListener(listener)
    {
        this._listeners.push(listener);
    }
    /**
    @param {LoudArrayListener<T>} listener
    @public*/ removeListener(listener)
    {
        const index = this._listeners.indexOf(listener);
        if (index >= 0)
            this._listeners.splice(index, 1);
    }
}