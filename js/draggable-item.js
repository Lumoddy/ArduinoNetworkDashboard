/**
@abstract*/ export class DraggableItem extends HTMLElement
{
    /**
    @readonly*/ static observedAttributes = /** @type {const} */(["pos"]);

    /**
    @public*/ constructor()
    {
        super();

        /**
        @type {{ x: number, y: number }}
        @private*/ this._pos = { x: 0.0, y: 0.0 };

        /**
        @type {number}
        @private*/ this._attributeCallbackSuppression = 0;
    }

    /**
    @type {{ x: number, y: number }}
    @public*/ get pos() { return this._pos }
    /**
    @public*/ set pos(value)
    {
        this._pos = value;

        this.style.setProperty("--pos-x", String(value.x));
        this.style.setProperty("--pos-y", String(value.y));

        ++this._attributeCallbackSuppression;
        this.setAttribute("pos", `${value.x} ${value.y}`);
        --this._attributeCallbackSuppression;
    }

    /**
    @param {typeof DraggableItem["observedAttributes"][number]} attributeName
    @param {string | null} oldValue
    @param {string | null} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        if (this._attributeCallbackSuppression > 0)
            return;

        ++this._attributeCallbackSuppression;

        switch (attributeName)
        {
            case "pos":
            {
                const match = newValue === null ? null :
                    /^.*?(-?(?:\d+(?:\.\d*)?|\d*\.\d+)(?:[eE][-+]\d+)?).*?(-?(?:\d+(?:\.\d*)?|\d*\.\d+)(?:[eE][-+]\d+)?)/.exec(newValue);

                this._pos = match === null ? { x: 0, y: 0 } :
                {
                    x: Number.parseFloat(match[1]),
                    y: Number.parseFloat(match[2]),
                };

                break;
            }
        }

        --this._attributeCallbackSuppression;
    }
}