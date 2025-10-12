import { ArduinoInterface } from "../../arduino/interface.js";
import { callLater, html } from "../../common.js";
import { newUIUID } from "../../common.js";
import { PinModeSwitch } from "../../elements/pin-mode-switch.js";
import { PinSwitch } from "../../elements/pin-switch.js";
/**
@import { UIUID } from "../../common.js"
*/

/**
@export @typedef {
{
    "disconnect": readonly [
        event:
        {
            readonly target: DevicePanel,
        }],
}
} DevicePanelEventMap
*/

const _devicePanelTemplate = html`
    <li class="new device-panel">
        <header>
            <div>
                <span class="device-name">
                    Device
                </span>
                <span
                    class="device-name editing"
                    style="display: none"
                    contenteditable="plaintext-only">
                    Device
                </span>
                <span
                    class="device-model">
                    (Connecting)
                </span>
            </div>
            <div>
                <button class="device-disconnect-button">
                    Disconnect
                </button>
                <label>
                    Edit
                    <input type="checkbox" class="device-edit-toggle">
                </label>
            </div>
        </header>
        <ul class="device-pins"></ul>
    </li>
`;

const _devicePinTemplate = html`
    <li class="new device-pin">
        <span class="device-pin-name"></span>
        <span class="device-pin-name editing" contenteditable></span>
        <pin-switch class="device-pin-control"></pin-switch>
        <pin-mode-switch class="device-pin-control editing"></pin-mode-switch>
    </li>
`;

// MARK: DevicePanel
/**
*/ export class DevicePanel
{
    /**
    @param {
    {
        readonly devicePanelContainer: Element | undefined?,
        readonly arduinoInterface: ArduinoInterface,
    }
    } options
    @public*/ constructor(options)
    {
        const
        {
            devicePanelContainer,
            arduinoInterface,
        }
        = options;

        if (!(devicePanelContainer instanceof Element))
            throw new TypeError(
                `Option's 'devicePanelContainer' must be an 'Element'.`);

        if (!(arduinoInterface instanceof ArduinoInterface))
            throw new TypeError(
                `Option's 'arduinoInterface' must be an 'ArduinoInterface'.`);

        /**
        @type {UIUID}
        @private*/ this._id = newUIUID();

        /**
        @type {
        {
            [K in keyof DevicePanelEventMap]?:
                ((this: unknown, ...args: DevicePanelEventMap[K]) => void)[]
        }
        }
        @private*/ this._listeners = {};

        devicePanelContainer.appendChild(
            _devicePanelTemplate.content.cloneNode(true));

        const newElement = devicePanelContainer.querySelector(
            "& > li.new.device-panel");
        if (!(newElement instanceof HTMLLIElement))
            throw new TypeError();

        newElement.classList.remove("new");

        newElement.id = this._id;
        // @ts-ignore: Contract.
        newElement.arduinoInterface = arduinoInterface;
        // @ts-ignore: Contract.
        newElement.handler = this;

        arduinoInterface.getConfig()
            .then((config) =>
            {
                this.queryDeviceModel().textContent = config.model;
                const pinContainer = this.queryPinContainer();
                for (const pin of config.pins)
                {
                    pinContainer.appendChild(
                        _devicePanelTemplate.content.cloneNode(true));

                    const newElement = pinContainer.querySelector(
                        "& > li.new.device-pin");
                    if (!(newElement instanceof HTMLLIElement))
                        throw new TypeError();

                    newElement.classList.remove("new");

                    newElement.setAttribute("pin-id", String(pin.id));
                }
            })
            .catch((error) =>
            {
                console.error(error);
                this.disconnect();
            });

        document.addEventListener("click", (e) =>
        {
            switch (true)
            {
                case this.isDisconnectButton(e.target):
                    this.disconnect();
                    break;
            }
        });
    }

    /**
    @public*/ disconnect()
    {
        const element = document.querySelector(`#${this._id}`);

        if (element !== null)
            element.remove();

        const arduinoInterface = this.queryArduinoInterface();

        arduinoInterface.release();
        arduinoInterface.port.close().catch(console.log).then(() =>
        {
            for (const listener of this._listeners["disconnect"] ?? [])
                callLater(
                    listener,
                    undefined,
                    {
                        target: this,
                    });
        });
    }

    /**
    @returns {ArduinoInterface}
    @public @readonly*/ queryArduinoInterface()
    {
        // @ts-ignore: Contract.
        const arduinoInterface = this.queryPanelElement().arduinoInterface;

        if (!(arduinoInterface instanceof ArduinoInterface))
            throw new TypeError(
                `Missing 'arduinoInterface' field of type 'ArduinoInterface' on device panel.`);

        return arduinoInterface;
    }

    /**
    @returns {HTMLDivElement}
    @public*/ queryPanelElement()
    {
        return DevicePanel.prototype._queryElementInPanel.call(
            this,
            "&",
            HTMLDivElement);
    }

    /**
    @param {unknown} element
    @returns {element is HTMLDivElement}
    @public*/ isPanelElement(element)
    {
        return DevicePanel.prototype._matchesElementInPanel.call(
            this,
            element,
            "& span.device-model",
            HTMLSpanElement);
    }

    /**
    @returns {HTMLSpanElement}
    @public*/ queryDeviceModel()
    {
        return DevicePanel.prototype._queryElementInPanel.call(
            this,
            "& span.device-model",
            HTMLSpanElement);
    }

    /**
    @param {unknown} element
    @returns {element is HTMLSpanElement}
    @public*/ isDeviceModel(element)
    {
        return DevicePanel.prototype._matchesElementInPanel.call(
            this,
            element,
            "& span.device-model",
            HTMLSpanElement);
    }

    /**
    @returns {HTMLSpanElement}
    @public*/ queryDeviceName()
    {
        return DevicePanel.prototype._queryElementInPanel.call(
            this,
            "& span.device-name:not(.editing)",
            HTMLSpanElement);
    }

    /**
    @param {unknown} element
    @returns {element is HTMLSpanElement}
    @public*/ isDeviceName(element)
    {
        return DevicePanel.prototype._matchesElementInPanel.call(
            this,
            element,
            "& span.device-name:not(.editing)",
            HTMLSpanElement);
    }

    /**
    @returns {HTMLSpanElement}
    @public*/ queryDeviceNameEditing()
    {
        return DevicePanel.prototype._queryElementInPanel.call(
            this,
            "& span.device-name.editing",
            HTMLSpanElement);
    }

    /**
    @param {unknown} element
    @returns {element is HTMLSpanElement}
    @public*/ isDeviceNameEditing(element)
    {
        return DevicePanel.prototype._matchesElementInPanel.call(
            this,
            element,
            "& span.device-name.editing",
            HTMLSpanElement);
    }

    /**
    @returns {HTMLButtonElement}
    @public*/ queryDisconnectButton()
    {
        return DevicePanel.prototype._queryElementInPanel.call(
            this,
            "& button.device-disconnect-button",
            HTMLButtonElement);
    }

    /**
    @param {unknown} element
    @returns {element is HTMLButtonElement}
    @public*/ isDisconnectButton(element)
    {
        return DevicePanel.prototype._matchesElementInPanel.call(
            this,
            element,
            "& button.device-disconnect-button",
            HTMLButtonElement);
    }

    /**
    @returns {HTMLInputElement}
    @public*/ queryEditToggle()
    {
        return DevicePanel.prototype._queryElementInPanel.call(
            this,
            "& button.device-edit-toggle",
            HTMLInputElement);
    }

    /**
    @param {unknown} element
    @returns {element is HTMLInputElement}
    @public*/ isEditToggle(element)
    {
        return DevicePanel.prototype._matchesElementInPanel.call(
            this,
            element,
            "& button.device-edit-toggle",
            HTMLInputElement);
    }

    /**
    @returns {HTMLUListElement}
    @public*/ queryPinContainer()
    {
        return DevicePanel.prototype._queryElementInPanel.call(
            this,
            "& ul.device-pins",
            HTMLUListElement);
    }

    /**
    @param {unknown} element
    @returns {element is HTMLUListElement}
    @public*/ isPinContainer(element)
    {
        return DevicePanel.prototype._matchesElementInPanel.call(
            this,
            element,
            "& ul.device-pins",
            HTMLUListElement);
    }

    /**
    @param {number} pinId
    @returns {HTMLLIElement}
    @public*/ queryPin(pinId)
    {
        return DevicePanel.prototype._queryElementInPanel.call(
            this,
            `& ul.device-pins > li.device-pin[pin-id="${pinId}"]`,
            HTMLLIElement);
    }

    /**
    @param {number} pinId
    @param {unknown} element
    @returns {element is HTMLLIElement}
    @public*/ isPin(pinId, element)
    {
        return DevicePanel.prototype._matchesElementInPanel.call(
            this,
            element,
            `& ul.device-pins > li.device-pin[pin-id="${pinId}"]`,
            HTMLLIElement);
    }

    /**
    @param {number} pinId
    @returns {HTMLSpanElement}
    @public*/ queryPinName(pinId)
    {
        return DevicePanel.prototype._queryElementInPanel.call(
            this,
            `& ul.device-pins > li.device-pin[pin-id="${pinId}"] > span.device-pin-name:not(.editing)`,
            HTMLSpanElement);
    }

    /**
    @param {number} pinId
    @param {unknown} element
    @returns {element is HTMLSpanElement}
    @public*/ isPinName(pinId, element)
    {
        return DevicePanel.prototype._matchesElementInPanel.call(
            this,
            element,
            `& ul.device-pins > li.device-pin[pin-id="${pinId}"] > span.device-pin-name:not(.editing)`,
            HTMLSpanElement);
    }

    /**
    @param {number} pinId
    @returns {HTMLSpanElement}
    @public*/ queryPinNameEditing(pinId)
    {
        return DevicePanel.prototype._queryElementInPanel.call(
            this,
            `& ul.device-pins > li.device-pin[pin-id="${pinId}"] > span.device-pin-name.editing`,
            HTMLSpanElement);
    }

    /**
    @param {number} pinId
    @param {unknown} element
    @returns {element is HTMLSpanElement}
    @public*/ isPinNameEditing(pinId, element)
    {
        return DevicePanel.prototype._matchesElementInPanel.call(
            this,
            element,
            `& ul.device-pins > li.device-pin[pin-id="${pinId}"] > span.device-pin-name.editing`,
            HTMLSpanElement);
    }

    /**
    @param {number} pinId
    @returns {PinSwitch}
    @public*/ queryPinControl(pinId)
    {
        return DevicePanel.prototype._queryElementInPanel.call(
            this,
            `& ul.device-pins > li.device-pin[pin-id="${pinId}"] > pin-switch.device-pin-control:not(.editing)`,
            PinSwitch);
    }

    /**
    @param {number} pinId
    @param {unknown} element
    @returns {element is PinSwitch}
    @public*/ isPinControl(pinId, element)
    {
        return DevicePanel.prototype._matchesElementInPanel.call(
            this,
            element,
            `& ul.device-pins > li.device-pin[pin-id="${pinId}"] > pin-switch.device-pin-control:not(.editing)`,
            PinSwitch);
    }

    /**
    @param {number} pinId
    @param {unknown} element
    @returns {element is PinModeSwitch}
    @public*/ isPinControlEditing(pinId, element)
    {
        return DevicePanel.prototype._matchesElementInPanel.call(
            this,
            element,
            `& ul.device-pins > li.device-pin[pin-id="${pinId}"] > pin-mode-switch.device-pin-control.editing`,
            PinModeSwitch);
    }

    /**
    @param {unknown} element
    @returns {number?}
    @public*/ idOfPin(element)
    {
        if (!(element instanceof Element))
            throw new TypeError(
                `'element' must be an 'Element'.`);

        const id = Number(element.getAttribute("pin-id"));
        if (Number.isNaN(id))
            return null;

        return id;
    }

    /**
    @template {Element} E
    @param {string} selector
    @param {new (...args: any[]) => E} type
    @returns {E}
    @private*/ _queryElementInPanel(selector, type)
    {
        selector = String.prototype.replaceAll.call(
            selector,
            "&",
            `li#${this._id}.device-panel`);

        const element = document.querySelector(selector);

        if (!(element instanceof type))
            throw new TypeError(
                `Missing '${type.name}' in device panel at '${selector}'.`);

        return element;
    }

    /**
    @template {Element} [E = Element]
    @param {unknown} element
    @param {string} selector
    @param {new (...args: any[]) => E} [type]
    @returns {element is E}
    @private*/ _matchesElementInPanel(element, selector, type)
    {
        selector = String.prototype.replaceAll.call(
            selector,
            "&",
            `li#${this._id}.device-panel`);

        return element instanceof (type ?? Element)
            && Element.prototype.matches.call(element, selector);
    }

    /**
    @template {keyof DevicePanelEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: DevicePanelEventMap[K]) => void} listener
    @public*/ addEventListener(type, listener)
    {
        switch (type)
        {
            case "disconnect":
                /**
                @type {((...args: DevicePanelEventMap[K]) => void)[]}
                */ const listenerList = this._listeners[type] ??= [];
                listenerList.push(listener);
            default:
                throw new TypeError(
                    `'${type}' is not a valid event type.`);
        }
    }

    /**
    @template {keyof DevicePanelEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: DevicePanelEventMap[K]) => void} listener
    @public*/ removeEventListener(type, listener)
    {
        const listenerList = this._listeners[type];
        if (listenerList === undefined)
            return;

        const index = listenerList.indexOf(listener);
        if (index !== -1)
            listenerList.splice(index, 1);
    }
}