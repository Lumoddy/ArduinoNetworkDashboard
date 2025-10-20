import { GraphView } from "./graph-view.js";

/**
*/ export class GraphElement
{
    /**
    @type {GraphView?}
    @private*/ _graph = null;

    /**
    @type {MutationObserver?}
    @private*/ _mutationObserver = null;

    /**
    @type {SVGGElement | undefined}
    @private*/ _element = undefined;

    /**
    @returns {GraphView?}
    @public @readonly*/ get graph()
    {
        return this._graph;
    }

    /**
    @returns {number}
    @public*/ get posX()
    {
        if (this._element === undefined)
            return 0;

        const result = Number(this._element.getAttribute("pos-x"));
        if (Number.isNaN(result))
            return 0;

        return result;
    }
    /**
    @public*/ set posX(value)
    {
        this.element.setAttribute("pos-x", String(value));
    }

    /**
    @returns {number}
    @public*/ get posY()
    {
        if (this._element === undefined)
            return 0;

        const result = Number(this._element.getAttribute("pos-y"));
        if (Number.isNaN(result))
            return 0;

        return result;
    }
    /**
    @public*/ set posY(value)
    {
        this.element.setAttribute("pos-y", String(value));
    }

    /**
    @returns {boolean}
    @public*/ get viewDraggable()
    {
        if (this._element === undefined)
            return false;

        const result = this._element.getAttribute("view-draggable");
        return result === "" || result === "true";
    }
    /**
    @public*/ set viewDraggable(value)
    {
        if (value)
            this.element.setAttribute("view-draggable", "");
        else if (this._element !== undefined)
            this._element.removeAttribute("view-draggable");
    }

    /**
    @returns {boolean}
    @public @readonly*/ get initialized() { return this._element !== undefined }

    /**
    @returns {SVGGElement}
    @public @readonly*/ get element()
    {
        if (this._element === undefined)
        {
            this._element = document.createElementNS("http://www.w3.org/2000/svg", "g");

            this._mutationObserver = new MutationObserver((entries) =>
            {
                for (const entry of entries)
                {
                    switch (entry.attributeName)
                    {
                        case "pos-x":
                            if (this._element !== undefined)
                                this._element.style.setProperty("--pos-x", String(this.posX));
                            break;
                        case "pos-y":
                            if (this._element !== undefined)
                                this._element.style.setProperty("--pos-y", String(this.posY));
                            break;
                    }
                }
            });
            this._mutationObserver.observe(this._element, { attributeFilter: ["pos-x", "pos-y"] });

            if (this._graph !== null)
                this._graph.container.appendChild(this._element);
        }

        return this._element;
    }

    /**
    @public*/ constructor() { }

    /**
    @public*/ remove()
    {
        if (this._graph !== null)
            this._graph.removeGraphElement(this);
    }
}