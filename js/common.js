
/**
@param {TemplateStringsArray} template
@param {any[]} substitutions
@returns {HTMLTemplateElement}
*/ export function html(template, ...substitutions)
{
    const container = document.createElement("template");
    container.innerHTML = String.raw(template, ...substitutions);
    return container;
}

/**
@template {unknown} T
@template {any[]} A
@template {unknown} R
@param {(this: T, ...args: A) => R} closure
@param {T} thisArg
@param {A} args
@returns {R}
*/ export function call(closure, thisArg, ...args)
{
    return Function.prototype.call.call(closure, thisArg, ...args);
}

/**
@template {unknown} T
@template {any[]} A
@template {unknown} R
@param {(this: T, ...args: A) => R} closure
@param {T} thisArg
@param {A} args
@returns {Promise<R>}
*/ export function callLater(closure, thisArg, ...args)
{
    return new Promise((resolve, reject) => setTimeout(() => 
    {
        try { resolve(call(closure, thisArg, ...args)) }
        catch (error) { reject(error) }
    }));
}

/**
@template {unknown} T
@template {any[]} const A1
@template {any[]} const A2
@template {unknown} R
@param {(this: T, ...args: [...A1, ...A2]) => R} closure
@param {T} thisArg
@param {A1} args
@returns {(this: unknown, ...args: A2) => R}
*/ export function bind(closure, thisArg, ...args)
{
    return Function.prototype.bind.call(closure, thisArg, ...args);
}

/**
@type {(a: "1", b: "2", c: "3", d: "4", e: "5") => void}
*/ const a = () => {};

const b = bind(a, null, "1")

/**
@typedef {string} UIUID
*/

/**
@returns {UIUID}
*/ export function newUIUID()
{
    while (true)
    {
        const id = "js" + Math.floor(
            (Math.random() * (35 * 36 * 36 * 36)) + (36 * 36 * 36))
            .toString(36);

        if (document.getElementById(id) === null)
            return id;
    }
}