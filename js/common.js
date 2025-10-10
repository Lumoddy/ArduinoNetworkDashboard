
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