import { GraphElement } from "./graph-element.js";
import { GraphView } from "./graph-view.js";
import { NetworkGraphModule } from "../network/network-graph-module.js";
/**
@import { NetworkNodeID } from "../network/network-graph-module.js"
*/

/**
*/ export class NetworkGraphView extends GraphView
{
    /**
    @type {Map<NetworkNodeID,
    {
        element: GraphElement,
        connections: Map<NetworkNodeID | SerialPort, { element: GraphElement }>,
    }>}
    @private*/ _graphNodeSlotElements = new Map();

    /**
    @type {Map<SerialPort,
    {
        element: GraphElement,
        connections: Map<NetworkNodeID, { element: GraphElement }>,
    }>}
    @private*/ _graphPortSlotElements = new Map();

    /**
    @type {NetworkGraphModule}
    @private*/ _module = new NetworkGraphModule();

    /**
    @override
    @protected @readonly*/ static observedAttributes =
    [
        ...GraphView.observedAttributes,
    ];

    /**
    @public*/ constructor()
    {
        super();

        {
            const node = super.createGraphElement();
            node.viewDraggable = true;
            const circle = node.element.appendChild(document.createElementNS("http://www.w3.org/2000/svg", "circle"));
            circle.setAttribute("r", "10");
            circle.setAttribute("fill", "black");
        }
        {
            const node = super.createGraphElement();
            node.viewDraggable = true;
            const circle = node.element.appendChild(document.createElementNS("http://www.w3.org/2000/svg", "circle"));
            circle.setAttribute("r", "10");
            circle.setAttribute("fill", "red");
        }

        this._module.addEventListener("portAdded", (e) =>
        {
            const element = this.appendGraphElement(new GraphElement());

            this._graphPortSlotElements.set(e.port,
            {
                element: element,
                connections: new Map(),
            });
        });
        this._module.addEventListener("portRemoved", (e) =>
        {
            const portSlot = this._graphPortSlotElements.get(e.port);
            if (portSlot === undefined)
                return;

            this.removeGraphElement(portSlot.element);
            this._graphPortSlotElements.delete(e.port);
        });

        this._module.addEventListener("nodeAdded", (e) =>
        {
            const element = this.appendGraphElement(new GraphElement());

            this._graphNodeSlotElements.set(e.node,
            {
                element: element,
                connections: new Map(),
            });
        });
        this._module.addEventListener("nodeRemoved", (e) =>
        {
            const slot = this._graphNodeSlotElements.get(e.node);
            if (slot === undefined)
                return;

            this.removeGraphElement(slot.element);
            this._graphNodeSlotElements.delete(e.node);
        });

        this._module.addEventListener("connectionAdded", (e) =>
        {
            const element = this.appendGraphElement(new GraphElement());

            if (e.fromIsPort)
            {
                const portSlot = this._graphPortSlotElements.get(e.from);
                if (portSlot === undefined)
                    return;

                portSlot.connections.set(e.to,
                {
                    element: element,
                });
            }
            else
            {
                const nodeSlot = this._graphNodeSlotElements.get(e.from);
                if (nodeSlot === undefined)
                    return;

                nodeSlot.connections.set(e.to,
                {
                    element: element,
                });
            }

            const otherNodeSlot = this._graphNodeSlotElements.get(e.to);
            if (otherNodeSlot === undefined)
                return;

            otherNodeSlot.connections.set(e.from,
            {
                element: element,
            });
        });
        this._module.addEventListener("connectionRemoved", (e) =>
        {
            if (e.fromIsPort)
            {
                const portSlot = this._graphPortSlotElements.get(e.from);
                if (portSlot === undefined)
                    return;

                const connection = portSlot.connections.get(e.to);
                if (connection === undefined)
                    return;

                connection.element.remove();
                portSlot.connections.delete(e.to);
            }
            else
            {
                const nodeSlot = this._graphNodeSlotElements.get(e.from);
                if (nodeSlot === undefined)
                    return;

                const connection = nodeSlot.connections.get(e.to);
                if (connection === undefined)
                    return;

                connection.element.remove();
                nodeSlot.connections.delete(e.to);
            }

            const otherNodeSlot = this._graphNodeSlotElements.get(e.to);
            if (otherNodeSlot === undefined)
                return;

            const otherConnection = otherNodeSlot.connections.get(e.from);
            if (otherConnection === undefined)
                return;

            otherNodeSlot.connections.delete(e.from);
        });
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