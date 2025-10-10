import { ArduinoInterface } from "../arduino/interface.js";
import { html } from "../common.js";
import { PinModeSwitch, PinModeSwitchChangeEvent } from "./pin-mode-switch.js";
import { ensureIsPinMode, PinSwitch, PinSwitchChangeEvent } from "./pin-switch.js";
/**
@import { ArduinoConfig } from "../arduino/interface.js"
@import { PinMode } from "./pin-switch.js"
*/

/**
*/ const _template = html`
    <div class="header">
        <div class="title">
            <h1>Device</h1>
            <h2>(Connecting)</h2>
        </div>
        <div class="buttons">
            <button class="disconnect">
                Disconnect
            </button>
            <label class="edit-pins-mode">
                <span>Edit Pins</span>
                <input type="checkbox">
            </label>
        </div>
    </div>
    <div class="pin-container"></div>
`;

/**
*/ export class ArduinoInterfacePanelPinChangeEvent extends Event
{
    /**
    @param {string} type
    @param {
        EventInit
        & {
            wasHigh: boolean,
            isHigh: boolean,
            pinId: number,
            pinName: string,
        }
    } init
    @public*/ constructor(type, init)
    {
        super(type, init);

        /**
        @type {boolean}
        @private*/ this._wasHigh = Boolean(init.wasHigh);

        /**
        @type {boolean}
        @private*/ this._isHigh = Boolean(init.isHigh);

        /**
        @type {number}
        @private*/ this._pinId = Number(init.pinId);

        /**
        @type {string}
        @private*/ this._pinName = String(init.pinName);
    }

    /**
    @returns {ArduinoInterfacePanel}
    @public @readonly @override*/ get target()
    {
        if (!(super.target instanceof ArduinoInterfacePanel))
            throw new TypeError(
                "ArduinoInterfacePanelPinChangeEvent can only be used on " +
                "ArduinoInterfacePanel.");

        return super.target;
    }

    /**
    @returns {boolean}
    @public @readonly*/ get wasHigh() { return this._wasHigh }

    /**
    @returns {boolean}
    @public @readonly*/ get isHigh() { return this._isHigh }

    /**
    @returns {number}
    @public @readonly*/ get pinId() { return this._pinId }

    /**
    @returns {string}
    @public @readonly*/ get pinName() { return this._pinName }
}

/**
*/ export class ArduinoInterfacePanelPinModeChangeEvent extends Event
{
    /**
    @param {string} type
    @param {
        EventInit
        & {
            oldMode: PinMode,
            newMode: PinMode,
            pinId: number,
            pinName: string,
        }
    } init
    @public*/ constructor(type, init)
    {
        super(type, init);

        /**
        @type {PinMode}
        @private*/ this._oldMode = ensureIsPinMode(init.oldMode);

        /**
        @type {PinMode}
        @private*/ this._newMode = ensureIsPinMode(init.newMode);

        /**
        @type {number}
        @private*/ this._pinId = Number(init.pinId);

        /**
        @type {string}
        @private*/ this._pinName = String(init.pinName);
    }

    /**
    @returns {ArduinoInterfacePanel}
    @public @readonly @override*/ get target()
    {
        if (!(super.target instanceof ArduinoInterfacePanel))
            throw new TypeError(
                "ArduinoInterfacePanelPinChangeEvent can only be used on " +
                "ArduinoInterfacePanel.");

        return super.target;
    }

    /**
    @returns {PinMode}
    @public @readonly*/ get oldMode() { return this._oldMode }

    /**
    @returns {PinMode}
    @public @readonly*/ get newMode() { return this._newMode }

    /**
    @returns {number}
    @public @readonly*/ get pinId() { return this._pinId }

    /**
    @returns {string}
    @public @readonly*/ get pinName() { return this._pinName }
}

/**
*/ export class ArduinoInterfacePanelDisconnectEvent extends Event
{
    /**
    @param {string} type
    @param {EventInit} init
    @public*/ constructor(type, init)
    {
        super(type, init);
    }

    /**
    @returns {ArduinoInterfacePanel}
    @public @readonly @override*/ get target()
    {
        if (!(super.target instanceof ArduinoInterfacePanel))
            throw new TypeError(
                "ArduinoInterfacePanelDisconnectEvent can only be used on " +
                "ArduinoInterfacePanel.");

        return super.target;
    }
}

/**
*/ export class ArduinoInterfacePanel extends HTMLElement
{
    /**
    @protected @readonly*/ static observedAttributes = /** @type {const} */(
    [
        "edit-pins-mode",
    ]);

    /**
    @public*/ constructor()
    {
        super();

        /**
        @type {boolean}
        @private*/ this._createdContents = false;
    }

    /**
    @protected*/ connectedCallback()
    {
        if (this._createdContents)
            return;

        this._createdContents = true;

        this.appendChild(_template.content.cloneNode(true));

        this.addEventListener("click", (e) =>
        {
            switch (true)
            {
                case e.target instanceof HTMLButtonElement
                    && e.target.classList.contains("disconnect"):
                {
                    e.stopPropagation();

                    this.dispatchEvent(new ArduinoInterfacePanelDisconnectEvent(
                        "disconnect",
                        {
                            bubbles: true,
                            cancelable: false,
                            composed: false,
                        }));

                    break;
                }
            }
        });

        this.addEventListener("change", (e) =>
        {
            switch (true)
            {
                case e.target instanceof HTMLInputElement
                    && e.target.parentElement instanceof HTMLElement
                    && e.target.parentElement.classList.contains("edit-pins-mode"):
                {
                    e.stopPropagation();

                    this.editPinsMode = e.target.checked;

                    break;
                }
                case e instanceof PinSwitchChangeEvent:
                {
                    e.stopPropagation();

                    const pinElement = e.target.parentElement;

                    const pinId = Number(pinElement?.getAttribute("pin-id") ?? NaN);
                    const pinName = pinElement?.getAttribute("pin-name");

                    if (Number.isNaN(pinId)
                        || typeof pinName !== "string")
                        break;

                    this.dispatchEvent(new ArduinoInterfacePanelPinChangeEvent(
                        "change",
                        {
                            wasHigh: e.wasHigh,
                            isHigh: e.isHigh,
                            pinId,
                            pinName,
                            bubbles: true,
                            cancelable: false,
                            composed: false,
                        }));

                    break;
                }
                case e instanceof PinModeSwitchChangeEvent:
                {
                    e.stopPropagation();

                    const pinElement = e.target.parentElement;

                    const pinId = Number(pinElement?.getAttribute("pin-id") ?? NaN);
                    const pinName = pinElement?.getAttribute("pin-name");

                    if (Number.isNaN(pinId)
                        || typeof pinName !== "string")
                        break;

                    this.dispatchEvent(new ArduinoInterfacePanelPinModeChangeEvent(
                        "change",
                        {
                            oldMode: e.oldMode,
                            newMode: e.newMode,
                            pinId,
                            pinName,
                            bubbles: true,
                            cancelable: false,
                            composed: false,
                        }));

                    break;
                }
            }
        });
    }

    /**
    @returns {boolean}
    @public*/ get editPinsMode()
    {
        return this.querySelector(
            "& > .header > .buttons > label.edit-pins-mode > input:checked") !== null;
    }
    /**
    @public*/ set editPinsMode(value)
    {
        if (value)
            this.setAttribute("edit-pins-mode", "");
        else
            this.removeAttribute("edit-pins-mode");
    }

    /**
    @returns {string}
    @public*/ get deviceName()
    {
        return this.querySelector(
            "& > .header > .title > h1")
            ?.textContent ?? "";
    }
    /**
    @public*/ set deviceName(value)
    {
        const element = this.querySelector(
            "& > .header > .title > h1");
        if (element)
            element.textContent = value;
    }

    /**
    @returns {string}
    @public*/ get deviceModel()
    {
        return this.querySelector(
            "& > .header > .title > h2")
            ?.textContent ?? "";
    }
    /**
    @public*/ set deviceModel(value)
    {
        const element = this.querySelector(
            "& > .header > .title > h2");
        if (element)
            element.textContent = value;
    }

    /**
    @returns {
        readonly {
            readonly id: number,
            readonly name: string,
            readonly mode: PinMode,
            readonly isHigh: boolean,
        }[]
    }
    @public*/ get pins()
    {
        return [...this.querySelectorAll(
            "& > .pin-container > .pin")]
            .map((element) =>
            {
                const id = Number(element.getAttribute("pin-id") ?? NaN);
                if (Number.isNaN(id))
                    return undefined;

                const name = element.getAttribute("pin-name");
                if (name === null)
                    return undefined;

                const controlElement = element.querySelector(
                    "& > :is(pin-switch, pin-mode-switch)")

                /**
                @type {PinMode}
                */ let mode = ensureIsPinMode(controlElement?.getAttribute("pin-mode"));

                let isHigh;
                switch (controlElement?.getAttribute("is-high"))
                {
                    case "":
                    case "true":
                        isHigh = true;
                        break;
                    default:
                        isHigh = false;
                        break;
                }

                return (
                {
                    id,
                    name,
                    mode,
                    isHigh,
                });
            })
            .filter((x) => x !== undefined);
    }
    /**
    @public*/ set pins(value)
    {
        let i = 0;

        const pinContainer = this.querySelector(
            "& > .pin-container");
        if (pinContainer === null)
            throw new TypeError("pinContainer === null");

        const existingPins = pinContainer.querySelectorAll(
            "& > .pin");

        for (let i = 0; i < existingPins.length; i++)
            existingPins[i].remove();

        const controlElementType = this.editPinsMode ? "pin-mode-switch" : "pin-switch";
        for (let i = 0; i < value.length; i++)
        {
            const replacerPin = value[i];

            const pinElement = pinContainer.appendChild(document.createElement("div"));
            pinElement.classList.add("pin");
            pinElement.setAttribute("pin-id", String(replacerPin.id));
            pinElement.setAttribute("pin-name", replacerPin.name);

            const nameElement = pinElement.appendChild(document.createElement("h3"));
            nameElement.textContent = replacerPin.name;

            const controlElement = pinElement.appendChild(document.createElement(controlElementType));
            controlElement.setAttribute("pin-mode", replacerPin.mode);
            if (replacerPin.isHigh)
                controlElement.setAttribute("is-high", "");
        }
    }

    /**
    @overload
    @param {string} pinName
    @returns {boolean}
    *//**
    @overload
    @param {number} pinId
    @returns {boolean}
    *//**
    @param {string | number} pin
    @returns {boolean}
    @public*/ getPin(pin)
    {
        const pinElement = this.querySelector(
            `& > .pin-container > .pin[${typeof pin === "number" ? "pin-id=" : "pin-name="}"${pin}"]`);

        if (!(pinElement instanceof HTMLElement))
            throw new Error(
                `Pin "${pin}" not found.`);

        const controlElement = pinElement.querySelector(
            "& > :is(pin-switch, pin-mode-switch)");

        if (!(controlElement instanceof HTMLElement))
            throw new Error(
                `Pin "${pin}" not found.`);

        switch (controlElement.getAttribute("is-high"))
        {
            case "":
            case "true":
                return true;
            default:
                return false;
        }
    }

    /**
    @overload
    @param {string} pinName
    @param {boolean} isHigh
    @returns {void}
    *//**
    @overload
    @param {number} pinId
    @param {boolean} isHigh
    @returns {void}
    *//**
    @param {string | number} pin
    @param {boolean} isHigh
    @public*/ setPin(pin, isHigh)
    {
        const pinElement = this.querySelector(
            `& > .pin-container > .pin[${typeof pin === "number" ? "pin-id=" : "pin-name="}"${pin}"]`);

        if (!(pinElement instanceof HTMLElement))
            throw new Error(
                `Pin "${pin}" not found.`);

        const controlElement = pinElement.querySelector(
            "& > :is(pin-switch, pin-mode-switch)");

        if (!(controlElement instanceof HTMLElement))
            throw new Error(
                `Pin "${pin}" not found.`);

        if (isHigh)
            controlElement.setAttribute("is-high", "");
        else
            controlElement.removeAttribute("is-high");
    }

    /**
    @overload
    @param {string} pinName
    @returns {"input" | "output"}
    *//**
    @overload
    @param {number} pinId
    @returns {"input" | "output"}
    *//**
    @param {string | number} pin
    @returns {string}
    @public*/ getPinMode(pin)
    {
        const pinElement = this.querySelector(
            `& > .pin-container > .pin[${typeof pin === "number" ? "pin-id=" : "pin-name="}"${pin}"]`);

        if (!(pinElement instanceof HTMLElement))
            throw new Error(
                `Pin "${pin}" not found.`);

        const controlElement = pinElement.querySelector(
            "& > :is(pin-switch, pin-mode-switch)");

        if (!(controlElement instanceof HTMLElement))
            throw new Error(
                `Pin "${pin}" not found.`);

        return controlElement.getAttribute("pin-mode") ?? "input";
    }

    /**
    @overload
    @param {string} pinName
    @param {"input" | "output"} mode
    @returns {void}
    *//**
    @overload
    @param {string} pinName
    @param {string} mode
    @returns {void}
    *//**
    @overload
    @param {number} pinId
    @param {"input" | "output"} mode
    @returns {void}
    *//**
    @overload
    @param {number} pinId
    @param {string} mode
    @returns {void}
    *//**
    @param {string | number} pin
    @param {string} mode
    @public*/ setPinMode(pin, mode)
    {
        const pinElement = this.querySelector(
            `& > .pin-container > .pin[${typeof pin === "number" ? "pin-id=" : "pin-name="}"${pin}"]`);

        if (!(pinElement instanceof HTMLElement))
            throw new Error(
                `Pin "${pin}" not found.`);

        const controlElement = pinElement.querySelector(
            "& > :is(pin-switch, pin-mode-switch)");

        if (!(controlElement instanceof HTMLElement))
            throw new Error(
                `Pin "${pin}" not found.`);

        controlElement.setAttribute("pin-mode", mode);
    }

    /**
    @param {typeof ArduinoInterfacePanel["observedAttributes"][number]} attributeName
    @param {string | null} oldValue
    @param {string | null} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        switch (attributeName)
        {
            case "edit-pins-mode":
            {
                const input = this.querySelector(
                    "& > .header > .buttons > label.edit-pins-mode > input");

                const editPinsMode = newValue === "" || newValue === "true";

                if (input instanceof HTMLInputElement)
                    input.checked = editPinsMode;

                this.pins = this.pins;
            }
        }
    }
}
customElements.define("arduino-interface-panel", ArduinoInterfacePanel);