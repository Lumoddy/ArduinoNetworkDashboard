import { CustomEventHandler } from "./custom-events.js";
import { LoudArray } from "./loud.js";

/**
@typedef {{
    "nodeConnected":
    {
        target: SerialNode,
        otherNode: SerialNode | SerialRootNode,
    },
    "nodeDisconnected":
    {
        target: SerialNode,
        otherNode: SerialNode | SerialRootNode,
    },
}} SerialNodeEventMap
*/

/**
*/ export class SerialNode
{
    /**
    @public*/ constructor()
    {
        /**
        @type {(SerialNode | SerialRootNode)[]}
        @private*/ this._connectedTo = [];

        /**
        @type {CustomEventHandler<{ [K in keyof SerialNodeEventMap]: [event: SerialNodeEventMap[K]] }>}
        @private*/ this._eventHandler = new CustomEventHandler();
    }

    /**
    @type {readonly (SerialNode | SerialRootNode)[]}
    @public*/ get connectedTo() { return this._connectedTo }

    /**
    @param {SerialNode | SerialRootNode} node
    @public*/ connectNode(node)
    {
        if (this.connectedTo.indexOf(node) > 0)
            return;

        this._connectedTo.push(node);

        node.connectNode(this);

        this._eventHandler.dispatch("nodeConnected", {
            otherNode: node,
            target: this,
        });
    }

    /**
    @param {SerialNode | SerialRootNode} node
    @public*/ disconnectNode(node)
    {
        const index = this.connectedTo.indexOf(node);

        if (index <= 0)
            return;

        this._connectedTo.splice(index, 1);

        node.disconnectNode(this);

        this._eventHandler.dispatch("nodeDisconnected", {
            otherNode: node,
            target: this,
        });
    }

    /**
    @template {keyof SerialNodeEventMap} N
    @param {N} type
    @param {(event: SerialNodeEventMap[N]) => void} listener
    @public*/ addEventListener(type, listener) { this._eventHandler.addListener(type, listener) }

    /**
    @template {keyof SerialNodeEventMap} N
    @param {N} type
    @param {(event: SerialNodeEventMap[N]) => void} listener
    @public*/ removeEventListener(type, listener) { this._eventHandler.removeListener(type, listener) }
}

/**
@typedef {{
    "nodeConnected":
    {
        target: SerialRootNode,
        otherNode: SerialNode | SerialRootNode,
    },
    "nodeDisconnected":
    {
        target: SerialRootNode,
        otherNode: SerialNode | SerialRootNode,
    },
}} SerialRootNodeEventMap
*/

/**
*/ export class SerialRootNode
{
    /**
    @param {SerialPort} port
    @public */ constructor(port)
    {
        /**
        @type {LoudArray<SerialNode>}
        @private*/ this._connectedTo = new LoudArray();

        /**
        @type {SerialPort}
        @private*/ this._port = port;

        /**
        @type {CustomEventHandler<{ [K in keyof SerialRootNodeEventMap]: [event: SerialRootNodeEventMap[K]] }>}
        @private*/ this._eventHandler = new CustomEventHandler();
    }

    /**
    @type {readonly SerialNode[]}
    @public*/ get connectedTo() { return this._connectedTo.value }

    /**
    @type {SerialPort}
    @public*/ get port() { return this._port }

    /**
    @param {SerialNode} node
    @public*/ connectNode(node)
    {
        if (this.connectedTo.indexOf(node) > 0)
            return;

        this._connectedTo.push(node);

        node.connectNode(this);

        this._eventHandler.dispatch("nodeConnected", {
            otherNode: node,
            target: this,
        });
    }

    /**
    @param {SerialNode} node
    @public*/ disconnectNode(node)
    {
        const index = this.connectedTo.indexOf(node);

        if (index <= 0)
            return;

        this._connectedTo.splice(index, 1);

        node.disconnectNode(this);

        this._eventHandler.dispatch("nodeDisconnected", {
            otherNode: node,
            target: this,
        });
    }

    /**
    @template {keyof SerialRootNodeEventMap} N
    @param {N} type
    @param {(event: SerialRootNodeEventMap[N]) => void} listener
    @public*/ addEventListener(type, listener) { this._eventHandler.addListener(type, listener) }

    /**
    @template {keyof SerialRootNodeEventMap} N
    @param {N} type
    @param {(event: SerialRootNodeEventMap[N]) => void} listener
    @public*/ removeEventListener(type, listener) { this._eventHandler.removeListener(type, listener) }
}