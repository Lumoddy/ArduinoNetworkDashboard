/**
*/ export class GraphView extends HTMLElement
{
    /**
    @private @readonly*/ static _viewXAttributeName = "view-x";
    /**
    @private @readonly*/ static _viewYAttributeName = "view-y";
    /**
    @private @readonly*/ static _viewScaleAttributeName = "view-scale";

    /**
    @protected*/ static observedAttributes = /** @type {const} */(
    [
        this._viewXAttributeName,
        this._viewYAttributeName,
        this._viewScaleAttributeName,
    ]);

    /**
    @returns {number}
    @public*/ get viewX() { return this._viewX }
    /**
    @public*/ set viewX(value)
    {
        ++this._attributeIgnoreStack;
        super.setAttribute(GraphView._viewXAttributeName, String(value));
        --this._attributeIgnoreStack;
    }

    /**
    @returns {number}
    @public*/ get viewY() { return this._viewY }
    /**
    @public*/ set viewY(value)
    {
        ++this._attributeIgnoreStack;
        super.setAttribute(GraphView._viewYAttributeName, String(value));
        --this._attributeIgnoreStack;
    }

    /**
    @returns {number}
    @public*/ get viewScale() { return this._viewScale }
    /**
    @public*/ set viewScale(value)
    {
        ++this._attributeIgnoreStack;
        super.setAttribute(GraphView._viewScaleAttributeName, String(value));
        --this._attributeIgnoreStack;
    }

    /**
    @public*/ constructor()
    {
        super();

        /**
        @private*/ this._viewX = 0;

        /**
        @private*/ this._viewY = 0;

        /**
        @private*/ this._viewScale = 1;

        /**
        @private*/ this._resizeObserver = new ResizeObserver(() =>
        {
            super.style.setProperty("--view-scale", `${this._viewScale * super.clientHeight}px`);
        });

        const fragment = document.createDocumentFragment();
        {
            /**
            @private*/ this._graphicContainer = document.createElementNS("http://www.w3.org/2000/svg", "svg");
            fragment.appendChild(this._graphicContainer);
        }
        super.appendChild(fragment);

        /**
        @private*/ this._attributeIgnoreStack = 0;
    }

    /**
    @param {typeof GraphView["observedAttributes"][number]} attributeName
    @param {string | null} oldValue
    @param {string | null} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        if (this._attributeIgnoreStack > 0)
            return;

        ++this._attributeIgnoreStack;

        switch (attributeName)
        {
            case GraphView._viewXAttributeName:
            case GraphView._viewYAttributeName:
            case GraphView._viewScaleAttributeName:
            {
                let key;
                switch (attributeName)
                {
                    case GraphView._viewXAttributeName: key = /** @type {const} */("_viewX"); break;
                    case GraphView._viewYAttributeName: key = /** @type {const} */("_viewY"); break;
                    case GraphView._viewScaleAttributeName: key = /** @type {const} */("_viewScale"); break;
                }

                if (newValue === null)
                {
                    super.setAttribute(attributeName, String(this[key]));
                    return;
                }

                const value = Number(newValue);
                if (Number.isNaN(value))
                {
                    super.setAttribute(attributeName, String(this[key]));
                    return;
                }

                this[key] = value;
                switch (attributeName)
                {
                    case GraphView._viewXAttributeName: super.style.setProperty("--view-x", String(value)); break;
                    case GraphView._viewYAttributeName: super.style.setProperty("--view-y", String(value)); break;
                    case GraphView._viewScaleAttributeName: super.style.setProperty("--view-scale", `${value * super.clientHeight}px`); break;
                }

                break;
            }
        }

        --this._attributeIgnoreStack;
    }
}
customElements.define("graph-view", GraphView);