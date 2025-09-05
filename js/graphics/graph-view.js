import { GraphElement } from "./graph-element.js";

/**
*/ export class GraphView extends HTMLElement
{
    /**
    @protected @readonly*/ static observedAttributes =
    [
        "view-x",
        "view-y",
        "view-scale",
        "view-reference",
        "view-draggable",
    ];

    /**
    @type {ResizeObserver}
    @private*/ _resizeObserver = new ResizeObserver((entries) =>
    {
        super.style.setProperty("--view-scale", String(this.viewScale));
        super.style.setProperty("--view-reference", `${this.viewReferenceInPixels}px`);
        this._graphicWindow.setAttribute("width", String(super.clientWidth));
        this._graphicWindow.setAttribute("height", String(super.clientHeight));
    });

    /**
    @type {SVGSVGElement}
    @private*/ _graphicWindow;

    /**
    @type {SVGGElement}
    @private*/ _graphicElementContainer;

    /**
    @type {GraphElement[]}
    @private*/ _nodes = [];

    /**
    @type {MutationObserver}
    @private*/ _nodeMutationObserver = new MutationObserver((entries) =>
    {
        if (entries.some((v) => v.type === "attributes"))
            this._upAllPointers((v) => "node" in v && !v.node.viewDraggable);
    });

    /**
    @type {(
        | { id: number, viewX: number, viewY: number }
        | { id: number, viewX: number, viewY: number, node: GraphElement }
    )[]}
    @private*/ _draggingPointers = [];

    /**
    @returns {number}
    @public*/ get viewX()
    {
        const result = Number(super.getAttribute("view-x"));
        if (Number.isNaN(result))
            return 0.0;
        return result;
    }
    /**
    @public*/ set viewX(value)
    {
        super.setAttribute("view-x", String(value));
    }

    /**
    @returns {number}
    @public*/ get viewY()
    {
        const result = Number(super.getAttribute("view-y"));
        if (Number.isNaN(result))
            return 0.0;
        return result;
    }
    /**
    @public*/ set viewY(value)
    {
        super.setAttribute("view-y", String(value));
    }

    /**
    @returns {number}
    @public*/ get viewScale()
    {
        const result = Number(super.getAttribute("view-scale"));
        if (result <= 0.0 || Number.isNaN(result))
            return 1.0;
        return result;
    }
    /**
    @public*/ set viewScale(value)
    {
        super.setAttribute("view-scale", String(value));
    }

    /**
    @returns {number}
    @public*/ get minViewScale()
    {
        const result = Number(super.getAttribute("view-min-scale"));
        if (result <= 0.0 || Number.isNaN(result))
            return 0.2;
        return result;
    }
    /**
    @public*/ set minViewScale(value)
    {
        super.setAttribute("view-min-scale", String(value));
    }

    /**
    @returns {number}
    @public*/ get maxViewScale()
    {
        const result = Number(super.getAttribute("view-max-scale"));
        if (result <= 0.0 || Number.isNaN(result))
            return 5.0;
        return result;
    }
    /**
    @public*/ set maxViewScale(value)
    {
        super.setAttribute("view-max-scale", String(value));
    }

    /**
    @returns {number}
    @public @readonly*/ get viewReferenceInPixels()
    {
        switch (this.viewReference)
        {
            case "width": return super.clientWidth;
            case "height": return super.clientHeight;
            case "pixel": return 1;
        }
    }

    /**
    @returns {number}
    @public*/ get viewScaleInPixels() { return this.viewScale * this.viewReferenceInPixels }
    /**
    @public*/ set viewScaleInPixels(value) { this.viewScale = value / this.viewReferenceInPixels }

    /**
    @returns {"width" | "height" | "pixel"}
    @public*/ get viewReference()
    {
        const result = super.getAttribute("view-reference");
        switch (result)
        {
            case "width":
            case "height":
            case "pixel":
                return result;
            default:
                return "height";
        }
    }
    /**
    @public*/ set viewReference(value)
    {
        super.setAttribute("view-reference", String(value));
    }

    /**
    @returns {boolean}
    @public*/ get viewDraggable()
    {
        const result = super.getAttribute("view-draggable");
        return result === "" || result === "true";
    }
    /**
    @public*/ set viewDraggable(value)
    {
        super.setAttribute("view-draggable", value ? "true" : "false");
    }

    /**
    @returns {SVGGElement}
    @public @readonly*/ get container() { return this._graphicElementContainer }

    /**
    @public*/ constructor()
    {
        super();

        super.style.padding = "0";

        this._graphicWindow =
            super.querySelector("& > svg:not([width], [height])")
            ?? super.appendChild(document.createElementNS("http://www.w3.org/2000/svg", "svg"));

        this._graphicWindow.style.position = "absolute";
        this._graphicWindow.style.top = "0";
        this._graphicWindow.style.left = "0";

        this._graphicElementContainer =
            this._graphicWindow.querySelector("& > g")
            ?? this._graphicWindow.appendChild(document.createElementNS("http://www.w3.org/2000/svg", "g"));
    }

    /**
    @protected*/ connectedCallback()
    {
        this._resizeObserver.observe(this);

        super.addEventListener("pointerdown", this._onPointerDown);
        super.addEventListener("pointerup", this._onPointerUp);
        super.addEventListener("pointercancel", this._onPointerUp);
        super.addEventListener("pointermove", this._onPointerMove);
        super.addEventListener("wheel", this._onWheel);
    }

    /**
    @protected*/ disconnectedCallback()
    {
        this._resizeObserver.unobserve(this);

        super.removeEventListener("pointerdown", this._onPointerDown);
        super.removeEventListener("pointerup", this._onPointerUp);
        super.removeEventListener("pointercancel", this._onPointerUp);
        super.removeEventListener("pointermove", this._onPointerMove);
        super.removeEventListener("wheel", this._onWheel);
    }

    /**
    @param {HTMLElementEventMap["pointerdown"]} event
    @private*/ _onPointerDown(event)
    {
        const index = this._draggingPointers.findIndex((v) => v.id === event.pointerId)
        if (index !== -1)
            return;

        super.setPointerCapture(event.pointerId);

        const [viewX, viewY] = this.offsetToView(event.offsetX, event.offsetY);

        if (event.button === 0 || event.pointerType === "touch")
        {
            for (let element = /** @type {Node | null} */(event.target);
                element != null && element != this;
                element = element.parentNode)
            {
                const node = this._nodes.find((v) => element === v.element)
                if (node !== undefined && node.viewDraggable)
                {
                    node.element.setAttribute("view-dragging", "");
                    this._draggingPointers.push(
                    {
                        id: event.pointerId,
                        viewX: viewX,
                        viewY: viewY,
                        node: node,
                    });
                    return;
                }
            }
        }

        if (this.viewDraggable && (event.button === 1 || event.button === 2 || event.pointerType === "touch"))
        {
            this._draggingPointers.push(
            {
                id: event.pointerId,
                viewX: viewX,
                viewY: viewY,
            });

            super.setAttribute("view-dragging", "");
        }

        event.preventDefault();
    }

    /**
    @param {HTMLElementEventMap["pointerdown"]} event
    @private*/ _onPointerUp(event)
    {
        const index = this._draggingPointers.findIndex((v) => v.id === event.pointerId)
        if (index === -1)
            return;

        const pointer = this._draggingPointers[index];
        if ("node" in pointer)
            pointer.node.element.removeAttribute("view-dragging");
        else if (this._draggingPointers.some((v) => !("node" in v)))
            super.removeAttribute("view-dragging");

        this._draggingPointers.splice(index, 1);
        super.releasePointerCapture(pointer.id);

        event.preventDefault();
    }

    /**
    @param {(pointer: typeof this._draggingPointers[number]) => boolean} [filter]
    @private*/ _upAllPointers(filter)
    {
        this._draggingPointers = this._draggingPointers.filter((pointer) =>
        {
            if (filter !== undefined && !filter(pointer))
                return false;

            if ("node" in pointer)
                pointer.node.element.removeAttribute("view-dragging");

            super.releasePointerCapture(pointer.id);

            return true;
        });

        super.removeAttribute("view-dragging");
    }

    /**
    @param {HTMLElementEventMap["pointermove"]} event
    @private*/ _onPointerMove(event)
    {
        const pointer = this._draggingPointers.find((v) => v.id === event.pointerId)
        if (pointer === undefined)
            return;

        const [viewX, viewY] = this.offsetToView(event.offsetX, event.offsetY);

        if ("node" in pointer)
        {
            pointer.node.posX += viewX - pointer.viewX;
            pointer.node.posY += viewY - pointer.viewY;
            pointer.viewX = viewX;
            pointer.viewY = viewY;
            return;
        }

        this.viewX += pointer.viewX - viewX;
        this.viewY += pointer.viewY - viewY;
    }

    /**
    @param {HTMLElementEventMap["wheel"]} event
    @private*/ _onWheel(event)
    {
        if (!this.viewDraggable || event.deltaY === 0.0)
            return;

        const [beforeViewX, beforeViewY] = this.offsetToView(event.offsetX, event.offsetY);

        const scrollDelta = event.deltaY / -100;

        let newViewScale = this.viewScale * Math.pow(1.2, scrollDelta);
        if (scrollDelta < 0)
        {
            const min = this.minViewScale;
            if (newViewScale < min)
                newViewScale = min;
        }
        else
        {
            const max = this.maxViewScale;
            if (newViewScale > max)
                newViewScale = max;
        }

        this.viewScale = newViewScale;

        const [afterViewX, afterViewY] = this.offsetToView(event.offsetX, event.offsetY);

        this.viewX += beforeViewX - afterViewX;
        this.viewY += beforeViewY - afterViewY;

        event.preventDefault();
    }

    /**
    @param {typeof GraphView["observedAttributes"][number]} attributeName
    @param {string | null} oldValue
    @param {string | null} newValue
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        switch (attributeName)
        {
            case "view-x":
                super.style.setProperty("--view-x", String(this.viewX));
                break;
            case "view-y":
                super.style.setProperty("--view-y", String(this.viewY));
                break;
            case "view-scale":
                super.style.setProperty("--view-scale", String(this.viewScale));
                break;
            case "view-reference":
                super.style.setProperty("--view-reference", `${this.viewReference}px`);
                break;
            case "view-draggable":
                if (!this.viewDraggable)
                    this._upAllPointers();
                break;
        }
    }

    /**
    @param {number} x
    @param {number} y
    @returns {[x: number, y: number]}
    @public*/ viewToOffset(x, y)
    {
        const viewScaleInPixels = this.viewScaleInPixels;
        return (
        [
            ((x - this.viewX) * viewScaleInPixels) + (super.clientWidth * 0.5),
            ((y - this.viewY) * viewScaleInPixels) + (super.clientHeight * 0.5),
        ]);
    }

    /**
    @param {number} x
    @param {number} y
    @returns {[x: number, y: number]}
    @public*/ offsetToView(x, y)
    {
        const viewScaleInPixels = this.viewScaleInPixels;
        return (
        [
            ((x - (super.clientWidth * 0.5)) / viewScaleInPixels) + this.viewX,
            ((y - (super.clientHeight * 0.5)) / viewScaleInPixels) + this.viewY,
        ]);
    }

    /**
    @returns {GraphElement}
    @public*/ createGraphElement()
    {
        return this.appendGraphElement(new GraphElement());
    }

    /**
    @param {GraphElement} node

    @returns {GraphElement}
    @public*/ appendGraphElement(node)
    {
        // @ts-ignore
        if (node._graph !== this && node._graph !== null) // @ts-ignore
            node._graph.removeGraphElement(node);

        const index = this._nodes.indexOf(node);
        if (index !== -1)
            return node;

        this._nodes.push(node); // @ts-ignore
        node._graph = this; // @ts-ignore
        this._graphicElementContainer.appendChild(node.element);

        this._nodeMutationObserver.observe(node.element, { attributeFilter: ["view-draggable"] });

        return node;
    }

    /**
    @param {GraphElement} node
    @public*/ removeGraphElement(node)
    {
        const index = this._nodes.indexOf(node);
        if (index === -1)
            return;

        this._nodes.splice(index, 1); // @ts-ignore
        node._graph = null;
        this._graphicElementContainer.removeChild(node.element);

        this._nodeMutationObserver.disconnect();
        for (const node of this._nodes)
            this._nodeMutationObserver.observe(node.element, { attributeFilter: ["view-draggable"] });
    }
}
customElements.define("graph-view", GraphView);