
/**
@template T
@param {
(
    tracer:
    {
        readonly boolean: (value: boolean) => Iterable<number, void, undefined>,
        readonly int8: (value: number) => Iterable<number, void, undefined>,
        readonly uint8: (value: number) => Iterable<number, void, undefined>,
        readonly int16: (value: number) => Iterable<number, void, undefined>,
        readonly uint16: (value: number) => Iterable<number, void, undefined>,
        readonly int32: (value: number) => Iterable<number, void, undefined>,
        readonly uint32: (value: number) => Iterable<number, void, undefined>,
        readonly bigInt64: (value: bigint) => Iterable<number, void, undefined>,
        readonly bigUint64: (value: bigint) => Iterable<number, void, undefined>,
        readonly next: () => Iterable<number, void, undefined>,
        readonly end: () => Iterable<number, void, undefined>,
        readonly value: <T>(
            value: T,
            via: (value: T) => Generator<number, void, undefined>)
            => Iterable<number, void, undefined>,
    },
    value: T,
) => Generator<number, void, undefined>
} generator
@returns {(value: T) => Generator<number, void, undefined>}
*/ export function serializerVia(generator)
{
    return function(value)
    {
        const dataView = new DataView(new ArrayBuffer(8));
        const bytes1 = new Uint8Array(1);
        const bytes2 = new Uint8Array(2);
        const bytes4 = new Uint8Array(4);
        const bytes8 = new Uint8Array(8);

        return generator(
            {
                boolean(value)
                {
                    dataView.setInt8(0, value ? 1 : 0);
                    bytes1[0] = dataView.getUint8(0);
                    return bytes1;
                },
                int8(value)
                {
                    dataView.setInt8(0, value);
                    bytes1[0] = dataView.getUint8(0);
                    return bytes1;
                },
                uint8(value)
                {
                    dataView.setUint8(0, value);
                    bytes1[0] = dataView.getUint8(0);
                    return bytes1;
                },
                int16(value)
                {
                    dataView.setInt16(0, value, true);
                    for (let i = 0; i < 2; i++)
                        bytes2[i] = dataView.getUint8(i);
                    return bytes2;
                },
                uint16(value)
                {
                    dataView.setUint16(0, value, true);
                    for (let i = 0; i < 2; i++)
                        bytes2[i] = dataView.getUint8(i);
                    return bytes2;
                },
                int32(value)
                {
                    dataView.setInt32(0, value, true);
                    for (let i = 0; i < 4; i++)
                        bytes4[i] = dataView.getUint8(i);
                    return bytes4;
                },
                uint32(value)
                {
                    dataView.setUint32(0, value, true);
                    for (let i = 0; i < 4; i++)
                        bytes4[i] = dataView.getUint8(i);
                    return bytes4;
                },
                bigInt64(value)
                {
                    dataView.setBigInt64(0, value, true);
                    for (let i = 0; i < 8; i++)
                        bytes8[i] = dataView.getUint8(i);
                    return bytes8;
                },
                bigUint64(value)
                {
                    dataView.setBigUint64(0, value, true);
                    for (let i = 0; i < 8; i++)
                        bytes8[i] = dataView.getUint8(i);
                    return bytes8;
                },
                next()
                {
                    bytes1[0] = 1;
                    return bytes1;
                },
                end()
                {
                    bytes1[0] = 0;
                    return bytes1;
                },
                value(value, via)
                {
                    return via(value)
                },
            },
            value);
    }
}

/**
*/ export class SMFSyntaxError extends SyntaxError { }

/**
@template const T
@param {
(
    visitor:
    {
        readonly boolean: () => Iterable<void, boolean, number>,
        readonly int8: () => Iterable<void, number, number>,
        readonly uint8: () => Iterable<void, number, number>,
        readonly int16: () => Iterable<void, number, number>,
        readonly uint16: () => Iterable<void, number, number>,
        readonly int32: () => Iterable<void, number, number>,
        readonly uint32: () => Iterable<void, number, number>,
        readonly bigInt64: () => Iterable<void, bigint, number>,
        readonly bigUint64: () => Iterable<void, bigint, number>,
        readonly isNext: () => Iterable<void, boolean, number>,
        readonly value: <T>(
            via: () => Generator<void, T, number>)
            => Iterable<void, T, number>,
    },
) => Generator<void, T, number>
} generator
@returns {() => Generator<void, T, number>}
*/ export function deserializeVia(generator)
{
    return function()
    {
        const dataView = new DataView(new ArrayBuffer(8));

        /**
        @template {keyof DataView} K
        @param {number} size
        @param {K} finalize
        */ function intVisitorFactory(size, finalize)
        {
            return (
            {
                /**
                @type {number}
                */ _state: 0,
                /**
                @param {[] | [number]} next
                @returns {IteratorResult<
                    void,
                    DataView[K] extends (
                        byteOffset: 0,
                        littleEndian: true) => infer T ? T : never>}
                */ next(...[value])
                {
                    switch (this._state)
                    {
                        case 0:
                            this._state = 1;
                            // Generator spec.
                            return { done: false, value: undefined };
                        case -1:
                            // @ts-expect-error: Generator spec.
                            return { done: true, value: undefined };
                        default:
                            if (typeof value !== "number")
                                throw new TypeError(
                                    "Only bytes can be given to deserializer.");
                            dataView.setUint8(this._state - 1, value);
                            this._state = this._state > size ? -1 : this._state + 1;
                            // @ts-expect-error: Contract.
                            return { done: true, value: dataView[finalize](0, true) };
                    }
                },
                [Symbol.iterator]() { return this },
            })
        }
        const int8 = intVisitorFactory(1, "getInt8");
        const uint8 = intVisitorFactory(1, "getUint8");
        const int16 = intVisitorFactory(2, "getInt16");
        const uint16 = intVisitorFactory(2, "getUint16");
        const int32 = intVisitorFactory(4, "getInt32");
        const uint32 = intVisitorFactory(4, "getUint32");
        const bigInt64 = intVisitorFactory(8, "getBigInt64");
        const bigUint64 = intVisitorFactory(8, "getBigUint64");
        const boolean =
        {
            /**
            @type {-1 | 0 | 1}
            */ _state: 0,
            /**
            @param {[] | [number]} next
            @returns {IteratorResult<void, boolean>}
            */ next(...[value])
            {
                switch (this._state)
                {
                    case 0:
                        this._state = 1;
                        // Generator spec.
                        return { done: false, value: undefined };
                    case -1:
                        // @ts-expect-error: Generator spec.
                        return { done: true, value: undefined };
                    case 1:
                        if (typeof value !== "number")
                            throw new TypeError(
                                "Only bytes can be given to deserializer.");
                        this._state = -1;
                        return { done: true, value: value !== 0 };
                }
            },
            [Symbol.iterator]() { return this },
        };
        const isNext =
        {
            /**
            @type {-1 | 0 | 1}
            */ _state: 0,
            /**
            @param {[] | [number]} next
            @returns {IteratorResult<void, boolean>}
            */ next(...[value])
            {
                switch (this._state)
                {
                    case 0:
                        this._state = 1;
                        // Generator spec.
                        return { done: false, value: undefined };
                    case -1:
                        // @ts-expect-error: Generator spec.
                        return { done: true, value: undefined };
                    case 1:
                        if (typeof value !== "number")
                            throw new TypeError(
                                "Only bytes can be given to deserializer.");
                        this._state = -1;
                        switch (value)
                        {
                            case 0:
                                return { done: true, value: false };
                            case 1:
                                return { done: true, value: true };
                            default:
                                throw new SMFSyntaxError(
                                    "Expected sequence decider but found arbitrary byte.");
                        }
                }
            },
            [Symbol.iterator]() { return this },
        };

        return generator(
        {
            boolean()
            {
                boolean._state = 0;
                return boolean;
            },
            int8()
            {
                int8._state = 0;
                return int8;
            },
            uint8()
            {
                uint8._state = 0;
                return uint8;
            },
            int16()
            {
                int16._state = 0;
                return int16;
            },
            uint16()
            {
                uint16._state = 0;
                return uint16;
            },
            int32()
            {
                int32._state = 0;
                return int32;
            },
            uint32()
            {
                uint32._state = 0;
                return uint32;
            },
            bigInt64()
            {
                bigInt64._state = 0;
                return bigInt64;
            },
            bigUint64()
            {
                bigUint64._state = 0;
                return bigUint64;
            },
            isNext()
            {
                isNext._state = 0;
                return isNext;
            },
            value(via)
            {
                return via()
            },
        });
    }
}