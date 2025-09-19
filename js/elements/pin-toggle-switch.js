
/**
*/ export class PinToggleSwitch extends HTMLElement
{
    /**
    @protected @readonly*/ static observedAttributes = /** @type {const} */(
    [
        "on-text",
        "off-text",
    ]);

    /**
    @type {NodeList?}
    @private*/ static _cachedContentTemplate = null;

    /**
    @public*/ constructor()
    {
        super();

        if (PinToggleSwitch._cachedContentTemplate === null)
        {
            const template = document.createElement("div");

            template.innerHTML =
            `
                <button>
                    <pin-clip>
                        <pin-center>
                            <pin-text class="on">On</pin-text>
                            <pin-text class="off">Off</pin-text>
                        </pin-center>
                    </pin-clip>
                    <pin-knob></pin-knob>
                </button>
            `;

            PinToggleSwitch._cachedContentTemplate = template.childNodes;
        }

        for (const child of PinToggleSwitch._cachedContentTemplate)
            this.appendChild(child.cloneNode(true));

        const button = /** @type {HTMLButtonElement} */(
            this.querySelector("& > button"));
        button.addEventListener("click", () => this.checked = !this.checked);
    }

    /**
    @returns {boolean}
    @public*/ get checked()
    {
        switch (super.getAttribute("checked"))
        {
            case "":
            case "true":
                return true;
            default:
                return false;
        }
    }
    /**
    @public*/ set checked(value)
    {
        if (value)
            this.setAttribute("checked", "");
        else
            this.removeAttribute("checked");
    }

    /**
    @param {typeof PinToggleSwitch["observedAttributes"][number]} attributeName
    @param {string | null} oldValue
    @param {string | null} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        switch (attributeName)
        {
            case "on-text":
            {
                const element = /** @type {HTMLElement} */(
                    this.querySelector("& > button > pin-clip > pin-center > pin-text.on"));
                element.textContent = newValue;
                break;
            }
            case "off-text":
            {
                const element = /** @type {HTMLElement} */(
                    this.querySelector("& > button > pin-clip > pin-center > pin-text.off"));
                element.textContent = newValue;
                break;
            }
        }
    }
}
customElements.define("pin-toggle-switch", PinToggleSwitch);