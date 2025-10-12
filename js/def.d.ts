
declare type DeeplyReadonly<T> =
{
    readonly [P in keyof T]: DeeplyReadonly<T[P]>;
};

declare type Mutable<T> =
{
    -readonly [P in keyof T]: T[P];
};

declare type DeeplyMutable<T> =
{
    -readonly [P in keyof T]: DeeplyMutable<T[P]>;
};