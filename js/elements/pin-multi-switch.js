import { parseCommaSeparatedStrings, stringifyCommaSeparatedStrings } from "../common.js";

/**
*/ export class PinMultiSwitch extends HTMLElement
{
    /**
    @protected @readonly*/ static observedAttributes = /** @type {const} */(
    [
        "options",
        "value",
    ]);

    /**
    @type {NodeList?}
    @private*/ static _cachedContentTemplate = null;

    /**
    @public*/ constructor()
    {
        super();

        if (PinMultiSwitch._cachedContentTemplate === null)
        {
            const template = document.createElement("div");

            template.innerHTML =
            `
                <pin-rail>
                    <pin-knob></pin-knob>
                </pin-rail>
                <pin-text></pin-text>
                <pin-overlay></pin-overlay>
            `;

            PinMultiSwitch._cachedContentTemplate = template.childNodes;
        }

        for (const child of PinMultiSwitch._cachedContentTemplate)
            this.appendChild(child.cloneNode(true));
    }

    /**
    @returns {string[]?}
    @public*/ get options()
    {
        const string = this.getAttribute("options");
        if (string === null)
            return null;
        else
            return parseCommaSeparatedStrings(string);
    }
    /**
    @public*/ set options(value)
    {
        if (value === null)
            this.removeAttribute("options");
        else
            this.setAttribute("options", stringifyCommaSeparatedStrings(value));
    }

    /**
    @returns {string?}
    @public*/ get value()
    {
        return this.getAttribute("value");
    }
    /**
    @public*/ set value(value)
    {
        if (value === null)
            this.removeAttribute("value");
        else
            this.setAttribute("value", value);
    }

    /**
    @param {typeof PinMultiSwitch["observedAttributes"][number]} attributeName
    @param {string | null} oldValue
    @param {string | null} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        switch (attributeName)
        {
            case "options":
            {
                const elements = parseCommaSeparatedStrings(newValue ?? "");

                const labelContainer = /** @type {HTMLElement} */(
                    this.querySelector("& > pin-text"));
                const labels = /** @type {NodeListOf<HTMLElement>} */(
                    labelContainer.querySelectorAll("& > pin-label"));

                let i = 0;
                let minLength = Math.min(labels.length, elements.length);

                for (; i < minLength; i++)
                    labels[i].textContent = elements[i];

                for (; i < labels.length; i++)
                    labels[i].remove();

                for (; i < elements.length; i++)
                {
                    const label = labelContainer.appendChild(
                        document.createElement("pin-label"));
                    label.textContent = elements[i];
                }

                const buttonContainer = /** @type {HTMLElement} */(
                    this.querySelector("& > pin-overlay"));
                const buttons = /** @type {NodeListOf<HTMLButtonElement>} */(
                    labelContainer.querySelectorAll("& > button"));

                i = 0;
                minLength = Math.min(buttons.length, elements.length);

                for (; i < minLength; i++)
                    buttons[i].textContent = elements[i];

                for (; i < buttons.length; i++)
                    buttons[i].remove();

                for (; i < elements.length; i++)
                {
                    const index = i;
                    const button = buttonContainer.appendChild(
                        document.createElement("button"));
                    button.addEventListener("click", () =>
                    {
                        const labels = /** @type {NodeListOf<HTMLElement>} */(
                            this.querySelectorAll("& > pin-text > pin-label"));

                        const labelValue = labels[index];
                        if (labelValue !== undefined)
                            this.setAttribute("value", labelValue.textContent);
                    });
                }

                const value = this.getAttribute("value");
                this.attributeChangedCallback("value", value, value);

                break;
            }
            case "value":
            {
                const labelContainer = /** @type {HTMLElement} */(
                    this.querySelector("& > pin-text"));
                const labels = /** @type {NodeListOf<HTMLElement>} */(
                    labelContainer.querySelectorAll("& > pin-label"));
                let i = 0;
                for (; i < labels.length; i++)
                    if (labels[i].textContent === newValue)
                        break;

                for (const label of labels)
                    label.classList.remove("active");

                if (i !== labels.length)
                {
                    const rail = /** @type {HTMLElement} */(
                        this.querySelector("& > pin-rail"));

                    rail.style.setProperty(
                        "--position",
                        `${i / (labels.length - 1)}`);
                    labels[i].classList.add("active");
                }

                this.dispatchEvent(new Event("change", { bubbles: true }));

                break;
            }
        }
    }
}
customElements.define("pin-multi-switch", PinMultiSwitch);