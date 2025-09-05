import { ByteArrayListTag, ByteArrayTag, ByteListTag, ByteTag, CompoundListTag, CompoundTag, DoubleListTag, DoubleTag, EmptyListTag, FloatListTag, FloatTag, IntArrayListTag, IntArrayTag, IntListTag, IntTag, ListListTag, ListTag, LongArrayListTag, LongArrayTag, LongListTag, LongTag, ShortListTag, ShortTag, StringListTag, StringTag, Tag } from "./types.js";

// Spec according to:
//  - https://github.com/acfoltzer/nbt/blob/master/NBT-spec.txt
//  - https://minecraft.wiki/w/NBT_format

/**
@param {CompoundTag} tag
@param {
{
    endian?: "big" | "little",
}
} [config]
@returns {Uint8Array<ArrayBuffer>}
*/ export function serialize(tag, config)
{
    const littleEndian = config?.endian === undefined
        ? undefined :
        config.endian === "little";

    const dataView = new DataView(new ArrayBuffer(8));
    const textEncoder = new TextEncoder();

    /**
    @type {number[]}
    */ const result = [];

    /**
    @param {number} count
    */ function transferFromDataViewToResult(count)
    {
        for (let i = 0; i < count; ++i)
            result.push(dataView.getUint8(i));
    }

    /**
    @param {Tag} tag
    */ function pushTypeOf(tag)
    {
        switch (true)
        {
            case tag instanceof ByteTag: result.push(1); break;
            case tag instanceof ShortTag: result.push(2); break;
            case tag instanceof IntTag: result.push(3); break;
            case tag instanceof LongTag: result.push(4); break;
            case tag instanceof FloatTag: result.push(5); break;
            case tag instanceof DoubleTag: result.push(6); break;
            case tag instanceof ByteArrayTag: result.push(7); break;
            case tag instanceof StringTag: result.push(8); break;
            case tag instanceof ListTag: result.push(9); break;
            case tag instanceof CompoundTag: result.push(10); break;
        }
    }

    /**
    @param {Tag} tag
    @param {string | undefined} debugPath
    */ function pushPayloadOf(tag, debugPath)
    {
        let i = -1;
        switch (true)
        {
            case tag instanceof ByteTag:
                result.push(tag.value);
                break;
            case tag instanceof ShortTag:
                dataView.setUint16(0, tag.value, littleEndian);
                transferFromDataViewToResult(2);
                break;
            case tag instanceof IntTag:
                dataView.setInt32(0, tag.value, littleEndian);
                transferFromDataViewToResult(4);
                break;
            case tag instanceof LongTag:
                dataView.setBigInt64(0, tag.value, littleEndian);
                transferFromDataViewToResult(8);
                break;
            case tag instanceof FloatTag:
                dataView.setFloat32(0, tag.value, littleEndian);
                transferFromDataViewToResult(4);
                break;
            case tag instanceof DoubleTag:
                dataView.setFloat64(0, tag.value, littleEndian);
                transferFromDataViewToResult(8);
                break;
            case tag instanceof ByteArrayTag:
                dataView.setInt32(0, tag.value.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const element of tag.value)
                    result.push(element);
                break;
            case tag instanceof StringTag:
                const stringBytes = textEncoder.encode(tag.value);
                dataView.setUint16(0, stringBytes.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const byte of stringBytes)
                    result.push(byte);
                break;
            case tag instanceof CompoundTag:
                for (const [name, value] of tag.entries())
                {
                    pushTypeOf(value);
                    pushPayloadOf(
                        new StringTag(name),
                        debugPath === undefined
                            ? name
                            : debugPath + "." + name);
                    pushPayloadOf(
                        value,
                        debugPath === undefined
                            ? name
                            : debugPath + "." + name);
                }
                result.push(0);
                break;
            case tag instanceof IntArrayTag:
                dataView.setInt32(0, tag.value.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const element of tag.value)
                {
                    dataView.setInt32(0, element, littleEndian);
                    transferFromDataViewToResult(4);
                }
                break;
            case tag instanceof LongArrayTag:
                dataView.setInt32(0, tag.value.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const element of tag.value)
                {
                    dataView.setBigInt64(0, element, littleEndian);
                    transferFromDataViewToResult(8);
                }
                break;
            case tag instanceof EmptyListTag:
                result.push(0);
                result.push(0);
                result.push(0);
                break;
            case tag instanceof ByteListTag:
                result.push(1);
                dataView.setInt32(0, tag.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const value of tag)
                    result.push(value);
                break;
            case tag instanceof ShortListTag:
                result.push(2);
                dataView.setInt32(0, tag.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const value of tag)
                {
                    dataView.setUint16(0, value, littleEndian);
                    transferFromDataViewToResult(2);
                }
                break;
            case tag instanceof IntListTag:
                result.push(3);
                dataView.setInt32(0, tag.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const value of tag)
                {
                    dataView.setInt32(0, value, littleEndian);
                    transferFromDataViewToResult(4);
                }
                break;
            case tag instanceof LongListTag:
                result.push(4);
                dataView.setInt32(0, tag.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const value of tag)
                {
                    dataView.setBigUint64(0, value, littleEndian);
                    transferFromDataViewToResult(8);
                }
                break;
            case tag instanceof FloatListTag:
                result.push(5);
                dataView.setInt32(0, tag.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const value of tag)
                {
                    dataView.setFloat32(0, value, littleEndian);
                    transferFromDataViewToResult(4);
                }
                break;
            case tag instanceof DoubleListTag:
                result.push(6);
                dataView.setInt32(0, tag.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const value of tag)
                {
                    dataView.setFloat64(0, value, littleEndian);
                    transferFromDataViewToResult(8);
                }
                break;
            case tag instanceof ByteArrayListTag:
                result.push(7);
                dataView.setInt32(0, tag.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const value of tag)
                {
                    dataView.setInt32(0, value.length, littleEndian);
                    transferFromDataViewToResult(4);
                    for (const byte of value)
                        result.push(byte);
                }
                break;
            case tag instanceof StringListTag:
                result.push(8);
                dataView.setInt32(0, tag.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const value of tag)
                {
                    const stringBytes = textEncoder.encode(value);
                    dataView.setUint16(0, stringBytes.length, littleEndian);
                    transferFromDataViewToResult(4);
                    for (const byte of stringBytes)
                        result.push(byte);
                }
                break;
            case tag instanceof ListListTag:
                result.push(9);
                dataView.setInt32(0, tag.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const value of tag)
                    pushPayloadOf(
                        value,
                        debugPath === undefined
                            ? `[${++i}]`
                            : debugPath + `[${++i}]`);
                break;
            case tag instanceof CompoundListTag:
                result.push(10);
                dataView.setInt32(0, tag.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const value of tag)
                    pushPayloadOf(
                        value,
                        debugPath === undefined
                            ? `[${++i}]`
                            : debugPath + `[${++i}]`);
                break;
            case tag instanceof IntArrayListTag:
                result.push(7);
                dataView.setInt32(0, tag.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const value of tag)
                {
                    dataView.setInt32(0, value.length, littleEndian);
                    transferFromDataViewToResult(4);
                    for (const element of value)
                    {
                        dataView.setInt32(0, element, littleEndian);
                        transferFromDataViewToResult(4);
                    }
                }
                break;
            case tag instanceof LongArrayListTag:
                result.push(7);
                dataView.setInt32(0, tag.length, littleEndian);
                transferFromDataViewToResult(4);
                for (const value of tag)
                {
                    dataView.setInt32(0, value.length, littleEndian);
                    transferFromDataViewToResult(4);
                    for (const element of value)
                    {
                        dataView.setBigInt64(0, element, littleEndian);
                        transferFromDataViewToResult(8);
                    }
                }
                break;
            default:
                throw new TypeError(
                    `Unsupported tag type at path '${debugPath ?? "."}'.`);
        }
    }

    pushPayloadOf(tag, undefined);

    return new Uint8Array(result);
}

/**
@param {Iterable<number>} bytes
@param {
{
    endian?: "big" | "little",
}
} [config]
@returns {CompoundTag}
*/ export function deserialize(bytes, config)
{
    const dataView = new DataView(new ArrayBuffer(8));
    const textDecoder = new TextDecoder();

    const littleEndian = config?.endian === undefined
        ? undefined :
        config.endian === "little";

    const iterator = bytes[Symbol.iterator]();
    let byteIndex = 0;

    let alreadyFoundEnd = false;

    /**
    @param {string | undefined} debugPath
    @returns {number}
    */ function readBytePayload(debugPath)
    {
        const { value, done } = iterator.next(); ++byteIndex;
        if (done)
            throw new SyntaxError(
                `Expected byte but found end of file at ` +
                `'${debugPath ?? "."}' (byte ${byteIndex}).`);
        dataView.setUint8(0, value);

        return dataView.getInt8(0);
    }

    /**
    @param {string | undefined} debugPath
    @returns {number}
    */ function readShortPayload(debugPath)
    {
        const { value: v0, done: d0 } = iterator.next(); ++byteIndex;
        const { value: v1, done: d1 } = iterator.next(); ++byteIndex;
        if (d0 || d1)
            throw new SyntaxError(
                `Expected short but found end of file at ` +
                `'${debugPath ?? "."}' (byte ${byteIndex}).`);
        dataView.setUint8(0, v0);
        dataView.setUint8(1, v1);

        return dataView.getInt16(0, littleEndian);
    }

    /**
    @param {string | undefined} debugPath
    @returns {number}
    */ function readIntPayload(debugPath)
    {
        const { value: v0, done: d0 } = iterator.next(); ++byteIndex;
        const { value: v1, done: d1 } = iterator.next(); ++byteIndex;
        const { value: v2, done: d2 } = iterator.next(); ++byteIndex;
        const { value: v3, done: d3 } = iterator.next(); ++byteIndex;
        if (d0 || d1 || d2 || d3)
            throw new SyntaxError(
                `Expected int but found end of file at ` +
                `'${debugPath ?? "."}' (byte ${byteIndex}).`);
        dataView.setUint8(0, v0);
        dataView.setUint8(1, v1);
        dataView.setUint8(2, v2);
        dataView.setUint8(3, v3);

        return dataView.getInt32(0, littleEndian);
    }

    /**
    @param {string | undefined} debugPath
    @returns {bigint}
    */ function readLongPayload(debugPath)
    {
        const { value: v0, done: d0 } = iterator.next(); ++byteIndex;
        const { value: v1, done: d1 } = iterator.next(); ++byteIndex;
        const { value: v2, done: d2 } = iterator.next(); ++byteIndex;
        const { value: v3, done: d3 } = iterator.next(); ++byteIndex;
        const { value: v4, done: d4 } = iterator.next(); ++byteIndex;
        const { value: v5, done: d5 } = iterator.next(); ++byteIndex;
        const { value: v6, done: d6 } = iterator.next(); ++byteIndex;
        const { value: v7, done: d7 } = iterator.next(); ++byteIndex;
        if (d0 || d1 || d2 || d3 || d4 || d5 || d6 || d7)
            throw new SyntaxError(
                `Expected long but found end of file at ` +
                `'${debugPath ?? "."}' (byte ${byteIndex}).`);
        dataView.setUint8(0, v0);
        dataView.setUint8(1, v1);
        dataView.setUint8(2, v2);
        dataView.setUint8(3, v3);
        dataView.setUint8(4, v4);
        dataView.setUint8(5, v5);
        dataView.setUint8(6, v6);
        dataView.setUint8(7, v7);

        return dataView.getBigInt64(0, littleEndian);
    }

    /**
    @param {string | undefined} debugPath
    @returns {number}
    */ function readFloatPayload(debugPath)
    {
        const { value: v0, done: d0 } = iterator.next(); ++byteIndex;
        const { value: v1, done: d1 } = iterator.next(); ++byteIndex;
        const { value: v2, done: d2 } = iterator.next(); ++byteIndex;
        const { value: v3, done: d3 } = iterator.next(); ++byteIndex;
        if (d0 || d1 || d2 || d3)
            throw new SyntaxError(
                `Expected int but found end of file at ` +
                `'${debugPath ?? "."}' (byte ${byteIndex}).`);
        dataView.setUint8(0, v0);
        dataView.setUint8(1, v1);
        dataView.setUint8(2, v2);
        dataView.setUint8(3, v3);

        return dataView.getFloat32(0, littleEndian);
    }

    /**
    @param {string | undefined} debugPath
    @returns {number}
    */ function readDoublePayload(debugPath)
    {
        const { value: v0, done: d0 } = iterator.next(); ++byteIndex;
        const { value: v1, done: d1 } = iterator.next(); ++byteIndex;
        const { value: v2, done: d2 } = iterator.next(); ++byteIndex;
        const { value: v3, done: d3 } = iterator.next(); ++byteIndex;
        const { value: v4, done: d4 } = iterator.next(); ++byteIndex;
        const { value: v5, done: d5 } = iterator.next(); ++byteIndex;
        const { value: v6, done: d6 } = iterator.next(); ++byteIndex;
        const { value: v7, done: d7 } = iterator.next(); ++byteIndex;
        if (d0 || d1 || d2 || d3 || d4 || d5 || d6 || d7)
            throw new SyntaxError(
                `Expected long but found end of file at ` +
                `'${debugPath ?? "."}' (byte ${byteIndex}).`);
        dataView.setUint8(0, v0);
        dataView.setUint8(1, v1);
        dataView.setUint8(2, v2);
        dataView.setUint8(3, v3);
        dataView.setUint8(4, v4);
        dataView.setUint8(5, v5);
        dataView.setUint8(6, v6);
        dataView.setUint8(7, v7);

        return dataView.getFloat64(0, littleEndian);
    }

    /**
    @param {string | undefined} debugPath
    @returns {Int8Array}
    */ function readByteArrayPayload(debugPath)
    {
        const { value: vl0, done: dl0 } = iterator.next(); ++byteIndex;
        const { value: vl1, done: dl1 } = iterator.next(); ++byteIndex;
        const { value: vl2, done: dl2 } = iterator.next(); ++byteIndex;
        const { value: vl3, done: dl3 } = iterator.next(); ++byteIndex;
        if (dl0 || dl1 || dl2 || dl3)
            throw new SyntaxError(
                `Expected byte array length but found end of file at ` +
                `'${debugPath ?? "."}' (byte ${byteIndex}).`);
        dataView.setUint8(0, vl0);
        dataView.setUint8(1, vl1);
        dataView.setUint8(2, vl2);
        dataView.setUint8(3, vl3);
        const result = new Int8Array(dataView.getInt32(0, littleEndian));
        for (let i = 0; i < result.length; ++i)
        {
            const { value, done } = iterator.next(); ++byteIndex;
            if (done)
                throw new SyntaxError(
                    `Expected byte array but found end of file at ` +
                    `'${debugPath ?? "."}' (byte ${byteIndex}).`);
            result[i] = value;
        }

        return result;
    }

    /**
    @param {string | undefined} debugPath
    @returns {string}
    */ function readStringPayload(debugPath)
    {
        console.log(`byteIndex = ${byteIndex}`);
        const { value: vl0, done: dl0 } = iterator.next(); ++byteIndex;
        const { value: vl1, done: dl1 } = iterator.next(); ++byteIndex;
        if (dl0 || dl1)
            throw new SyntaxError(
                `Expected string length but found end of file at ` +
                `'${debugPath ?? "."}' (byte ${byteIndex}).`);
        dataView.setUint8(0, vl0);
        dataView.setUint8(1, vl1);
        const result = new Uint8Array(dataView.getUint16(0, littleEndian));
        console.log(result.length);
        for (let i = 0; i < result.length; ++i)
        {
            const { value, done } = iterator.next(); ++byteIndex;
            if (done)
                throw new SyntaxError(
                    `Expected string but found end of file at ` +
                    `'${debugPath ?? "."}' (byte ${byteIndex}).`);
            result[i] = value;
        }
        console.log(textDecoder.decode(result));

        return textDecoder.decode(result);
    }

    /**
    @param {string | undefined} debugPath
    @returns {ListTag}
    */ function readAndGetListPayload(debugPath)
    {
        const { value: vt, done: dt } = iterator.next(); ++byteIndex;
        if (dt)
            throw new SyntaxError(
                `Expected list type but found end of file at ` +
                `'${debugPath ?? "."}' (byte ${byteIndex}).`);
        const { value: vl0, done: dl0 } = iterator.next(); ++byteIndex;
        const { value: vl1, done: dl1 } = iterator.next(); ++byteIndex;
        const { value: vl2, done: dl2 } = iterator.next(); ++byteIndex;
        const { value: vl3, done: dl3 } = iterator.next(); ++byteIndex;
        if (dl0 || dl1 || dl2 || dl3)
            throw new SyntaxError(
                `Expected list length but found end of file at ` +
                `'${debugPath ?? "."}' (byte ${byteIndex}).`);
        dataView.setUint8(0, vl0);
        dataView.setUint8(1, vl1);
        dataView.setUint8(2, vl2);
        dataView.setUint8(3, vl3);

        switch (vt)
        {
            case 0:
            {
                if (dataView.getInt32(0, littleEndian) !== 0)
                    throw new SyntaxError(
                        `Found invalid list element type at ` +
                        `'${debugPath ?? "."}' (byte ${byteIndex}).`);

                return new EmptyListTag();
            }
            case 1:
            {
                /**
                @type {ByteListTag["value"]}
                */ const list = new Array(dataView.getInt32(0, littleEndian));
                for (let i = 0; i < list.length; ++i)
                    list[i] = readBytePayload(debugPath === undefined
                        ? `[${i}]`
                        : debugPath + `[${i}]`);

                return new ByteListTag(list);
            }
            case 2:
            {
                /**
                @type {ShortListTag["value"]}
                */ const list = new Array(dataView.getInt32(0, littleEndian));
                for (let i = 0; i < list.length; ++i)
                    list[i] = readShortPayload(debugPath === undefined
                        ? `[${i}]`
                        : debugPath + `[${i}]`);

                return new ShortListTag(list);
            }
            case 3:
            {
                /**
                @type {IntListTag["value"]}
                */ const list = new Array(dataView.getInt32(0, littleEndian));
                for (let i = 0; i < list.length; ++i)
                    list[i] = readIntPayload(debugPath === undefined
                        ? `[${i}]`
                        : debugPath + `[${i}]`);

                return new IntListTag(list);
            }
            case 4:
            {
                /**
                @type {LongListTag["value"]}
                */ const list = new Array(dataView.getInt32(0, littleEndian));
                for (let i = 0; i < list.length; ++i)
                    list[i] = readLongPayload(debugPath === undefined
                        ? `[${i}]`
                        : debugPath + `[${i}]`);

                return new LongListTag(list);
            }
            case 5:
            {
                /**
                @type {FloatListTag["value"]}
                */ const list = new Array(dataView.getInt32(0, littleEndian));
                for (let i = 0; i < list.length; ++i)
                    list[i] = readFloatPayload(debugPath === undefined
                        ? `[${i}]`
                        : debugPath + `[${i}]`);

                return new FloatListTag(list);
            }
            case 6:
            {
                /**
                @type {DoubleListTag["value"]}
                */ const list = new Array(dataView.getInt32(0, littleEndian));
                for (let i = 0; i < list.length; ++i)
                    list[i] = readDoublePayload(debugPath === undefined
                        ? `[${i}]`
                        : debugPath + `[${i}]`);

                return new DoubleListTag(list);
            }
            case 7:
            {
                /**
                @type {ByteArrayListTag["value"]}
                */ const list = new Array(dataView.getInt32(0, littleEndian));
                for (let i = 0; i < list.length; ++i)
                    list[i] = readByteArrayPayload(debugPath === undefined
                        ? `[${i}]`
                        : debugPath + `[${i}]`);

                return new ByteArrayListTag(list);
            }
            case 8:
            {
                /**
                @type {StringListTag["value"]}
                */ const list = new Array(dataView.getInt32(0, littleEndian));
                for (let i = 0; i < list.length; ++i)
                    list[i] = readStringPayload(debugPath === undefined
                        ? `[${i}]`
                        : debugPath + `[${i}]`);

                return new StringListTag(list);
            }
            case 9:
            {
                /**
                @type {ListTag[]}
                */ const list = new Array(dataView.getInt32(0, littleEndian));
                for (let i = 0; i < list.length; ++i)
                    list[i] = readAndGetListPayload(debugPath === undefined
                        ? `[${i}]`
                        : debugPath + `[${i}]`);

                return new ListListTag(list);
            }
            case 10:
            {
                /**
                @type {CompoundTag[]}
                */ const list = new Array(dataView.getInt32(0, littleEndian));
                for (let i = 0; i < list.length; ++i)
                    list[i] = readAndGetCompoundPayload(debugPath === undefined
                        ? `[${i}]`
                        : debugPath + `[${i}]`);

                return new CompoundListTag(list);
            }
            default:
                throw new SyntaxError(
                    `Found invalid list element type at ` +
                    `'${debugPath ?? "."}' (byte ${byteIndex}).`);
        }
    }

    /**
    @param {string | undefined} debugPath
    @returns {CompoundTag}
    */ function readAndGetCompoundPayload(debugPath)
    {
        const result = new CompoundTag();

        while (true)
        {
            const { value: vt, done: dt } = iterator.next(); ++byteIndex;
            if (dt)
            {
                if (alreadyFoundEnd)
                    throw new SyntaxError(
                        `Expected compound element type but found end of file at ` +
                        `'${debugPath ?? "."}' (byte ${byteIndex}).`);

                alreadyFoundEnd = true;
                return result;
            }

            if (vt === 0)
                return result;

            const name = readStringPayload(debugPath);

            switch (vt)
            {
                case 1:
                    result.set(
                        name,
                        new ByteTag(readBytePayload(debugPath === undefined
                            ? name
                            : debugPath + "." + name)));
                    break;
                case 2:
                    result.set(
                        name,
                        new ShortTag(readShortPayload(debugPath === undefined
                            ? name
                            : debugPath + "." + name)));
                    break;
                case 3:
                    result.set(
                        name,
                        new IntTag(readIntPayload(debugPath === undefined
                            ? name
                            : debugPath + "." + name)));
                    break;
                case 4:
                    result.set(
                        name,
                        new LongTag(readLongPayload(debugPath === undefined
                            ? name
                            : debugPath + "." + name)));
                    break;
                case 5:
                    result.set(
                        name,
                        new FloatTag(readFloatPayload(debugPath === undefined
                            ? name
                            : debugPath + "." + name)));
                    break;
                case 6:
                    result.set(
                        name,
                        new DoubleTag(readDoublePayload(debugPath === undefined
                            ? name
                            : debugPath + "." + name)));
                    break;
                case 7:
                    result.set(
                        name,
                        new ByteArrayTag(readByteArrayPayload(debugPath === undefined
                            ? name
                            : debugPath + "." + name)));
                    break;
                case 8:
                    result.set(
                        name,
                        new StringTag(readStringPayload(debugPath === undefined
                            ? name
                            : debugPath + "." + name)));
                    break;
                case 9:
                    result.set(
                        name,
                        readAndGetListPayload(debugPath === undefined
                            ? name
                            : debugPath + "." + name));
                    break;
                case 10:
                    result.set(
                        name,
                        readAndGetCompoundPayload(debugPath === undefined
                            ? name
                            : debugPath + "." + name));
                    break;
                default:
                    throw new SyntaxError(
                        `Found invalid compound element type at ` +
                        `'${debugPath ?? "."}' (byte ${byteIndex}).`);
            }
        }
    }

    /**
    @param {string | undefined} debugPath
    @returns {Int32Array}
    */ function readIntArrayPayload(debugPath)
    {
        const { value: vl0, done: dl0 } = iterator.next(); ++byteIndex;
        const { value: vl1, done: dl1 } = iterator.next(); ++byteIndex;
        const { value: vl2, done: dl2 } = iterator.next(); ++byteIndex;
        const { value: vl3, done: dl3 } = iterator.next(); ++byteIndex;
        if (dl0 || dl1 || dl2 || dl3)
            throw new SyntaxError(
                `Expected int array length but found end of file at ` +
                `'${debugPath ?? "."}' (byte ${byteIndex}).`);
        dataView.setUint8(0, vl0);
        dataView.setUint8(1, vl1);
        dataView.setUint8(2, vl2);
        dataView.setUint8(3, vl3);
        const result = new Int32Array(dataView.getInt32(0, littleEndian));
        for (let i = 0; i < result.length; ++i)
        {
            const { value: v0, done: d0 } = iterator.next(); ++byteIndex;
            const { value: v1, done: d1 } = iterator.next(); ++byteIndex;
            const { value: v2, done: d2 } = iterator.next(); ++byteIndex;
            const { value: v3, done: d3 } = iterator.next(); ++byteIndex;
            if (d0 || d1 || d2 || d3)
                throw new SyntaxError(
                    `Expected int array but found end of file at ` +
                    `'${debugPath ?? "."}' (byte ${byteIndex}).`);
            dataView.setUint8(0, v0);
            dataView.setUint8(1, v1);
            dataView.setUint8(2, v2);
            dataView.setUint8(3, v3);
            result[i] = dataView.getInt32(0, littleEndian);
        }

        return result;
    }

    /**
    @param {string | undefined} debugPath
    @returns {BigInt64Array}
    */ function readIntArrayPayload(debugPath)
    {
        const { value: vl0, done: dl0 } = iterator.next(); ++byteIndex;
        const { value: vl1, done: dl1 } = iterator.next(); ++byteIndex;
        const { value: vl2, done: dl2 } = iterator.next(); ++byteIndex;
        const { value: vl3, done: dl3 } = iterator.next(); ++byteIndex;
        if (dl0 || dl1 || dl2 || dl3)
            throw new SyntaxError(
                `Expected long array length but found end of file at ` +
                `'${debugPath ?? "."}' (byte ${byteIndex}).`);
        dataView.setUint8(0, vl0);
        dataView.setUint8(1, vl1);
        dataView.setUint8(2, vl2);
        dataView.setUint8(3, vl3);
        const result = new BigInt64Array(dataView.getInt32(0, littleEndian));
        for (let i = 0; i < result.length; ++i)
        {
            const { value: v0, done: d0 } = iterator.next(); ++byteIndex;
            const { value: v1, done: d1 } = iterator.next(); ++byteIndex;
            const { value: v2, done: d2 } = iterator.next(); ++byteIndex;
            const { value: v3, done: d3 } = iterator.next(); ++byteIndex;
            const { value: v4, done: d4 } = iterator.next(); ++byteIndex;
            const { value: v5, done: d5 } = iterator.next(); ++byteIndex;
            const { value: v6, done: d6 } = iterator.next(); ++byteIndex;
            const { value: v7, done: d7 } = iterator.next(); ++byteIndex;
            if (d0 || d1 || d2 || d3 || d4 || d5 || d6 || d7)
                throw new SyntaxError(
                    `Expected long array but found end of file at ` +
                    `'${debugPath ?? "."}' (byte ${byteIndex}).`);
            dataView.setUint8(0, v0);
            dataView.setUint8(1, v1);
            dataView.setUint8(2, v2);
            dataView.setUint8(3, v3);
            dataView.setUint8(4, v4);
            dataView.setUint8(5, v5);
            dataView.setUint8(6, v6);
            dataView.setUint8(7, v7);
            result[i] = dataView.getBigInt64(0, littleEndian);
        }

        return result;
    }

    return readAndGetCompoundPayload(undefined);
}