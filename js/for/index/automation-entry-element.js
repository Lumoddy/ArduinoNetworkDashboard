
/**
@typedef {
{
    "automation-change": AutomationChangeEvent,
}
} AutomationEntryEventMap
*/

import { PinSwitch } from "../../elements/pin-switch.js";

/**
*/ export class AutomationChangeEvent extends Event
{
    /**
    @param {
        EventInit
        & {}
    } init
    @public*/ constructor(init)
    {
        super("automation-entry-name-change", init);
    }

    /**
    @returns {AutomationEntryElement}
    @public @override @readonly*/ get target()
    {
        const target = super.target;
        if (target instanceof AutomationEntryElement)
            return target;
        throw new TypeError();
    }
}

/**
@typedef {
    | "every-seconds"
    | "when-pin"
    | "when-pin-for-seconds"
    | "get-pin"
    | "get-variable"
    | "set-pin"
    | "set-variable"
    | "record-message"
    | "wait-seconds"
    | "call-automation"
} AutomationEntryType
*/

/**
*/ export class AutomationEntryElement extends HTMLElement
{
    /**
    @protected @readonly*/ static observedAttributes = /** @type {const} */(
    [
        "type",
    ]);

    /**
    @public*/ constructor()
    {
        super();

        /**
        @type {boolean}
        @private*/ this._initialized = false;

        this.addEventListener("change", () =>
        {
            this.dispatchEvent(new AutomationChangeEvent(
            {
                bubbles: true,
                cancelable: false,
                composed: false,
            }));
        });

        this.addEventListener("input", () =>
        {
            this.dispatchEvent(new AutomationChangeEvent(
            {
                bubbles: true,
                cancelable: false,
                composed: false,
            }));
        });
    }

    /**
    @protected*/ connectedCallback()
    {
        if (this._initialized)
            return;

        this._initialized = true;
    }

    /**
    @returns {AutomationEntryType?}
    @public*/ get type()
    {
        const value = this.getAttribute("type");
        switch (value)
        {
            case "every-seconds":
            case "when-pin":
            case "when-pin-for-seconds":
            case "get-pin":
            case "get-variable":
            case "set-pin":
            case "set-variable":
            case "record-message":
            case "wait-seconds":
            case "call-automation":
                return value;
            default:
                return null;
        }
    }
    /**
    @public*/ set type(value)
    {
        if (value === null)
            this.removeAttribute("type");
        else
            this.setAttribute("type", value);
    }

    /**
    @returns {
        | {
            type: "every-seconds",
            seconds: HTMLSpanElement,
        }
        | {
            type: "when-pin",
            pin: HTMLSpanElement,
            device: HTMLSpanElement,
            value: PinSwitch,
        }
        | {
            type: "when-pin-for-seconds",
            pin: HTMLSpanElement,
            device: HTMLSpanElement,
            value: PinSwitch,
            seconds: HTMLSpanElement,
        }
        | {
            type: "get-pin",
            pin: HTMLSpanElement,
            device: HTMLSpanElement,
            value: PinSwitch,
        }
        | {
            type: "get-variable",
            name: HTMLSpanElement,
            value: HTMLSpanElement,
        }
        | {
            type: "set-pin",
            pin: HTMLSpanElement,
            device: HTMLSpanElement,
            value: PinSwitch,
        }
        | {
            type: "set-variable",
            name: HTMLSpanElement,
            value: HTMLSpanElement,
        }
        | {
            type: "record-message",
            message: HTMLSpanElement,
        }
        | {
            type: "wait-seconds",
            seconds: HTMLSpanElement,
        }
        | {
            type: "call-automation",
            name: HTMLSpanElement,
        }
        | null
    }
    @public*/ forceQueryInputElements()
    {
        switch (this.type)
        {
            case "every-seconds":
            {
                const seconds = this.querySelector("& span.seconds");

                if (!(seconds instanceof HTMLSpanElement))
                    throw new TypeError();

                return (
                {
                    type: "every-seconds",
                    seconds
                });
            }
            case "when-pin":
            {
                const device = this.querySelector("& span.device");

                if (!(device instanceof HTMLSpanElement))
                    throw new TypeError();

                const pin = this.querySelector("& span.pin");

                if (!(pin instanceof HTMLSpanElement))
                    throw new TypeError();

                const value = this.querySelector("& pin-switch.value");

                if (!(value instanceof PinSwitch))
                    throw new TypeError();

                return (
                {
                    type: "when-pin",
                    device,
                    pin,
                    value,
                });
            }
            case "when-pin-for-seconds":
            {
                const device = this.querySelector("& span.device");

                if (!(device instanceof HTMLSpanElement))
                    throw new TypeError();

                const pin = this.querySelector("& span.pin");

                if (!(pin instanceof HTMLSpanElement))
                    throw new TypeError();

                const value = this.querySelector("& pin-switch.value");

                if (!(value instanceof PinSwitch))
                    throw new TypeError();

                const seconds = this.querySelector("& span.seconds");

                if (!(seconds instanceof HTMLSpanElement))
                    throw new TypeError();

                return (
                {
                    type: "when-pin-for-seconds",
                    device,
                    pin,
                    value,
                    seconds,
                });
            }
            case "get-pin":
            {
                const device = this.querySelector("& span.device");

                if (!(device instanceof HTMLSpanElement))
                    throw new TypeError();

                const pin = this.querySelector("& span.pin");

                if (!(pin instanceof HTMLSpanElement))
                    throw new TypeError();

                const value = this.querySelector("& pin-switch.value");

                if (!(value instanceof PinSwitch))
                    throw new TypeError();

                return (
                {
                    type: "get-pin",
                    device,
                    pin,
                    value,
                });
            }
            case "get-variable":
            {
                const name = this.querySelector("& span.name");

                if (!(name instanceof HTMLSpanElement))
                    throw new TypeError();

                const value = this.querySelector("& span.value");

                if (!(value instanceof HTMLSpanElement))
                    throw new TypeError();

                return (
                {
                    type: "get-variable",
                    name,
                    value,
                });
            }
            case "set-pin":
            {
                const device = this.querySelector("& span.device");

                if (!(device instanceof HTMLSpanElement))
                    throw new TypeError();

                const pin = this.querySelector("& span.pin");

                if (!(pin instanceof HTMLSpanElement))
                    throw new TypeError();

                const value = this.querySelector("& pin-switch.value");

                if (!(value instanceof PinSwitch))
                    throw new TypeError();

                return (
                {
                    type: "set-pin",
                    device,
                    pin,
                    value,
                });
            }
            case "set-variable":
            {
                const name = this.querySelector("& span.name");

                if (!(name instanceof HTMLSpanElement))
                    throw new TypeError();

                const value = this.querySelector("& span.value");

                if (!(value instanceof HTMLSpanElement))
                    throw new TypeError();

                return (
                {
                    type: "set-variable",
                    name,
                    value,
                });
            }
            case "record-message":
            {
                const message = this.querySelector("& span.message");

                if (!(message instanceof HTMLSpanElement))
                    throw new TypeError();

                return (
                {
                    type: "record-message",
                    message,
                });
            }
            case "wait-seconds":
            {
                const seconds = this.querySelector("& span.seconds");

                if (!(seconds instanceof HTMLSpanElement))
                    throw new TypeError();

                return (
                {
                    type: "wait-seconds",
                    seconds,
                });
            }
            case "call-automation":
            {
                const name = this.querySelector("& span.name");

                if (!(name instanceof HTMLSpanElement))
                    throw new TypeError();

                return (
                {
                    type: "call-automation",
                    name,
                });
            }
            default:
                return null;
        }
    }

    /**
    @param {typeof AutomationEntryElement["observedAttributes"][number]} attributeName
    @param {string?} oldValue
    @param {string?} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        const buttons = /*html*/`
            <button
                class="automation-remove-button">
                <svg width="10" height="10" viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M1 1L9 9M1 9L5 5L9 1" stroke="white" stroke-width="2" stroke-linecap="round"/>
                </svg>
            </button>
            <button
                class="automation-up-button">
                <svg width="11" height="6" viewBox="0 0 11 6" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M1.5 5L5.5 1L9.5 5" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
            </button>
            <button
                class="automation-down-button">
                <svg width="10" height="6" viewBox="0 0 10 6" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M1 1L5 5L9 1" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
            </button>
        `;

        switch (attributeName)
        {
            case "type":
            {
                switch (/** @type {AutomationEntryType} */(newValue))
                {
                    case "every-seconds":
                        this.innerHTML = /*html*/`
                            <div>
                                Every
                                <span
                                    class="seconds"
                                    contenteditable="plaintext-only"></span>
                                seconds
                            </div>
                            ${buttons}
                        `;
                        break;
                    case "when-pin":
                        this.innerHTML = /*html*/`
                            <div>
                                When pin named
                                <span
                                    class="pin"
                                    contenteditable="plaintext-only"></span>
                                in device named
                                <span
                                    class="device"
                                    contenteditable="plaintext-only"></span>
                                changes to
                                <pin-switch
                                    class="value"
                                    pin-mode="output"></pin-switch>
                            </div>
                            ${buttons}
                        `;
                        break;
                    case "when-pin-for-seconds":
                        this.innerHTML = /*html*/`
                            <div>
                                When pin named
                                <span
                                    class="pin"
                                    contenteditable="plaintext-only"></span>
                                in device named
                                <span
                                    class="device"
                                    contenteditable="plaintext-only"></span>
                                is
                                <pin-switch
                                    class="value"
                                    pin-mode="output"></pin-switch>
                                for
                                <span
                                    class="seconds"
                                    contenteditable="plaintext-only"></span>
                                seconds
                            </div>
                            ${buttons}
                        `;
                        break;
                    case "get-pin":
                        this.innerHTML = /*html*/`
                            <div>
                                If pin named
                                <span
                                    class="pin"
                                    contenteditable="plaintext-only"></span>
                                in device named
                                <span
                                    class="device"
                                    contenteditable="plaintext-only"></span>
                                is
                                <pin-switch
                                    class="value"
                                    pin-mode="output"></pin-switch>
                            </div>
                            ${buttons}
                        `;
                        break;
                    case "set-pin":
                        this.innerHTML = /*html*/`
                            <div>
                                Set pin named
                                <span
                                    class="pin"
                                    contenteditable="plaintext-only"></span>
                                in device named
                                <span
                                    class="device"
                                    contenteditable="plaintext-only"></span>
                                to
                                <pin-switch
                                    class="value"
                                    pin-mode="output"></pin-switch>
                            </div>
                            ${buttons}
                        `;
                        break;
                    case "record-message":
                        this.innerHTML = /*html*/`
                            <div>
                                Record message:
                                <span
                                    class="message"
                                    contenteditable="plaintext-only"></span>
                            </div>
                            ${buttons}
                        `;
                        break;
                    case "wait-seconds":
                        this.innerHTML = /*html*/`
                            <div>
                                Wait for
                                <span
                                    class="seconds"
                                    contenteditable="plaintext-only"></span>
                                seconds
                            </div>
                            ${buttons}
                        `;
                        break;
                    default:
                        this.innerHTML = "";
                        break;
                }

                break;
            }
        }
    }

    /**
    @template {keyof AutomationEntryEventMap} K
    @overload
    @param {K} type
    @param {(this: Document, e: AutomationEntryEventMap[K]) => void} listener
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
    @public @override*/ addEventListener(type, listener, options)
    {
        return EventTarget.prototype.addEventListener
            .call(this, type, listener, options);
    }

    /**
    @param {Event} event
    @returns {boolean}
    @public @override*/ dispatchEvent(event)
    {
        return EventTarget.prototype.dispatchEvent
            .call(this, event);
    }

    /**
    @template {keyof AutomationEntryEventMap} K
    @overload
    @param {string} type
    @param {(this: Document, e: AutomationEntryEventMap[K]) => void} listener
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
    @public @override*/ removeEventListener(type, listener, options)
    {
        return EventTarget.prototype.removeEventListener
            .call(this, type, listener, options);
    }
}
customElements.define("automation-entry", AutomationEntryElement);