import * as DeviceManager from "./index/device-manager.js";
import * as AutomationManager from "./index/automation-manager.js";

if (navigator.serial === undefined)
    alert("This app is not supported on this browser.");

// @ts-expect-error: Console access.
window.DeviceManager = DeviceManager;
// @ts-expect-error: Console access.
window.AutomationManager = AutomationManager;