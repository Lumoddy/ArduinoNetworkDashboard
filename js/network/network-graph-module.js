import * as SerialMessage from "./serial-message.js";

/**
@export @typedef {bigint} NetworkID
*/

/**
@export @typedef {NetworkID} NetworkNodeID
*/

/**
*/ const _networkIDEncodingFormat = /** @satisfies {SerialMessage.EncodingFormat} @type {const} */(
    "Int64");
/**
*/ const _networkIDDecodingFormat = /** @satisfies {SerialMessage.DecodingFormat} @type {const} */(
    _networkIDEncodingFormat);

/**
*/ const _messagePrefixEncodingFormat = /** @satisfies {SerialMessage.EncodingFormat} @type {const} */(
["{}",
    ["sender", _networkIDEncodingFormat],
    ["targetPath", ["[]", _networkIDEncodingFormat]],
    ["type", "Char8Array"],
]);
/**
*/ const _messagePrefixDecodingFormat = /** @satisfies {SerialMessage.DecodingFormat} @type {const} */(
    _messagePrefixEncodingFormat);

/**
@export @typedef {{
    "nodeAdded": [event:
        | { readonly target: NetworkGraphModule, readonly node: NetworkNodeID }],
    "nodeRemoved": [event:
        | { readonly target: NetworkGraphModule, readonly node: NetworkNodeID }],
    "portAdded": [event:
        | { readonly target: NetworkGraphModule, readonly port: SerialPort }],
    "portRemoved": [event:
        | { readonly target: NetworkGraphModule, readonly port: SerialPort }],
    "connectionAdded": [event:
        | { readonly target: NetworkGraphModule, readonly fromIsPort: true, readonly from: SerialPort, readonly to: NetworkNodeID }
        | { readonly target: NetworkGraphModule, readonly fromIsPort: false, readonly from: NetworkNodeID, readonly to: NetworkNodeID }],
    "connectionRemoved": [event:
        | { readonly target: NetworkGraphModule, readonly fromIsPort: true, readonly from: SerialPort, readonly to: NetworkNodeID }
        | { readonly target: NetworkGraphModule, readonly fromIsPort: false, readonly from: NetworkNodeID, readonly to: NetworkNodeID }],
}} NetworkGraphModuleEventMap
*/

/**
*/ export class NetworkGraphModule
{
    /**
    @type {Map<NetworkNodeID, {}>}
    @private*/ _nodes = new Map();

    /**
    @type {Map<SerialPort, { reader: ReadableStreamDefaultReader<Uint8Array>, writer: WritableStreamDefaultWriter<Uint8Array> }>}
    @private*/ _ports = new Map();

    /**
    @type {SerialOptions}
    @private*/ _serialOptions;

    /**
    @type {{ [K in keyof NetworkGraphModuleEventMap]: ((...args: NetworkGraphModuleEventMap[K]) => void)[] | null }}
    @private*/ _eventListeners =
    {
        "nodeAdded": null,
        "nodeRemoved": null,
        "portAdded": null,
        "portRemoved": null,
        "connectionAdded": null,
        "connectionRemoved": null,
    };

    /**
    @param {SerialOptions} [options]
    @public*/ constructor(options)
    {
        const
        {
            baudRate = 9600,
            bufferSize,
            dataBits,
            flowControl,
            parity,
            stopBits,
        }
        = options ?? {};

        this._serialOptions =
        {
            baudRate: baudRate,
            bufferSize: bufferSize,
            dataBits: dataBits,
            flowControl: flowControl,
            parity: parity,
            stopBits: stopBits,
        };
    }

    /**
    @param {SerialPort} port
    @param {{
        preventOpen?: boolean,
    }} [options]
    @public*/ connectPort(port, options)
    {
        if (this._ports.has(port))
            throw new Error("Connecting already connected port.");

        if (port.readable === null || port.writable === null)
            throw new Error("Failed to open port.");

        if (options !== undefined && !options.preventOpen)
            port.open(this._serialOptions);

        this._ports.set(port,
        {
            reader: port.readable.getReader(),
            writer: port.writable.getWriter(),
        });

        this.dispatchEvent("portAdded",
        {
            target: this,
            port: port,
        });
    }

    /**
    @param {SerialPort} port
    @param {{
        preventClose?: boolean,
    }} [options]
    @public*/ disconnectPort(port, options)
    {
        const portData = this._ports.get(port);

        if (portData === undefined)
            throw new Error("Disconnecting already disconnected port.");

        portData.reader.releaseLock();
        portData.writer.releaseLock();

        this._ports.delete(port);

        if (options !== undefined && !options.preventClose)
            port.close();

        this.dispatchEvent("portRemoved",
        {
            target: this,
            port: port,
        });
    }

    /**
    @param {SerialPort} port
    @returns {boolean}
    @public*/ hasPort(port)
    {
        return this._ports.has(port);
    }

    /**
    @template {keyof NetworkGraphModuleEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: NetworkGraphModuleEventMap[K]) => void} listener
    @public*/ addEventListener(type, listener)
    {
        const listenerList = this._eventListeners[type];
        if (listenerList === null) // @ts-ignore
            this._eventListeners[type] = [listener];
        else
            listenerList.push(listener);
    }

    /**
    @template {keyof NetworkGraphModuleEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: NetworkGraphModuleEventMap[K]) => void} listener
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
    @template {keyof NetworkGraphModuleEventMap} const K
    @param {K} type
    @param {NetworkGraphModuleEventMap[K]} parameters
    @public*/ dispatchEvent(type, ...parameters)
    {
        const listenerList = this._eventListeners[type];
        if (listenerList === null)
            return;

        for (const listener of listenerList)
            listener(...parameters);
    }
}