import { html } from "../common.js";
/**
@import { PinMode } from "./pin-switch.js"
*/

/**
*/ const _template = html`
    <style>
        :host
        {
            position: relative;
            margin: 4px 2px 8px;
            width: 64px;
            height: 26px;
            display: inline-flex;
            user-select: none;
            flex-flow: row nowrap;
            gap: 2px;
            justify-content: stretch;
            font-size: 10px;
            font-family: var(--font-normal);
            font-weight: 600;

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
                flex: 1;

                & > div
                {
                    height: 100%;
                    padding: 0 4px;
                    color: var(--color-line);
                    background-color: var(--color-block);
                    align-content: center;
                    text-align: center;
                    box-shadow: var(--shadow-outset);
                    transform: translateY(0);
                    cursor: pointer;
                    outline: var(--outline--hidden);
                    outline-offset: var(--outline-offset--hidden);
                    transition:
                        var(--transition-outline--hidden),
                        box-shadow ease-in 0.1s,
                        transform ease-in 0.1s,
                        var(--transition-light--interact);
                }

                & > input:focus-visible ~ div
                {
                    outline: var(--outline);
                    outline-offset: var(--outline-offset);
                    transition:
                        var(--transition-outline),
                        box-shadow ease-in 0.1s,
                        transform ease-in 0.1s,
                        var(--transition-light--interact);
                }

                & > input:checked ~ div
                {
                    color: oklch(from var(--color-line) calc(l - 0.1) c h);
                    background-color: oklch(from var(--color-block) calc(l - 0.1) c h);
                    box-shadow: transparent 0 0 0;
                    transform: translateY(3px);
                }

                &:first-of-type > div
                {
                    border-bottom-left-radius: var(--border-radius);
                    border-top-left-radius: var(--border-radius);
                }

                &:last-of-type > div
                {
                    border-bottom-right-radius: var(--border-radius);
                    border-top-right-radius: var(--border-radius);
                }
            }
        }
    </style>
    <label>
        <input type="radio" autocomplete="off" value="input">
        <div>IN</div>
    </label>
    <label>
        <input type="radio" autocomplete="off" value="ignore" checked>
        <div></div>
    </label>
    <label>
        <input type="radio" autocomplete="off" value="output">
        <div>OUT</div>
    </label>
`;

/**
*/ export class PinModeSwitchChangeEvent extends Event
{
    /**
    @param {string} type
    @param {
        EventInit
        & {
            oldMode: PinMode,
            newMode: PinMode,
        }
    } init
    @public*/ constructor(type, init)
    {
        super(type, init);

        /**
        @type {PinMode}
        @private*/ this._oldMode = init.oldMode;

        /**
        @type {PinMode}
        @private*/ this._newMode = init.newMode;
    }

    /**
    @returns {PinModeSwitch}
    @public @readonly @override*/ get target()
    {
        if (!(super.target instanceof PinModeSwitch))
            throw new TypeError(
                "PinModeSwitchChangeEvent can only be used on PinModeSwitch.");

        return super.target;
    }

    /**
    @returns {PinMode}
    @public @readonly*/ get oldMode() { return this._oldMode }

    /**
    @returns {PinMode}
    @public @readonly*/ get newMode() { return this._newMode }
}

/**
*/ export class PinModeSwitch extends HTMLElement
{
    /**
    @protected @readonly*/ static observedAttributes = /** @type {const} */(
    [
        "pin-mode",
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
            {
                // @ts-ignore: checked internally.
                this.mode = e.target.value;
            }

            e.stopPropagation();
        });
    }

    /**
    @returns {PinMode}
    @public*/ get mode()
    {
        const input = this._shadowRoot.querySelector(
            ":host > label > input:checked");
        switch (input instanceof HTMLInputElement ? input.value : undefined)
        {
            case "input": return "input";
            case "output": return "output";
            default: return "ignore";
        }
    }
    /**
    @public*/ set mode(value)
    {
        this.setAttribute("pin-mode", value);
    }

    /**
    @param {typeof PinModeSwitch["observedAttributes"][number]} attributeName
    @param {string | null} oldValue
    @param {string | null} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        switch (attributeName)
        {
            case "pin-mode":
            {
                /**
                @type {PinMode}
                */ let oldMode;
                switch (oldValue)
                {
                    case "input": oldMode = "input"; break;
                    case "output": oldMode = "output"; break;
                    default: oldMode = "ignore"; break;
                }

                /**
                @type {PinMode}
                */ let newMode;
                switch (newValue)
                {
                    case "input": newMode = "input"; break;
                    case "output": newMode = "output"; break;
                    default: newMode = "ignore"; break;
                }

                const otherInputs = this._shadowRoot.querySelectorAll(
                    `:host > label > input:not([value="${newMode}"])`);
                for (let i = 0; i < otherInputs.length; i++)
                {
                    const input = otherInputs[i];
                    if (input instanceof HTMLInputElement)
                        input.checked = false;
                }

                const input = this._shadowRoot.querySelector(
                    `:host > label > input[value="${newMode}"]`);
                if (input instanceof HTMLInputElement)
                    input.checked = true;

                this.dispatchEvent(new PinModeSwitchChangeEvent(
                    "change",
                    {
                        oldMode,
                        newMode,
                        bubbles: true,
                        cancelable: false,
                        composed: false,
                    }));

                break;
            }
        }
    }
}
customElements.define("pin-mode-switch", PinModeSwitch);