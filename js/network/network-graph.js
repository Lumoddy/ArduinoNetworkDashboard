import { GraphNode } from "../graphics/graph-node.js";
import { GraphView } from "../graphics/graph-view.js";
import { NetworkNode, NetworkPort } from "./network-node.js";

/**
*/ export class NetworkGraphView extends GraphView
{
    /**
    @type {{ graphNode: GraphNode, heldNode: NetworkNode | null }[]}
    @private*/ _graphSlotNodes = [];

    /**
    @type {GraphNode[]}
    @private*/ _graphPortSlotNodes = [];

    /**
    @type {NetworkNode[]}
    @private*/ _graphNetworkNodes = [];

    /**
    @type {NetworkPort[]}
    @private*/ _graphPortNetworkNodes = [];

    /**
    @override
    @protected @readonly*/ static observedAttributes =
    [
        ...GraphView.observedAttributes
    ];

    /**
    @public*/ constructor()
    {
        super();

        {
            const node = super.createGraphNode();
            node.draggable = true;
            const circle = node.element.appendChild(document.createElementNS("http://www.w3.org/2000/svg", "circle"));
            circle.setAttribute("r", "10");
            circle.setAttribute("fill", "black");
        }
        {
            const node = super.createGraphNode();
            node.draggable = true;
            const circle = node.element.appendChild(document.createElementNS("http://www.w3.org/2000/svg", "circle"));
            circle.setAttribute("r", "10");
            circle.setAttribute("fill", "red");
        }
    }

    /**
    @override
    @protected*/ connectedCallback()
    {
        super.connectedCallback();
    }

    /**
    @override
    @protected*/ disconnectedCallback()
    {
        super.disconnectedCallback();
    }

    /**
    @param {typeof NetworkGraphView["observedAttributes"][number]} attributeName
    @param {string | null} oldValue
    @param {string | null} newValue
    @override
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        switch (attributeName)
        {
            default:
                super.attributeChangedCallback(attributeName, oldValue, newValue);
                break;
        }
    }
}
customElements.define("network-graph-view", NetworkGraphView);