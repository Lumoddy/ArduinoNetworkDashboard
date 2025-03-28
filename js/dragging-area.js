import { Loud } from "./loud.js";

/**
*/ export class DraggingArea extends HTMLElement
{
    /**
    @readonly*/ static observedAttributes = /** @type {const} */(["center", "zoom"]);

    /**
    @public*/ constructor()
    {
        super();

        /**
        @type {Loud<{ x: number, y: number }>}
        @private*/ this._viewCenter = new Loud({ x: 0.0, y: 0.0 });

        /**
        @type {Loud<number>}
        @private*/ this._viewZoom = new Loud(1.0);

        /**
        @type {ResizeObserver}
        @private*/ this._resizeObserver = new ResizeObserver((entries) =>
        {
            
        });
        this._resizeObserver.observe(this, { box: "content-box" });

        /**
        @type {number}
        @private*/ this._attributeCallbackSuppression = 0;
    }

    /**
    @param {typeof DraggingArea["observedAttributes"][number]} attributeName
    @param {string | null} oldValue
    @param {string | null} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        if (this._attributeCallbackSuppression > 0)
            return;

        ++this._attributeCallbackSuppression;

        switch (attributeName)
        {
            case "center":
            {
                const match = newValue === null ? null :
                    /(-?(?:\d+(?:\.\d*)?|\d*\.\d+)(?:[eE][-+]\d+)?).*?(-?(?:\d+(?:\.\d*)?|\d*\.\d+)(?:[eE][-+]\d+)?)/.exec(newValue);

                this._viewCenter.value =
                    match === null ? { x: 0, y: 0 } :
                    {
                        x: Number.parseFloat(match[1]),
                        y: Number.parseFloat(match[2]),
                    };

                break;
            }
            case "zoom":
            {
                this._viewZoom.value = newValue === null ? 1 : Number.parseFloat(newValue);
                break;
            }
        }

        --this._attributeCallbackSuppression;
    }
}
customElements.define("dragging-area", DraggingArea);