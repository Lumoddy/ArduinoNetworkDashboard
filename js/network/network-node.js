
/**
@export @typedef {{
    "connectionAdded": [event: { target: NetworkPort, other: NetworkNode }],
    "connectionRemoved": [event: { target: NetworkPort, other: NetworkNode }],
}} NetworkPortEventMap
*/

/**
*/ export class NetworkPort
{
    /**
    @returns {SerialPort}
    @public @readonly*/ get port() { return this._port }

    /**
    @returns {readonly NetworkNode[]}
    @public*/ get connections() { return this._connections }

    /**
    @param {SerialPort} port
    @public*/ constructor(port)
    {
        /**
        @type {SerialPort}
        @private*/ this._port = port;

        /**
        @type {NetworkNode[]}
        @private*/ this._connections = [];

        /**
        @type {{ [K in keyof NetworkPortEventMap]: ((...args: NetworkPortEventMap[K]) => void)[] | null }}
        @private*/ this._eventListeners =
        {
            "connectionAdded": null,
            "connectionRemoved": null,
        };
    }

    /**
    @template {keyof NetworkPortEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: NetworkPortEventMap[K]) => void} listener
    @public*/ addEventListener(type, listener)
    {
        const listenerList = this._eventListeners[type];
        if (listenerList === null)
            this._eventListeners[type] = [listener];
        else
            listenerList.push(listener);
    }

    /**
    @template {keyof NetworkPortEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: NetworkPortEventMap[K]) => void} listener
    @public*/ removeEventListener(type, listener)
    {
        const listenerList = this._eventListeners[type];
        if (listenerList === null)
            return;

        const index = listenerList.indexOf(listener);
        if (index !== -1)
            listenerList.splice(index, 1);
    }

    /**
    @template {keyof NetworkPortEventMap} const K
    @param {K} type
    @param {NetworkPortEventMap[K]} parameters
    @private*/ _dispatchEvent(type, ...parameters)
    {
        const listenerList = this._eventListeners[type];
        if (listenerList === null)
            return;

        for (const listener of listenerList)
            listener(...parameters);
    }
}

/**
@export @typedef {{
    "connectionAdded": [event: { target: NetworkNode, other: NetworkNode | NetworkPort }],
    "connectionRemoved": [event: { target: NetworkNode, other: NetworkNode | NetworkPort }],
}} NetworkNodeEventMap
*/

/**
*/ export class NetworkNode
{
    /**
    @returns {bigint}
    @public @readonly*/ get id() { return this._id }

    /**
    @returns {readonly (NetworkNode | NetworkPort)[]}
    @public*/ get connections() { return this._connections }

    /**
    @param {NetworkNode | NetworkPort} connectedTo
    @param {bigint} id
    @public*/ constructor(connectedTo, id)
    {
        /**
        @type {bigint}
        @private*/ this._id = id;

        /**
        @type {(NetworkNode | NetworkPort)[]}
        @private*/ this._connections = [connectedTo];

        /**
        @type {{ [K in keyof NetworkNodeEventMap]: ((...args: NetworkNodeEventMap[K]) => void)[] | null }}
        @private*/ this._eventListeners =
        {
            "connectionAdded": null,
            "connectionRemoved": null,
        };
    }

    /**
    @template {keyof NetworkNodeEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: NetworkNodeEventMap[K]) => void} listener
    @public*/ addEventListener(type, listener)
    {
        const listenerList = this._eventListeners[type];
        if (listenerList === null)
            this._eventListeners[type] = [listener];
        else
            listenerList.push(listener);
    }

    /**
    @template {keyof NetworkNodeEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: NetworkNodeEventMap[K]) => void} listener
    @public*/ removeEventListener(type, listener)
    {
        const listenerList = this._eventListeners[type];
        if (listenerList === null)
            return;

        const index = listenerList.indexOf(listener);
        if (index !== -1)
            listenerList.splice(index, 1);
    }

    /**
    @template {keyof NetworkNodeEventMap} const K
    @param {K} type
    @param {NetworkNodeEventMap[K]} parameters
    @private*/ _dispatchEvent(type, ...parameters)
    {
        const listenerList = this._eventListeners[type];
        if (listenerList === null)
            return;

        for (const listener of listenerList)
            listener(...parameters);
    }
}