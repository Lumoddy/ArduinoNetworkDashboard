import { CustomEventHandler } from "./custom-events.js";

/**
@typedef {{
    "dataSent": {
        target: SerialNode,
        data: string,
    },
    "dataRecived": {
        target: SerialNode,
        data: string,
    },
    "forgotten": {
        target: SerialNode,
    },
    "nodeConnected": {
        target: SerialNode,
        otherNode: SerialNode | SerialRootNode,
    },
    "nodeDisconnected": {
        target: SerialNode,
        otherNode: SerialNode | SerialRootNode,
    },
}} SerialNodeEventMap
*/

/**
*/ export class SerialNode
{
    /**
    @type {SerialNode[]}
    @private*/ static _registeredNodes = [];

    /**
    @type {readonly (SerialNode | SerialRootNode)[]}
    @public*/ get connectedNodes() { return this._connectedNodes }
    /**
    @type {boolean}
    @public*/ get isForgotten() { return this._connectedNodes.length === 0 }

    /**
    @param {SerialNode | SerialRootNode} connectedTo
    @public*/ constructor(connectedTo)
    {
        /**
        @type {(SerialNode | SerialRootNode)[]}
        @private*/ this._connectedNodes = [connectedTo];

        /**
        @type {CustomEventHandler<{ [K in keyof SerialNodeEventMap]: [event: SerialNodeEventMap[K]] }>}
        @private*/ this._eventHandler = new CustomEventHandler();

        SerialNode._registeredNodes.push(this);
    }

    /**
    @param {SerialNode | SerialRootNode} node
    @public*/ connectNode(node)
    {
        if (this.isForgotten)
            throw new Error("SerialNode cannot reconnect to other nodes since it was forgotten.");

        if (node instanceof SerialRootNode)
        {
            if (node.connectedNode !== this)
                return node.setConnectedNode(this);

            this._connectedNodes.push(node);
        }
        else
        {
            if (node._connectedNodes.indexOf(this) > 0)
            {
                console.warn("SerialNode is trying to connect to already connected node. Ignoring...");
                return;
            }

            this._connectedNodes.push(node);
            node._connectedNodes.push(this);

            node._eventHandler.dispatch("nodeConnected", {
                target: node,
                otherNode: this,
            });
        }

        this._eventHandler.dispatch("nodeConnected", {
            target: this,
            otherNode: node,
        });
    }

    /**
    @param {SerialNode | SerialRootNode} node
    @public*/ disconnectNode(node)
    {
        if (node instanceof SerialRootNode)
        {
            if (node.connectedNode === this)
                return node.setConnectedNode(null);

            const index = this._connectedNodes.indexOf(node);
            if (index === -1)
            {
                console.warn("SerialNode is trying to disconnect to already disconnected node. Ignoring...");
                return;
            }

            this._connectedNodes.splice(index, 1);
        }
        else
        {
            {
                const index = this._connectedNodes.indexOf(node);
                if (index === -1)
                {
                    console.warn("SerialNode is trying to disconnect to already disconnected node. Ignoring...");
                    return;
                }

                this._connectedNodes.splice(index, 1);
            }
            {
                const index = node._connectedNodes.indexOf(this);
                if (index === -1)
                    throw 0;

                node._connectedNodes.splice(index, 1);
            }

            node._eventHandler.dispatch("nodeDisconnected", {
                target: node,
                otherNode: this,
            });

            if (!node._canPathToRoot())
            {
                SerialNode._registeredNodes.splice(SerialNode._registeredNodes.indexOf(node), 1);
                node._connectedNodes.splice(0);

                node._eventHandler.dispatch("forgotten", {
                    target: node,
                });
            }
        }

        this._eventHandler.dispatch("nodeDisconnected", {
            target: this,
            otherNode: node,
        });

        if (!this._canPathToRoot())
        {
            SerialNode._registeredNodes.splice(SerialNode._registeredNodes.indexOf(this), 1);
            this._connectedNodes.splice(0);

            this._eventHandler.dispatch("forgotten", {
                target: this,
            });
        }
    }

    /**
    @param {((node: SerialNode) => boolean) | [SerialNode]} [disallow]
    @private*/ _canPathToRoot(disallow)
    {
        const checkedNodes = /** @type {[boolean, SerialNode][]} */([[true, this]]);

        if (disallow instanceof Array)
        {
            for (const disallowed of disallow)
            {
                if (checkedNodes.findIndex((v) => v[1] === disallowed) > 0)
                    continue;

                checkedNodes.push([false, disallowed]);
            }

            disallow = undefined;
        }

        for (let i = 0; i < checkedNodes.length; i++)
        {
            if (!checkedNodes[i][0])
                continue;

            for (const connected of checkedNodes[i][1]._connectedNodes)
            {
                if (connected instanceof SerialRootNode)
                    return true;

                if (checkedNodes.findIndex((v) => v[1] === connected) > 0)
                    continue;

                checkedNodes.push([disallow === undefined || !disallow(connected), connected]);
            }
        }

        return false;
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
    "nodeConnected": {
        target: SerialRootNode,
        otherNode: SerialNode,
    },
    "nodeDisconnected": {
        target: SerialRootNode,
        otherNode: SerialNode,
    },
}} SerialRootNodeEventMap
*/

/**
*/ export class SerialRootNode
{
    /**
    @type {SerialRootNode[]}
    @private*/ static _registeredRoots = [];

    /**
    @param {SerialPort} port
    @public */ constructor(port)
    {
        for (const root of SerialRootNode._registeredRoots)
            if (root._port === port)
                throw new Error("Cannot construct multiple SerialRootNodes with the same SerialPort.");

        /**
        @type {SerialNode | null}
        @private*/ this._connectedNode = null;

        /**
        @type {SerialPort}
        @private*/ this._port = port;

        /**
        @type {CustomEventHandler<{ [K in keyof SerialRootNodeEventMap]: [event: SerialRootNodeEventMap[K]] }>}
        @private*/ this._eventHandler = new CustomEventHandler();

        SerialRootNode._registeredRoots.push(this);
    }

    /**
    @type {SerialNode | null}
    @public*/ get connectedNode() { return this._connectedNode }

    /**
    @type {SerialPort}
    @public*/ get port() { return this._port }

    /**
    @param {SerialNode | null} node
    @public*/ setConnectedNode(node)
    {
        if (node === this._connectedNode)
        {
            console.warn("SerialRootNode is trying to connect to already connected node. Ignoring...");
            return;
        }

        const oldNode = this._connectedNode;
        this._connectedNode = node;

        if (oldNode !== null)
        {
            oldNode.disconnectNode(this);

            this._eventHandler.dispatch("nodeDisconnected", {
                target: this,
                otherNode: oldNode,
            });
        }

        if (node !== null)
        {
            node.connectNode(this);

            this._eventHandler.dispatch("nodeConnected", {
                target: this,
                otherNode: node,
            });
        }
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

/**
@typedef {{
    "dataSent": {
        target: PlaceholderSerialNode,
        targetNode: SerialNode,
        data: string,
    },
    "dataRecived": {
        target: PlaceholderSerialNode,
        targetNode: SerialNode,
        data: string,
    },
    "nodeConnected": {
        target: PlaceholderSerialNode,
        targetNode: SerialNode,
        otherNode: SerialNode | SerialRootNode,
    },
    "nodeDisconnected": {
        target: PlaceholderSerialNode,
        targetNode: SerialNode,
        otherNode: SerialNode | SerialRootNode,
    },
    "serialNodeAssigned": {
        target: PlaceholderSerialNode,
        serialNode: SerialNode,
    },
    "serialNodeUnassigned": {
        target: PlaceholderSerialNode,
        serialNode: SerialNode,
    },
}} PlaceholderSerialNodeEventMap
*/

/**
*/ export class PlaceholderSerialNode
{
    /**
    @param {SerialNode | null} [serialNode]
    @public*/ constructor(serialNode)
    {
        /**
        @type {SerialNode | null}
        @public*/ this._serialNode = null;

        /**
        @type {CustomEventHandler<{ [K in keyof PlaceholderSerialNodeEventMap]: [event: PlaceholderSerialNodeEventMap[K]] }>}
        @private*/ this._eventHandler = new CustomEventHandler();

        /**
        @type {(event: SerialNodeEventMap["dataSent"]) => void}
        @private*/ this._dataSentEventListener = (e) =>
        {
            if (this._serialNode === null)
                throw 0;

            this._eventHandler.dispatch("dataSent", {
                target: this,
                targetNode: this._serialNode,
                data: e.data,
            });
        }

        /**
        @type {(event: SerialNodeEventMap["dataRecived"]) => void}
        @private*/ this._dataRecivedEventListener = (e) =>
        {
            if (this._serialNode === null)
                throw 0;

            this._eventHandler.dispatch("dataRecived", {
                target: this,
                targetNode: this._serialNode,
                data: e.data,
            });
        }

        /**
        @type {(event: SerialNodeEventMap["nodeConnected"]) => void}
        @private*/ this._nodeConnectedEventListener = (e) =>
        {
            if (this._serialNode === null)
                throw 0;

            this._eventHandler.dispatch("nodeConnected", {
                target: this,
                targetNode: this._serialNode,
                otherNode: e.otherNode,
            });
        }

        /**
        @type {(event: SerialNodeEventMap["nodeDisconnected"]) => void}
        @private*/ this._nodeDisconnectedEventListener = (e) =>
        {
            if (this._serialNode === null)
                throw 0;

            this._eventHandler.dispatch("nodeDisconnected", {
                target: this,
                targetNode: this._serialNode,
                otherNode: e.otherNode,
            });
        }

        /**
        @type {(event: SerialNodeEventMap["forgotten"]) => void}
        @private*/ this._forgottenEventListener = () => { this.setAssignedSerialNode(null) }

        if (serialNode !== undefined)
            this.setAssignedSerialNode(serialNode);
    }


    /**
    @param {SerialNode | null} node
    @public*/ setAssignedSerialNode(node)
    {
        const oldNode = this._serialNode;
        this._serialNode = node;

        if (oldNode !== null)
        {
            oldNode.removeEventListener("dataSent", this._dataSentEventListener);
            oldNode.removeEventListener("dataRecived", this._dataRecivedEventListener);
            oldNode.removeEventListener("forgotten", this._forgottenEventListener);
            oldNode.removeEventListener("nodeConnected", this._nodeConnectedEventListener);
            oldNode.removeEventListener("nodeDisconnected", this._nodeDisconnectedEventListener);

            this._eventHandler.dispatch("serialNodeUnassigned", {
                target: this,
                serialNode: oldNode,
            });
        }

        if (node !== null)
        {
            node.removeEventListener("dataSent", this._dataSentEventListener);
            node.removeEventListener("dataRecived", this._dataRecivedEventListener);
            node.removeEventListener("forgotten", this._forgottenEventListener);
            node.removeEventListener("nodeConnected", this._nodeConnectedEventListener);
            node.removeEventListener("nodeDisconnected", this._nodeDisconnectedEventListener);

            this._eventHandler.dispatch("serialNodeAssigned", {
                target: this,
                serialNode: node,
            });
        }
    }

    /**
    @template {keyof PlaceholderSerialNodeEventMap} N
    @param {N} type
    @param {(event: PlaceholderSerialNodeEventMap[N]) => void} listener
    @public*/ addEventListener(type, listener) { this._eventHandler.addListener(type, listener) }

    /**
    @template {keyof PlaceholderSerialNodeEventMap} N
    @param {N} type
    @param {(event: PlaceholderSerialNodeEventMap[N]) => void} listener
    @public*/ removeEventListener(type, listener) { this._eventHandler.removeListener(type, listener) }
}