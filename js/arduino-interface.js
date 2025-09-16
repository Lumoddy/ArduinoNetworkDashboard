import * as NBT from "./nbt.js";

/**
@exports
@typedef {
{
    readonly name: string,
    readonly pins: (
        {
            id: string,
            name: string,
            allowAnalogRead: false,
            allowAnalogWrite: false,
        }
    )[]
}
} ArduinoConfig
*/

/**
*/ export class ArduinoError extends Error { }

/**
*/ class ArduinoInterfaceInternals extends Error { }

/**
@template T
@typedef {
{
    readonly resolve: (value: T) => void,
    readonly reject: (reason?: any) => void,
    readonly stacktrace: string,
}
} _Responder
*/

/**
@typedef {
{
    reader: ReadableStreamDefaultReader<Uint8Array<ArrayBufferLike>>,
    writer: WritableStreamDefaultWriter<Uint8Array<ArrayBufferLike>>,
    configResponder: IteratorResult<_Responder<ArduinoConfig>[], ArduinoConfig>,
    getPinResponders: Record<string, _Responder<boolean>[]>,
    getPinQueued: Record<string, null>,
    setPinResponders: Record<string, _Responder<void>[]>,
    setPinQueued: Record<string, boolean>,
    getPinModeResponders: Record<string, _Responder<string>[]>,
    getPinModeQueued: Record<string, null>,
    setPinModeResponders: Record<string, _Responder<void>[]>,
    setPinModeQueued: Record<string, string>,
    pinChangeResponders: Record<string, _Responder<boolean>[]>,
}
} _State
*/

/**
*/ export class ArduinoInterface
{
    // MARK: Constructor
    /**
    @param {SerialPort} port
    @public*/ constructor(port)
    {
        /**
        @type {SerialPort}
        @private*/ this._port = port;

        if (this._port.readable === null || this._port.writable === null)
            throw new Error(
                "Cannot initialize arduino interface with a closed serial " +
                "port.");

        let reader;

        try { reader = this._port.readable.getReader() }
        catch (error)
        {
            this.release();

            if (error instanceof Error)
            {
                throw new Error(
                    "Cannot initialize arduino interface with an already " +
                    "used serial port.",
                    { cause: error });
            }
            else
                throw error;
        }

        let writer;

        try { writer = this._port.writable.getWriter() }
        catch (error)
        {
            this.release();

            if (error instanceof Error)
            {
                throw new Error(
                    "Cannot initialize arduino interface with an already " +
                    "used serial port.",
                    { cause: error });
            }
            else
                throw error;
        }

        /**
        @type {_State?}
        @private*/ this._state =
        {
            reader,
            writer,
            configResponder: { done: false, value: [] },
            getPinResponders: {},
            getPinQueued: {},
            setPinResponders: {},
            setPinQueued: {},
            setPinModeResponders: {},
            setPinModeQueued: {},
            getPinModeResponders: {},
            getPinModeQueued: {},
            pinChangeResponders: {},
        };

        /**
        @type {boolean}
        @private*/ this._blockQueue = false;

        (async () =>
        {
            /**
            @type {NBT.EntryDeserializer?}
            */ let deserializer = null;
            let isControlByte = false;

            reading: while (true)
            {
                if (this._state === null)
                    return;

                try
                {
                    console.log("reading");

                    /**
                    @type {IteratorResult<Uint8Array<ArrayBufferLike>>}
                    */ const { done, value } = await new Promise((resolve, reject) =>
                    {
                        let timedOut = false;
                        let timeoutHandle = undefined;

                        if (deserializer !== null)
                            timeoutHandle = setTimeout(
                                () =>
                                {
                                    timedOut = true;
                                    reject(new Error(
                                        "Interface read timed out during message read."));
                                },
                                ArduinoInterface._TIMEOUT_MILLI);

                        // @ts-ignore
                        this._state.reader.read().then((value) =>
                        {
                            if (timedOut)
                                return;

                            clearTimeout(timeoutHandle);
                            resolve(value);
                        });
                    });
                    if (done)
                        return;

                    console.log("read " + value.length);

                    for (let byte of value)
                    {
                        let result;

                        if (isControlByte)
                        {
                            switch (byte)
                            {
                                case ArduinoInterface._START_TEXT_BYTE:
                                    deserializer = new NBT.EntryDeserializer(
                                        { endian: "little" });
                                    console.log(
                                        "Starting read of new response.");
                                    break;
                                case ArduinoInterface._CONTROL_BYTE:
                                    byte = ArduinoInterface._CONTROL_BYTE;
                                    break;
                                default:
                                    console.warn(
                                        `Received invalid control byte '${byte
                                            .toString(16)
                                            .toUpperCase()
                                            .padStart(2, "0")}'.`);
                                    continue;
                            }

                            isControlByte = false;
                        }
                        else
                        {
                            switch (byte)
                            {
                                case ArduinoInterface._CONTROL_BYTE:
                                    isControlByte = true;
                                    continue;
                            }
                        }

                        if (deserializer !== null)
                        {
                            try
                            {
                                result = deserializer.push(byte);
                                if (result === null)
                                    continue;

                                deserializer = null;
                            }
                            catch (error)
                            {
                                console.error(error);
                                deserializer = null;
                                continue reading;
                            }

                            this._onTagReceived(result[0], result[1]);
                        }
                    }
                }
                catch (error)
                {
                    if (error instanceof Error
                        && error.message === "Interface read timed out during message read.")
                        console.error(error);
                    else
                        throw error;
                }
            }
        })();

        this._trySendNextQueued();
    }

    // MARK: API
    /**
    @returns {Promise<ArduinoConfig>}
    @public*/ getConfig()
    {
        return new Promise((resolve, reject) =>
        {
            if (this._state === null)
                return reject(new Error(
                    "Interface was released."));

            if (this._state.configResponder.done)
            {
                try { resolve(this._state.configResponder.value) }
                catch (error) { console.error(error) }

                return;
            }

            this._state.configResponder.value.push(
            {
                resolve,
                reject,
                stacktrace: new Error().stack ?? "",
            });

            this._trySendNextQueued();
        });
    }

    /**
    @param {string} pin
    @returns {Promise<boolean>}
    @public*/ getPin(pin)
    {
        return new Promise((resolve, reject) =>
        {
            if (this._state === null)
                return reject(new Error(
                    "Interface was released."));

            this._state.getPinQueued[pin] = null;
            (this._state.getPinResponders[pin] ??= []).push(
            {
                resolve,
                reject,
                stacktrace: new Error().stack ?? "",
            });

            this._trySendNextQueued();
        });
    }

    /**
    @param {string} pin
    @param {boolean} isHigh
    @returns {Promise<void>}
    @public*/ setPin(pin, isHigh)
    {
        return new Promise((resolve, reject) =>
        {
            if (this._state === null)
                return reject(new Error(
                    "Interface was released."));

            this._state.setPinQueued[pin] = isHigh;
            (this._state.setPinResponders[pin] ??= []).push(
            {
                resolve,
                reject,
                stacktrace: new Error().stack ?? "",
            });

            this._trySendNextQueued();
        });
    }

    /**
    @overload
    @param {string} pin
    @returns {Promise<"digital-input" | "digital-output">}
    *//**
    @param {string} pin
    @returns {Promise<string>}
    @public*/ getPinMode(pin)
    {
        return new Promise((resolve, reject) =>
        {
            if (this._state === null)
                return reject(new Error(
                    "Interface was released."));

            this._state.getPinModeQueued[pin] = null;
            (this._state.getPinModeResponders[pin] ??= []).push(
            {
                resolve,
                reject,
                stacktrace: new Error().stack ?? "",
            });

            this._trySendNextQueued();
        });
    }

    /**
    @overload
    @param {string} pin
    @param {"input" | "output" | "digital-input" | "digital-output"} mode
    @returns {Promise<void>}
    *//**
    @overload
    @param {string} pin
    @param {string} mode
    @returns {Promise<void>}
    *//**
    @param {string} pin
    @param {string} mode
    @returns {Promise<void>}
    @public*/ setPinMode(pin, mode)
    {
        return new Promise((resolve, reject) =>
        {
            if (this._state === null)
                return reject(new Error(
                    "Interface was released."));

            this._state.setPinModeQueued[pin] = mode;
            (this._state.setPinModeResponders[pin] ??= []).push(
            {
                resolve,
                reject,
                stacktrace: new Error().stack ?? "",
            });

            this._trySendNextQueued();
        });
    }

    /**
    @public*/ release()
    {
        if (this._state === null)
            return;

        this._state.reader.releaseLock();
        this._state.writer.releaseLock();
        this._blockQueue = true;

        if (!this._state.configResponder.done)
        {
            for (const { reject, stacktrace } of this._state.configResponder.value)
            {
                const error = new Error(
                    "Interface was released.",
                    { cause: new ArduinoInterfaceInternals() });
                error.stack = stacktrace;
                reject(error);
            }
        }

        for (const pin in this._state.getPinResponders)
        {
            for (const { reject, stacktrace } of this._state.getPinResponders[pin])
            {
                const error = new Error(
                    "Interface was released.",
                    { cause: new ArduinoInterfaceInternals() });
                error.stack = stacktrace;
                reject(error);
            }
        }

        for (const pin in this._state.setPinResponders)
        {
            for (const { reject, stacktrace } of this._state.setPinResponders[pin])
            {
                const error = new Error(
                    "Interface was released.",
                    { cause: new ArduinoInterfaceInternals() });
                error.stack = stacktrace;
                reject(error);
            }
        }

        for (const pin in this._state.getPinModeResponders)
        {
            for (const { reject, stacktrace } of this._state.getPinModeResponders[pin])
            {
                const error = new Error(
                    "Interface was released.",
                    { cause: new ArduinoInterfaceInternals() });
                error.stack = stacktrace;
                reject(error);
            }
        }

        for (const pin in this._state.setPinModeResponders)
        {
            for (const { reject, stacktrace } of this._state.setPinModeResponders[pin])
            {
                const error = new Error(
                    "Interface was released.",
                    { cause: new ArduinoInterfaceInternals() });
                error.stack = stacktrace;
                reject(error);
            }
        }

        for (const pin in this._state.pinChangeResponders)
        {
            for (const { reject, stacktrace } of this._state.pinChangeResponders[pin])
            {
                const error = new Error(
                    "Interface was released.",
                    { cause: new ArduinoInterfaceInternals() });
                error.stack = stacktrace;
                reject(error);
            }
        }

        this._state = null;
    }

    // MARK: Private
    /**
    @type {number}
    @private @readonly*/ static _CONTROL_BYTE = "".charCodeAt(0);

    /**
    @type {number}
    @private @readonly*/ static _START_TEXT_BYTE = "".charCodeAt(0);

    /**
    @type {number}
    @private @readonly*/ static _TIMEOUT_MILLI = 500;

    /**
    @private*/ _trySendNextQueued()
    {
        if (this._state === null || this._blockQueue)
            return;

        if (!this._state.configResponder.done)
        {
            this._sendTag("get-config", new NBT.CompoundTag(new Map()));
            return;
        }

        for (const pin in this._state.getPinQueued)
        {
            this._sendTag("get-pin", new NBT.CompoundTag(
            [
                ["pin", new NBT.StringTag(pin)],
            ]));

            delete this._state.getPinModeQueued[pin];
            return;
        }

        for (const pin in this._state.setPinQueued)
        {
            this._sendTag("set-pin", new NBT.CompoundTag(
            [
                ["pin", new NBT.StringTag(pin)],
                ["is-high", new NBT.ByteTag(this._state.setPinQueued[pin])],
            ]));

            delete this._state.setPinModeQueued[pin];
            return;
        }

        for (const pin in this._state.getPinModeQueued)
        {
            this._sendTag("get-pin-mode", new NBT.CompoundTag(
            [
                ["pin", new NBT.StringTag(pin)],
            ]));

            delete this._state.getPinModeQueued[pin];
            return;
        }

        for (const pin in this._state.setPinModeQueued)
        {
            this._sendTag("set-pin-mode", new NBT.CompoundTag(
            [
                ["pin", new NBT.StringTag(pin)],
                ["mode", new NBT.StringTag(this._state.setPinModeQueued[pin])],
            ]));

            delete this._state.setPinModeQueued[pin];
            return;
        }
    }

    /**
    @param {string} name
    @param {NBT.Tag} tag
    @returns {Promise<void>}
    @private*/ _sendTag(name, tag)
    {
        if (this._state === null)
            throw new Error(
                "Interface was released.");

        return this._state.writer.write(new Uint8Array(function*()
        {
            yield ArduinoInterface._CONTROL_BYTE;
            yield ArduinoInterface._START_TEXT_BYTE;

            const deserializer = new NBT.EntrySerializer([name, tag], { endian: "little" });

            while (true)
            {
                const { done, value } = deserializer.next();
                if (done)
                    return;

                if (value === ArduinoInterface._CONTROL_BYTE)
                    yield ArduinoInterface._CONTROL_BYTE;

                yield value;
            }
        }()));
    }

    /**
    @param {string} name
    @param {NBT.TagUnion} tag
    @private*/ _onTagReceived(name, tag)
    {
        if (this._state === null)
            throw new Error(
                "Interface was released.");

        this._blockQueue = false;
        this._trySendNextQueued();

        switch (name)
        {
            case "+get-pin":
            {
                let pinTag;
                if (!(tag instanceof NBT.CompoundTag)
                    || !((pinTag = tag.get("pin")) instanceof NBT.StringTag))
                    return console.warn(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                let isHighTag, errorTag;
                if ((isHighTag = tag.get("is-high")) instanceof NBT.ByteTag)
                {
                    const pin = pinTag.value;
                    const isHigh = isHighTag.asBoolean;

                    for (const { resolve } of this._state.getPinResponders[pin] ?? [])
                    {
                        try { resolve(isHigh) }
                        catch (error) { console.error(error) }
                    }

                    delete this._state.getPinResponders[pin];

                    for (const { resolve } of this._state.pinChangeResponders[pin] ?? [])
                    {
                        try { resolve(isHigh) }
                        catch (error) { console.error(error) }
                    }

                    delete this._state.pinChangeResponders[pin];
                }
                else if ((errorTag = tag.get("error")) instanceof NBT.StringTag)
                {
                    const pin = pinTag.value;
                    const message = errorTag.value;

                    for (const { reject, stacktrace } of this._state.getPinResponders[pin] ?? [])
                    {
                        const error = new ArduinoError(
                            message,
                            { cause: new ArduinoInterfaceInternals() });
                        error.stack = stacktrace;
                        reject(error);
                    }

                    delete this._state.getPinResponders[pin];
                }
                else
                    return console.warn(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                break;
            }
            case "+set-pin":
            {
                let pinTag;
                if (!(tag instanceof NBT.CompoundTag)
                    || !((pinTag = tag.get("pin")) instanceof NBT.StringTag))
                    return console.warn(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                let errorTag;
                if ((errorTag = tag.get("error")) instanceof NBT.StringTag)
                {
                    const pin = pinTag.value;
                    const message = errorTag.value;

                    for (const { reject, stacktrace } of this._state.setPinResponders[pin] ?? [])
                    {
                        const error = new ArduinoError(
                            message,
                            { cause: new ArduinoInterfaceInternals() });
                        error.stack = stacktrace;
                        reject(error);
                    }

                    delete this._state.setPinResponders[pin];
                }
                else
                {
                    const pin = pinTag.value;

                    for (const { resolve } of this._state.setPinResponders[pin] ?? [])
                    {
                        try { resolve() }
                        catch (error) { console.error(error) }
                    }

                    delete this._state.setPinResponders[pin];
                }

                break;
            }
            case "+get-pin-mode":
            {
                let pinTag;
                if (!(tag instanceof NBT.CompoundTag)
                    || !((pinTag = tag.get("pin")) instanceof NBT.StringTag))
                    return console.warn(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                let modeTag, errorTag;
                if ((modeTag = tag.get("mode")) instanceof NBT.StringTag)
                {
                    const pin = pinTag.value;
                    const mode = modeTag.value;

                    for (const { resolve } of this._state.getPinModeResponders[pin] ?? [])
                    {
                        try { resolve(mode) }
                        catch (error) { console.error(error) }
                    }

                    delete this._state.getPinModeResponders[pin];
                }
                else if ((errorTag = tag.get("error")) instanceof NBT.StringTag)
                {
                    const pin = pinTag.value;
                    const message = errorTag.value;

                    for (const { reject, stacktrace } of this._state.getPinModeResponders[pin] ?? [])
                    {
                        const error = new ArduinoError(
                            message,
                            { cause: new ArduinoInterfaceInternals() });
                        error.stack = stacktrace;
                        reject(error);
                    }

                    delete this._state.getPinModeResponders[pin];
                }
                else
                    return console.warn(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                break;
            }
            case "+set-pin-mode":
            {
                let pinTag;
                if (!(tag instanceof NBT.CompoundTag)
                    || !((pinTag = tag.get("pin")) instanceof NBT.StringTag))
                    return console.warn(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                let errorTag;
                if ((errorTag = tag.get("error")) instanceof NBT.StringTag)
                {
                    const pin = pinTag.value;
                    const message = errorTag.value;

                    for (const { reject, stacktrace } of this._state.setPinModeResponders[pin] ?? [])
                    {
                        const error = new ArduinoError(
                            message,
                            { cause: new ArduinoInterfaceInternals() });
                        error.stack = stacktrace;
                        reject(error);
                    }

                    delete this._state.setPinModeResponders[pin];
                }
                else
                {
                    const pin = pinTag.value;

                    for (const { resolve } of this._state.setPinModeResponders[pin] ?? [])
                    {
                        try { resolve() }
                        catch (error) { console.error(error) }
                    }

                    delete this._state.setPinModeResponders[pin];
                }

                break;
            }
            case "pin-changed":
            {
                let pinTag, isHighTag;
                if (!(tag instanceof NBT.CompoundTag)
                    || !((pinTag = tag.get("pin")) instanceof NBT.StringTag)
                    || !((isHighTag = tag.get("is-high")) instanceof NBT.ByteTag))
                    return console.warn(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                const pin = pinTag.value;
                const isHigh = isHighTag.asBoolean;

                for (const { resolve } of this._state.pinChangeResponders[pin] ?? [])
                {
                    try { resolve(isHigh) }
                    catch (error) { console.error(error) }
                }

                delete this._state.pinChangeResponders[pin];

                break;
            }
            case "error":
            {
                let messageTag, pathTag;
                if (!(tag instanceof NBT.CompoundTag)
                    || !((messageTag = tag.get("message")) instanceof NBT.StringTag)
                    || !((pathTag = tag.get("path")) instanceof NBT.StringTag))
                    return console.error(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                const message = messageTag.value;
                const path = pathTag.value;

                if (!this._state.configResponder.done)
                {
                    for (const { reject, stacktrace } of this._state.configResponder.value)
                    {
                        const error = new ArduinoError(
                            `'${path}': ${message}`,
                            { cause: new ArduinoInterfaceInternals() });
                        error.stack = stacktrace;
                        reject(error);
                    }

                    this._state.configResponder.value = [];
                }

                for (const key in this._state.getPinResponders)
                {
                    for (const { reject, stacktrace } of this._state.getPinResponders[key])
                    {
                        const error = new Error(
                            `'${path}': ${message}`,
                            { cause: new ArduinoInterfaceInternals() });
                        error.stack = stacktrace;
                        reject(error);
                    }
                }

                this._state.getPinResponders = {};

                for (const key in this._state.setPinResponders)
                {
                    for (const { reject, stacktrace } of this._state.setPinResponders[key])
                    {
                        const error = new Error(
                            `'${path}': ${message}`,
                            { cause: new ArduinoInterfaceInternals() });
                        error.stack = stacktrace;
                        reject(error);
                    }
                }

                this._state.setPinResponders = {};

                for (const key in this._state.getPinModeResponders)
                {
                    for (const { reject, stacktrace } of this._state.getPinModeResponders[key])
                    {
                        const error = new Error(
                            `'${path}': ${message}`,
                            { cause: new ArduinoInterfaceInternals() });
                        error.stack = stacktrace;
                        reject(error);
                    }
                }

                this._state.getPinModeResponders = {};

                for (const key in this._state.setPinModeResponders)
                {
                    for (const { reject, stacktrace } of this._state.setPinModeResponders[key])
                    {
                        const error = new Error(
                            `'${path}': ${message}`,
                            { cause: new ArduinoInterfaceInternals() });
                        error.stack = stacktrace;
                        reject(error);
                    }
                }

                this._state.setPinModeResponders = {};

                for (const key in this._state.pinChangeResponders)
                {
                    for (const { reject, stacktrace } of this._state.pinChangeResponders[key])
                    {
                        const error = new Error(
                            `'${path}': ${message}`,
                            { cause: new ArduinoInterfaceInternals() });
                        error.stack = stacktrace;
                        reject(error);
                    }
                }

                this._state.pinChangeResponders = {};
            }
            default:
                return console.warn(
                    "Arduino interface received unknown tag from Arduino:",
                    name,
                    tag);
        }
    }
}