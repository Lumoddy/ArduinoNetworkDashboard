
/**
@returns {HTMLButtonElement?}
*/ export function queryRecordClearButton()
{
    return document.querySelector(`button#record-clear`);
}

/**
@returns {HTMLButtonElement}
*/ export function forceQueryRecordClearButton()
{
    const element = queryRecordClearButton();
    if (element === null)
        throw new TypeError(
            `Missing clear button.`);
    return element;
}

/**
@param {EventTarget?} element
@returns {element is HTMLButtonElement}
*/ export function isRecordClearButton(element)
{
    return element instanceof HTMLButtonElement
        && Element.prototype.matches.call(
            element,
            "button#record-clear");
}

/**
@returns {HTMLDivElement?}
*/ export function queryRecordLogElement()
{
    return document.querySelector(`div#record-log`);
}

/**
@returns {HTMLDivElement}
*/ export function forceQueryRecordLogElement()
{
    const element = queryRecordLogElement();
    if (element === null)
        throw new TypeError(
            `Missing record log element.`);
    return element;
}

/**
@param {EventTarget?} element
@returns {element is HTMLDivElement}
*/ export function isRecordLogElement(element)
{
    return element instanceof HTMLDivElement
        && Element.prototype.matches.call(
            element,
            "div#record-log");
}

forceQueryRecordLogElement().append(localStorage.getItem("log") ?? "");

setInterval(() =>
    {
        const log = forceQueryRecordLogElement().textContent;

        if (log.length !== 0)
            localStorage.setItem("log", log);
    },
    5000)