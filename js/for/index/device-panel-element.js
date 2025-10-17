import { ArduinoInterface } from "../../arduino/interface.js";
import { DevicePinElement } from "./device-pin-element.js";
/**
@import { DevicePinEventMap } from "./device-pin-element.js"
*/

/**
@typedef {
DevicePinEventMap
& {
    "device-name-change": DeviceNameChangeEvent,
    "device-disconnected": DeviceRemovedEvent,
}
} DevicePanelEventMap
*/

/**
*/ export class DeviceNameChangeEvent extends Event
{
    /**
    @param {
        EventInit
        & {
            deviceName: string,
        }
    } init
    @public*/ constructor(init)
    {
        super("device-name-change", init);

        /**
        @type {string}
        @private*/ this._deviceName = init.deviceName;

        if (typeof this._deviceName !== "string")
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

    /**
    @returns {string}
    @public @readonly*/ get deviceName() { return this._deviceName }
}

/**
*/ export class DeviceRemovedEvent extends Event
{
    /**
    @param {
        EventInit
        & {}
    } init
    @public*/ constructor(init)
    {
        super("device-removed", init);
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
*/ export class DevicePanelElement extends HTMLElement
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

        /**
        @type {ArduinoInterface?}
        @private*/ this._arduinoInterface = null;

        this.addEventListener("click", (e) =>
        {
            switch (true)
            {
                case this.isDisconnectButton(e.target):
                    this.disconnect();
                    break;
            }
        });

        this.addEventListener("change", (e) =>
        {
            switch (true)
            {
                case this.isEditToggle(e.target):
                    this.setEditMode(e.target.checked);
                    break;
            }
        });

        this.addEventListener("input", (e) =>
        {
            switch (true)
            {
                case this.isDeviceNameEditingElement(e.target):
                    this.forceQueryDeviceNameElement().textContent = e.target.textContent;
                    this.dispatchEvent(new DeviceNameChangeEvent(
                    {
                        deviceName: e.target.textContent,
                        bubbles: true,
                        cancelable: false,
                        composed: false,
                    }));
                    break;
            }
        });

        this.addEventListener("device-pin-change", (e) =>
        {
            if (this._arduinoInterface === null)
                return;

            this._arduinoInterface.setPin(
                e.target.forceQueryPinId(),
                e.pinIsHigh);
        });

        this.addEventListener("device-pin-mode-change", (e) =>
        {
            if (this._arduinoInterface === null)
                return;

            this._arduinoInterface.setPinMode(
                e.target.forceQueryPinId(),
                e.pinMode === "output" ? "digital-output" : "digital-input");
        });
    }

    /**
    @protected*/ connectedCallback()
    {
        if (this._initialized)
            return;

        this._initialized = true;

        this.innerHTML = /*html*/`
            <header>
                <div>
                    <span
                        class="device-name"
                        style="display: none">New Device</span>
                    <span
                        class="device-name editing"
                        contenteditable="plaintext-only">New Device</span>
                    <span
                        class="device-model">(Uninitialized)</span>
                </div>
                <div>
                    <label
                        class="show-after-connect"
                        style="display: none">
                        <span>Edit</span>
                        <input
                            type="checkbox"
                            class="device-edit-toggle"
                            checked>
                    </label>
                    <button
                        class="device-disconnect-button">Disconnect</button>
                </div>
            </header>
            <div class="device-pins"></div>
        `;
    }

    /**
    @returns {Promise<void>}
    @public*/ async requestAndAssignPort()
    {
        const port = await navigator.serial.requestPort();
        await port.open({ baudRate: 9600 });

        DevicePanelElement.prototype.assignArduinoInterface
            .call(this, new ArduinoInterface(port));
    }

    /**
    @param {ArduinoInterface} arduinoInterface
    @public*/ assignArduinoInterface(arduinoInterface)
    {
        if (this._arduinoInterface !== null)
            throw new Error(
                "Cannot reassign ArduinoInterface.");

        this._arduinoInterface = arduinoInterface;

        DevicePanelElement.prototype.forceQueryDeviceModelElement
            .call(this).textContent = "(Connecting)";

        this._arduinoInterface.addEventListener("release", () =>
        {
            EventTarget.prototype.dispatchEvent
                .call(this, new DeviceRemovedEvent(
                {
                    bubbles: true,
                    cancelable: false,
                    composed: false,
                }));
        });

        this._arduinoInterface.addEventListener("pin-change", (e) =>
        {
            for (const element of DevicePanelElement.prototype.queryPinElements
                .call(this))
                if (element.queryPinId() === e.pinId)
                    element.setPin(e.pinIsHigh);
        });

        this._arduinoInterface.getConfig().then((config) =>
        {
            const pinContainer = this.forceQueryPinContainer();

            for (const pin of config.pins)
            {
                const element = pinContainer.appendChild(
                    document.createElement("device-pin"));
                if (!(element instanceof DevicePinElement))
                    throw new TypeError();

                element.setPinId(pin.id);
                element.setPinModel(pin.name);
                element.setPinName(pin.name);
            }

            DevicePanelElement.prototype.forceQueryDeviceModelElement
                .call(this).textContent = config.model;

            for (const element of this.querySelectorAll("& .show-after-connect"))
                if (element instanceof HTMLElement)
                    element.style.display = "";
        });
    }

    /**
    @public*/ disconnect()
    {
        if (this._arduinoInterface?.released !== false)
            return;

        this._arduinoInterface.release();

        this.forceQueryDeviceModelElement().textContent = "(Disconnected)";
    }

    /**
    @param {boolean} editing
    @public*/ setEditMode(editing)
    {
        if (editing)
        {
            for (const element of Element.prototype.querySelectorAll.call(
                this,
                `& span.device-name.editing,` +
                `& button.device-disconnect-button,` +
                `& device-pin span.device-pin-name.editing,` +
                `& device-pin pin-mode-switch.device-pin-control.editing`))
                if (element instanceof HTMLElement)
                    element.style.display = "";

            for (const element of Element.prototype.querySelectorAll.call(
                this,
                `& span.device-name:not(.editing),` +
                `& device-pin span.device-pin-name:not(.editing),` +
                `& device-pin pin-switch.device-pin-control:not(.editing)`))
                if (element instanceof HTMLElement)
                    element.style.display = "none";
        }
        else
        {
            for (const element of Element.prototype.querySelectorAll.call(
                this,
                `& span.device-name.editing,` +
                `& button.device-disconnect-button,` +
                `& device-pin span.device-pin-name.editing,` +
                `& device-pin pin-mode-switch.device-pin-control.editing`))
                if (element instanceof HTMLElement)
                    element.style.display = "none";

            for (const element of Element.prototype.querySelectorAll.call(
                this,
                `& span.device-name:not(.editing),` +
                `& device-pin span.device-pin-name:not(.editing),` +
                `& device-pin pin-switch.device-pin-control:not(.editing)`))
                if (element instanceof HTMLElement)
                    element.style.display = "";
        }

        DevicePanelElement.prototype.forceQueryEditToggle
            .call(this).checked = editing;
    }

    /**
    @returns {ArduinoInterface?}
    @public*/ queryArduinoInterface()
    {
        return this._arduinoInterface;
    }

    /**
    @returns {ArduinoInterface}
    @public*/ forceQueryArduinoInterface()
    {
        const arduinoInterface = DevicePanelElement.prototype.queryArduinoInterface
            .call(this);
        if (arduinoInterface === null)
            throw new TypeError(
                `Missing device name element.`);
        return arduinoInterface;
    }

    /**
    @returns {boolean}
    @public*/ hasArduinoInterface()
    {
        return this._arduinoInterface !== null;
    }

    /**
    @returns {HTMLSpanElement?}
    @public*/ queryDeviceNameElement()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& span.device-name:not(.editing)`);
    }

    /**
    @returns {HTMLSpanElement}
    @public*/ forceQueryDeviceNameElement()
    {
        const element = DevicePanelElement.prototype.queryDeviceNameElement
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing device name element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLSpanElement}
    @public*/ isDeviceNameElement(element)
    {
        return element instanceof Element
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `device-panel span.device-name:not(.editing)`);
    }

    /**
    @returns {HTMLSpanElement?}
    @public*/ queryDeviceNameEditingElement()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& span.device-name.editing`);
    }

    /**
    @returns {HTMLSpanElement}
    @public*/ forceQueryDeviceNameEditingElement()
    {
        const element = DevicePanelElement.prototype.queryDeviceNameEditingElement
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing device name element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLSpanElement}
    @public*/ isDeviceNameEditingElement(element)
    {
        return element instanceof Element
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `device-panel span.device-name.editing`);
    }

    /**
    @returns {HTMLSpanElement?}
    @public*/ queryDeviceModelElement()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& span.device-model:not(.editing)`);
    }

    /**
    @returns {HTMLSpanElement}
    @public*/ forceQueryDeviceModelElement()
    {
        const element = DevicePanelElement.prototype.queryDeviceModelElement
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing device model element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLSpanElement}
    @public*/ isDeviceModelElement(element)
    {
        return element instanceof HTMLSpanElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `device-panel span.device-model:not(.editing)`);
    }

    /**
    @returns {HTMLDivElement?}
    @public*/ queryPinContainer()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& div.device-pins`);
    }

    /**
    @returns {HTMLDivElement}
    @public*/ forceQueryPinContainer()
    {
        const element = DevicePanelElement.prototype.queryPinContainer
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing pin container element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLDivElement}
    @public*/ isPinContainer(element)
    {
        return element instanceof HTMLDivElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `device-panel div.device-pins`);
    }

    /**
    @returns {HTMLInputElement?}
    @public*/ queryEditToggle()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& input.device-edit-toggle`);
    }

    /**
    @returns {HTMLInputElement}
    @public*/ forceQueryEditToggle()
    {
        const element = DevicePanelElement.prototype.queryEditToggle
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing edit toggle.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLInputElement}
    @public*/ isEditToggle(element)
    {
        return element instanceof HTMLInputElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `device-panel input.device-edit-toggle`);
    }

    /**
    @returns {HTMLInputElement?}
    @public*/ queryDisconnectButton()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& button.device-disconnect-button`);
    }

    /**
    @returns {HTMLInputElement}
    @public*/ forceQueryDisconnectButton()
    {
        const element = DevicePanelElement.prototype.queryDisconnectButton
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing disconnect button.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLInputElement}
    @public*/ isDisconnectButton(element)
    {
        return element instanceof HTMLInputElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `device-panel button.device-disconnect-button`);
    }

    /**
    @returns {NodeListOf<DevicePinElement>}
    @public*/ queryPinElements()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelectorAll.call(
            this,
            `& device-pin`);
    }

    /**
    @param {EventTarget?} element
    @returns {element is DevicePinElement}
    @public*/ isPinElement(element)
    {
        return element instanceof DevicePinElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `device-panel device-pin`);
    }

    /**
    @param {typeof DevicePanelElement["observedAttributes"][number]} attributeName
    @param {string?} oldValue
    @param {string?} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        
    }

    /**
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
    @public @override*/ removeEventListener(type, listener, options)
    {
        return EventTarget.prototype.removeEventListener
            .call(this, type, listener, options);
    }
}
customElements.define("device-panel", DevicePanelElement);