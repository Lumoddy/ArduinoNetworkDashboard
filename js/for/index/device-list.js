import { ArduinoInterface } from "../../arduino/interface.js";
import { PinModeSwitch } from "../../elements/pin-mode-switch.js";
import { PinSwitch } from "../../elements/pin-switch.js";

/**
@type {
{
    interface: ArduinoInterface,
    panel: HTMLDivElement,
    panelDisconnectButton: HTMLButtonElement,
    panelModelName: HTMLSpanElement,
    panelDeviceName: HTMLSpanElement,
    panelDeviceNameEditing: HTMLInputElement,
    panelPinContainer: HTMLSpanElement,
    panelPins:
    {
        container: HTMLDivElement,
        name: HTMLSpanElement,
        nameEditing: HTMLInputElement,
        pin: PinSwitch,
        pinEditing: PinModeSwitch,
    }[],
}[]
}
*/ const _devices = [];

const _list = (() =>
{
    const element = document.querySelector(
        "#device-list");
    if (!(element instanceof HTMLElement))
        throw new Error(
            "Missing element with id 'device-list'.");
    return element;
})();

const _editToggle = (() =>
{
    const element = document.querySelector(
        "input#device-edit[type=\"checkbox\"]");
    if (!(element instanceof HTMLInputElement))
        throw new Error(
            "Missing checkbox with id 'device-edit'.");
    return element;
})();

const _newDeviceButton = (() =>
{
    const element = document.querySelector(
        "button#new-device");
    if (!(element instanceof HTMLButtonElement))
        throw new Error(
            "Missing button with id 'new-device'.");
    return element;
})();

_editToggle.addEventListener("change", () => // MARK: Toggling Edit Mode
{
    if (_editToggle.checked)
    {
        for (const device of _devices)
        {
            device.panelDeviceName.style.display = "none";
            device.panelDeviceNameEditing.style.display = "";

            for (const pin of device.panelPins)
            {
                pin.name.style.display = "none";
                pin.nameEditing.style.display = "";

                pin.pin.style.display = "none";
                pin.pinEditing.style.display = "";
            }
        }
    }
    else
    {
        for (const device of _devices)
        {
            device.panelDeviceNameEditing.style.display = "none";
            device.panelDeviceName.style.display = "";

            for (const pin of device.panelPins)
            {
                pin.nameEditing.style.display = "none";
                pin.name.style.display = "";

                pin.pinEditing.style.display = "none";
                pin.pin.style.display = "";
            }
        }
    }
});

_newDeviceButton.addEventListener("click", async () => // MARK: New Device
{
    _newDeviceButton.disabled = true;

    try
    {
        const port = await navigator.serial.requestPort();

        await port.open({ baudRate: 9600 });

        const arduinoInterface = new ArduinoInterface(port);

        const panel = _list.appendChild(
            document.createElement("div"));
        panel.classList.add("device");

        // @ts-ignore: Console access.
        panel.arduinoInterface = arduinoInterface;

        const header = panel.appendChild(
            document.createElement("header"));

        const titles = header.appendChild(
            document.createElement("div"));

        const panelDeviceName = titles.appendChild(
            document.createElement("span"));
        panelDeviceName.classList.add("name");
        panelDeviceName.textContent = "Device";

        const panelDeviceNameEditing = titles.appendChild(
            document.createElement("input"));
        panelDeviceNameEditing.classList.add("name", "editing");
        panelDeviceNameEditing.type = "text";
        panelDeviceNameEditing.value = "Device";

        (_editToggle.checked
            ? panelDeviceName
            : panelDeviceNameEditing)
            .style.display = "none";

        const panelModelName = titles.appendChild(
            document.createElement("span"));
        panelModelName.classList.add("model");
        panelModelName.textContent = "(Connecting)";

        const panelDisconnectButton = header.appendChild(
            document.createElement("button"));
        panelDisconnectButton.textContent = "Disconnect";

        const panelPinContainer = _list.appendChild(
            document.createElement("div"));
        panel.classList.add("pins");

        panelDisconnectButton.addEventListener("click", () =>
        {
            arduinoInterface.release();
        });

        panelDeviceNameEditing.addEventListener("change", () =>
        {
            panelDeviceName.textContent = panelDeviceNameEditing.value;
        });

        /**
        @type {typeof _devices[number]["panelPins"]}
        */ const panelPins = [];

        _devices.push(
        {
            interface: arduinoInterface,
            panel,
            panelDisconnectButton,
            panelModelName,
            panelDeviceName,
            panelDeviceNameEditing,
            panelPinContainer,
            panelPins,
        });

        let config;
        try
        {
            config = await arduinoInterface.getConfig();
        }
        catch (error)
        {
            console.error(error);
            arduinoInterface.release();
            panel.remove();
            await arduinoInterface.port.close();
            return;
        }

        panelModelName.textContent = config.model;

        for (const configPin of config.pins)
        {
            const container = panelPinContainer.appendChild(
                document.createElement("div"));
            container.classList.add("pin");
            container.setAttribute("pin-id", String(configPin.id));

            const name = container.appendChild(
                document.createElement("span"));
            name.textContent = configPin.name;

            const nameEditing = container.appendChild(
                document.createElement("input"));
            nameEditing.type = "text";
            nameEditing.value = configPin.name;

            (_editToggle.checked
                ? name
                : nameEditing)
                .style.display = "none";

            const control = container.appendChild(
                document.createElement("pin-switch"));
            if (!(control instanceof PinSwitch))
                throw new TypeError(
                    "'pin-switch' is not registered.");

            const controlEditing = container.appendChild(
                document.createElement("pin-mode-switch"));
            if (!(controlEditing instanceof PinModeSwitch))
                throw new TypeError(
                    "'pin-mode-switch' is not registered.");

            (_editToggle.checked
                ? control
                : controlEditing)
                .style.display = "none";

            panelPins.push(
            {
                container,
                name,
                nameEditing,
                pin: control,
                pinEditing: controlEditing,
            });

            nameEditing.addEventListener("change", () =>
            {
                name.textContent = nameEditing.value;
            });

            const pinId = configPin.id;

            control.addEventListener("change", () =>
            {
                arduinoInterface.setPin(pinId, control.isHigh);
            });

            controlEditing.addEventListener("change", () =>
            {
                switch (control.mode = controlEditing.mode)
                {
                    case "output":
                        arduinoInterface.setPinMode(pinId, "digital-output");
                        break;
                    default:
                        arduinoInterface.setPinMode(pinId, "digital-input");
                        break;
                }
            });
        }

        arduinoInterface.addEventListener("release", () =>
        {
            panel.remove();
            arduinoInterface.port.close();
        });
    }
    finally
    {
        _newDeviceButton.disabled = false;
    }
});