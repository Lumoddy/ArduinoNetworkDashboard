import { ArduinoInterface } from "../../arduino/interface.js";
import { callLater } from "../../common.js";
import { DevicePanel } from "./device-panel.js";

/**
@export @typedef {
{
    "device-added": readonly [
        event:
        {
            readonly target: DeviceManager,
            readonly panel: DevicePanel,
        }],
    "device-removed": readonly [
        event:
        {
            readonly target: DeviceManager,
            readonly panel: DevicePanel,
        }],
    "device-disconnected": readonly [
        event:
        {
            readonly target: DeviceManager,
            readonly panel: DevicePanel,
        }],
}
} DeviceManagerEventMap
*/

// MARK: DeviceManager
/**
*/ export class DeviceManager
{
    /**
    @param {
    {
        
    }
    } options
    @public*/ constructor(options)
    {
        /**
        @type {
        {
            [K in keyof DeviceManagerEventMap]?:
                ((this: unknown, ...args: DeviceManagerEventMap[K]) => void)[]
        }
        }
        @private*/ this._listeners = {};

        this.queryNewDeviceButton().addEventListener("click", async () =>
        {
            const port = await navigator.serial.requestPort();
            await port.open({ baudRate: 9600 });

            const panel = new DevicePanel(
            {
                devicePanelContainer: this.queryDevicePanelContainer(),
                arduinoInterface: new ArduinoInterface(port),
            });

            panel.addEventListener("disconnect", (e) =>
            {
                for (const listener of this._listeners["device-disconnected"] ?? [])
                    callLater(
                        listener,
                        undefined,
                        {
                            target: this,
                            panel: e.target,
                        });
            });

            for (const listener of this._listeners["device-added"] ?? [])
                callLater(
                    listener,
                    undefined,
                    {
                        target: this,
                        panel,
                    });
        });
    }

    /**
    @returns {DevicePanel[]}
    @public*/ queryDevices()
    {
        /**
        @type {DevicePanel[]}
        */ const devices = [];

        for (const element of document.querySelectorAll(
            "#device-list > li.device-panel"))
        {
            // @ts-ignore: Contract.
            const handler = element.handler;

            if (handler instanceof DevicePanel)
                devices.push(handler);
        }

        return devices;
    }

    /**
    @returns {HTMLButtonElement}
    @public*/ queryDevicePanelContainer()
    {
        return DeviceManager.prototype._queryElement.call(
            this,
            "#device-list",
            HTMLButtonElement);
    }

    /**
    @param {unknown} element
    @returns {element is HTMLButtonElement}
    @public*/ isDevicePanelContainer(element)
    {
        return DeviceManager.prototype._matchesElement.call(
            this,
            element,
            "#device-list",
            HTMLButtonElement);
    }

    /**
    @returns {HTMLButtonElement}
    @public*/ queryNewDeviceButton()
    {
        return DeviceManager.prototype._queryElement.call(
            this,
            "#new-device",
            HTMLButtonElement);
    }

    /**
    @param {unknown} element
    @returns {element is HTMLButtonElement}
    @public*/ isNewDeviceButton(element)
    {
        return DeviceManager.prototype._matchesElement.call(
            this,
            element,
            "#new-device",
            HTMLButtonElement);
    }

    /**
    @template {Element} E
    @param {string} selector
    @param {new (...args: any[]) => E} type
    @returns {E}
    @private*/ _queryElement(selector, type)
    {
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
    @private*/ _matchesElement(element, selector, type)
    {
        return element instanceof (type ?? Element)
            && Element.prototype.matches.call(element, selector);
    }

    /**
    @template {keyof DeviceManagerEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: DeviceManagerEventMap[K]) => void} listener
    @public*/ addEventListener(type, listener)
    {
        switch (type)
        {
            case "device-added":
            case "device-removed":
                /**
                @type {((...args: DeviceManagerEventMap[K]) => void)[]}
                */ const listenerList = this._listeners[type] ??= [];
                listenerList.push(listener);
            default:
                throw new TypeError(
                    `'${type}' is not a valid event type.`);
        }
    }

    /**
    @template {keyof DeviceManagerEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: DeviceManagerEventMap[K]) => void} listener
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