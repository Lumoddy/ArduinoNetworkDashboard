import { AutomationPanelElement } from "./automation-panel-element.js";
/**
@import { AutomationPanelEventMap } from "./automation-panel-element.js"
*/

// Console access.
export * from "./automation-panel-element.js";

/**
@returns {HTMLDivElement?}
*/ export function queryAutomationListElement()
{
    return document.querySelector(`div#automation-list`);
}

/**
@returns {HTMLDivElement}
*/ export function forceQueryAutomationListElement()
{
    const element = queryAutomationListElement();
    if (element === null)
        throw new TypeError(
            `Missing automation list element.`);
    return element;
}

/**
@param {EventTarget?} element
@returns {element is HTMLDivElement}
*/ export function isAutomationListElement(element)
{
    return element instanceof Element
        && element.matches("div#automation-list");
}

/**
@returns {HTMLButtonElement?}
*/ export function queryNewAutomationButton()
{
    return document.querySelector(`ul#automation-list`);
}

/**
@returns {HTMLButtonElement}
*/ export function forceQueryNewAutomationButton()
{
    const element = queryNewAutomationButton();
    if (element === null)
        throw new TypeError(
            `Missing automation list element.`);
    return element;
}

/**
@param {EventTarget?} element
@returns {element is HTMLButtonElement}
*/ export function isNewAutomationButton(element)
{
    return element instanceof Element
        && element.matches("button#new-automation");
}

/**
*/ export class AutomationAddedEvent extends Event
{
    /**
    @param {
    {
        automation: AutomationPanelElement,
    }
    } detail
    @public*/ constructor(detail)
    {
        super("automation-added");

        /**
        @type {AutomationPanelElement}
        @private*/ this._automation = detail.automation;

        if (!(this._automation instanceof AutomationPanelElement))
            throw new TypeError();
    }

    /**
    @returns {AutomationPanelElement}
    @public @override @readonly*/ get target()
    {
        const target = super.target;
        if (target instanceof AutomationPanelElement)
            return target;
        throw new TypeError();
    }
}

/**
*/ export class AutomationRemovedEvent extends Event
{
    /**
    @param {
    {
        automation: AutomationPanelElement,
    }
    } detail
    @public*/ constructor(detail)
    {
        super("automation-removed");

        /**
        @type {AutomationPanelElement}
        @private*/ this._automation = detail.automation;

        if (!(this._automation instanceof AutomationPanelElement))
            throw new TypeError();
    }

    /**
    @returns {AutomationPanelElement}
    @public @override @readonly*/ get target()
    {
        const target = super.target;
        if (target instanceof AutomationPanelElement)
            return target;
        throw new TypeError();
    }
}

/**
@overload
@param {"automation-added"} type
@param {(this: Document, e: AutomationAddedEvent) => void} listener
@param {AddEventListenerOptions | boolean} [options]
@returns {void}
*//**
@overload
@param {"automation-removed"} type
@param {(this: Document, e: AutomationRemovedEvent) => void} listener
@param {AddEventListenerOptions | boolean} [options]
@returns {void}
*//**
@template {keyof AutomationPanelEventMap} K
@overload
@param {K} type
@param {(this: Document, e: AutomationPanelEventMap[K]) => void} listener
@param {AddEventListenerOptions | boolean} [options]
@returns {void}
*//**
@overload
@param {string} type
@param {EventListenerOrEventListenerObject} listener
@param {AddEventListenerOptions | boolean} [options]
@returns {void}
*//**
@param {string} type
@param {EventListenerOrEventListenerObject} listener
@param {AddEventListenerOptions | boolean} [options]
@returns {void}
*/ export function addEventListener(type, listener, options)
{
    return document.addEventListener(type, listener, options);
}

/**
@param {Event} event
@returns {boolean}
*/ export function dispatchEvent(event)
{
    return document.dispatchEvent(event);
}

/**
@template {keyof AutomationPanelEventMap} K
@overload
@param {string} type
@param {(this: Document, e: AutomationPanelEventMap[K]) => void} listener
@param {EventListenerOptions | boolean} [options]
@returns {void}
*//**
@overload
@param {string} type
@param {EventListenerOrEventListenerObject} listener
@param {EventListenerOptions | boolean} [options]
@returns {void}
*//**
@param {string} type
@param {EventListenerOrEventListenerObject} listener
@param {EventListenerOptions | boolean} [options]
@returns {void}
*/ export function removeEventListener(type, listener, options)
{
    return document.removeEventListener(type, listener, options);
}

document.addEventListener("click", (e) =>
{
    switch (true)
    {
        case isNewAutomationButton(e.target):
        {
            const automation = forceQueryAutomationListElement().appendChild(
                document.createElement("automation-panel"));
            if (!(automation instanceof AutomationPanelElement))
                throw new TypeError();

            dispatchEvent(new AutomationAddedEvent({ automation }));

            break;
        }
    }
});

addEventListener("automation-disconnected", (e) =>
{
    e.target.remove();
    dispatchEvent(new AutomationRemovedEvent({ automation: e.target }));
});