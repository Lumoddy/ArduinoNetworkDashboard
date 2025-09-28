import { ArduinoInterface } from "../arduino-interface.js";
import { html } from "../common.js";
import { PinModeSwitch, PinModeSwitchChangeEvent } from "./pin-mode-switch.js";
import { PinSwitch, PinSwitchChangeEvent } from "./pin-switch.js";
/**
@import { ArduinoConfig } from "../arduino-interface.js"
*/

/**
*/ const _template = html`
    <style>
        :host
        {

        }
    </style>
    <div class="header">
        <div class="title">
            <h1>Device</h1>
            <h2>(Missing)</h2>
        </div>
        <div class="buttons">
            <button class="disconnect">
                Disconnect
            </button>
            <label class="edit-pins-mode">
                Edit Pins<input type="checkbox">
            </label>
        </div>
    </div>
    <div class="pin-container"></div>
`;

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
        @type {ShadowRoot}
        @private*/ this._shadowRoot = this.attachShadow({ mode: "open" });
        this._shadowRoot.appendChild(_template.content.cloneNode(true));

        /**
        @type {WeakMap<
            Element,
            {
                onIsHighChanged?: (e: PinSwitchChangeEvent) => void,
                onModeChanged?: (e: PinModeSwitchChangeEvent) => void,
            }>
        }
        @private*/ this._pinData = new WeakMap();
    }

    /**
    @returns {boolean}
    @public*/ get editPinsMode()
    {
        return this._shadowRoot.querySelector(
            ":host > .header > label.edit-pins-mode > input:checked") !== null;
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
    @returns {
        readonly {
            readonly id: number,
            readonly name: string,
            readonly onIsHighChanged?: (e: PinSwitchChangeEvent) => void,
            readonly onModeChanged?: (e: PinModeSwitchChangeEvent) => void,
        }[]
    }
    @public*/ get pins()
    {
        return [...this._shadowRoot.querySelectorAll(":host > .pin-container > .pin")]
            .map((element) =>
            {
                const id = Number(element.getAttribute("pin-id") ?? NaN);
                if (Number.isNaN(id))
                    return undefined;

                const name = element.getAttribute("pin-name");
                if (name === null)
                    return undefined;

                const pinData = this._pinData.get(element) ?? {};

                return (
                {
                    id,
                    name,
                    onIsHighChanged: pinData.onIsHighChanged,
                    onModeChanged: pinData.onModeChanged,
                });
            })
            .filter((x) => x !== undefined);
    }
    /**
    @public*/ set pins(value)
    {
        let i = 0;

        const pinContainer = this._shadowRoot.querySelector(
            ":host > .pin-container");
        if (pinContainer === null)
            throw new TypeError("pinContainer === null"); 

        const existingPins = pinContainer.querySelectorAll(
            "> .pin");

        const minLength = Math.min(value.length, existingPins.length);

        for (; i < minLength; i++)
        {
            const existingPin = existingPins[i];
            const replacerPin = value[i];

            existingPin.setAttribute("pin-id", String(replacerPin.id));
            existingPin.setAttribute("pin-name", replacerPin.name);
            this._pinData.set(
                existingPin,
                {
                    onIsHighChanged: replacerPin.onIsHighChanged,
                    onModeChanged: replacerPin.onModeChanged,
                });
        }

        for (; i < existingPins.length; i++)
        {
            const existingPin = existingPins[i];
            this._pinData.delete(existingPin);
            existingPin.remove();
        }

        for (; i < value.length; i++)
        {
            const existingPin = existingPins[i];
            this._pinData.delete(existingPin);
            existingPin.remove();
        }
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
                const input = this._shadowRoot.querySelector(
                    ":host > .header > label.edit-pins-mode > input");

                const editPinsMode = newValue === "" || newValue === "true";

                if (input instanceof HTMLInputElement)
                    input.checked = editPinsMode;
            }
        }
    }
}
customElements.define("arduino-interface-panel", ArduinoInterfacePanel);