import { PinModeSwitch, PinModeSwitchChangeEvent } from "../../elements/pin-mode-switch.js";
import { PinSwitchChangeEvent } from "../../elements/pin-switch.js";
/**
@import { PinMode, PinSwitch } from "../../elements/pin-switch.js"
*/

/**
@typedef {
{
    "device-pin-name-change": DevicePinNameChangeEvent,
    "device-pin-change": DevicePinChangeEvent,
    "device-pin-mode-change": DevicePinModeChangeEvent,
}
} DevicePinEventMap
*/

/**
*/ export class DevicePinNameChangeEvent extends Event
{
    /**
    @param {
        EventInit
        & {
            pinName: string,
        }
    } init
    @public*/ constructor(init)
    {
        super("device-pin-name-change", init);

        /**
        @type {string}
        @private*/ this._pinName = init.pinName;

        if (typeof this._pinName !== "string")
            throw new TypeError();
    }

    /**
    @returns {DevicePinElement}
    @public @override @readonly*/ get target()
    {
        const target = super.target;
        if (target instanceof DevicePinElement)
            return target;
        throw new TypeError();
    }

    /**
    @returns {string}
    @public @readonly*/ get pinName() { return this._pinName }
}

/**
*/ export class DevicePinChangeEvent extends Event
{
    /**
    @param {
        EventInit
        & {
            pinIsHigh: boolean,
        }
    } init
    @public*/ constructor(init)
    {
        super("device-pin-change", init);

        /**
        @type {boolean}
        @private*/ this._pinIsHigh = init.pinIsHigh;

        if (typeof this._pinIsHigh !== "boolean")
            throw new TypeError();
    }

    /**
    @returns {DevicePinElement}
    @public @override @readonly*/ get target()
    {
        const target = super.target;
        if (target instanceof DevicePinElement)
            return target;
        throw new TypeError();
    }

    /**
    @returns {boolean}
    @public @readonly*/ get pinIsHigh() { return this._pinIsHigh }
}

/**
*/ export class DevicePinModeChangeEvent extends Event
{
    /**
    @param {
        EventInit
        & {
            pinMode: PinMode,
        }
    } init
    @public*/ constructor(init)
    {
        super("device-pin-mode-change", init);

        /**
        @type {PinMode}
        @private*/ this._pinMode = init.pinMode;

        switch (this._pinMode)
        {
            case "ignore":
            case "input":
            case "output":
                break;
            default:
                throw new TypeError();
        }
    }

    /**
    @returns {DevicePinElement}
    @public @override @readonly*/ get target()
    {
        const target = super.target;
        if (target instanceof DevicePinElement)
            return target;
        throw new TypeError();
    }

    /**
    @returns {PinMode}
    @public @readonly*/ get pinMode() { return this._pinMode }
}

/**
*/ export class DevicePinElement extends HTMLElement
{
    /**
    @protected @readonly*/ static observedAttributes = /** @type {const} */([]);

    /**
    @public*/ constructor()
    {
        super();

        /**
        @type {boolean}
        @private*/ this._initialized = false;

        this.addEventListener("change", (e) =>
        {
            switch (true)
            {
                case e instanceof PinSwitchChangeEvent:
                    this.dispatchEvent(new DevicePinChangeEvent(
                    {
                        pinIsHigh: e.isHigh,
                        bubbles: true,
                        cancelable: false,
                        composed: false,
                    }));
                    break;
                case e instanceof PinModeSwitchChangeEvent:
                    this.forceQueryPinControl().mode = e.newMode;
                    this.dispatchEvent(new DevicePinModeChangeEvent(
                    {
                        pinMode: e.newMode,
                        bubbles: true,
                        cancelable: false,
                        composed: false,
                    }));
                    break;
            }
        });

        this.addEventListener("input", (e) =>
        {
            switch (true)
            {
                case this.isPinNameEditingElement(e.target):
                    this.forceQueryPinNameElement().textContent = e.target.textContent;
                    this.dispatchEvent(new DevicePinNameChangeEvent(
                    {
                        pinName: e.target.textContent,
                        bubbles: true,
                        cancelable: false,
                        composed: false,
                    }));
                    break;
            }
        });
    }

    /**
    @protected*/ connectedCallback()
    {
        if (this._initialized)
            return;

        this._initialized = true;

        this.innerHTML = /*html*/`
            <span
                class="device-pin-name"
                style="display: none"></span>
            <span
                class="device-pin-name editing"
                contenteditable></span>
            <span
                class="device-pin-model">?</span>
            <pin-switch
                class="device-pin-control"
                style="display: none"></pin-switch>
            <pin-mode-switch
                class="device-pin-control editing"></pin-mode-switch>
        `;
    }

    /**
    @returns {number?}
    @public*/ queryPinId()
    {
        const value = Number(this.getAttribute("pin-id"));
        if (Number.isNaN(value))
            return null;
        return value;
    }

    /**
    @returns {number}
    @public*/ forceQueryPinId()
    {
        const value = DevicePinElement.prototype.queryPinId
            .call(this);
        if (value === null)
            throw new TypeError(
                `Missing pin id.`);
        return value;
    }

    /**
    @param {number} pinId
    @public*/ setPinId(pinId)
    {
        this.setAttribute("pin-id", String(pinId));
    }

    /**
    @param {string} pinName
    @public*/ setPinModel(pinName)
    {
        this.forceQueryPinModelElement().textContent = pinName;
    }

    /**
    @param {string} pinName
    @public*/ setPinName(pinName)
    {
        this.forceQueryPinNameElement().textContent = pinName;
        this.forceQueryPinNameEditingElement().textContent = pinName;
    }

    /**
    @param {boolean} isHigh
    @public*/ setPin(isHigh)
    {
        this.forceQueryPinControl().isHigh = isHigh;
    }

    /**
    @returns {boolean}
    @public*/ getPin()
    {
        return this.forceQueryPinControl().isHigh;
    }

    /**
    @param {PinMode} mode
    @public*/ setPinMode(mode)
    {
        this.forceQueryPinEditingControl().mode = mode;
    }

    /**
    @returns {PinMode}
    @public*/ getPinMode()
    {
        return this.forceQueryPinEditingControl().mode;
    }

    /**
    @returns {HTMLSpanElement?}
    @public*/ queryPinModelElement()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& span.device-pin-model`);
    }

    /**
    @returns {HTMLSpanElement}
    @public*/ forceQueryPinModelElement()
    {
        const element = DevicePinElement.prototype.queryPinModelElement
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing pin model element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLSpanElement}
    @public*/ isPinModelElement(element)
    {
        return element instanceof Element
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `device-pin span.device-pin-model`);
    }

    /**
    @returns {HTMLSpanElement?}
    @public*/ queryPinNameElement()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& span.device-pin-name:not(.editing)`);
    }

    /**
    @returns {HTMLSpanElement}
    @public*/ forceQueryPinNameElement()
    {
        const element = DevicePinElement.prototype.queryPinNameElement
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing pin name element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLSpanElement}
    @public*/ isPinNameElement(element)
    {
        return element instanceof Element
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `device-pin span.device-pin-name:not(.editing)`);
    }

    /**
    @returns {HTMLSpanElement?}
    @public*/ queryPinNameEditingElement()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& span.device-pin-name.editing`);
    }

    /**
    @returns {HTMLSpanElement}
    @public*/ forceQueryPinNameEditingElement()
    {
        const element = DevicePinElement.prototype.queryPinNameEditingElement
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing pin name element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLSpanElement}
    @public*/ isPinNameEditingElement(element)
    {
        return element instanceof Element
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `device-pin span.device-pin-name.editing`);
    }

    /**
    @returns {PinSwitch?}
    @public*/ queryPinControl()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& pin-switch.device-pin-control:not(.editing)`);
    }

    /**
    @returns {PinSwitch}
    @public*/ forceQueryPinControl()
    {
        const element = DevicePinElement.prototype.queryPinControl
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing pin name element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is PinSwitch}
    @public*/ isPinControl(element)
    {
        return element instanceof Element
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `device-pin pin-switch.device-pin-control:not(.editing)`);
    }

    /**
    @returns {PinModeSwitch?}
    @public*/ queryPinEditingControl()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& pin-mode-switch.device-pin-control.editing`);
    }

    /**
    @returns {PinModeSwitch}
    @public*/ forceQueryPinEditingControl()
    {
        const element = DevicePinElement.prototype.queryPinEditingControl
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing pin name element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is PinModeSwitch}
    @public*/ isPinEditingControl(element)
    {
        return element instanceof Element
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `device-pin pin-mode-switch.device-pin-control.editing`);
    }

    /**
    @param {typeof DevicePinElement["observedAttributes"][number]} attributeName
    @param {string?} oldValue
    @param {string?} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        
    }

    /**
    @template {keyof DevicePinEventMap} K
    @overload
    @param {K} type
    @param {(this: Document, e: DevicePinEventMap[K]) => void} listener
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
    @template {keyof DevicePinEventMap} K
    @overload
    @param {string} type
    @param {(this: Document, e: DevicePinEventMap[K]) => void} listener
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
customElements.define("device-pin", DevicePinElement);