import { CustomEventHandler } from "./custom-events.js";
import { SerialNode, SerialRootNode } from "./serial-nodes.js";

/**
@typedef {{
    "serialNodeChanged":
    {
        target: NetworkNode,
        oldSerialNode: SerialNode | null,
        newSerialNode: SerialNode | null,
    },
    "serialNodeConnected":
    {
        target: NetworkNode,
        targetSerialNode: SerialNode,
        otherSerialNode: SerialNode | SerialRootNode,
    },
    "serialNodeDisconnected":
    {
        target: NetworkNode,
        targetSerialNode: SerialNode,
        otherSerialNode: SerialNode | SerialRootNode,
    },
}} NetworkNodeEventMap
*/

/**
*/ export class NetworkNode
{
    /**
    @public*/ constructor()
    {
        /**
        @type {SerialNode | null}
        @private*/ this._serialNode = null;

        /**
        @type {CustomEventHandler<{ [K in keyof NetworkNodeEventMap]: [event: NetworkNodeEventMap[K]] }>}
        @private*/ this._eventHandler = new CustomEventHandler();

        /**
        @type {(event: import("./serial-nodes").SerialNodeEventMap["nodeConnected"]) => void}
        @private*/ this._serialOnNodeConnected = (e) =>
        {
            this._eventHandler.dispatch("serialNodeConnected", {
                target: this,
                targetSerialNode: e.target,
                otherSerialNode: e.otherNode,
            });
        };

        /**
        @type {(event: import("./serial-nodes").SerialNodeEventMap["nodeConnected"]) => void}
        @private*/ this._serialOnNodeDisconnected = (e) =>
        {
            this._eventHandler.dispatch("serialNodeDisconnected", {
                target: this,
                targetSerialNode: e.target,
                otherSerialNode: e.otherNode,
            });
        };
    }

    /**
    @returns {SerialNode | null}
    @public*/ get serialNode() { return this._serialNode }
    /**
    @param {SerialNode | null} node
    @public*/ setSerialNode(node)
    {
        if (this._serialNode !== null)
        {
            this._serialNode.addEventListener("nodeConnected", this._serialOnNodeConnected);
            this._serialNode.addEventListener("nodeDisconnected", this._serialOnNodeDisconnected);
        }

        const oldNode = this._serialNode;
        this._serialNode = node;

        this._eventHandler.dispatch("serialNodeChanged", {
            target: this,
            oldSerialNode: oldNode,
            newSerialNode: node,
        });
    }
}

/**
@typedef {{
    "serialNodeChanged":
    {
        target: NetworkRootNode,
        oldSerialNode: SerialRootNode | null,
        newSerialNode: SerialRootNode | null,
    },
    "serialNodeConnected":
    {
        target: NetworkRootNode,
        targetSerialNode: SerialRootNode,
        otherSerialNode: SerialNode | SerialRootNode,
    },
    "serialNodeDisconnected":
    {
        target: NetworkRootNode,
        targetSerialNode: SerialRootNode,
        otherSerialNode: SerialNode | SerialRootNode,
    },
}} NetworkRootNodeEventMap
*/

/**
*/ export class NetworkRootNode
{
    /**
    @public*/ constructor()
    {
        /**
        @type {SerialRootNode | null}
        @private*/ this._serialNode = null;

        /**
        @type {CustomEventHandler<{ [K in keyof NetworkRootNodeEventMap]: [event: NetworkRootNodeEventMap[K]] }>}
        @private*/ this._eventHandler = new CustomEventHandler();

        /**
        @type {(event: import("./serial-nodes").SerialRootNodeEventMap["nodeConnected"]) => void}
        @private*/ this._serialOnNodeConnected = (e) =>
        {
            this._eventHandler.dispatch("serialNodeConnected", {
                target: this,
                targetSerialNode: e.target,
                otherSerialNode: e.otherNode,
            });
        };

        /**
        @type {(event: import("./serial-nodes").SerialRootNodeEventMap["nodeConnected"]) => void}
        @private*/ this._serialOnNodeDisconnected = (e) =>
        {
            this._eventHandler.dispatch("serialNodeDisconnected", {
                target: this,
                targetSerialNode: e.target,
                otherSerialNode: e.otherNode,
            });
        };
    }

    /**
    @returns {SerialRootNode | null}
    @public*/ get serialNode() { return this._serialNode }
    /**
    @param {SerialRootNode | null} node
    @public*/ setSerialNode(node)
    {
        if (this._serialNode !== null)
        {
            this._serialNode.addEventListener("nodeConnected", this._serialOnNodeConnected);
            this._serialNode.addEventListener("nodeDisconnected", this._serialOnNodeDisconnected);
        }

        const oldNode = this._serialNode;
        this._serialNode = node;

        this._eventHandler.dispatch("serialNodeChanged", {
            target: this,
            oldSerialNode: oldNode,
            newSerialNode: node,
        });
    }
}