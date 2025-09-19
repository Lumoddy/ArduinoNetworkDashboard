import { parseCommaSeparatedStrings, stringifyCommaSeparatedStrings } from "../common.js";

/**
*/ export class PinIndicatorSwitch extends HTMLElement
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

        if (PinIndicatorSwitch._cachedContentTemplate === null)
        {
            const template = document.createElement("div");

            template.innerHTML =
            `
                <pin-display>
                    <div></div>
                </pin-display>
            `;

            PinIndicatorSwitch._cachedContentTemplate = template.childNodes;
        }

        for (const child of PinIndicatorSwitch._cachedContentTemplate)
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
    @param {typeof PinIndicatorSwitch["observedAttributes"][number]} attributeName
    @param {string | null} oldValue
    @param {string | null} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        switch (attributeName)
        {
            case "options":
            {
                const labelContainer = /** @type {HTMLElement} */(
                    this.querySelector("& > pin-display > div"));
                const labels = /** @type {NodeListOf<HTMLElement>} */(
                    labelContainer.querySelectorAll("& > pin-label"));
                const elements = parseCommaSeparatedStrings(newValue ?? "");

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

                break;
            }
            case "value":
            {
                const labelContainer = /** @type {HTMLElement} */(
                    this.querySelector("& > pin-display > div"));
                const labels = /** @type {NodeListOf<HTMLElement>} */(
                    labelContainer.querySelectorAll("& > pin-label"));
                let index = 0;
                for (; index < labels.length; index++)
                    if (labels[index].textContent === newValue)
                        break;

                if (index !== labels.length)
                    labelContainer.style.setProperty("--position", `${index}`);

                break;
            }
        }
    }
}
customElements.define("pin-indicator-switch", PinIndicatorSwitch);