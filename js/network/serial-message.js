
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
@returns {SerialMessageEncodingCallback<SerialMessageEncodingValueOf<F>>}
*/ function _encodeCallback(format)
{
    if (format instanceof Function)
        return format;

    switch (format)
    {
        case "Int8":
        case "Uint8":
        case "int8":
        case "uint8": // @ts-ignore
            return function* (value) { yield value & 0xFF }

        case "Int16":
        case "Uint16":
        case "int16":
        case "uint16":
            return function* (value)
            { // @ts-ignore
                const masked = value & 0xFFFF;
                for (let shift = 0; shift < 16; shift += 8)
                    yield (masked >> shift) & 0xFF;
            }

        case "Int32":
        case "Uint32":
        case "int32":
        case "uint32":
            return function* (value)
            { // @ts-ignore
                const masked = value & 0xFFFFFFFF;
                for (let shift = 0; shift < 32; shift += 8)
                    yield (masked >> shift) & 0xFF;
            }

        case "Int64":
        case "Uint64":
        case "int64":
        case "uint64":
            return function* (value)
            { // @ts-ignore
                const masked = value & 0xFFFFFFFFFFFFFFFFn;
                for (let shift = 0n; shift < 64n; shift += 8n)
                    yield Number((masked >> shift) & 0xFFn);
            }

        case "Int8Array":
        case "Uint8Array":
        case "int8[]":
        case "uint8[]":
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
        case "Uint16Array":
        case "int16[]":
        case "uint16[]":
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
        case "Uint32Array":
        case "int32[]":
        case "uint32[]":
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

        case "Int64Array":
        case "Uint64Array":
        case "int64[]":
        case "uint64[]":
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
        case "char8[]":
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

        case "Int8NullTerminate":
        case "Uint8NullTerminate":
        case "int8...0":
        case "uint8...0":
            return function* (value)
            { // @ts-ignore
                if (Symbol.iterator in value)
                { // @ts-ignore
                    for (const element of value)
                    {
                        const char = element & 0xFF;
                        if (char === 0)
                            throw new RangeError("Null termination encoder cannot encode '0'.");
                        yield char;
                    }
                }
                else if ("length" in value)
                {
                    const length = value.length;
                    for (let i = 0; i < length; i++)
                    {
                        const char = value[i] & 0xFF;
                        if (char === 0)
                            throw new RangeError("Null termination encoder cannot encode '0'.");
                        yield char;
                    }
                }
                else
                {
                    while (true)
                    {
                        const { value: element, done } = value.next();
                        if (done)
                            break;
                        const char = element & 0xFF;
                        if (char === 0)
                            throw new RangeError("Null termination encoder cannot encode '0'.");
                        yield char;
                    }
                }

                yield 0x00;
            };

        case "Char8NullTerminate":
        case "char8...0":
            return function* (value)
            { // @ts-ignore
                if (Symbol.iterator in value)
                { // @ts-ignore
                    for (const element of value)
                    {
                        const char = element.charCodeAt(0) & 0xFF;
                        if (char === 0)
                            throw new RangeError("Null termination encoder cannot encode '\\0'.");
                        yield char;
                    }
                }
                else if ("length" in value)
                {
                    const length = value.length;
                    for (let i = 0; i < length; i++)
                    {
                        const char = value[i].charCodeAt(0) & 0xFF;
                        if (char === 0)
                            throw new RangeError("Null termination encoder cannot encode '\\0'.");
                        yield char;
                    }
                }
                else
                {
                    while (true)
                    {
                        const { value: element, done } = value.next();
                        if (done)
                            break;
                        const char = element.charCodeAt(0) & 0xFF;
                        if (char === 0)
                            throw new RangeError("Null termination encoder cannot encode '\\0'.");
                        yield char;
                    }
                }

                yield 0x00;
            };
    }

    if (format instanceof Array)
    {
        switch (format[0])
        {
            case "Array":
            case "array":
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
            case "tuple":
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
            case "object":
            case "struct":
            case "class":
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