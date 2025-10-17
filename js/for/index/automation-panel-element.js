import { ArduinoInterface } from "../../arduino/interface.js";
import { AutomationEntryElement } from "./automation-entry-element.js";
/**
@import { AutomationEntryEventMap } from "./automation-entry-element.js"
*/

/**
@typedef {
AutomationEntryEventMap
& {
    "automation-name-change": AutomationNameChangeEvent,
    "automation-active-change": AutomationActiveChangeEvent,
    "automation-disconnected": AutomationRemovedEvent,
}
} AutomationPanelEventMap
*/

/**
*/ export class AutomationNameChangeEvent extends Event
{
    /**
    @param {
        EventInit
        & {
            automationName: string,
        }
    } init
    @public*/ constructor(init)
    {
        super("automation-name-change", init);

        /**
        @type {string}
        @private*/ this._automationName = init.automationName;

        if (typeof this._automationName !== "string")
            throw new TypeError();
    }

    /**
    @returns {AutomationPanelElement}
    @public @override @readonly*/ get target()
    {
        const target = super.target;
        if (target instanceof AutomationPanelElement)
            return target;
        throw new TypeError();
    }

    /**
    @returns {string}
    @public @readonly*/ get automationName() { return this._automationName }
}

/**
*/ export class AutomationActiveChangeEvent extends Event
{
    /**
    @param {
        EventInit
        & {
            isActive: boolean,
        }
    } init
    @public*/ constructor(init)
    {
        super("automation-active-change", init);

        /**
        @type {boolean}
        @private*/ this._isActive = init.isActive;

        if (typeof this._isActive !== "boolean")
            throw new TypeError();
    }

    /**
    @returns {AutomationPanelElement}
    @public @override @readonly*/ get target()
    {
        const target = super.target;
        if (target instanceof AutomationPanelElement)
            return target;
        throw new TypeError();
    }

    /**
    @returns {boolean}
    @public @readonly*/ get isActive() { return this._isActive }
}

/**
*/ export class AutomationRemovedEvent extends Event
{
    /**
    @param {
        EventInit
        & {}
    } init
    @public*/ constructor(init)
    {
        super("automation-removed", init);
    }

    /**
    @returns {AutomationPanelElement}
    @public @override @readonly*/ get target()
    {
        const target = super.target;
        if (target instanceof AutomationPanelElement)
            return target;
        throw new TypeError();
    }
}

/**
*/ export class AutomationPanelElement extends HTMLElement
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
                case this.isActiveToggle(e.target):
                    this.setActive(e.target.checked);
                    break;
                case this.isNewTriggerButton(e.target):
                {
                    if (e.target.value === "")
                        break;

                    this.forceQueryTriggerContainer().appendChild(
                        document.createElement("automation-entry"))
                        .setAttribute("type", e.target.value);
                    e.target.value = "";

                    break;
                }
                case this.isNewConditionButton(e.target):
                {
                    if (e.target.value === "")
                        break;

                    this.forceQueryConditionContainer().appendChild(
                        document.createElement("automation-entry"))
                        .setAttribute("type", e.target.value);
                    e.target.value = "";

                    break;
                }
                case this.isNewActionButton(e.target):
                {
                    if (e.target.value === "")
                        break;

                    this.forceQueryActionContainer().appendChild(
                        document.createElement("automation-entry"))
                        .setAttribute("type", e.target.value);
                    e.target.value = "";

                    break;
                }
            }
        });

        this.addEventListener("input", (e) =>
        {
            switch (true)
            {
                case this.isAutomationNameEditingElement(e.target):
                    this.forceQueryAutomationNameElement().textContent = e.target.textContent;
                    this.dispatchEvent(new AutomationNameChangeEvent(
                    {
                        automationName: e.target.textContent,
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
            <header>
                <div>
                    <span
                        class="automation-name"
                        style="display: none">New Automation</span>
                    <span
                        class="automation-name editing"
                        contenteditable="plaintext-only">New Automation</span>
                </div>
                <div>
                    <label>
                        <span>Edit</span>
                        <input
                            type="checkbox"
                            class="automation-edit-toggle"
                            checked>
                    </label>
                    <label>
                        <span>Active</span>
                        <input
                            type="checkbox"
                            class="automation-active-toggle"
                            disabled>
                    </label>
                    <button
                        class="automation-disconnect-button">Remove</button>
                </div>
            </header>
            <span>When any:</span>
            <div class="automation-triggers">
                <select
                    class="new-automation-trigger">
                    <option value="">Add Trigger</option>
                    <option value="every-seconds">Every Some Seconds</option>
                    <option value="when-pin">When Pin Changes</option>
                    <option value="when-pin-for-seconds">When Pin Stays</option>
                </select>
            </div>
            <span>And all:</span>
            <div class="automation-conditions">
                <select
                    class="new-automation-condition">
                    <option value="">Add Condition</option>
                    <option value="get-pin">If Pin</option>
                    <option value="get-variable">If Variable</option>
                </select>
            </div>
            <span>Then:</span>
            <div class="automation-actions">
                <select
                    class="new-automation-action">
                    <option value="">Add Action</option>
                    <option value="set-pin">Set Pin</option>
                    <option value="set-variable">Set Variable</option>
                    <option value="record-message">Record Message</option>
                    <option value="wait-seconds">Wait</option>
                    <option value="call-automation">Trigger Automation</option>
                </select>
            </div>
        `;
    }

    /**
    @public*/ disconnect()
    {
        EventTarget.prototype.dispatchEvent
            .call(this, new AutomationRemovedEvent(
            {
                bubbles: true,
                cancelable: false,
                composed: false,
            }));
    }

    /**
    @param {boolean} editing
    @public*/ setEditMode(editing)
    {
        const activeToggle = this.forceQueryActiveToggle();

        if (editing)
        {
            for (const element of Element.prototype.querySelectorAll.call(
                this,
                `& span.automation-name.editing,` +
                `& button.automation-disconnect-button,` +
                `& select.new-automation-trigger,` +
                `& select.new-automation-condition,` +
                `& select.new-automation-action,` +
                `& automation-entry span.automation-entry-name.editing,` +
                `& automation-entry entry-mode-switch.automation-entry-control.editing`))
                if (element instanceof HTMLElement)
                    element.style.display = "";

            for (const element of Element.prototype.querySelectorAll.call(
                this,
                `& span.automation-name:not(.editing),` +
                `& automation-entry span.automation-entry-name:not(.editing),` +
                `& automation-entry entry-switch.automation-entry-control:not(.editing)`))
                if (element instanceof HTMLElement)
                    element.style.display = "none";

            activeToggle.checked = false;
            activeToggle.disabled = true;
        }
        else
        {
            for (const element of Element.prototype.querySelectorAll.call(
                this,
                `& span.automation-name.editing,` +
                `& button.automation-disconnect-button,` +
                `& select.new-automation-trigger,` +
                `& select.new-automation-condition,` +
                `& select.new-automation-action,` +
                `& automation-entry span.automation-entry-name.editing,` +
                `& automation-entry entry-mode-switch.automation-entry-control.editing`))
                if (element instanceof HTMLElement)
                    element.style.display = "none";

            for (const element of Element.prototype.querySelectorAll.call(
                this,
                `& span.automation-name:not(.editing),` +
                `& automation-entry span.automation-entry-name:not(.editing),` +
                `& automation-entry entry-switch.automation-entry-control:not(.editing)`))
                if (element instanceof HTMLElement)
                    element.style.display = "";

            activeToggle.disabled = false;
        }

        AutomationPanelElement.prototype.forceQueryEditToggle
            .call(this).checked = editing;
    }

    /**
    @param {boolean} active
    @public*/ setActive(active)
    {
        AutomationPanelElement.prototype.forceQueryActiveToggle
            .call(this).checked = active;
    }

    /**
    @returns {HTMLSpanElement?}
    @public*/ queryAutomationNameElement()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& span.automation-name:not(.editing)`);
    }

    /**
    @returns {HTMLSpanElement}
    @public*/ forceQueryAutomationNameElement()
    {
        const element = AutomationPanelElement.prototype.queryAutomationNameElement
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing automation name element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLSpanElement}
    @public*/ isAutomationNameElement(element)
    {
        return element instanceof Element
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `automation-panel span.automation-name:not(.editing)`);
    }

    /**
    @returns {HTMLSpanElement?}
    @public*/ queryAutomationNameEditingElement()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& span.automation-name.editing`);
    }

    /**
    @returns {HTMLSpanElement}
    @public*/ forceQueryAutomationNameEditingElement()
    {
        const element = AutomationPanelElement.prototype.queryAutomationNameEditingElement
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing automation name element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLSpanElement}
    @public*/ isAutomationNameEditingElement(element)
    {
        return element instanceof Element
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `automation-panel span.automation-name.editing`);
    }

    /**
    @returns {HTMLInputElement?}
    @public*/ queryEditToggle()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& input.automation-edit-toggle`);
    }

    /**
    @returns {HTMLInputElement}
    @public*/ forceQueryEditToggle()
    {
        const element = AutomationPanelElement.prototype.queryEditToggle.call(this);
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
                `automation-panel input.automation-edit-toggle`);
    }

    /**
    @returns {HTMLInputElement?}
    @public*/ queryActiveToggle()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& input.automation-active-toggle`);
    }

    /**
    @returns {HTMLInputElement}
    @public*/ forceQueryActiveToggle()
    {
        const element = AutomationPanelElement.prototype.queryActiveToggle.call(this);
        if (element === null)
            throw new TypeError(
                `Missing active toggle.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLInputElement}
    @public*/ isActiveToggle(element)
    {
        return element instanceof HTMLInputElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `automation-panel input.automation-active-toggle`);
    }

    /**
    @returns {HTMLButtonElement?}
    @public*/ queryDisconnectButton()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& button.automation-disconnect-button`);
    }

    /**
    @returns {HTMLButtonElement}
    @public*/ forceQueryDisconnectButton()
    {
        const element = AutomationPanelElement.prototype.queryDisconnectButton
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing disconnect button.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLButtonElement}
    @public*/ isDisconnectButton(element)
    {
        return element instanceof HTMLButtonElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `automation-panel button.automation-disconnect-button`);
    }

    /**
    @returns {HTMLSelectElement?}
    @public*/ queryNewTriggerButton()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& select.new-automation-trigger`);
    }

    /**
    @returns {HTMLSelectElement}
    @public*/ forceQueryNewTriggerButton()
    {
        const element = AutomationPanelElement.prototype.queryNewTriggerButton
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing new trigger button.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLSelectElement}
    @public*/ isNewTriggerButton(element)
    {
        return element instanceof HTMLSelectElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `automation-panel select.new-automation-trigger`);
    }

    /**
    @returns {HTMLSelectElement?}
    @public*/ queryNewConditionButton()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& select.new-automation-condition`);
    }

    /**
    @returns {HTMLSelectElement}
    @public*/ forceQueryNewConditionButton()
    {
        const element = AutomationPanelElement.prototype.queryNewConditionButton
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing new condition button.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLSelectElement}
    @public*/ isNewConditionButton(element)
    {
        return element instanceof HTMLSelectElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `automation-panel select.new-automation-condition`);
    }

    /**
    @returns {HTMLSelectElement?}
    @public*/ queryNewActionButton()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& select.new-automation-action`);
    }

    /**
    @returns {HTMLSelectElement}
    @public*/ forceQueryNewActionButton()
    {
        const element = AutomationPanelElement.prototype.queryNewActionButton
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing new action button.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLSelectElement}
    @public*/ isNewActionButton(element)
    {
        return element instanceof HTMLSelectElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `automation-panel select.new-automation-action`);
    }

    /**
    @returns {HTMLDivElement?}
    @public*/ queryTriggerContainer()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& div.automation-triggers`);
    }

    /**
    @returns {HTMLDivElement}
    @public*/ forceQueryTriggerContainer()
    {
        const element = AutomationPanelElement.prototype.queryTriggerContainer
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing trigger container element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLDivElement}
    @public*/ isTriggerContainer(element)
    {
        return element instanceof HTMLDivElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `automation-panel div.automation-triggers`);
    }

    /**
    @returns {HTMLDivElement?}
    @public*/ queryConditionContainer()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& div.automation-conditions`);
    }

    /**
    @returns {HTMLDivElement}
    @public*/ forceQueryConditionContainer()
    {
        const element = AutomationPanelElement.prototype.queryConditionContainer
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing condition container element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLDivElement}
    @public*/ isConditionContainer(element)
    {
        return element instanceof HTMLDivElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `automation-panel div.automation-conditions`);
    }

    /**
    @returns {HTMLDivElement?}
    @public*/ queryActionContainer()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelector.call(
            this,
            `& div.automation-actions`);
    }

    /**
    @returns {HTMLDivElement}
    @public*/ forceQueryActionContainer()
    {
        const element = AutomationPanelElement.prototype.queryActionContainer
            .call(this);
        if (element === null)
            throw new TypeError(
                `Missing action container element.`);
        return element;
    }

    /**
    @param {EventTarget?} element
    @returns {element is HTMLDivElement}
    @public*/ isActionContainer(element)
    {
        return element instanceof HTMLDivElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `automation-panel div.automation-action`);
    }

    /**
    @returns {NodeListOf<AutomationEntryElement>}
    @public*/ queryTriggerEntries()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelectorAll.call(
            this,
            `& div.automation-triggers automation-entry`);
    }

    /**
    @param {EventTarget?} element
    @returns {element is AutomationEntryElement}
    @public*/ isTriggerEntry(element)
    {
        return element instanceof AutomationEntryElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `automation-panel div.automation-triggers automation-entry`);
    }

    /**
    @returns {NodeListOf<AutomationEntryElement>}
    @public*/ queryConditionEntries()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelectorAll.call(
            this,
            `& div.automation-conditions automation-entry`);
    }

    /**
    @param {EventTarget?} element
    @returns {element is AutomationEntryElement}
    @public*/ isConditionEntry(element)
    {
        return element instanceof AutomationEntryElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `automation-panel div.automation-conditions automation-entry`);
    }

    /**
    @returns {NodeListOf<AutomationEntryElement>}
    @public*/ queryActionEntries()
    {
        // @ts-expect-error
        return HTMLElement.prototype.querySelectorAll.call(
            this,
            `& div.automation-actions automation-entry`);
    }

    /**
    @param {EventTarget?} element
    @returns {element is AutomationEntryElement}
    @public*/ isActionEntry(element)
    {
        return element instanceof AutomationEntryElement
            && Node.prototype.contains.call(this, element)
            && Element.prototype.matches.call(
                element,
                `automation-panel div.automation-actions automation-entry`);
    }

    /**
    @param {typeof AutomationPanelElement["observedAttributes"][number]} attributeName
    @param {string?} oldValue
    @param {string?} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        
    }

    /**
    @template {keyof AutomationPanelEventMap} K
    @overload
    @param {K} type
    @param {(this: Document, e: AutomationPanelEventMap[K]) => void} listener
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
    @template {keyof AutomationPanelEventMap} K
    @overload
    @param {string} type
    @param {(this: Document, e: AutomationPanelEventMap[K]) => void} listener
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
customElements.define("automation-panel", AutomationPanelElement);