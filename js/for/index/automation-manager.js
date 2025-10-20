import * as DeviceManager from "./device-manager.js";
import * as RecordManager from "./record-manager.js";
import { AutomationPanelElement } from "./automation-panel-element.js";
import { AutomationEntryElement } from "./automation-entry-element.js";
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
@returns {NodeListOf<AutomationPanelElement>}
*/ export function queryAutomationPanels()
{
    return document.querySelectorAll(
        `div#automation-list > automation-panel`);
}

/**
@param {EventTarget?} element
@returns {element is AutomationPanelElement}
*/ export function isAutomationPanel(element)
{
    return element instanceof AutomationPanelElement
        && Element.prototype.matches.call(
            element,
            `div#automation-list > automation-panel`);
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

addEventListener("click", (e) =>
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

addEventListener("automation-removed", (e) =>
{
    e.target.remove();
});

/**
@type {Map<
    AutomationPanelElement,
    {
        pinChangeListens:
        {
            deviceName: string,
            pinName: string,
            forHigh: boolean,
            glowElement: AutomationEntryElement,
        }[],
        pinHoldListens:
        {
            deviceName: string,
            pinName: string,
            forHigh: boolean,
            holdTime: number,
            holdHandle: ReturnType<typeof setInterval>?,
            holdResult: boolean?,
            glowElement: AutomationEntryElement,
        }[],
        intervalHandles:
        {
            intervalHandle: ReturnType<typeof setInterval>,
            glowElement: AutomationEntryElement,
        }[],
    }>}
*/ const automationListens = new Map();

/**
@type {{ [K in string]?: string }}
*/ const variables = {};

addEventListener("automation-active-change", (e) =>
{
    if (e.isActive)
    {
        /**
        @type {typeof automationListens extends WeakMap<any, infer T> ? T : never}
        */ const listens =
        {
            pinChangeListens: [],
            pinHoldListens: [],
            intervalHandles: [],
        };

        for (const entry of e.target.queryTriggerEntries())
        {
            const input = entry.forceQueryInputElements();
            switch (input?.type)
            {
                case "every-seconds":
                {
                    let seconds;
                    seconds = input.seconds.textContent;
                    seconds = Number(seconds);
                    seconds = seconds > 0.01 ? seconds : 0.01;

                    listens.intervalHandles.push(
                    {
                        intervalHandle: setInterval(
                            () =>
                            {
                                triggerAutomation(e.target);
                                entry.setAttribute("active", "");
                                requestAnimationFrame(() => entry.removeAttribute("active"));
                            },
                            seconds * 1000),
                        glowElement: entry,
                    });
                    break;
                }
                case "when-pin":
                {
                    listens.pinChangeListens.push(
                    {
                        deviceName: input.device.textContent,
                        pinName: input.pin.textContent,
                        forHigh: input.value.isHigh,
                        glowElement: entry,
                    });
                    break;
                }
                case "when-pin-for-seconds":
                {
                    let seconds;
                    seconds = input.seconds.textContent;
                    seconds = Number(seconds);
                    seconds = seconds > 0.01 ? seconds : 0.01;

                    listens.pinHoldListens.push(
                    {
                        deviceName: input.device.textContent,
                        pinName: input.pin.textContent,
                        forHigh: input.value.isHigh,
                        holdTime: seconds,
                        holdHandle: null,
                        holdResult: null,
                        glowElement: entry,
                    });
                    break;
                }
            }
        }

        automationListens.set(e.target, listens);
    }
    else
    {
        const listens = automationListens.get(e.target);
        if (listens !== undefined)
        {
            for (const listen of listens.intervalHandles)
                clearInterval(listen.intervalHandle);

            for (const listen of listens.pinHoldListens)
                if (listen.holdHandle !== null)
                    clearTimeout(listen.holdHandle);
        }
        automationListens.delete(e.target);
    }
});

DeviceManager.addEventListener("device-pin-change", (e) =>
{
    nextListen: for (const [automation, listens] of automationListens)
    {
        let listensForPin = false;

        for (const listen of listens.pinChangeListens)
        {
            if (listen.pinName !== e.target.forceQueryPinNameElement().textContent)
                continue nextListen;

            const device = e.target.closest("device-panel");
            if (!(device instanceof DeviceManager.DevicePanelElement))
                continue;

            if (listen.deviceName !== device.forceQueryDeviceNameElement().textContent)
                continue nextListen;

            if (e.target.forceQueryPinControl().isHigh !== listen.forHigh)
                continue nextListen;

            listensForPin = true;

            listen.glowElement.setAttribute("active", "");
            requestAnimationFrame(() => listen.glowElement.removeAttribute("active"));
        }

        if (!listensForPin)
            continue nextListen;

        triggerAutomation(automation);
    }

    for (const [automation, listens] of automationListens)
    {
        for (const listen of listens.pinHoldListens)
        {
            if (listen.pinName !== e.target.forceQueryPinNameElement().textContent)
                continue;

            const device = e.target.closest("device-panel");
            if (!(device instanceof DeviceManager.DevicePanelElement))
                continue;

            if (listen.deviceName !== device.forceQueryDeviceNameElement().textContent)
                continue;

            if (e.target.forceQueryPinControl().isHigh === listen.forHigh)
            {
                if (listen.holdHandle === null
                    && listen.holdResult !== listen.forHigh)
                {
                    listen.holdHandle = setTimeout(
                        () =>
                        {
                            listen.holdResult = listen.forHigh;

                            triggerAutomation(automation);
                            listen.glowElement.setAttribute("active", "");
                            requestAnimationFrame(() => listen.glowElement.removeAttribute("active"));
                        },
                        listen.holdTime * 1000);
                }
            }
            else if (listen.holdHandle !== null)
            {
                clearTimeout(listen.holdHandle);
                listen.holdHandle = null;
                listen.holdResult = null;
            }
        }
    }
});

/**
@param {AutomationPanelElement} automation
*/ export async function triggerAutomation(automation)
{
    for (const entry of automation.queryConditionEntries())
    {
        const input = entry.forceQueryInputElements();

        switch (input?.type)
        {
            case "get-pin":
            {
                const device = [...DeviceManager.queryDevicePanels()]
                    .find((device) =>
                    {
                        return device.forceQueryDeviceNameElement().textContent
                            === input.device.textContent;
                    });

                if (device === undefined)
                    return;

                const pin = [...device.queryPinElements()]
                    .find((pin) =>
                    {
                        return pin.forceQueryPinNameElement().textContent
                            === input.pin.textContent;
                    });

                if (pin === undefined)
                    return;

                if (pin.forceQueryPinControl().isHigh !== input.value.isHigh)
                    return;

                break;
            }
            case "get-variable":
            {
                if ((variables[input.name.textContent] ?? "") !== input.value.textContent)
                    return;

                break;
            }
        }
    }

    for (const entry of automation.queryActionEntries())
    {
        const input = entry.forceQueryInputElements();

        switch (input?.type)
        {
            case "set-pin":
            {
                const device = [...DeviceManager.queryDevicePanels()]
                    .find((device) =>
                    {
                        return device.forceQueryDeviceNameElement().textContent
                            === input.device.textContent;
                    });

                if (device === undefined)
                    break;

                const pin = [...device.queryPinElements()]
                    .find((pin) =>
                    {
                        return pin.forceQueryPinNameElement().textContent
                            === input.pin.textContent;
                    });

                if (pin === undefined)
                    break;

                const control = pin.forceQueryPinControl();
                const high = input.value.isHigh;
                if (control.mode === "output" && control.isHigh !== high)
                {
                    control.isHigh = high;
                    control.dispatchEvent(new Event(
                        "change",
                        {
                            bubbles: true,
                            cancelable: false,
                            composed: false,
                        }));
                }

                entry.setAttribute("active", "");
                requestAnimationFrame(() => entry.removeAttribute("active"));

                break;
            }
            case "set-variable":
            {
                variables[input.name.textContent] = input.value.textContent;

                entry.setAttribute("active", "");
                requestAnimationFrame(() => entry.removeAttribute("active"));

                break;
            }
            case "record-message":
            {
                const log = RecordManager.forceQueryRecordLogElement();

                const isScrolledToBottom
                    = log.scrollHeight - log.clientHeight
                    <= log.scrollTop + 1;

                log.append(
                    `[${new Date().toLocaleTimeString()}] ${input.message.textContent}\n`);

                if (isScrolledToBottom)
                    log.scrollTop = log.scrollHeight - log.clientHeight;

                entry.setAttribute("active", "");
                requestAnimationFrame(() => entry.removeAttribute("active"));

                break;
            }
            case "wait-seconds":
            {
                let seconds;
                seconds = input.seconds.textContent;
                seconds = Number(seconds);
                seconds = seconds > 0.01 ? seconds : 0.01;

                entry.setAttribute("active", "");
                requestAnimationFrame(() => entry.removeAttribute("active"));

                await new Promise((resolve) => setTimeout(resolve, seconds * 1000));

                break;
            }
            case "call-automation":
            {
                const automation = [...queryAutomationPanels()]
                    .find((automation) =>
                    {
                        return automation.forceQueryAutomationNameElement().textContent
                            === input.name.textContent;
                    });

                if (automation?.forceQueryActiveToggle().checked === true)
                    triggerAutomation(automation);

                entry.setAttribute("active", "");
                requestAnimationFrame(() => entry.removeAttribute("active"));

                break;
            }
        }
    }
}