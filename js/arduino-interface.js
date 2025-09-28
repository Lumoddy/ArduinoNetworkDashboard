import * as NBT from "./nbt.js";

/**
@exports
@typedef {
{
    readonly name: string,
    readonly pins: readonly { readonly id: number, readonly name: string }[],
}
} ArduinoConfig
*/

/**
*/ export class ArduinoError extends Error
{
    /**
    @param {string} message
    @param {string} file
    @param {number} line
    @public*/ constructor(message, file, line)
    {
        const cause = new Error(message);
        cause.stack = `\n    at ${file}:${line}`;
        cause.name = "Error (in connected Arduino)";
        super(message, { cause });
    }
}

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
    getPinResponders: Record<number, _Responder<boolean>[]>,
    getPinQueued: Record<number, null>,
    setPinResponders: Record<number, _Responder<void>[]>,
    setPinQueued: Record<number, boolean>,
    getPinModeResponders: Record<number, _Responder<string>[]>,
    getPinModeQueued: Record<number, null>,
    setPinModeResponders: Record<number, _Responder<void>[]>,
    setPinModeQueued: Record<number, string>,
}
} _State
*/

/**
@export @typedef {{
    "pinChange": [
        event:
        {
            target: ArduinoInterface,
            pinId: number,
            pin: string,
            pinIsHigh: boolean,
        }
        ],
    "release": [event: { target: ArduinoInterface }],
}} ArduinoInterfaceEventMap
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
            getPinResponders: [],
            getPinQueued: [],
            setPinResponders: [],
            setPinQueued: [],
            setPinModeResponders: [],
            setPinModeQueued: [],
            getPinModeResponders: [],
            getPinModeQueued: [],
        };

        /**
        @type {boolean}
        @private*/ this._blockQueue = false;

        /**
        @type {
        {
            [K in keyof ArduinoInterfaceEventMap]?:
                ((...args: ArduinoInterfaceEventMap[K]) => void)[]
        }
        }
        @private*/ this._listeners = {};

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
                    /**
                    @type {IteratorResult<Uint8Array<ArrayBufferLike>>}
                    */ const { done, value } = await new Promise((resolve, reject) =>
                    {
                        let timedOut = false;
                        let timeoutHandle = undefined;

                        if (deserializer !== null)
                        {
                            timeoutHandle = setTimeout(
                                () =>
                                {
                                    timedOut = true;
                                    deserializer = null;
                                    reject(new Error(
                                        "Interface read timed out during message read."));
                                },
                                ArduinoInterface._TIMEOUT_MILLI);
                        }

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

                    for (const byte of value)
                    {
                        if (isControlByte)
                        {
                            isControlByte = false;

                            switch (byte)
                            {
                                case ArduinoInterface._START_TEXT_BYTE:
                                    deserializer = new NBT.EntryDeserializer(
                                        { endian: "little" });
                                    console.log(
                                        "Starting read of new response.");
                                    continue;
                                case ArduinoInterface._CONTROL_BYTE:
                                    break;
                                default:
                                    console.warn(
                                        `Received invalid control byte '0x${byte
                                            .toString(16)
                                            .toUpperCase()
                                            .padStart(2, "0")}'.`);
                                    continue;
                            }
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
                            let result;

                            try
                            {
                                result = deserializer.push(byte);
                                if (result === null)
                                    continue;

                                deserializer = null;
                            }
                            catch (error)
                            {
                                console.error(new Error(
                                    "Arduino sent invalid tag binary.",
                                    { cause: error }));
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
    @overload
    @param {string} pinName
    @returns {Promise<boolean>}
    *//**
    @overload
    @param {number} pinId
    @returns {Promise<boolean>}
    *//**
    @param {string | number} pin
    @returns {Promise<boolean>}
    @public*/ getPin(pin)
    {
        return new Promise(async (resolve, reject) =>
        {
            if (typeof pin !== "number")
            {
                const config = await this.getConfig();
                pin = config.pins.findIndex((v) => v.name === pin);

                if (pin === -1)
                    throw new Error("Invalid pin name.");
            }

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
    @overload
    @param {string} pinName
    @param {boolean} isHigh
    @returns {Promise<void>}
    *//**
    @overload
    @param {number} pinId
    @param {boolean} isHigh
    @returns {Promise<void>}
    *//**
    @param {string | number} pin
    @param {boolean} isHigh
    @returns {Promise<void>}
    @public*/ setPin(pin, isHigh)
    {
        return new Promise(async (resolve, reject) =>
        {
            if (typeof pin !== "number")
            {
                const config = await this.getConfig();
                pin = config.pins.findIndex((v) => v.name === pin);

                if (pin === -1)
                    throw new Error("Invalid pin name.");
            }

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
    @param {string} pinName
    @returns {Promise<"digital-input" | "digital-output">}
    *//**
    @overload
    @param {number} pinId
    @returns {Promise<"digital-input" | "digital-output">}
    *//**
    @param {string | number} pin
    @returns {Promise<string>}
    @public*/ getPinMode(pin)
    {
        return new Promise(async (resolve, reject) =>
        {
            if (typeof pin !== "number")
            {
                const config = await this.getConfig();
                pin = config.pins.findIndex((v) => v.name === pin);

                if (pin === -1)
                    throw new Error("Invalid pin name.");
            }

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
    @param {string} pinName
    @param {"input" | "output" | "digital-input" | "digital-output"} mode
    @returns {Promise<void>}
    *//**
    @overload
    @param {string} pinName
    @param {string} mode
    @returns {Promise<void>}
    *//**
    @overload
    @param {number} pinId
    @param {"input" | "output" | "digital-input" | "digital-output"} mode
    @returns {Promise<void>}
    *//**
    @overload
    @param {number} pinId
    @param {string} mode
    @returns {Promise<void>}
    *//**
    @param {string | number} pin
    @param {string} mode
    @returns {Promise<void>}
    @public*/ setPinMode(pin, mode)
    {
        return new Promise(async (resolve, reject) =>
        {
            if (typeof pin !== "number")
            {
                const config = await this.getConfig();
                pin = config.pins.findIndex((v) => v.name === pin);

                if (pin === -1)
                    throw new Error("Invalid pin name.");
            }

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

        try
        {
            this._state.reader.releaseLock();
            this._state.writer.releaseLock();
        }
        catch (error) { console.error(error) }

        this._blockQueue = true;

        if (!this._state.configResponder.done)
        {
            for (const { reject, stacktrace } of this._state.configResponder.value)
            {
                const error = new Error(
                    "Interface was released.",
                    { cause: new ArduinoInterfaceInternals() });
                error.stack = stacktrace;
                try { reject(error) }
                catch (error) { console.error(error) }
            }
        }

        for (const id in this._state.getPinResponders)
        {
            for (const { reject, stacktrace } of this._state.getPinResponders[Number(id)])
            {
                const error = new Error(
                    "Interface was released.",
                    { cause: new ArduinoInterfaceInternals() });
                error.stack = stacktrace;
                try { reject(error) }
                catch (error) { console.error(error) }
            }
        }

        for (const id in this._state.setPinResponders)
        {
            for (const { reject, stacktrace } of this._state.setPinResponders[Number(id)])
            {
                const error = new Error(
                    "Interface was released.",
                    { cause: new ArduinoInterfaceInternals() });
                error.stack = stacktrace;
                try { reject(error) }
                catch (error) { console.error(error) }
            }
        }

        for (const id in this._state.getPinModeResponders)
        {
            for (const { reject, stacktrace } of this._state.getPinModeResponders[Number(id)])
            {
                const error = new Error(
                    "Interface was released.",
                    { cause: new ArduinoInterfaceInternals() });
                error.stack = stacktrace;
                try { reject(error) }
                catch (error) { console.error(error) }
            }
        }

        for (const id in this._state.setPinModeResponders)
        {
            for (const { reject, stacktrace } of this._state.setPinModeResponders[Number(id)])
            {
                const error = new Error(
                    "Interface was released.",
                    { cause: new ArduinoInterfaceInternals() });
                error.stack = stacktrace;
                try { reject(error) }
                catch (error) { console.error(error) }
            }
        }

        this._state = null;

        this._dispatchEvent("release", { target: this });
    }

    /**
    @template {keyof ArduinoInterfaceEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: ArduinoInterfaceEventMap[K]) => void} listener
    @public*/ addEventListener(type, listener)
    {
        const listenerList = this._listeners[type];
        if (listenerList === undefined) // @ts-ignore
            this._listeners[type] = [listener];
        else
            listenerList.push(listener);
    }

    /**
    @template {keyof ArduinoInterfaceEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: ArduinoInterfaceEventMap[K]) => void} listener
    @public*/ removeEventListener(type, listener)
    {
        const listenerList = this._listeners[type];
        if (listenerList === undefined)
            return;

        const index = listenerList.indexOf(listener);
        if (index !== -1)
            listenerList.splice(index, 1);
    }

    // MARK: Private
    /**
    @template {keyof ArduinoInterfaceEventMap} const K
    @param {K} type
    @param {ArduinoInterfaceEventMap[K]} parameters
    @private*/ _dispatchEvent(type, ...parameters)
    {
        const listenerList = this._listeners[type];
        if (listenerList === undefined)
            return;

        for (const listener of listenerList)
        {
            try { listener(...parameters) }
            catch (error) { console.error(error) }
        }
    }

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

        console.log(this._state.getPinQueued);

        for (const pin in this._state.getPinQueued)
        {
            this._sendTag("get-pin", new NBT.CompoundTag(
            [
                ["pin", new NBT.ByteTag(Number(pin))],
            ]));

            delete this._state.getPinModeQueued[pin];
            return;
        }

        for (const pin in this._state.setPinQueued)
        {
            this._sendTag("set-pin", new NBT.CompoundTag(
            [
                ["pin", new NBT.ByteTag(Number(pin))],
                ["is-high", new NBT.ByteTag(this._state.setPinQueued[pin])],
            ]));

            delete this._state.setPinModeQueued[pin];
            return;
        }

        for (const pin in this._state.getPinModeQueued)
        {
            this._sendTag("get-pin-mode", new NBT.CompoundTag(
            [
                ["pin", new NBT.ByteTag(Number(pin))],
            ]));

            delete this._state.getPinModeQueued[pin];
            return;
        }

        for (const pin in this._state.setPinModeQueued)
        {
            this._sendTag("set-pin-mode", new NBT.CompoundTag(
            [
                ["pin", new NBT.ByteTag(Number(pin))],
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

        console.log(name, "/", tag);

        switch (name)
        {
            case "+get-config":
            {
                let nameTag, pinsTag;
                if (!(tag instanceof NBT.CompoundTag)
                    || !((nameTag = tag.get("name")) instanceof NBT.StringTag)
                    || !((pinsTag = tag.get("pins")) instanceof NBT.CompoundListTag))
                    return console.error(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                /**
                @type {(ArduinoConfig["pins"][number] | undefined)[]}
                */ const pins = [];
                for (const tag of pinsTag)
                {
                    let idTag, nameTag, modesTag;
                    if (!(tag instanceof NBT.CompoundTag)
                        || !((idTag = tag.get("id")) instanceof NBT.ByteTag)
                        || !((nameTag = tag.get("name")) instanceof NBT.StringTag)
                        || !((modesTag = tag.get("modes")) instanceof NBT.StringListTag)
                        || !modesTag.value.some((v) => v === "digital-input")
                        || !modesTag.value.some((v) => v === "digital-output"))
                        return console.error(
                            "Arduino interface received malformed tag from Arduino:",
                            name,
                            tag);

                    pins[idTag.value] =
                    {
                        id: idTag.value,
                        name: nameTag.value,
                    };
                }

                const config =
                {
                    name: nameTag.value,
                    pins: pins.filter((x) => x !== undefined),
                };

                if (!this._state.configResponder.done)
                {
                    for (const { resolve } of this._state.configResponder.value)
                    {
                        try { resolve(config) }
                        catch (error) { console.error(error) }
                    }
                }

                this._state.configResponder = { done: true, value: config };

                break;
            }
            case "+get-pin":
            {
                let pinTag;
                if (!(tag instanceof NBT.CompoundTag)
                    || !((pinTag = tag.get("pin")) instanceof NBT.ByteTag))
                    return console.error(
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
                    delete this._state.getPinQueued[pin];

                    this.getConfig().then((config) =>
                    {
                        this._dispatchEvent(
                            "pinChange",
                            {
                                target: this,
                                pinId: pin,
                                pin: config.pins[pin].name,
                                pinIsHigh: isHigh,
                            });
                    });
                }
                else if ((errorTag = tag.get("error")) instanceof NBT.StringTag)
                {
                    let fileTag, lineTag;
                    if (!(tag instanceof NBT.CompoundTag)
                        || !((fileTag = tag.get("file")) instanceof NBT.StringTag)
                        || !((lineTag = tag.get("line")) instanceof NBT.IntTag))
                        return console.error(
                            "Arduino interface received malformed tag from Arduino:",
                            name,
                            tag);

                    const pin = pinTag.value;
                    const message = errorTag.value;
                    const file = fileTag.value;
                    const line = lineTag.value;

                    for (const { reject, stacktrace } of this._state.getPinResponders[pin] ?? [])
                    {
                        const error = new ArduinoError(message, file, line);
                        error.stack = stacktrace;
                        try { reject(error) }
                        catch (error) { console.error(error) }
                    }

                    delete this._state.getPinResponders[pin];
                    delete this._state.getPinQueued[pin];
                }
                else
                    return console.error(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                break;
            }
            case "+set-pin":
            {
                let pinTag;
                if (!(tag instanceof NBT.CompoundTag)
                    || !((pinTag = tag.get("pin")) instanceof NBT.ByteTag))
                    return console.error(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                let errorTag;
                if ((errorTag = tag.get("error")) instanceof NBT.StringTag)
                {
                    let fileTag, lineTag;
                    if (!(tag instanceof NBT.CompoundTag)
                        || !((fileTag = tag.get("file")) instanceof NBT.StringTag)
                        || !((lineTag = tag.get("line")) instanceof NBT.IntTag))
                        return console.error(
                            "Arduino interface received malformed tag from Arduino:",
                            name,
                            tag);

                    const pin = pinTag.value;
                    const message = errorTag.value;
                    const file = fileTag.value;
                    const line = lineTag.value;

                    for (const { reject, stacktrace } of this._state.setPinResponders[pin] ?? [])
                    {
                        const error = new ArduinoError(message, file, line);
                        error.stack = stacktrace;
                        try { reject(error) }
                        catch (error) { console.error(error) }
                    }

                    delete this._state.setPinResponders[pin];
                    delete this._state.setPinQueued[pin];
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
                    delete this._state.setPinQueued[pin];
                }

                break;
            }
            case "+get-pin-mode":
            {
                let pinTag;
                if (!(tag instanceof NBT.CompoundTag)
                    || !((pinTag = tag.get("pin")) instanceof NBT.ByteTag))
                    return console.error(
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
                    delete this._state.getPinModeQueued[pin];
                }
                else if ((errorTag = tag.get("error")) instanceof NBT.StringTag)
                {
                    let fileTag, lineTag;
                    if (!(tag instanceof NBT.CompoundTag)
                        || !((fileTag = tag.get("file")) instanceof NBT.StringTag)
                        || !((lineTag = tag.get("line")) instanceof NBT.IntTag))
                        return console.error(
                            "Arduino interface received malformed tag from Arduino:",
                            name,
                            tag);

                    const pin = pinTag.value;
                    const message = errorTag.value;
                    const file = fileTag.value;
                    const line = lineTag.value;

                    for (const { reject, stacktrace } of this._state.getPinModeResponders[pin] ?? [])
                    {
                        const error = new ArduinoError(message, file, line);
                        error.stack = stacktrace;
                        try { reject(error) }
                        catch (error) { console.error(error) }
                    }

                    delete this._state.getPinModeResponders[pin];
                    delete this._state.getPinModeQueued[pin];
                }
                else
                    return console.error(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                break;
            }
            case "+set-pin-mode":
            {
                let pinTag;
                if (!(tag instanceof NBT.CompoundTag)
                    || !((pinTag = tag.get("pin")) instanceof NBT.ByteTag))
                    return console.error(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                let errorTag;
                if ((errorTag = tag.get("error")) instanceof NBT.StringTag)
                {
                    let fileTag, lineTag;
                    if (!(tag instanceof NBT.CompoundTag)
                        || !((fileTag = tag.get("file")) instanceof NBT.StringTag)
                        || !((lineTag = tag.get("line")) instanceof NBT.IntTag))
                        return console.error(
                            "Arduino interface received malformed tag from Arduino:",
                            name,
                            tag);

                    const pin = pinTag.value;
                    const message = errorTag.value;
                    const file = fileTag.value;
                    const line = lineTag.value;

                    for (const { reject, stacktrace } of this._state.setPinModeResponders[pin] ?? [])
                    {
                        const error = new ArduinoError(message, file, line);
                        error.stack = stacktrace;
                        try { reject(error) }
                        catch (error) { console.error(error) }
                    }

                    delete this._state.setPinModeResponders[pin];
                    delete this._state.setPinModeQueued[pin];
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
                    delete this._state.setPinModeQueued[pin];
                }

                break;
            }
            case "pin-changed":
            {
                let pinTag, isHighTag;
                if (!(tag instanceof NBT.CompoundTag)
                    || !((pinTag = tag.get("pin")) instanceof NBT.ByteTag)
                    || !((isHighTag = tag.get("is-high")) instanceof NBT.ByteTag))
                    return console.error(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                this.getConfig().then((config) =>
                {
                    this._dispatchEvent(
                        "pinChange",
                        {
                            target: this,
                            pinId: pinTag.value,
                            pin: config.pins[pinTag.value].name,
                            pinIsHigh: isHighTag.asBoolean,
                        });
                });

                break;
            }
            case "error":
            {
                let messageTag, pathTag, fileTag, lineTag;
                if (!(tag instanceof NBT.CompoundTag)
                    || !((messageTag = tag.get("message")) instanceof NBT.StringTag)
                    || !((pathTag = tag.get("path")) instanceof NBT.StringTag)
                    || !((fileTag = tag.get("file")) instanceof NBT.StringTag)
                    || !((lineTag = tag.get("line")) instanceof NBT.IntTag))
                    return console.error(
                        "Arduino interface received malformed tag from Arduino:",
                        name,
                        tag);

                const message = `'${pathTag.value}': ${messageTag.value}`;
                const file = fileTag.value;
                const line = lineTag.value;

                let returnedError = false;

                if (!this._state.configResponder.done)
                {
                    for (const { reject, stacktrace } of this._state.configResponder.value)
                    {
                        const error = new ArduinoError(message, file, line);
                        error.stack = stacktrace;
                        try { reject(error) }
                        catch (error) { console.error(error) }
                        returnedError = true;
                    }

                    this._state.configResponder.value = [];
                }

                for (const key in this._state.getPinResponders)
                {
                    for (const { reject, stacktrace } of this._state.getPinResponders[key])
                    {
                        const error = new ArduinoError(message, file, line);
                        error.stack = stacktrace;
                        try { reject(error) }
                        catch (error) { console.error(error) }
                        returnedError = true;
                    }
                }

                this._state.getPinResponders = {};
                this._state.getPinQueued = {};

                for (const key in this._state.setPinResponders)
                {
                    for (const { reject, stacktrace } of this._state.setPinResponders[key])
                    {
                        const error = new ArduinoError(message, file, line);
                        error.stack = stacktrace;
                        try { reject(error) }
                        catch (error) { console.error(error) }
                        returnedError = true;
                    }
                }

                this._state.setPinResponders = {};
                this._state.setPinQueued = {};

                for (const key in this._state.getPinModeResponders)
                {
                    for (const { reject, stacktrace } of this._state.getPinModeResponders[key])
                    {
                        const error = new ArduinoError(message, file, line);
                        error.stack = stacktrace;
                        try { reject(error) }
                        catch (error) { console.error(error) }
                        returnedError = true;
                    }
                }

                this._state.getPinModeResponders = {};
                this._state.getPinModeQueued = {};

                for (const key in this._state.setPinModeResponders)
                {
                    for (const { reject, stacktrace } of this._state.setPinModeResponders[key])
                    {
                        const error = new ArduinoError(message, file, line);
                        error.stack = stacktrace;
                        try { reject(error) }
                        catch (error) { console.error(error) }
                        returnedError = true;
                    }
                }

                this._state.setPinModeResponders = {};
                this._state.setPinModeQueued = {};

                if (!returnedError)
                    console.error(new ArduinoError(message, file, line));

                break;
            }
            default:
                return console.error(
                    "Arduino interface received unknown tag from Arduino:",
                    name,
                    tag);
        }

        this._blockQueue = false;
        this._trySendNextQueued();
    }
}