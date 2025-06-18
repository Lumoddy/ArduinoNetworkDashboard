
type Byte = number

type SerialMessageBuiltInField =
    | "Int8" | "signed char" | "i8"
    | "Byte" | "Uint8" | "unsigned char" | "u8"
    | "Int16" | "signed short" | "signed short int" | "i16"
    | "Uint16" | "unsigned short" | "unsigned short int" | "u16"
    | "Int32" | "signed int" | "i32"
    | "Uint32" | "unsigned int" | "u32"
    | "Int64" | "signed long" | "signed long int" | "i64"
    | "Uint64" | "unsigned long" | "unsigned long int" | "u64"
    | "Char8" | "char" | "c8"
    | "Int8Array" | "signed char[]" | "[i8]"
    | "ByteArray" | "Uint8Array" | "unsigned char[]" | "[u8]"
    | "Int16Array" | "signed short[]" | "signed short int[]" | "[i16]"
    | "Uint16Array" | "unsigned short[]" | "unsigned short int[]" | "[u16]"
    | "Int32Array" | "signed int[]" | "[i32]"
    | "Uint32Array" | "unsigned int[]" | "[u32]"
    | "BigInt64Array" | "signed long[]" | "signed long int[]" | "[i64]"
    | "BigUint64Array" | "unsigned long[]" | "unsigned long int[]" | "[u64]"
    | "Char8Array" | "char[]" | "[c8]"

type SerialMessageEncodingCallback<V = any> = (value: V) => ArrayLike<Byte> | Iterator<Byte>

type SerialMessageEncodingArray = readonly [type: "Array" | "[]", SerialMessageEncodingField]

type SerialMessageEncodingTuple = readonly [type: "Tuple" | "()", ...SerialMessageEncodingField[]]

type SerialMessageEncodingObject = readonly [type: "Object" | "class" | "struct" | "{}", ...[key: string, value: SerialMessageEncodingField][]]

type SerialMessageEncodingField =
    | SerialMessageBuiltInField
    | SerialMessageEncodingArray
    | SerialMessageEncodingTuple
    | SerialMessageEncodingObject
    | SerialMessageEncodingCallback


type SerialMessageEncodingFormat = SerialMessageEncodingField

type SerialMessageDecodingCallback<V = any> = (nextByte: () => IteratorResult<Byte>) => V

type SerialMessageDecodingArray = readonly [type: "Array" | "[]", SerialMessageDecodingField]

type SerialMessageDecodingTuple = readonly [type: "Tuple" | "()", ...SerialMessageDecodingField[]]

type SerialMessageDecodingObject = readonly [type: "Object" | "class" | "struct" | "{}", ...[key: string, value: SerialMessageDecodingField][]]

type SerialMessageDecodingField =
    | SerialMessageBuiltInField
    | SerialMessageDecodingArray
    | SerialMessageDecodingTuple
    | SerialMessageDecodingObject
    | SerialMessageDecodingCallback

type SerialMessageDecodingFormat = SerialMessageDecodingField

type SerialMessageEncodingValueOfBuiltIn<F extends string> =
    F extends "Int8" | "signed char" | "i8" ? number :
    F extends "Byte" | "Uint8" | "unsigned char" | "u8" ? number :
    F extends "Int16" | "signed short" | "signed short int" | "i16" ? number :
    F extends "Uint16" | "unsigned short" | "unsigned short int" | "u16" ? number :
    F extends "Int32" | "signed int" | "i32" ? number :
    F extends "Uint32" | "unsigned int" | "u32" ? number :
    F extends "Int64" | "signed long" | "signed long int" | "i64" ? bigint :
    F extends "Uint64" | "unsigned long" | "unsigned long int" | "u64" ? bigint :
    F extends "Char8" | "char" | "c8" ? string :
    F extends "Int8Array" | "signed char[]" | "[i8]" ? ArrayLike<number> :
    F extends "ByteArray" | "Uint8Array" | "unsigned char[]" | "[u8]" ? ArrayLike<number> :
    F extends "Int16Array" | "signed short[]" | "signed short int[]" | "[i16]" ? ArrayLike<number> :
    F extends "Uint16Array" | "unsigned short[]" | "unsigned short int[]" | "[u16]" ? ArrayLike<number> :
    F extends "Int32Array" | "signed int[]" | "[i32]" ? ArrayLike<number> :
    F extends "Uint32Array" | "unsigned int[]" | "[u32]" ? ArrayLike<number> :
    F extends "BigInt64Array" | "signed long[]" | "signed long int[]" | "[i64]" ? ArrayLike<bigint> :
    F extends "BigUint64Array" | "unsigned long[]" | "unsigned long int[]" | "[u64]" ? ArrayLike<bigint> :
    F extends "Char8Array" | "char[]" | "[c8]" ? string :
    F extends string ? any :
    never

type SerialMessageEncodingValueOfCallback<F extends SerialMessageEncodingCallback> =
    F extends SerialMessageEncodingCallback<infer V> ? V :
    never

type SerialMessageEncodingArrayOf<F extends SerialMessageEncodingCallback> =
    ArrayLike<SerialMessageEncodingValueOf<F>>

type SerialMessageEncodingValueOfTuple<F extends SerialMessageEncodingField[]> =
    F extends readonly [] ? [] :
    F extends readonly [
        infer V extends SerialMessageEncodingField,
        ...infer R extends SerialMessageEncodingField[]]
        ? [SerialMessageEncodingValueOf<V>, ...SerialMessageEncodingValueOfTuple<R>] :
    F extends readonly [
        ...infer R extends SerialMessageEncodingField[],
        infer V extends SerialMessageEncodingField]
        ? [...SerialMessageEncodingValueOfTuple<R>, SerialMessageEncodingValueOf<V>] :
    F extends readonly (infer V extends SerialMessageEncodingField)[]
        ? SerialMessageEncodingValueOf<V>[] :
    never

type SerialMessageEncodingValueOfObject<F extends [key: string, value: SerialMessageEncodingField][]> =
    F extends readonly [] ? {} :
    F extends readonly [
        [infer N extends string, infer V extends SerialMessageEncodingField | SerialMessageDecodingField],
        ...infer R extends [string, SerialMessageEncodingField | SerialMessageDecodingField][]]
        ? {
            [K in N | keyof SerialMessageEncodingValueOf<R>]:
                string extends K ? SerialMessageEncodingValueOf<V> | SerialMessageEncodingValueOf<R>[keyof SerialMessageEncodingValueOf<R>] :
                | (K extends N ? SerialMessageEncodingValueOf<V> : never)
                | (K extends keyof SerialMessageEncodingValueOf<R> ? SerialMessageEncodingValueOf<R>[K] : never)
        } :
    F extends readonly [
        ...infer R extends [string, SerialMessageEncodingField | SerialMessageDecodingField][],
        [infer N extends string, infer V extends SerialMessageEncodingField | SerialMessageDecodingField]]
        ? {
            [K in N | keyof SerialMessageEncodingValueOf<R>]:
                string extends K ? SerialMessageEncodingValueOf<V> | SerialMessageEncodingValueOf<R>[keyof SerialMessageEncodingValueOf<R>] :
                | (K extends N ? SerialMessageEncodingValueOf<V> : never)
                | (K extends keyof SerialMessageEncodingValueOf<R> ? SerialMessageEncodingValueOf<R>[K] : never)
        } :
    F extends readonly [infer N extends string | number, infer V extends SerialMessageEncodingField | SerialMessageDecodingFormat][]
        ? { [K in N]: SerialMessageEncodingValueOf<V> } :
    never

type SerialMessageEncodingValueOf<F extends SerialMessageEncodingField> =
    F extends string ? SerialMessageEncodingValueOfBuiltIn<F> :
    F extends SerialMessageEncodingCallback ? SerialMessageEncodingValueOfCallback<F> :
    F extends readonly ["Array" | "[]", infer V extends SerialMessageEncodingField] ? SerialMessageEncodingArrayOf<V> :
    F extends readonly ["Tuple" | "()", ...infer V extends SerialMessageEncodingField[]] ? SerialMessageEncodingValueOfTuple<V> :
    F extends readonly ["Object" | "class" | "struct" | "{}", ...infer V extends [string, SerialMessageEncodingField][]] ? SerialMessageEncodingValueOfObject<V> :
    never

type SerialMessageDecodingValueOfBuiltIn<F extends string> =
    F extends "Int8" | "signed char" | "i8" ? number :
    F extends "Byte" | "Uint8" | "unsigned char" | "u8" ? number :
    F extends "Int16" | "signed short" | "signed short int" | "i16" ? number :
    F extends "Uint16" | "unsigned short" | "unsigned short int" | "u16" ? number :
    F extends "Int32" | "signed int" | "i32" ? number :
    F extends "Uint32" | "unsigned int" | "u32" ? number :
    F extends "Int64" | "signed long" | "signed long int" | "i64" ? bigint :
    F extends "Uint64" | "unsigned long" | "unsigned long int" | "u64" ? bigint :
    F extends "Char8" | "char" | "c8" ? string :
    F extends "Int8Array" | "signed char[]" | "[i8]" ? Int8Array :
    F extends "ByteArray" | "Uint8Array" | "unsigned char[]" | "[u8]" ? Uint8Array :
    F extends "Int16Array" | "signed short[]" | "signed short int[]" | "[i16]" ? Int16Array :
    F extends "Uint16Array" | "unsigned short[]" | "unsigned short int[]" | "[u16]" ? Uint16Array :
    F extends "Int32Array" | "signed int[]" | "[i32]" ? Int32Array :
    F extends "Uint32Array" | "unsigned int[]" | "[u32]" ? Uint32Array :
    F extends "BigInt64Array" | "signed long[]" | "signed long int[]" | "[i64]" ? BigInt64Array :
    F extends "BigUint64Array" | "unsigned long[]" | "unsigned long int[]" | "[u64]" ? BigUint64Array :
    F extends "Char8Array" | "char[]" | "[c8]" ? string :
    F extends string ? any :
    never

type SerialMessageDecodingValueOfCallback<F extends SerialMessageDecodingCallback> =
    F extends SerialMessageDecodingCallback<infer V> ? V :
    never

type SerialMessageDecodingArrayOf<F extends SerialMessageDecodingCallback> =
    ArrayLike<SerialMessageDecodingValueOf<F>>

type SerialMessageDecodingValueOfTuple<F extends SerialMessageDecodingField[]> =
    F extends readonly [] ? [] :
    F extends readonly [
        infer V extends SerialMessageDecodingField,
        ...infer R extends SerialMessageDecodingField[]]
        ? [SerialMessageDecodingValueOf<V>, ...SerialMessageDecodingValueOfTuple<R>] :
    F extends readonly [
        ...infer R extends SerialMessageDecodingField[],
        infer V extends SerialMessageDecodingField]
        ? [...SerialMessageDecodingValueOfTuple<R>, SerialMessageDecodingValueOf<V>] :
    F extends readonly (infer V extends SerialMessageDecodingField)[]
        ? SerialMessageDecodingValueOf<V>[] :
    never

type SerialMessageDecodingValueOfObject<F extends [key: string, value: SerialMessageDecodingField][]> =
    F extends readonly [] ? {} :
    F extends readonly [
        [infer N extends string, infer V extends SerialMessageDecodingField | SerialMessageDecodingField],
        ...infer R extends [string, SerialMessageDecodingField | SerialMessageDecodingField][]]
        ? {
            [K in N | keyof SerialMessageDecodingValueOf<R>]:
                string extends K ? SerialMessageDecodingValueOf<V> | SerialMessageDecodingValueOf<R>[keyof SerialMessageDecodingValueOf<R>] :
                | (K extends N ? SerialMessageDecodingValueOf<V> : never)
                | (K extends keyof SerialMessageDecodingValueOf<R> ? SerialMessageDecodingValueOf<R>[K] : never)
        } :
    F extends readonly [
        ...infer R extends [string, SerialMessageDecodingField | SerialMessageDecodingField][],
        [infer N extends string, infer V extends SerialMessageDecodingField | SerialMessageDecodingField]]
        ? {
            [K in N | keyof SerialMessageDecodingValueOf<R>]:
                string extends K ? SerialMessageDecodingValueOf<V> | SerialMessageDecodingValueOf<R>[keyof SerialMessageDecodingValueOf<R>] :
                | (K extends N ? SerialMessageDecodingValueOf<V> : never)
                | (K extends keyof SerialMessageDecodingValueOf<R> ? SerialMessageDecodingValueOf<R>[K] : never)
        } :
    F extends readonly [infer N extends string | number, infer V extends SerialMessageDecodingField | SerialMessageDecodingFormat][]
        ? { [K in N]: SerialMessageDecodingValueOf<V> } :
    never

type SerialMessageDecodingValueOf<F extends SerialMessageDecodingField> =
    F extends string ? SerialMessageDecodingValueOfBuiltIn<F> :
    F extends SerialMessageDecodingCallback ? SerialMessageDecodingValueOfCallback<F> :
    F extends readonly ["Array" | "[]", infer V extends SerialMessageDecodingField] ? SerialMessageDecodingArrayOf<V> :
    F extends readonly ["Tuple" | "()", ...infer V extends SerialMessageDecodingField[]] ? SerialMessageDecodingValueOfTuple<V> :
    F extends readonly ["Object" | "class" | "struct" | "{}", ...infer V extends [string, SerialMessageDecodingField][]] ? SerialMessageDecodingValueOfObject<V> :
    never