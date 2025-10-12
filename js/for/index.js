import { DeviceManager } from "./index/device-manager.js";

if (navigator.serial === undefined)
    alert("This app is not supported on this browser.");

const _deviceManager = new DeviceManager({});

// @ts-ignore: Console access.
window.deviceManager = _deviceManager;