import { DraggableItem } from "./draggable-item.js";
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
        this._viewCenter.addListener((e) =>
        {
            this.style.setProperty("--view-center-x", String(e.newValue.x));
            this.style.setProperty("--view-center-y", String(e.newValue.y));

            ++this._attributeCallbackSuppression;
            this.setAttribute("center", `${e.newValue.x} ${e.newValue.y}`);
            --this._attributeCallbackSuppression;
        });

        /**
        @type {Loud<number>}
        @private*/ this._viewZoom = new Loud(1.0);
        this._viewZoom.addListener((e) =>
        {
            this.style.setProperty("--view-zoom", String(e.newValue));
            this.style.setProperty("--view-scale", `${this.clientHeight / e.newValue}px`);

            ++this._attributeCallbackSuppression;
            this.setAttribute("zoom", String(e.newValue));
            --this._attributeCallbackSuppression;
        });

        /**
        @type {ResizeObserver}
        @private*/ this._resizeObserver = new ResizeObserver((e) =>
        {
            this.style.setProperty("--view-scale", `${this.clientHeight / this._viewZoom.value}px`);
        });
        this._resizeObserver.observe(this, { box: "content-box" });

        /**
        @type {number}
        @private*/ this._attributeCallbackSuppression = 0;

        /**
        @type {{ [I in number]: { x: number, y: number } } & { count: number }}
        @private*/ this._draggingStartPoints = { count: 0 };

        /**
        @type {{ [I in number]: { x: number, y: number, item: DraggableItem } } & { count: number }}
        @private*/ this._dragging = { count: 0 };

        this.addEventListener("pointerdown", (e) =>
        {
            if (e.target === this)
            {
                const style = window.getComputedStyle(this);

                if (!(e.pointerId in this._draggingStartPoints))
                {
                    if (this._draggingStartPoints.count === 0)
                        this.setAttribute("dragging", "");

                    ++this._draggingStartPoints.count;
                    this.setPointerCapture(e.pointerId);
                }

                this._draggingStartPoints[e.pointerId] = this.clientToViewPoint(
                    parseFloat(style.paddingLeft) + e.offsetX,
                    parseFloat(style.paddingTop) + e.offsetY);
            }
            else if (e.target instanceof DraggableItem && e.target.parentNode === this)
            {
                const style = window.getComputedStyle(this);

                if (!(e.pointerId in this._draggingStartPoints))
                {
                    if (this._draggingStartPoints.count === 0)
                        this.setAttribute("dragging", "");

                    ++this._draggingStartPoints.count;
                    this.setPointerCapture(e.pointerId);
                }

                this._draggingStartPoints[e.pointerId] =
                {
                    x: parseFloat(style.paddingLeft) + e.offsetX,
                    y: parseFloat(style.paddingTop) + e.offsetY,
                }
            }
        });

        this.addEventListener("pointerup", (e) =>
        {
            if (e.pointerId in this._draggingStartPoints)
            {
                --this._draggingStartPoints.count;

                if (this._draggingStartPoints.count === 0)
                    this.removeAttribute("dragging");

                this.releasePointerCapture(e.pointerId);

                delete this._draggingStartPoints[e.pointerId];
            }
        });

        this.addEventListener("pointercancel", (e) =>
        {
            if (e.pointerId in this._draggingStartPoints)
            {
                --this._draggingStartPoints.count;

                if (this._draggingStartPoints.count === 0)
                    this.removeAttribute("dragging");

                this.releasePointerCapture(e.pointerId);

                delete this._draggingStartPoints[e.pointerId];
            }
        });

        this.addEventListener("pointermove", (e) =>
        {
            if (e.pointerId in this._draggingStartPoints)
            {
                const style = window.getComputedStyle(this);

                const lastPosition = this._draggingStartPoints[e.pointerId];
                const currentPosition = this.clientToViewPoint(
                    parseFloat(style.paddingLeft) + e.offsetX,
                    parseFloat(style.paddingTop) + e.offsetY);

                this._viewCenter.value = {
                    x: this._viewCenter.value.x + (lastPosition.x - currentPosition.x),
                    y: this._viewCenter.value.y + (lastPosition.y - currentPosition.y),
                };

                console.log(this._viewCenter.value.x, "+", "(", currentPosition.x, "-", lastPosition.x, ")", "=", this._viewCenter.value.x + (currentPosition.x - lastPosition.x));
            }
        });
    }

    /**
    @overload
    @param {number} x
    @param {number} y
    @returns {{ x: number, y: number }}
    *//**
    @overload
    @param {{x: number, y: number}} point
    @returns {{ x: number, y: number }}
    *//**
    @param {number | { x: number, y: number }} x
    @param {number} y
    @returns {{ x: number, y: number }}
    @public*/ clientToViewPoint(x, y = 0)
    {
        if (typeof x !== "number")
        {
            y = x.y;
            x = x.x;
        }

        x = (((x - (this.clientWidth * 0.5)) / this.clientHeight) * this._viewZoom.value) + this._viewCenter.value.x;
        y = (((y - (this.clientHeight * 0.5)) / this.clientHeight) * this._viewZoom.value) + this._viewCenter.value.y;

        return { x: x, y: y };
    }

    /**
    @overload
    @param {number} x
    @param {number} y
    @returns {{ x: number, y: number }}
    *//**
    @overload
    @param {{x: number, y: number}} point
    @returns {{ x: number, y: number }}
    *//**
    @param {number | { x: number, y: number }} x
    @param {number} y
    @returns {{ x: number, y: number }}
    @public*/ viewToClientPoint(x, y = 0)
    {
        if (typeof x !== "number")
        {
            y = x.y;
            x = x.x;
        }

        x = ((((x - this._viewCenter.value.x) / this._viewZoom.value) * this.clientHeight) + (this.clientHeight * 0.5));
        y = ((((y - this._viewCenter.value.y) / this._viewZoom.value) * this.clientHeight) + (this.clientWidth * 0.5));

        return { x: x, y: y };
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