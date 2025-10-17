import { DevicePanelElement } from "./device-panel-element.js";
/**
@import { DevicePanelEventMap } from "./device-panel-element.js"
*/

// Console access.
export * from "./device-panel-element.js";

/**
@returns {HTMLDivElement?}
*/ export function queryDeviceListElement()
{
    return document.querySelector(`div#device-list`);
}

/**
@returns {HTMLDivElement}
*/ export function forceQueryDeviceListElement()
{
    const element = queryDeviceListElement();
    if (element === null)
        throw new TypeError(
            `Missing device list element.`);
    return element;
}

/**
@param {EventTarget?} element
@returns {element is HTMLDivElement}
*/ export function isDeviceListElement(element)
{
    return element instanceof Element
        && element.matches("div#device-list");
}

/**
@returns {HTMLButtonElement?}
*/ export function queryNewDeviceButton()
{
    return document.querySelector(`ul#device-list`);
}

/**
@returns {HTMLButtonElement}
*/ export function forceQueryNewDeviceButton()
{
    const element = queryNewDeviceButton();
    if (element === null)
        throw new TypeError(
            `Missing device list element.`);
    return element;
}

/**
@param {EventTarget?} element
@returns {element is HTMLButtonElement}
*/ export function isNewDeviceButton(element)
{
    return element instanceof Element
        && element.matches("button#new-device");
}

/**
*/ export class DeviceAddedEvent extends Event
{
    /**
    @param {
    {
        device: DevicePanelElement,
    }
    } detail
    @public*/ constructor(detail)
    {
        super("device-added");

        /**
        @type {DevicePanelElement}
        @private*/ this._device = detail.device;

        if (!(this._device instanceof DevicePanelElement))
            throw new TypeError();
    }

    /**
    @returns {DevicePanelElement}
    @public @override @readonly*/ get target()
    {
        const target = super.target;
        if (target instanceof DevicePanelElement)
            return target;
        throw new TypeError();
    }
}

/**
*/ export class DeviceRemovedEvent extends Event
{
    /**
    @param {
    {
        device: DevicePanelElement,
    }
    } detail
    @public*/ constructor(detail)
    {
        super("device-removed");

        /**
        @type {DevicePanelElement}
        @private*/ this._device = detail.device;

        if (!(this._device instanceof DevicePanelElement))
            throw new TypeError();
    }

    /**
    @returns {DevicePanelElement}
    @public @override @readonly*/ get target()
    {
        const target = super.target;
        if (target instanceof DevicePanelElement)
            return target;
        throw new TypeError();
    }
}

/**
@overload
@param {"device-added"} type
@param {(this: Document, e: DeviceAddedEvent) => void} listener
@param {AddEventListenerOptions | boolean} [options]
@returns {void}
*//**
@overload
@param {"device-removed"} type
@param {(this: Document, e: DeviceRemovedEvent) => void} listener
@param {AddEventListenerOptions | boolean} [options]
@returns {void}
*//**
@template {keyof DevicePanelEventMap} K
@overload
@param {K} type
@param {(this: Document, e: DevicePanelEventMap[K]) => void} listener
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
@template {keyof DevicePanelEventMap} K
@overload
@param {string} type
@param {(this: Document, e: DevicePanelEventMap[K]) => void} listener
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
        case isNewDeviceButton(e.target):
        {
            const device = forceQueryDeviceListElement().appendChild(
                document.createElement("device-panel"));
            if (!(device instanceof DevicePanelElement))
                throw new TypeError();

            dispatchEvent(new DeviceAddedEvent({ device }));

            device.requestAndAssignPort()
                .catch(() =>
                {
                    device.remove();
                });

            break;
        }
    }
});

addEventListener("device-disconnected", (e) =>
{
    e.target.remove();
    dispatchEvent(new DeviceRemovedEvent({ device: e.target }));
});