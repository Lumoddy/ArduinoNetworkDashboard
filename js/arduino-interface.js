import * as NBT from "./nbt.js";

/**
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
*/ export class ArduinoInterface
{
    /**
    @type {number}
    @private @readonly*/ static _CONTROL_BYTE = "".charCodeAt(0);

    /**
    @type {number}
    @private @readonly*/ static _START_TEXT_BYTE = "".charCodeAt(0);

    /**
    @type {number}
    @private @readonly*/ static _TIMEOUT_MILLI = 500;

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
                "Cannot initialize arduino interface with an errored serial " +
                "port.");

        try
        {
            /**
            @type {ReadableStreamDefaultReader<Uint8Array<ArrayBufferLike>>?}
            @private*/ this._reader = this._port.readable.getReader();
        }
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

        try
        {
            /**
            @type {WritableStreamDefaultWriter<Uint8Array<ArrayBufferLike>>?}
            @private*/ this._writer = this._port.writable.getWriter();
        }
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
        @type {
        {
            "+get-config": IteratorResult<
                (
                    {
                        readonly resolve: (value: ArduinoConfig) => void,
                        readonly reject: (reason?: any) => void,
                        readonly stacktrace: string,
                    }
                )[],
                ArduinoConfig>,
            "+set-pin":
            {
                [K in string]?: (
                    {
                        readonly resolve: (value: void) => void,
                        readonly reject: (reason?: any) => void,
                        readonly stacktrace: string,
                    }
                )[]
            },
            "+get-pin":
            {
                [K in string]?: (
                    {
                        readonly resolve: (value: number) => void,
                        readonly reject: (reason?: any) => void,
                        readonly stacktrace: string,
                    }
                )[]
            },
            "+set-pin-mode":
            {
                [K in string]?: (
                    {
                        readonly resolve: (value: void) => void,
                        readonly reject: (reason?: any) => void,
                        readonly stacktrace: string,
                    }
                )[]
            },
        }?
        }
        @private*/ this._resolves =
        {
            "+get-config": { done: false, value: [] },
            "+set-pin": {},
            "+get-pin": {},
            "+set-pin-mode": {},
        };

        /**
        @type {
        {
            "get-config"?: [],
            "set-pin"?: [pin: string, power: number],
            "get-pin"?: [pin: string],
            "set-pin-mode"?: [pin: string, mode: string],
        }
        }
        @private*/ this._commandQueue = {};

        /**
        @type {ReturnType<typeof setTimeout>?}
        @private*/ this._commandTimeoutHandle = null;

        /**
        @type {number[]}
        @private*/ this._buffered_reading_bytes = [];
    }

    // MARK: API
    /**
    @returns {Promise<ArduinoConfig>}
    @public*/ getConfig()
    {
        return new Promise((resolve, reject) =>
        {
            if (this._resolves === null)
                return reject(new Error(
                    "Interface was released."));

            if (this._resolves["+get-config"].done)
                return resolve(this._resolves["+get-config"].value);

            this._resolves["+get-config"].value.push(
            {
                resolve,
                reject,
                stacktrace: new Error().stack ?? "",
            });

            this._commandQueue["get-config"] = [];
        });
    }

    /**
    @param {string} pin
    @param {number} power
    @returns {Promise<void>}
    @public*/ setPinPower(pin, power)
    {
        return new Promise((resolve, reject) =>
        {
            if (this._resolves === null)
                return reject(new Error(
                    "Interface was released."));

            (this._resolves["+set-pin"][pin] ??= []).push(
            {
                resolve,
                reject,
                stacktrace: new Error().stack ?? "",
            });

            this._commandQueue["set-pin"] = [pin, Math.min(Math.max(power, 0), 1)];
        });
    }

    /**
    @param {string} pin
    @returns {Promise<number>}
    @public*/ getPinPower(pin)
    {
        return new Promise((resolve, reject) =>
        {
            if (this._resolves === null)
                return reject(new Error(
                    "Interface was released."));

            (this._resolves["+get-pin"][pin] ??= []).push(
            {
                resolve,
                reject,
                stacktrace: new Error().stack ?? "",
            });

            this._commandQueue["get-pin"] = [pin];
        });
    }

    /**
    @param {string} pin
    @param {"input" | "output" | "digital-input" | "digital-output"} mode
    @returns {Promise<void>}
    @public*/ setPinMode(pin, mode)
    {
        return new Promise((resolve, reject) =>
        {
            if (this._resolves === null)
                return reject(new Error(
                    "Interface was released."));

            (this._resolves["+set-pin-mode"][pin] ??= []).push(
            {
                resolve,
                reject,
                stacktrace: new Error().stack ?? "",
            });

            this._commandQueue["set-pin-mode"] = [pin, mode];
        });
    }

    /**
    @public*/ release()
    {
        this._reader?.releaseLock()
        this._reader = null;

        this._writer?.releaseLock()
        this._writer = null;

        this._commandTimeoutHandle = null;

        if (this._resolves !== null)
        {
            if (!this._resolves["+get-config"].done)
            {
                for (const { reject, stacktrace } of this._resolves["+get-config"].value)
                {
                    const error = new Error(
                        "Interface was released.",
                        { cause: new Error() });
                    error.stack = stacktrace;
                    reject(error);
                }
            }

            for (const id in this._resolves["+set-pin"])
            {
                const resolves = this._resolves["+set-pin"][id];

                for (const { reject, stacktrace } of resolves ?? [])
                {
                    const error = new Error(
                        "Interface was released.",
                        { cause: new Error() });
                    error.stack = stacktrace;
                    reject(error);
                }
            }

            for (const id in this._resolves["+get-pin"])
            {
                const resolves = this._resolves["+get-pin"][id];

                for (const { reject, stacktrace } of resolves ?? [])
                {
                    const error = new Error(
                        "Interface was released.",
                        { cause: new Error() });
                    error.stack = stacktrace;
                    reject(error);
                }
            }

            for (const id in this._resolves["+set-pin-mode"])
            {
                const resolves = this._resolves["+set-pin-mode"][id];

                for (const { reject, stacktrace } of resolves ?? [])
                {
                    const error = new Error(
                        "Interface was released.",
                        { cause: new Error() });
                    error.stack = stacktrace;
                    reject(error);
                }
            }
        }
    }

    /**
    @param {Iterable<number>} bytes
    @returns {Promise<void>}
    @private*/ async _sendMessage(bytes)
    {
        if (this._resolves === null || this._writer === null)
            throw new Error(
                "Interface was released.");

        const buffer =
        [
            ArduinoInterface._CONTROL_BYTE,
            ArduinoInterface._START_TEXT_BYTE,
        ];

        for (const byte of bytes)
        {
            switch (byte)
            {
                case ArduinoInterface._CONTROL_BYTE:
                    buffer.push(ArduinoInterface._CONTROL_BYTE);
                    buffer.push(ArduinoInterface._CONTROL_BYTE);
                    break;
                default:
                    buffer.push(byte);
                    break;
            }
        }

        await this._writer.write(new Uint8Array(buffer));
    }

    // MARK: _Queue
    /**
    @private*/ _trySendNextInQueue()
    {
        if (this._resolves === null || this._writer === null)
            throw new Error(
                "Interface was released.");

        if (this._commandTimeoutHandle !== null)
            return;

        switch (true)
        {
            case this._commandQueue["get-config"] !== undefined:
            {
                this._sendMessage(new Uint8Array(new NBT.EntrySerializer(
                [
                    "get-config", new NBT.CompoundTag({}),
                ])));

                this._commandTimeoutHandle = setTimeout(
                    () =>
                    {
                        if (this._resolves === null)
                            return;

                        if (!this._resolves["+get-config"].done)
                        {
                            for (const { reject, stacktrace } of this._resolves["+get-config"].value)
                            {
                                const error = new Error(
                                    "Interface was released.",
                                    { cause: new Error() });
                                error.stack = stacktrace;
                                reject(error);
                            }
                        }

                        this._commandTimeoutHandle = null;
                        this._trySendNextInQueue();
                    },
                    ArduinoInterface._TIMEOUT_MILLI);

                break;
            }
            case this._commandQueue["set-pin"] !== undefined:
            {
                this._sendMessage(new Uint8Array(new NBT.EntrySerializer(
                [
                    "set-pin", new NBT.CompoundTag(
                    {
                        "pin": new NBT.StringTag(this._commandQueue["set-pin"][0]),
                        "state": new NBT.ByteTag(this._commandQueue["set-pin"][1] * 255),
                    }),
                ])));

                this._commandTimeoutHandle = setTimeout(
                    () =>
                    {
                        if (this._resolves === null)
                            return;

                        for (const id in this._resolves["+set-pin"])
                        {
                            const resolves = this._resolves["+set-pin"][id];

                            for (const { reject, stacktrace } of resolves ?? [])
                            {
                                const error = new ArduinoError(
                                    "Timed out.",
                                    { cause: new Error() });
                                error.stack = stacktrace;
                                reject(error);
                            }
                        }

                        this._commandTimeoutHandle = null;
                        this._trySendNextInQueue();
                    },
                    ArduinoInterface._TIMEOUT_MILLI);

                break;
            }
            case this._commandQueue["get-pin"] !== undefined:
            {
                this._sendMessage(new Uint8Array(new NBT.EntrySerializer(
                [
                    "get-pin", new NBT.CompoundTag(
                    {
                        "pin": new NBT.StringTag(this._commandQueue["get-pin"][0]),
                    }),
                ])));

                this._commandTimeoutHandle = setTimeout(
                    () =>
                    {
                        if (this._resolves === null)
                            return;

                        for (const id in this._resolves["+get-pin"])
                        {
                            const resolves = this._resolves["+get-pin"][id];

                            for (const { reject, stacktrace } of resolves ?? [])
                            {
                                const error = new ArduinoError(
                                    "Timed out.",
                                    { cause: new Error() });
                                error.stack = stacktrace;
                                reject(error);
                            }
                        }

                        this._commandTimeoutHandle = null;
                        this._trySendNextInQueue();
                    },
                    ArduinoInterface._TIMEOUT_MILLI);

                break;
            }
            case this._commandQueue["set-pin-mode"] !== undefined:
            {
                this._sendMessage(new Uint8Array(new NBT.EntrySerializer(
                [
                    "set-pin-mode", new NBT.CompoundTag(
                    {
                        "pin": new NBT.StringTag(this._commandQueue["set-pin-mode"][0]),
                        "mode": new NBT.StringTag(this._commandQueue["set-pin-mode"][1]),
                    }),
                ])));

                this._commandTimeoutHandle = setTimeout(
                    () =>
                    {
                        if (this._resolves === null)
                            return;

                        for (const id in this._resolves["+set-pin-mode"])
                        {
                            const resolves = this._resolves["+set-pin-mode"][id];

                            for (const { reject, stacktrace } of resolves ?? [])
                            {
                                const error = new ArduinoError(
                                    "Timed out.",
                                    { cause: new Error() });
                                error.stack = stacktrace;
                                reject(error);
                            }
                        }

                        this._commandTimeoutHandle = null;
                        this._trySendNextInQueue();
                    },
                    ArduinoInterface._TIMEOUT_MILLI);

                break;
            }
        }
    }
}