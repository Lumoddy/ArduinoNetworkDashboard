import { ArduinoInterface } from "../arduino/interface.js";
import { ArduinoInterfacePanel, ArduinoInterfacePanelDisconnectEvent, ArduinoInterfacePanelPinChangeEvent, ArduinoInterfacePanelPinModeChangeEvent } from "../elements/arduino-interface-panel.js";

const _deviceListElement = /** @type {HTMLElement} */(
    document.getElementById("device-list"));

const _newDevice = /** @type {HTMLButtonElement} */(
    document.getElementById("new-device"));

_newDevice.addEventListener("click", async () =>
{
    const port = await navigator.serial.requestPort();

    if (port.readable === null)
        await port.open({ baudRate: 9600 });

    const arduinoInterface = new ArduinoInterface(port);

    const devicePanel = /** @type {ArduinoInterfacePanel} */(
        _deviceListElement.appendChild(document.createElement("arduino-interface-panel")));

    // @ts-ignore: allow use in debug console.
    devicePanel.arduinoInterface = arduinoInterface

    await new Promise((resolve) => setTimeout(resolve, 2000));

    const config = await arduinoInterface.getConfig();

    devicePanel.deviceModel = config.model;
    devicePanel.pins = config.pins.map((pin) => (
    {
        id: pin.id,
        name: pin.name,
        mode: "ignore",
        isHigh: false,
    }));

    devicePanel.addEventListener("change", async (e) =>
    {
        switch (true)
        {
            case e instanceof ArduinoInterfacePanelPinChangeEvent:
            {
                await arduinoInterface.setPin(e.pinId, e.isHigh);
                break;
            }
            case e instanceof ArduinoInterfacePanelPinModeChangeEvent:
            {
                await arduinoInterface.setPinMode(
                    e.pinId,
                    e.newMode === "output" ? "digital-output" : "digital-input");
                break;
            }
        }
    });

    devicePanel.addEventListener("disconnect", async (e) =>
    {
        switch (true)
        {
            case e instanceof ArduinoInterfacePanelDisconnectEvent:
            {
                arduinoInterface.release();
                await arduinoInterface.port.close();
                devicePanel.remove();
                break;
            }
        }
    })

    arduinoInterface.addEventListener("release", (e) =>
    {
        devicePanel.remove();
    });

    arduinoInterface.addEventListener("pin-change", (e) =>
    {
        devicePanel.setPin(e.pinId, e.pinIsHigh);
    });
});