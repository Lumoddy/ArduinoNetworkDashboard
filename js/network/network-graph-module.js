
/**
@typedef {bigint} NetworkID
*/

/**
*/ export class NetworkGraphModule
{
    /**
    @type {Map<NetworkID, {}>}
    @private*/ _nodes = new Map();

    /**
    @type {Map<SerialPort, {}>}
    @private*/ _ports = new Map();

    /**
    @type {SerialOptions}
    @private*/ _serialOptions;

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

        this._ports.set(port, {});

        if (options !== undefined && !options.preventOpen)
            port.open(this._serialOptions);
    }

    /**
    @param {SerialPort} port
    @param {{
        preventClose?: boolean,
    }} [options]
    @public*/ disconnectPort(port, options)
    {
        if (!this._ports.has(port))
            throw new Error("Disconnecting already disconnected port.");

        this._ports.delete(port);

        if (options !== undefined && !options.preventClose)
            port.close();
    }

    /**
    @param {SerialPort} port
    @returns {boolean}
    @public*/ hasPort(port)
    {
        return this._ports.has(port);
    }
}