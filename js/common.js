
/**
@param {string} string
@returns {string[]}
*/ export function parseCommaSeparatedStrings(string)
{
    const regex = /\s*(?<v>(?:[^,\\]|\\[^])*?)\s*,|\s*(?<v>(?:[^,\\]|\\[^])+?)\s*$/gy;

    /**
    @type {string[]}
    */ const result = [];

    let v;
    while ((v = regex.exec(string)) !== null && (v = v.groups?.v) !== undefined)
        result.push(v.replace(/\\(.|$)/g, "$1"));

    return result;
}

/**
@param {Iterable<string>} strings
@returns {string}
*/ export function stringifyCommaSeparatedStrings(strings)
{
    const array = [...strings];
    if (array.length !== 0 && array[array.length - 1].length === 0)
        array[array.length - 1] = "\\";

    return array.map((x) => x.replace(/[,\s]/g, "\\$0")).join(", ");
}