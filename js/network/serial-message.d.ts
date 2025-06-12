
type Byte = number

type SerialMessageBuiltInField = 
    | "Int8"
    | "Uint8"
    | "Int16"
    | "Uint16"
    | "Int32"
    | "Uint32"
    | "Int64"
    | "Uint64"
    | "Int8Array"
    | "Uint8Array"
    | "Int16Array"
    | "Uint16Array"
    | "Int32Array"
    | "Uint32Array"
    | "Int64Array"
    | "Uint64Array"
    | "Char8Array"
    | "Int8NullTerminate"
    | "Uint8NullTerminate"
    | "Char8NullTerminate"
    | "int8"
    | "uint8"
    | "int16"
    | "uint16"
    | "int32"
    | "uint32"
    | "int64"
    | "uint64"
    | "int8[]"
    | "uint8[]"
    | "int16[]"
    | "uint16[]"
    | "int32[]"
    | "uint32[]"
    | "int64[]"
    | "uint64[]"
    | "char8[]"
    | "int8...0"
    | "uint8...0"
    | "char8...0"

type SerialMessageEncodingCallback<V = any> = (value: V) => ArrayLike<Byte> | Iterator<Byte>

type SerialMessageEncodingArray = readonly [type: "Array" | "array", SerialMessageEncodingField]

type SerialMessageEncodingTuple = readonly [type: "Tuple" | "tuple", ...SerialMessageEncodingField[]]

type SerialMessageEncodingObject = readonly [type: "Object" | "object" | "struct" | "class", ...[key: string, value: SerialMessageEncodingField][]]

type SerialMessageEncodingField = 
    | SerialMessageBuiltInField
    | SerialMessageEncodingArray
    | SerialMessageEncodingTuple
    | SerialMessageEncodingObject
    | SerialMessageEncodingCallback


type SerialMessageEncodingFormat = SerialMessageEncodingField

type SerialMessageDecodingCallback<V = any> = (nextByte: () => IteratorResult<Byte, undefined>) => V

type SerialMessageDecodingArray = readonly [type: "Array" | "array", SerialMessageDecodingField]

type SerialMessageDecodingTuple = readonly [type: "Tuple" | "tuple", ...SerialMessageDecodingField[]]

type SerialMessageDecodingObject = readonly [type: "Object" | "object" | "struct" | "class", ...[key: string, value: SerialMessageDecodingField][]]

type SerialMessageDecodingField = 
    | SerialMessageBuiltInField
    | SerialMessageDecodingArray
    | SerialMessageDecodingTuple
    | SerialMessageDecodingObject
    | SerialMessageDecodingCallback

type SerialMessageDecodingFormat = SerialMessageDecodingField

type SerialMessageEncodingValueOf<F extends SerialMessageEncodingFormat | SerialMessageDecodingFormat> =
    F extends SerialMessageEncodingCallback<infer V> ? V :
    F extends "Int8" ? number :
    F extends "Uint8" ? number :
    F extends "Int16" ? number :
    F extends "Uint16" ? number :
    F extends "Int32" ? number :
    F extends "Uint32" ? number :
    F extends "Int64" ? bigint :
    F extends "Uint64" ? bigint :
    F extends "Int8Array" ? ArrayLike<number> :
    F extends "Uint8Array" ? ArrayLike<number> :
    F extends "Int16Array" ? ArrayLike<number> :
    F extends "Uint16Array" ? ArrayLike<number> :
    F extends "Int32Array" ? ArrayLike<number> :
    F extends "Uint32Array" ? ArrayLike<number> :
    F extends "Int64Array" ? ArrayLike<bigint> :
    F extends "Uint64Array" ? ArrayLike<bigint> :
    F extends "Char8Array" ? string :
    F extends "Int8NullTerminate" ? ArrayLike<number> | Iterable<number> | Iterator<number> :
    F extends "Uint8NullTerminate" ? ArrayLike<number> | Iterable<number> | Iterator<number> :
    F extends "Char8NullTerminate" ? ArrayLike<string> | Iterable<string> | Iterator<string> :
    F extends "int8" ? number :
    F extends "uint8" ? number :
    F extends "int16" ? number :
    F extends "uint16" ? number :
    F extends "int32" ? number :
    F extends "uint32" ? number :
    F extends "int64" ? bigint :
    F extends "uint64" ? bigint :
    F extends "int8[]" ? ArrayLike<number> :
    F extends "uint8[]" ? ArrayLike<number> :
    F extends "int16[]" ? ArrayLike<number> :
    F extends "uint16[]" ? ArrayLike<number> :
    F extends "int32[]" ? ArrayLike<number> :
    F extends "uint32[]" ? ArrayLike<number> :
    F extends "int64[]" ? ArrayLike<bigint> :
    F extends "uint64[]" ? ArrayLike<bigint> :
    F extends "char8[]" ? string :
    F extends "int8...0" ? ArrayLike<number> | Iterable<number> | Iterator<number> :
    F extends "uint8...0" ? ArrayLike<number> | Iterable<number> | Iterator<number> :
    F extends "char8...0" ? ArrayLike<string> | Iterable<string> | Iterator<string> :
    F extends string ? any :
    F extends readonly ["Array" | "array",
        infer V extends SerialMessageEncodingFormat | SerialMessageDecodingFormat]
        ? ArrayLike<SerialMessageEncodingValueOf<V>> :
    F extends readonly ["Tuple" | "tuple"] ? [] :
    F extends readonly ["Tuple" | "tuple",
        infer V extends SerialMessageEncodingFormat | SerialMessageDecodingFormat,
        ...infer R extends (SerialMessageEncodingFormat | SerialMessageDecodingFormat)[]]
        ? [SerialMessageEncodingValueOf<V>, ...SerialMessageEncodingValueOf<["Tuple", ...R]>] :
    F extends readonly ["Tuple" | "tuple",
        ...(infer V extends SerialMessageEncodingFormat | SerialMessageDecodingFormat)[]]
        ? SerialMessageEncodingValueOf<V>[] :
    F extends readonly ["Object" | "object" | "struct" | "class"] ? {} :
    F extends readonly ["Object" | "object" | "struct" | "class",
        [infer N extends string, infer V extends SerialMessageEncodingFormat | SerialMessageDecodingFormat],
        ...infer R extends [string, SerialMessageEncodingFormat | SerialMessageDecodingFormat][]]
        ? {
            [K in N | keyof SerialMessageEncodingValueOf<["Object", ...R]>]:
                string extends K ? SerialMessageEncodingValueOf<V> | SerialMessageEncodingValueOf<["Object", ...R]>[keyof SerialMessageEncodingValueOf<["Object", ...R] & string>] :
                | (K extends N ? SerialMessageEncodingValueOf<V> : never)
                | (K extends keyof SerialMessageEncodingValueOf<["Object", ...R]> ? SerialMessageEncodingValueOf<["Object", ...R]>[K] : never)
        } :
    F extends readonly ["Object" | "object" | "struct" | "class",
        ...infer R extends [string, SerialMessageEncodingFormat | SerialMessageDecodingFormat][],
        [infer N extends string, infer V extends SerialMessageEncodingFormat | SerialMessageDecodingFormat]]
        ? {
            [K in N | keyof SerialMessageEncodingValueOf<["Object", ...R]>]:
                string extends K ? SerialMessageEncodingValueOf<V> | SerialMessageEncodingValueOf<["Object", ...R]>[keyof SerialMessageEncodingValueOf<["Object", ...R] & string>] :
                | (K extends N ? SerialMessageEncodingValueOf<V> : never)
                | (K extends keyof SerialMessageEncodingValueOf<["Object", ...R]> ? SerialMessageEncodingValueOf<["Object", ...R]>[K] : never)
        } :
    F extends readonly ["Object" | "object" | "struct" | "class",
        ...[infer N extends string | number, infer V extends SerialMessageEncodingFormat | SerialMessageDecodingFormat][]]
        ? { [K in N]: SerialMessageEncodingValueOf<V> } :
    never

type SerialMessageDecodingResultOf<F extends SerialMessageEncodingFormat | SerialMessageDecodingFormat> =
    F extends SerialMessageDecodingCallback<infer V> ? V :
    F extends "Int8" ? number :
    F extends "Uint8" ? number :
    F extends "Int16" ? number :
    F extends "Uint16" ? number :
    F extends "Int32" ? number :
    F extends "Uint32" ? number :
    F extends "Int64" ? bigint :
    F extends "Uint64" ? bigint :
    F extends "Int8Array" ? Int8Array :
    F extends "Uint8Array" ? Uint8Array :
    F extends "Int16Array" ? Int16Array :
    F extends "Uint16Array" ? Uint16Array :
    F extends "Int32Array" ? Int32Array :
    F extends "Uint32Array" ? Uint32Array :
    F extends "Int64Array" ? BigInt64Array :
    F extends "Uint64Array" ? BigUint64Array :
    F extends "Char8Array" ? string :
    F extends "Int8NullTerminate" ? Iterable<number> :
    F extends "Uint8NullTerminate" ? Iterable<number> :
    F extends "Char8NullTerminate" ? Iterable<string> :
    F extends "int8" ? number :
    F extends "uint8" ? number :
    F extends "int16" ? number :
    F extends "uint16" ? number :
    F extends "int32" ? number :
    F extends "uint32" ? number :
    F extends "int64" ? bigint :
    F extends "uint64" ? bigint :
    F extends "int8[]" ? Int8Array :
    F extends "uint8[]" ? Uint8Array :
    F extends "int16[]" ? Int16Array :
    F extends "uint16[]" ? Uint16Array :
    F extends "int32[]" ? Int32Array :
    F extends "uint32[]" ? Uint32Array :
    F extends "int64[]" ? BigInt64Array :
    F extends "uint64[]" ? BigUint64Array :
    F extends "char8[]" ? string :
    F extends "int8...0" ? Iterable<number> :
    F extends "uint8...0" ? Iterable<number> :
    F extends "char8...0" ? Iterable<string> :
    F extends string ? any :
    F extends readonly ["Array" | "array",
        infer V extends SerialMessageEncodingFormat | SerialMessageDecodingFormat]
        ? SerialMessageDecodingResultOf<V>[] :
    F extends readonly ["Tuple" | "tuple"] ? [] :
    F extends readonly ["Tuple" | "tuple",
        infer V extends SerialMessageEncodingFormat | SerialMessageDecodingFormat,
        ...infer R extends (SerialMessageEncodingFormat | SerialMessageDecodingFormat)[]]
        ? [SerialMessageDecodingResultOf<V>, ...SerialMessageDecodingResultOf<["Tuple", ...R]>] :
    F extends readonly ["Tuple" | "tuple",
        ...(infer V extends SerialMessageEncodingFormat | SerialMessageDecodingFormat)[]]
        ? SerialMessageDecodingResultOf<V>[] :
    F extends readonly ["Object" | "object" | "struct" | "class"] ? {} :
    F extends readonly ["Object" | "object" | "struct" | "class",
        [infer N extends string, infer V extends SerialMessageEncodingFormat | SerialMessageDecodingFormat],
        ...infer R extends [string, SerialMessageEncodingFormat | SerialMessageDecodingFormat][]]
        ? {
            [K in N | keyof SerialMessageDecodingResultOf<["Object", ...R]>]:
                string extends K ? SerialMessageDecodingResultOf<V> | SerialMessageDecodingResultOf<["Object", ...R]>[keyof SerialMessageDecodingResultOf<["Object", ...R] & string>] :
                | (K extends N ? SerialMessageDecodingResultOf<V> : never)
                | (K extends keyof SerialMessageDecodingResultOf<["Object", ...R]> ? SerialMessageDecodingResultOf<["Object", ...R]>[K] : never)
        } :
    F extends readonly ["Object" | "object" | "struct" | "class",
        ...infer R extends [string, SerialMessageEncodingFormat | SerialMessageDecodingFormat][],
        [infer N extends string, infer V extends SerialMessageEncodingFormat | SerialMessageDecodingFormat]]
        ? {
            [K in N | keyof SerialMessageDecodingResultOf<["Object", ...R]>]:
                string extends K ? SerialMessageDecodingResultOf<V> | SerialMessageDecodingResultOf<["Object", ...R]>[keyof SerialMessageDecodingResultOf<["Object", ...R] & string>] :
                | (K extends N ? SerialMessageDecodingResultOf<V> : never)
                | (K extends keyof SerialMessageDecodingResultOf<["Object", ...R]> ? SerialMessageDecodingResultOf<["Object", ...R]>[K] : never)
        } :
    F extends readonly ["Object" | "object" | "struct" | "class",
        ...[infer N extends string | number, infer V extends SerialMessageEncodingFormat | SerialMessageDecodingFormat][]]
        ? { [K in N]: SerialMessageDecodingResultOf<V> } :
    never