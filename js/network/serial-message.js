
/**
@template {SerialMessageEncodingFormat} const F
@param {F} format
@param {SerialMessageEncodingValueOf<F>} value
@returns {Uint8Array}
*/ export function encode(format, value)
{
    const encoded = _encodeCallback(format)(value);
    if ("next" in encoded)
        return new Uint8Array({ [Symbol.iterator]: () => encoded });
    else
        return new Uint8Array(encoded);
}

/**
@template {SerialMessageEncodingFormat} const F
@param {F} format
@returns {(value: SerialMessageEncodingValueOf<F>) => Uint8Array}
*/ export function createEncoder(format)
{
    const callback = _encodeCallback(format);
    return (value) =>
    {
        const encoded = callback(value);
        if ("next" in encoded)
            return new Uint8Array({ [Symbol.iterator]: () => encoded });
        else
            return new Uint8Array(encoded);
    };
}

/**
@template {SerialMessageEncodingFormat} const F
@param {F} format
@returns {SerialMessageEncodingCallback<SerialMessageEncodingValueOf<F>>}
*/ function _encodeCallback(format)
{
    if (format instanceof Function)
        return format;

    switch (format)
    {
        case "Int8":
        case "signed char":
        case "i8":
        case "Byte":
        case "Uint8":
        case "unsigned char":
        case "u8": // @ts-ignore
            return function* (value) { yield value & 0xFF }

        case "Int16":
        case "signed short":
        case "signed short int":
        case "i16":
        case "Uint16":
        case "unsigned short":
        case "unsigned short int":
        case "u16":
            return function* (value)
            { // @ts-ignore
                const masked = value & 0xFFFF;
                for (let shift = 0; shift < 16; shift += 8)
                    yield (masked >> shift) & 0xFF;
            }

        case "Int32":
        case "signed int":
        case "i32":
        case "Uint32":
        case "unsigned int":
        case "u32":
            return function* (value)
            { // @ts-ignore
                const masked = value & 0xFFFFFFFF;
                for (let shift = 0; shift < 32; shift += 8)
                    yield (masked >> shift) & 0xFF;
            }

        case "Int64":
        case "signed long":
        case "signed long int":
        case "i64":
        case "Uint64":
        case "unsigned long":
        case "unsigned long int":
        case "u64":
            return function* (value)
            { // @ts-ignore
                const masked = value & 0xFFFFFFFFFFFFFFFFn;
                for (let shift = 0n; shift < 64n; shift += 8n)
                    yield Number((masked >> shift) & 0xFFn);
            }

        case "Char8":
        case "char":
        case "c8":
            return function* (value)
            { // @ts-ignore
                return value.charCodeAt(0) & 0xFF;
            }

        case "Int8Array":
        case "signed char[]":
        case "[i8]":
        case "ByteArray":
        case "Uint8Array":
        case "unsigned char[]":
        case "[u8]":
            return function* (value)
            { // @ts-ignore
                if (value.length >= 0xFF)
                {
                    yield 0xFF; // @ts-ignore
                    yield (value.length >> 0) & 0xFF; // @ts-ignore
                    yield (value.length >> 8) & 0xFF;
                }
                else // @ts-ignore
                    yield value.length & 0xFF;
                // @ts-ignore
                for (const element of value)
                    yield element & 0xFF;
            };

        case "Int16Array":
        case "signed short[]":
        case "signed short int[]":
        case "[i16]":
        case "Uint16Array":
        case "unsigned short[]":
        case "unsigned short int[]":
        case "[u16]":
            return function* (value)
            { // @ts-ignore
                if (value.length >= 0xFF)
                {
                    yield 0xFF; // @ts-ignore
                    yield (value.length >> 0) & 0xFF; // @ts-ignore
                    yield (value.length >> 8) & 0xFF;
                }
                else // @ts-ignore
                    yield value.length & 0xFF;
                // @ts-ignore
                for (const element of value)
                {
                    const masked = element & 0xFFFF;
                    for (let shift = 0; shift < 16; shift += 8)
                        yield (masked >> shift) & 0xFF;
                }
            };

        case "Int32Array":
        case "signed int[]":
        case "[i32]":
        case "Uint32Array":
        case "unsigned int[]":
        case "[u32]":
            return function* (value)
            { // @ts-ignore
                if (value.length >= 0xFF)
                {
                    yield 0xFF; // @ts-ignore
                    yield (value.length >> 0) & 0xFF; // @ts-ignore
                    yield (value.length >> 8) & 0xFF;
                }
                else // @ts-ignore
                    yield value.length & 0xFF;
                // @ts-ignore
                for (const element of value)
                {
                    const masked = element & 0xFFFFFFFF;
                    for (let shift = 0; shift < 32; shift += 8)
                        yield (masked >> shift) & 0xFF;
                }
            };

        case "BigInt64Array":
        case "signed long[]":
        case "signed long int[]":
        case "[i64]":
        case "BigUint64Array":
        case "unsigned long[]":
        case "unsigned long int[]":
        case "[u64]":
            return function* (value)
            { // @ts-ignore
                if (value.length >= 0xFF)
                {
                    yield 0xFF; // @ts-ignore
                    yield (value.length >> 0) & 0xFF; // @ts-ignore
                    yield (value.length >> 8) & 0xFF;
                }
                else // @ts-ignore
                    yield value.length & 0xFF;
                // @ts-ignore
                for (const element of value)
                {
                    const masked = element & 0xFFFFFFFFFFFFFFFFn;
                    for (let shift = 0n; shift < 64n; shift += 8n)
                        yield Number((masked >> shift) & 0xFFn);
                }
            };

        case "Char8Array":
        case "char[]":
        case "[c8]":
            return function* (value)
            { // @ts-ignore
                if (value.length >= 0xFF)
                {
                    yield 0xFF; // @ts-ignore
                    yield (value.length >> 0) & 0xFF; // @ts-ignore
                    yield (value.length >> 8) & 0xFF;
                }
                else // @ts-ignore
                    yield value.length & 0xFF;
                // @ts-ignore
                for (const element of value)
                    yield element.charCodeAt(0) & 0xFF;
            };
    }

    if (format instanceof Array)
    {
        switch (format[0])
        {
            case "Array":
            case "[]":
            {
                const encoder = _encodeCallback(format[1]);
                return function* (value)
                { // @ts-ignore
                    const length = value.length;
                    if (length >= 0xFF)
                    {
                        yield 0xFF; // @ts-ignore
                        yield (length >> 0) & 0xFF; // @ts-ignore
                        yield (length >> 8) & 0xFF;
                    }
                    else // @ts-ignore
                        yield length & 0xFF;
                    // @ts-ignore
                    for (let i = 0; i < length; i++)
                    {
                        const encoded = encoder(value[i]);
                        if ("next" in encoded)
                        {
                            while (true)
                            {
                                const { value, done } = encoded.next();
                                if (done)
                                    break;
                                yield value;
                            }
                        }
                        else
                        {
                            const length = encoded.length;
                            for (let i = 0; i < length; i++)
                                yield encoded[i];
                        }
                    }
                };
            }

            case "Tuple":
            case "()":
            {
                return function* (value)
                {
                    const length = format.length;
                    for (let i = 1; i < length; i++)
                    { // @ts-ignore
                        const encoded = _encodeCallback(format[i])(value[i - 1]);
                        if ("next" in encoded)
                        {
                            while (true)
                            {
                                const { value, done } = encoded.next();
                                if (done)
                                    break;
                                yield value;
                            }
                        }
                        else
                        {
                            const length = encoded.length;
                            for (let i = 0; i < length; i++)
                                yield encoded[i];
                        }
                    }
                };
            }

            case "Object":
            case "class":
            case "struct":
            case "{}":
            {
                return function* (value)
                {
                    const length = format.length;
                    for (let i = 1; i < length; i++)
                    { // @ts-ignore
                        const encoded = _encodeCallback(format[i][1])(value[format[i][0]]);
                        if ("next" in encoded)
                        {
                            while (true)
                            {
                                const { value, done } = encoded.next();
                                if (done)
                                    break;
                                yield value;
                            }
                        }
                        else
                        {
                            const length = encoded.length;
                            for (let i = 0; i < length; i++)
                                yield encoded[i];
                        }
                    }
                };
            }
        }
    }

    throw new TypeError("Invalid format.");
}

/**
@template {SerialMessageDecodingFormat} const F
@param {F} format
@param {ArrayLike<number> | Iterator<number> | (() => IteratorResult<Byte>)} value
@returns {SerialMessageDecodingValueOf<F>}
*/ export function decode(format, value)
{
    if (Symbol.iterator in value) // @ts-ignore
        value = value[Symbol.iterator];

    if ("length" in value && !(value instanceof Function))
    {
        const array = value;
        let index = 0;
        value = () =>
        {
            if (index < array.length)
                return { done: false, value: array[index++] };
            else
                return { done: true, value: undefined };
        };
    }
    else if ("next" in value)
        value = value.next;

    return _decodeCallback(format)(value);
}

/**
@template {SerialMessageDecodingFormat} const F
@param {F} format
@returns {(encoded: Uint8Array) => SerialMessageDecodingValueOf<F>}
*/ export function createDecoder(format)
{
    const callback = _decodeCallback(format);
    return (value) =>
    {
        let index = 0;
        return callback(() =>
        {
            if (index < value.length)
                return { done: false, value: value[index] };
            else
                return { done: true, value: undefined };
        });
    };
}

/**
@template {SerialMessageDecodingFormat} const F
@param {F} format
@returns {SerialMessageDecodingCallback<SerialMessageDecodingValueOf<F>>}
*/ function _decodeCallback(format)
{
    if (format instanceof Function)
        return format;

    /**
    @param {IteratorResult<number>} pair
    @returns {number}
    */ function byteOrThrow(pair)
    {
        if (pair.done)
            throw new RangeError("Unexpected end of encoded data found.");

        return pair.value & 0xFF;
    }

    switch (format)
    {
        case "Byte":
        case "Uint8":
        case "unsigned char":
        case "u8": // @ts-ignore
            return (nextByte) =>
            {
                return byteOrThrow(nextByte());
            };

        case "Int8":
        case "signed char":
        case "i8": // @ts-ignore
            return (nextByte) =>
            {
                const byte = byteOrThrow(nextByte());
                return byte >= 0x80 ? byte - 0xFF : byte;
            };

        case "Uint16":
        case "unsigned short":
        case "unsigned short int":
        case "u16": // @ts-ignore
            return (nextByte) =>
            {
                let result = 0;
                for (let shift = 0; shift < 16; shift += 8)
                {
                    const byte = byteOrThrow(nextByte());
                    result |= byte << shift;
                }
                return result;
            };

        case "Int16":
        case "signed short":
        case "signed short int":
        case "i16": // @ts-ignore
            return (nextByte) =>
            {
                let result = 0;
                for (let shift = 0; shift < 16; shift += 8)
                {
                    const byte = byteOrThrow(nextByte());
                    result |= byte << shift;
                }
                return result >= 0x8000 ? result - 0xFFFF : result;
            };

        case "Uint32":
        case "unsigned int":
        case "u32": // @ts-ignore
            return (nextByte) =>
            {
                let result = 0;
                for (let shift = 0; shift < 32; shift += 8)
                {
                    const byte = byteOrThrow(nextByte());
                    result |= byte << shift;
                }
                return result;
            };

        case "Int32":
        case "signed int":
        case "i32": // @ts-ignore
            return (nextByte) =>
            {
                let result = 0;
                for (let shift = 0; shift < 32; shift += 8)
                {
                    const byte = byteOrThrow(nextByte());
                    result |= byte << shift;
                }
                return result >= 0x80000000 ? result - 0xFFFFFFFF : result;
            };

        case "Uint64":
        case "unsigned long":
        case "unsigned long int":
        case "u64": // @ts-ignore
            return (nextByte) =>
            {
                let result = 0n;
                for (let shift = 0n; shift < 64n; shift += 8n)
                {
                    const byte = byteOrThrow(nextByte());
                    result |= BigInt(byte) << shift;
                }
                return result;
            };

        case "Int64":
        case "signed long":
        case "signed long int":
        case "i64": // @ts-ignore
            return (nextByte) =>
            {
                let result = 0n;
                for (let shift = 0n; shift < 64n; shift += 8n)
                {
                    const byte = byteOrThrow(nextByte());
                    result |= BigInt(byte) << shift;
                }
                return result >= 0x8000000000000000n ? result - 0xFFFFFFFFFFFFFFFFn : result;
            };

        case "Char8":
        case "char":
        case "c8": // @ts-ignore
            return (nextByte) =>
            {
                return String.fromCharCode(byteOrThrow(nextByte()));
            };

        case "ByteArray":
        case "Uint8Array":
        case "unsigned char[]":
        case "[u8]": // @ts-ignore
            return (nextByte) =>
            {
                let length;
                {
                    const byte0 = byteOrThrow(nextByte());

                    if (byte0 === 0xFF)
                        length = (byteOrThrow(nextByte()) << 0) | (byteOrThrow(nextByte()) << 8);
                    else
                        length = byte0;
                }

                const result = new Uint8Array(length);
                for (let i = 0; i < length; i++)
                    result[i] = byteOrThrow(nextByte());

                return result;
            };

        case "Int8Array":
        case "signed char[]":
        case "[i8]": // @ts-ignore
            return (nextByte) =>
            {
                let length;
                {
                    const byte0 = byteOrThrow(nextByte());

                    if (byte0 === 0xFF)
                        length = (byteOrThrow(nextByte()) << 0) | (byteOrThrow(nextByte()) << 8);
                    else
                        length = byte0;
                }

                const result = new Int8Array(length);
                for (let i = 0; i < length; i++)
                {
                    const byte = byteOrThrow(nextByte());
                    result[i] = byte >= 0x80 ? byte - 0xFF : byte;
                }

                return result;
            };

        case "Uint16Array":
        case "unsigned short[]":
        case "unsigned short int[]":
        case "[u16]": // @ts-ignore
            return (nextByte) =>
            {
                let length;
                {
                    const byte0 = byteOrThrow(nextByte());

                    if (byte0 === 0xFF)
                        length = (byteOrThrow(nextByte()) << 0) | (byteOrThrow(nextByte()) << 8);
                    else
                        length = byte0;
                }

                const result = new Uint16Array(length);
                for (let i = 0; i < length; i++)
                {
                    let element = 0;
                    for (let shift = 0; shift < 16; shift += 8)
                    {
                        const byte = byteOrThrow(nextByte());
                        element |= byte << shift;
                    }
                    result[i] = element;
                }

                return result;
            };

        case "Int16Array":
        case "signed short[]":
        case "signed short int[]":
        case "[i16]": // @ts-ignore
            return (nextByte) =>
            {
                let length;
                {
                    const byte0 = byteOrThrow(nextByte());

                    if (byte0 === 0xFF)
                        length = (byteOrThrow(nextByte()) << 0) | (byteOrThrow(nextByte()) << 8);
                    else
                        length = byte0;
                }

                const result = new Int16Array(length);
                for (let i = 0; i < length; i++)
                {
                    let element = 0;
                    for (let shift = 0; shift < 16; shift += 8)
                    {
                        const byte = byteOrThrow(nextByte());
                        element |= byte << shift;
                    }
                    result[i] = element >= 0x8000 ? element - 0xFFFF : element;
                }

                return result;
            };

        case "Uint32Array":
        case "unsigned int[]":
        case "[u32]": // @ts-ignore
            return (nextByte) =>
            {
                let length;
                {
                    const byte0 = byteOrThrow(nextByte());

                    if (byte0 === 0xFF)
                        length = (byteOrThrow(nextByte()) << 0) | (byteOrThrow(nextByte()) << 8);
                    else
                        length = byte0;
                }

                const result = new Uint32Array(length);
                for (let i = 0; i < length; i++)
                {
                    let element = 0;
                    for (let shift = 0; shift < 32; shift += 8)
                    {
                        const byte = byteOrThrow(nextByte());
                        element |= byte << shift;
                    }
                    result[i] = element;
                }

                return result;
            };

        case "Int32Array":
        case "signed int[]":
        case "[i32]": // @ts-ignore
            return (nextByte) =>
            {
                let length;
                {
                    const byte0 = byteOrThrow(nextByte());

                    if (byte0 === 0xFF)
                        length = (byteOrThrow(nextByte()) << 0) | (byteOrThrow(nextByte()) << 8);
                    else
                        length = byte0;
                }

                const result = new Int32Array(length);
                for (let i = 0; i < length; i++)
                {
                    let element = 0;
                    for (let shift = 0; shift < 32; shift += 8)
                    {
                        const byte = byteOrThrow(nextByte());
                        element |= byte << shift;
                    }
                    result[i] = element >= 0x80000000 ? element - 0xFFFFFFFF : element;
                }

                return result;
            };

        case "BigUint64Array":
        case "unsigned long[]":
        case "unsigned long int[]":
        case "[u64]": // @ts-ignore
            return (nextByte) =>
            {
                let length;
                {
                    const byte0 = byteOrThrow(nextByte());

                    if (byte0 === 0xFF)
                        length = (byteOrThrow(nextByte()) << 0) | (byteOrThrow(nextByte()) << 8);
                    else
                        length = byte0;
                }

                const result = new BigUint64Array(length);
                for (let i = 0; i < length; i++)
                {
                    let element = 0n;
                    for (let shift = 0n; shift < 32n; shift += 8n)
                    {
                        const byte = byteOrThrow(nextByte());
                        element |= BigInt(byte) << shift;
                    }
                    result[i] = element;
                }

                return result;
            };

        case "BigInt64Array":
        case "signed long[]":
        case "signed long int[]":
        case "[i64]": // @ts-ignore
            return (nextByte) =>
            {
                let length;
                {
                    const byte0 = byteOrThrow(nextByte());

                    if (byte0 === 0xFF)
                        length = (byteOrThrow(nextByte()) << 0) | (byteOrThrow(nextByte()) << 8);
                    else
                        length = byte0;
                }

                const result = new BigInt64Array(length);
                for (let i = 0; i < length; i++)
                {
                    let element = 0n;
                    for (let shift = 0n; shift < 64n; shift += 8n)
                    {
                        const byte = byteOrThrow(nextByte());
                        element |= BigInt(byte) << shift;
                    }
                    result[i] = element >= 0x8000000000000000n ? element - 0xFFFFFFFFFFFFFFFFn : element;
                }

                return result;
            };

        case "Char8Array":
        case "char[]":
        case "[c8]": // @ts-ignore
            return (nextByte) =>
            {
                let length;
                {
                    const byte0 = byteOrThrow(nextByte());

                    if (byte0 === 0xFF)
                        length = (byteOrThrow(nextByte()) << 0) | (byteOrThrow(nextByte()) << 8);
                    else
                        length = byte0;
                }

                const result = new Array(length);
                for (let i = 0; i < length; i++)
                    result[i] = byteOrThrow(nextByte());

                return String.fromCharCode(...result);
            };
    }

    if (format instanceof Array)
    {
        switch (format[0])
        {
            case "Array":
            case "[]":
            {
                const decoder = _decodeCallback(format[1]); // @ts-ignore
                return (nextByte) =>
                {
                    let length;
                    {
                        const byte0 = byteOrThrow(nextByte());

                        if (byte0 === 0xFF)
                            length = (byteOrThrow(nextByte()) << 0) | (byteOrThrow(nextByte()) << 8);
                        else
                            length = byte0;
                    }

                    const result = new Array(length);
                    for (let i = 0; i < length; i++)
                        result[i] = decoder(nextByte);

                    return result;
                };
            }

            case "Tuple":
            case "()":
            {
                const [_, ...formatContents] = format;
                const elementFormats = formatContents.map(_decodeCallback); // @ts-ignore
                return (nextByte) =>
                {
                    const result = new Array(format.length - 1);
                    for (let i = 1; i < format.length; i++) // @ts-ignore
                        result[i] = elementFormats[i](nextByte);

                    return result;
                };
            }

            case "Object":
            case "class":
            case "struct":
            case "{}":
            {
                const elementFormats = /** @type {{ [K in string]: SerialMessageDecodingCallback<any> }} */({});
                for (let i = 1; i < format.length; i++)
                {
                    const [key, value] = format[i]; // @ts-ignore
                    elementFormats[key] = _decodeCallback(value);
                }
                // @ts-ignore
                return (nextByte) =>
                {
                    const result = /** @type {{ [K in string]: any }} */({});
                    for (const key in elementFormats)
                        result[key] = elementFormats[key](nextByte);

                    return result;
                };
            }
        }
    }

    throw new TypeError("Invalid format.");
}