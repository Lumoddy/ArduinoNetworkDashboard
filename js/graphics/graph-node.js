import { GraphView } from "./graph-view.js";

/**
*/ export class GraphNode
{
    /**
    @type {GraphView | null}
    @private*/ _graph = null;

    /**
    @type {MutationObserver | null}
    @private*/ _mutationObserver = null;

    /**
    @type {SVGGElement | undefined}
    @private*/ _element = undefined;

    /**
    @returns {GraphView | null}
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
    @returns {SVGGElement}
    @public*/ get element()
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
                this._graph.appendChild(this._element);
        }

        return this._element;
    }

    /**
    @public*/ constructor() { }

    /**
    @template {SVGElement} T
    @param {T} node
    @public*/ appendChild(node)
    {
        this.element.appendChild(node);
    }

    /**
    @template {keyof SVGElementTagNameMap} K
    @overload
    @param {K} selectors
    @returns {SVGElementTagNameMap[K] | null}
    *//**
    @template {SVGElement} E
    @overload
    @param {string} selectors
    @returns {E | null}
    *//**
    @param {string} selectors
    @returns {SVGElement | null}
    @public*/ querySelector(selectors)
    {
        if (this._element === undefined)
            return null;

        return this._element.querySelector(selectors);
    }

    /**
    @template {keyof SVGElementTagNameMap} K
    @overload
    @param {K} selectors
    @returns {NodeListOf<SVGElementTagNameMap[K]>}
    *//**
    @template {SVGElement} E
    @overload
    @param {string} selectors
    @returns {NodeListOf<E>}
    *//**
    @param {string} selectors
    @returns {NodeListOf<SVGGElement>}
    @public*/ querySelectorAll(selectors)
    {
        if (this._element === undefined)
        {
            const result = /** @type {NodeListOf<SVGGElement> & []} */([]);
            result.item = function (index) { return this[index] };
            return result;
        }

        return this._element.querySelectorAll(selectors);
    }
}