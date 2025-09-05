
export type BuiltInField =
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

export type EncodingCallback<V = any> = (value: V) => ArrayLike<Byte> | Iterator<Byte>

export type EncodingArray = readonly [type: "Array" | "[]", EncodingField]

export type EncodingTuple = readonly [type: "Tuple" | "()", ...EncodingField[]]

export type EncodingObject = readonly [type: "Object" | "class" | "struct" | "{}", ...[key: string, value: EncodingField][]]

export type EncodingField =
    | BuiltInField
    | EncodingArray
    | EncodingTuple
    | EncodingObject
    | EncodingCallback


export type EncodingFormat = EncodingField

export type DecodingCallback<V = any> = (nextByte: () => IteratorResult<Byte>) => V

export type DecodingArray = readonly [type: "Array" | "[]", DecodingField]

export type DecodingTuple = readonly [type: "Tuple" | "()", ...DecodingField[]]

export type DecodingObject = readonly [type: "Object" | "class" | "struct" | "{}", ...[key: string, value: DecodingField][]]

export type DecodingField =
    | BuiltInField
    | DecodingArray
    | DecodingTuple
    | DecodingObject
    | DecodingCallback

export type DecodingFormat = DecodingField

export type EncodingValueOfBuiltIn<F extends string> =
    F extends "Int8" | "signed char" | "i8" ? number | bigint :
    F extends "Byte" | "Uint8" | "unsigned char" | "u8" ? number | bigint :
    F extends "Int16" | "signed short" | "signed short int" | "i16" ? number | bigint :
    F extends "Uint16" | "unsigned short" | "unsigned short int" | "u16" ? number | bigint :
    F extends "Int32" | "signed int" | "i32" ? number | bigint :
    F extends "Uint32" | "unsigned int" | "u32" ? number | bigint :
    F extends "Int64" | "signed long" | "signed long int" | "i64" ? bigint :
    F extends "Uint64" | "unsigned long" | "unsigned long int" | "u64" ? bigint :
    F extends "Char8" | "char" | "c8" ? string :
    F extends "Int8Array" | "signed char[]" | "[i8]" ? ArrayLike<number | bigint> :
    F extends "ByteArray" | "Uint8Array" | "unsigned char[]" | "[u8]" ? ArrayLike<number | bigint> :
    F extends "Int16Array" | "signed short[]" | "signed short int[]" | "[i16]" ? ArrayLike<number | bigint> :
    F extends "Uint16Array" | "unsigned short[]" | "unsigned short int[]" | "[u16]" ? ArrayLike<number | bigint> :
    F extends "Int32Array" | "signed int[]" | "[i32]" ? ArrayLike<number | bigint> :
    F extends "Uint32Array" | "unsigned int[]" | "[u32]" ? ArrayLike<number | bigint> :
    F extends "BigInt64Array" | "signed long[]" | "signed long int[]" | "[i64]" ? ArrayLike<bigint> :
    F extends "BigUint64Array" | "unsigned long[]" | "unsigned long int[]" | "[u64]" ? ArrayLike<bigint> :
    F extends "Char8Array" | "char[]" | "[c8]" ? string :
    F extends string ? any :
    never

export type EncodingValueOfCallback<F extends EncodingCallback> =
    F extends EncodingCallback<infer V> ? V :
    never

export type EncodingValueOfTuple<F extends EncodingField[]> =
    F extends readonly [] ? [] :
    F extends readonly [
        infer V extends EncodingField,
        ...infer R extends readonly EncodingField[]]
        ? readonly [EncodingValueOf<V>, ...EncodingValueOfTuple<R>] :
    F extends readonly [
        ...infer R extends readonly EncodingField[],
        infer V extends EncodingField]
        ? readonly [...EncodingValueOfTuple<R>, EncodingValueOf<V>] :
    F extends readonly (infer V extends EncodingField)[]
        ? readonly EncodingValueOf<V>[] :
    never

export type EncodingValueOfObject<F extends [key: string, value: EncodingField][]> =
    F extends readonly [] ? {} :
    F extends readonly [
        readonly [infer N extends string, infer V extends EncodingField],
        ...infer R extends readonly (readonly [string, EncodingField])[]]
        ? {
            readonly [K in N | keyof EncodingValueOfObject<R>]:
                string extends K ? EncodingValueOf<V> | EncodingValueOfObject<R>[keyof EncodingValueOfObject<R>] :
                | (K extends N ? EncodingValueOf<V> : never)
                | (K extends keyof EncodingValueOfObject<R> ? EncodingValueOfObject<R>[K] : never)
        } :
    F extends readonly [
        ...infer R extends readonly (readonly [string, EncodingField])[],
        readonly [infer N extends string, infer V extends EncodingField]]
        ? {
            readonly [K in N | keyof EncodingValueOfObject<R>]:
                string extends K ? EncodingValueOf<V> | EncodingValueOfObject<R>[keyof EncodingValueOfObject<R>] :
                | (K extends N ? EncodingValueOf<V> : never)
                | (K extends keyof EncodingValueOfObject<R> ? EncodingValueOfObject<R>[K] : never)
        } :
    F extends readonly (readonly [infer N extends string, infer V extends EncodingField])[]
        ? { readonly [K in N]: EncodingValueOf<V> } :
    never

export type EncodingValueOf<F extends EncodingField> =
    F extends string ? EncodingValueOfBuiltIn<F> :
    F extends EncodingCallback ? EncodingValueOfCallback<F> :
    F extends readonly ["Array" | "[]", infer V extends EncodingField] ? ArrayLike<EncodingValueOf<V>> :
    F extends readonly ["Tuple" | "()", ...infer V extends EncodingField[]] ? EncodingValueOfTuple<V> :
    F extends readonly ["Object" | "class" | "struct" | "{}", ...infer V extends [string, EncodingField][]] ? EncodingValueOfObject<V> :
    never

export type DecodingValueOfBuiltIn<F extends string> =
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

export type DecodingValueOfCallback<F extends DecodingCallback> =
    F extends DecodingCallback<infer V> ? V :
    never

export type DecodingValueOfTuple<F extends DecodingField[]> =
    F extends readonly [] ? [] :
    F extends readonly [
        infer V extends DecodingField,
        ...infer R extends DecodingField[]]
        ? [DecodingValueOf<V>, ...DecodingValueOfTuple<R>] :
    F extends readonly [
        ...infer R extends DecodingField[],
        infer V extends DecodingField]
        ? [...DecodingValueOfTuple<R>, DecodingValueOf<V>] :
    F extends readonly (infer V extends DecodingField)[]
        ? DecodingValueOf<V>[] :
    never

export type DecodingValueOfObject<F extends [key: string, value: DecodingField][]> =
    F extends readonly [] ? {} :
    F extends readonly [
        [infer N extends string, infer V extends DecodingField],
        ...infer R extends [string, DecodingField][]]
        ? {
            [K in N | keyof DecodingValueOfObject<R>]:
                string extends K ? DecodingValueOf<V> | DecodingValueOfObject<R>[keyof DecodingValueOfObject<R>] :
                | (K extends N ? DecodingValueOf<V> : never)
                | (K extends keyof DecodingValueOfObject<R> ? DecodingValueOfObject<R>[K] : never)
        } :
    F extends readonly [
        ...infer R extends [string, DecodingField][],
        [infer N extends string, infer V extends DecodingField]]
        ? {
            [K in N | keyof DecodingValueOfObject<R>]:
                string extends K ? DecodingValueOf<V> | DecodingValueOfObject<R>[keyof DecodingValueOfObject<R>] :
                | (K extends N ? DecodingValueOf<V> : never)
                | (K extends keyof DecodingValueOfObject<R> ? DecodingValueOfObject<R>[K] : never)
        } :
    F extends readonly [infer N extends string, infer V extends DecodingField][]
        ? { [K in N]: DecodingValueOf<V> } :
    never

export type DecodingValueOf<F extends DecodingField> =
    F extends string ? DecodingValueOfBuiltIn<F> :
    F extends DecodingCallback ? DecodingValueOfCallback<F> :
    F extends readonly ["Array" | "[]", infer V extends DecodingField] ? DecodingValueOf<V>[] :
    F extends readonly ["Tuple" | "()", ...infer V extends DecodingField[]] ? DecodingValueOfTuple<V> :
    F extends readonly ["Object" | "class" | "struct" | "{}", ...infer V extends [string, DecodingField][]] ? DecodingValueOfObject<V> :
    never