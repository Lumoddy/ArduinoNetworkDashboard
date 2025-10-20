import { html } from "../common.js";

/**
*/ const _template = html`
    <style>
        :host
        {
            position: relative;
            margin: 4px 2px 8px;
            width: 64px;
            height: 26px;
            display: inline-block;
            user-select: none;

            & input
            {
                position: absolute;
                width: 0;
                height: 0;
                margin: 0;
                border: 0;
                padding: 0;
                opacity: 0;
            }

            & > label
            {
                position: absolute;
                top: 0;
                bottom: 0;
                left: 0;
                right: 0;
                margin: 0;
                border: 0;
                padding: 0;
                border-radius: var(--border-radius);

                & > .rail
                {
                    position: absolute;
                    top: 0;
                    bottom: 0;
                    left: 0;
                    right: 0;
                    border-radius: var(--border-radius);
                    background-color: var(--color-inset);
                    box-shadow: var(--shadow-inset);
                    outline: var(--outline--hidden);
                    outline-offset: var(--outline-offset--hidden);
                    transition:
                        var(--transition-outline--hidden),
                        var(--transition-light--fade);
                }

                & > .knob
                {
                    position: absolute;
                    top: 50%;
                    bottom: 50%;
                    left: 50%;
                    right: 50%;
                    border-radius: var(--border-radius);
                    background-color: var(--color-block);
                    box-shadow: transparent 0 0 0;
                    transition:
                        top ease-in 0.3s,
                        bottom ease-in 0.3s,
                        left ease-in 0.3s,
                        right ease-in 0.3s,
                        var(--transition-light--fade);
                }
            }
        }

        :host([pin-mode="input"])
        {
            & > label
            {
                & > .rail
                {
                    background-color: var(--color-inset--down);
                    transition:
                        var(--transition-outline--hidden),
                        var(--transition-light);
                }

                & > .knob
                {
                    top: calc(50% - 4px);
                    bottom: calc(50% - 4px);
                    left: calc(50% - 10px);
                    right: calc(50% - 10px);
                    box-shadow: var(--shadow-outset--small);
                    background-color: var(--color-block--down);
                    transition:
                        top ease-in 0.3s,
                        bottom ease-in 0.3s,
                        left ease-in 0.1s,
                        right ease-in 0.1s,
                        var(--transition-light);
                }

                & > input[type="checkbox"]:checked ~ .rail
                {
                    background-color: var(--color-inset--up);
                }

                & > input[type="checkbox"]:checked ~ .knob
                {
                    background-color: var(--color-block--up);
                }
            }
        }

        :host([pin-mode="output"])
        {
            & > label
            {
                cursor: pointer;

                & > .rail
                {
                    background-color: var(--color-inset--down);
                    transition:
                        var(--transition-outline--hidden),
                        var(--transition-light--interact) 0.05s;
                }

                & > .knob
                {
                    top: 0;
                    bottom: 0;
                    left: 0;
                    right: calc(100% - 26px);
                    box-shadow: var(--shadow-outset);
                    background-color: var(--color-block--down);
                    transition:
                        top ease-in 0.1s,
                        bottom ease-in 0.1s,
                        left ease-in 0.1s,
                        right ease-in 0.1s,
                        var(--transition-light--interact) 0.05s;
                }

                & > input[type="checkbox"]:focus-visible ~ .rail
                {
                    outline: var(--outline);
                    outline-offset: var(--outline-offset);
                    transition:
                        var(--transition-outline),
                        var(--transition-light--interact) 0.05s;
                }

                & > input[type="checkbox"]:checked ~ .rail
                {
                    background-color: var(--color-inset--up);
                }

                & > input[type="checkbox"]:checked ~ .knob
                {
                    left: calc(100% - 26px);
                    right: 0;
                    background-color: var(--color-block--up);
                }

                &:active
                {
                    & > .knob
                    {
                        background-color: oklch(from var(--color-block--down) calc(l - 0.1) c h);
                        transition:
                            top ease-in 0.1s,
                            bottom ease-in 0.1s,
                            left ease-in 0.1s,
                            right ease-in 0.1s,
                            var(--transition-light--interact) 0.05s,
                            background-color 0s;
                    }

                    & > input[type="checkbox"]:focus-visible ~ .rail
                    {
                        outline: var(--outline);
                        outline-offset: var(--outline-offset);
                        transition:
                            var(--transition-outline),
                            var(--transition-light--interact) 0.05s,
                            background-color 0s;
                    }

                    & > input[type="checkbox"]:checked ~ .knob
                    {
                        left: calc(100% - 26px);
                        right: 0;
                        background-color: oklch(from var(--color-block--up) calc(l - 0.1) c h);
                    }
                }
            }
        }
    </style>
    <label>
        <input type="checkbox" autocomplete="off">
        <div class="rail"></div>
        <div class="knob"></div>
    </label>
`;

/**
@typedef {"input" | "output" | "ignore"} PinMode
*/

/**
@param {string | undefined?} value
@returns {PinMode}
*/ export function ensureIsPinMode(value)
{
    switch (value)
    {
        case "input": return "input";
        case "output": return "output";
        default: return "ignore";
    }
}

/**
*/ export class PinSwitchChangeEvent extends Event
{
    /**
    @param {string} type
    @param {
        EventInit
        & {
            wasHigh: boolean,
            isHigh: boolean,
        }
    } init
    @public*/ constructor(type, init)
    {
        super(type, init);

        /**
        @type {boolean}
        @private*/ this._wasHigh = Boolean(init.wasHigh);

        /**
        @type {boolean}
        @private*/ this._isHigh = Boolean(init.isHigh);
    }

    /**
    @returns {PinSwitch}
    @public @readonly @override*/ get target()
    {
        if (!(super.target instanceof PinSwitch))
            throw new TypeError(
                "PinSwitchChangeEvent can only be used on PinSwitch.");

        return super.target;
    }

    /**
    @returns {boolean}
    @public @readonly*/ get wasHigh() { return this._wasHigh }

    /**
    @returns {boolean}
    @public @readonly*/ get isHigh() { return this._isHigh }
}

/**
*/ export class PinSwitch extends HTMLElement
{
    /**
    @protected @readonly*/ static observedAttributes = /** @type {const} */(
    [
        "pin-mode",
        "is-high",
        "disabled",
    ]);

    /**
    @public*/ constructor()
    {
        super();

        /**
        @type {ShadowRoot}
        @private*/ this._shadowRoot = this.attachShadow({ mode: "open" });
        this._shadowRoot.appendChild(_template.content.cloneNode(true));

        this._shadowRoot.addEventListener("change", (e) =>
        {
            if (e.target instanceof HTMLInputElement)
                this.isHigh = e.target.checked;

            e.stopPropagation();
        });
    }

    /**
    @returns {PinMode}
    @public*/ get mode()
    {
        return ensureIsPinMode(this.getAttribute("pin-mode"));
    }
    /**
    @public*/ set mode(value)
    {
        this.setAttribute("pin-mode", value);
    }

    /**
    @returns {boolean}
    @public*/ get isHigh()
    {
        return this._shadowRoot.querySelector(
            ":host > label > input:checked") !== null;
    }
    /**
    @public*/ set isHigh(value)
    {
        if (value)
            this.setAttribute("is-high", "");
        else
            this.removeAttribute("is-high");
    }

    /**
    @returns {boolean}
    @public*/ get disabled()
    {
        switch (this.getAttribute("disabled"))
        {
            case "true":
            case "":
                return true;
            default:
                return false;
        }
    }
    /**
    @public*/ set disabled(value)
    {
        if (value)
            this.setAttribute("disabled", "");
        else
            this.removeAttribute("disabled");
    }

    /**
    @param {typeof PinSwitch["observedAttributes"][number]} attributeName
    @param {string?} oldValue
    @param {string?} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        switch (attributeName)
        {
            case "pin-mode":
            {
                const input = this._shadowRoot.querySelector(
                    ":host > label > input");
                if (input instanceof HTMLInputElement)
                    input.disabled = newValue !== "output";

                break;
            }
            case "is-high":
            {
                const input = this._shadowRoot.querySelector(
                    ":host > label > input");

                const wasHigh = oldValue === "" || oldValue === "true";
                const isHigh = newValue === "" || newValue === "true";

                if (input instanceof HTMLInputElement)
                    input.checked = isHigh;

                switch (this.getAttribute("pin-mode"))
                {
                    case "input":
                    case "output":
                        this.dispatchEvent(new PinSwitchChangeEvent(
                            "change",
                            {
                                wasHigh,
                                isHigh,
                                bubbles: true,
                                cancelable: false,
                                composed: false,
                            }));
                        break;
                }

                break;
            }
            case "disabled":
            {
                const disabled = newValue === "" || newValue === "true";

                for (const element of this._shadowRoot.querySelectorAll(
                    `:host input`))
                    if (element instanceof HTMLInputElement)
                        element.disabled = disabled;

                break;
            }
        }
    }
}
customElements.define("pin-switch", PinSwitch);