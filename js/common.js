
/**
@param {TemplateStringsArray} template
@param {unknown[]} substitutions
@returns {HTMLTemplateElement}
*/ export function html(template, ...substitutions)
{
    const container = document.createElement("template");
    container.innerHTML = String.raw(template, ...substitutions);
    return container;
}

// /**
// @typedef {string} UIUID
// */

// /**
// @type {UIUID[]}
// */ const usedIds = [];

// /**
// @returns {UIUID}
// */ export function newUIUID()
// {
//     while (true)
//     {
//         const id = "js" + Math.floor(
//             (Math.random() * (35 * 36 * 36 * 36)) + (36 * 36 * 36))
//             .toString(36);

//         if (usedIds.indexOf(id) === -1)
//         {
//             usedIds.push(id);
//             return id;
//         }
//     }
// }

// /**
// @param {UIUID} id
// */ export function releaseUIUID(id)
// {
//     const index = usedIds.indexOf(id);
//     if (index === -1)
//         return;

//     usedIds.splice(index, 1);
// }