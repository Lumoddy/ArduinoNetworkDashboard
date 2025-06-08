import { GraphNode } from "./graph-node.js";

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
    @private*/ _resizeObserver;

    /**
    @type {SVGSVGElement}
    @private*/ _graphicContainer;

    /**
    @type {SVGGElement}
    @private*/ _graphicElementContainer;

    /**
    @type {GraphNode[]}
    @private*/ _nodes = [];

    /**
    @type {(
        | { id: number, viewX: number, viewY: number }
        | { id: number, viewX: number, viewY: number, node: GraphNode }
    )[]}
    @private*/ _draggingPointers = [];

    /**
    @returns {number}
    @public*/ get viewX()
    {
        const result = Number(super.getAttribute("view-x"));
        if (Number.isNaN(result))
            return 0;
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
            return 0;
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
        if (Number.isNaN(result))
            return 0;
        return result;
    }
    /**
    @public*/ set viewScale(value)
    {
        super.setAttribute("view-scale", String(value));
    }

    /**
    @returns {number}
    @public*/ get viewScaleInPixels()
    {
        switch (this.viewReference)
        {
            case "width": return this.viewScale * super.clientWidth;
            case "height": return this.viewScale * super.clientHeight;
            case "pixel": return this.viewScale;
        }
    }
    /**
    @public*/ set viewScaleInPixels(value)
    {
        switch (this.viewReference)
        {
            case "width": this.viewScale = value / super.clientWidth;
            case "height": this.viewScale = value / super.clientHeight;
            case "pixel": this.viewScale = value;
        }
    }

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
    @public*/ constructor()
    {
        super();

        super.style.padding = "0";

        this._resizeObserver = new ResizeObserver(() =>
        {
            super.style.setProperty("--view-scale", `${this.viewScaleInPixels}px`);
            this._graphicContainer.setAttribute("width", String(super.clientWidth));
            this._graphicContainer.setAttribute("height", String(super.clientHeight));
        });

        this._graphicContainer =
            super.querySelector("& > svg:not([width], [height])")
            ?? super.appendChild(document.createElementNS("http://www.w3.org/2000/svg", "svg"));

        this._graphicContainer.style.position = "position";
        this._graphicContainer.style.top = "0";
        this._graphicContainer.style.left = "0";

        this._graphicElementContainer =
            this._graphicContainer.querySelector("& > g")
            ?? this._graphicContainer.appendChild(document.createElementNS("http://www.w3.org/2000/svg", "g"));
    }

    /**
    @protected*/ connectedCallback()
    {
        this._resizeObserver.observe(this);

        super.addEventListener("pointerdown", this._onPointerDown);
        super.addEventListener("pointerup", this._onPointerUp);
        super.addEventListener("pointercancel", this._onPointerUp);
        super.addEventListener("pointermove", this._onPointerMove);
    }

    /**
    @protected*/ disconnectedCallback()
    {
        this._resizeObserver.unobserve(this);

        super.removeEventListener("pointerdown", this._onPointerDown);
        super.removeEventListener("pointerup", this._onPointerUp);
        super.removeEventListener("pointercancel", this._onPointerUp);
        super.removeEventListener("pointermove", this._onPointerMove);
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

        for (let element = /** @type {Node | null} */(event.target);
            element != null && element != this;
            element = element.parentNode)
        {
            const node = this._nodes.find((v) => element === v.element)
            if (node !== undefined)
            {
                node.element.setAttribute("dragging", "");
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

        this._draggingPointers.push(
        {
            id: event.pointerId,
            viewX: viewX,
            viewY: viewY,
        });
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
            pointer.node.element.removeAttribute("dragging");

        this._draggingPointers.splice(index, 1);
        super.releasePointerCapture(pointer.id);
    }

    /**
    @private*/ _upAllPointers()
    {
        for (const pointer of this._draggingPointers)
        {
            if ("node" in pointer)
                pointer.node.element.removeAttribute("dragging");

            super.releasePointerCapture(pointer.id);
        }
        this._draggingPointers.length = 0;
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
            case "view-reference":
                super.style.setProperty("--view-scale", `${this.viewScaleInPixels}px`);
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
    @returns {GraphNode}
    @public*/ createGraphNode()
    {
        return this.appendGraphNode(new GraphNode());
    }

    /**
    @param {GraphNode} node
    @returns {GraphNode}
    @public*/ appendGraphNode(node)
    {
        const privateNode = /**
        @type {Pick<GraphNode, keyof GraphNode> &
        {
            _graph: GraphView | null,
            _element: SVGGElement | undefined,
        }}
        */(/** @type {unknown} */(node));

        if (privateNode._graph !== this && privateNode._graph !== null)
            privateNode._graph.removeGraphNode(node);

        const index = this._nodes.indexOf(node);
        if (index !== -1)
            return node;

        this._nodes.push(node);
        privateNode._graph = this;
        if (privateNode._element !== undefined)
            this._graphicElementContainer.appendChild(privateNode._element);
        return node;
    }

    /**
    @param {GraphNode} node
    @public*/ removeGraphNode(node)
    {
        const privateNode = /**
        @type {Pick<GraphNode, keyof GraphNode> &
        {
            _graph: GraphView | null,
            _element: SVGGElement | undefined,
        }}
        */(/** @type {unknown} */(node));

        const index = this._nodes.indexOf(node);
        if (index === -1)
            return;

        this._nodes.splice(index, 1);
        privateNode._graph = null;
        if (privateNode._element !== undefined)
            this._graphicElementContainer.removeChild(privateNode._element);
    }
}
customElements.define("graph-view", GraphView);