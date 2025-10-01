import { deserializeVia, serializerVia } from "../smf/serialization.js";
import { arduinoRequestSerializer } from "./request.js";
import { arduinoResponseDeserializer } from "./response.js";
/**
@import { ArduinoPinId } from "./pin-id.js"
@import { ArduinoPinMode } from "./pin-mode.js"
@import { ArduinoRequest } from "./request.js"
@import { ArduinoResponse } from "./response.js"
*/

/**
@exports
@typedef {
{
    readonly model: string,
    readonly pins: readonly
    {
        readonly id: number,
        readonly name: string,
        readonly supportsDigitalInput: true,
        readonly supportsDigitalOutput: true,
    }[],
}
} ArduinoConfig
*/

/**
@template T
@typedef {
{
    readonly resolve: (value: T) => void,
    readonly reject: (reason?: any) => void,
    readonly stack: string | undefined,
}
} _Responder
*/

// https://symbl.cc/en/unicode-table/

/**
@type {number}
@private @readonly*/ const _CONTROL_BYTE = "".charCodeAt(0);

/**
@type {number}
@private @readonly*/ const _START_TEXT_BYTE = "".charCodeAt(0);

/**
@typedef {
{
    "release": readonly [
        event:
        {
            readonly target: ArduinoInterface,
        },
    ],
    "pin-change": readonly [
        event:
        {
            readonly target: ArduinoInterface,
            readonly pinId: number,
            readonly pinName: string,
            readonly pinIsHigh: boolean,
        },
    ],
}
} ArduinoInterfaceEventMap
*/

/**
*/ export class ArduinoInterface
{
    /**
    @param {SerialPort} port
    @public*/ constructor(port)
    {
        if (!(port instanceof SerialPort))
            throw new TypeError(
                "Expected SerialPort at argument 1 ArduinoInterface constructor.");

        if (port.readable === null || port.writable === null)
            throw new Error(
                "Cannot connect ArduinoInterface to locked port.");

        /**
        @type {boolean}
        @private*/ this._released = false;

        /**
        @type {SerialPort}
        @private*/ this._port = port;

        /**
        @type {ReadableStreamDefaultReader<Uint8Array<ArrayBufferLike>>}
        @private*/ this._reader = port.readable.getReader();

        /**
        @type {WritableStreamDefaultWriter<Uint8Array<ArrayBufferLike>>}
        @private*/ this._writer = port.writable.getWriter();

        /**
        @type {ArduinoConfig?}
        @private*/ this._cachedConfig = null;

        /**
        @type {_Responder<ArduinoConfig>[]?}
        @private*/ this._getConfigResponders = null;

        /**
        @type {{ [K in number]?: true }}
        @private*/ this._getPinRequest = {};

        /**
        @type {{ [K in number]?: _Responder<boolean>[] }}
        @private*/ this._getPinResponders = {};

        /**
        @type {{ [K in number]?: boolean }}
        @private*/ this._setPinRequest = {};

        /**
        @type {{ [K in number]?: _Responder<void>[] }}
        @private*/ this._setPinResponders = {};

        /**
        @type {{ [K in number]?: true }}
        @private*/ this._getPinModeRequest = {};

        /**
        @type {{ [K in number]?: _Responder<ArduinoPinMode>[] }}
        @private*/ this._getPinModeResponders = {};

        /**
        @type {{ [K in number]?: ArduinoPinMode }}
        @private*/ this._setPinModeRequest = {};

        /**
        @type {{ [K in number]?: _Responder<void>[] }}
        @private*/ this._setPinModeResponders = {};

        /**
        @type {
        {
            [K in keyof ArduinoInterfaceEventMap]:
                ((...args: ArduinoInterfaceEventMap[K]) => void)[]
        }}
        @private*/ this._eventListeners =
        {
            "release": [],
            "pin-change": [],
        };

        /**
        @type {((value: void) => void)?}
        @private*/ this._unpauseQueue = () =>
        {
            earlyUnpauseQueue = true;
        };

        let earlyUnpauseQueue = false;

        /**
        @type {((value: void) => void)?}
        */ let responseUnpauseQueue = null;

        /**
        @type {(() => void)?}
        */ let delayTimeout = null;

        setTimeout(async () => // MARK: Writer Loop
        {
            /**
            @param {ArduinoRequest} request
            */ const send = async (request) =>
            {
                const bytes = new Uint8Array(function*()
                {
                    yield _CONTROL_BYTE;
                    yield _START_TEXT_BYTE;

                    const serializer = arduinoRequestSerializer(request);

                    while (true)
                    {
                        const { done, value } = serializer.next();
                        if (done)
                            return;

                        if (value === _CONTROL_BYTE)
                            yield _CONTROL_BYTE;

                        yield value;
                    }
                }());

                const response = new Promise((resolve) =>
                {
                    /**
                    @type {NodeJS.Timeout?}
                    */ let timeoutHandle = null;

                    delayTimeout = () =>
                    {
                        if (timeoutHandle !== null)
                            clearTimeout(timeoutHandle);

                        timeoutHandle = setTimeout(
                            () =>
                            {
                                responseUnpauseQueue = null;
                                console.warn(new Error(
                                    "Response timed out, retrying..."));
                                resolve(undefined);
                            },
                            500);
                    };

                    delayTimeout();

                    responseUnpauseQueue = () =>
                    {
                        if (timeoutHandle !== null)
                            clearTimeout(timeoutHandle);

                        resolve(undefined);
                    };
                });

                await this._writer.write(bytes);
                await response;

                responseUnpauseQueue = null;
            }

            if (earlyUnpauseQueue)
                earlyUnpauseQueue = false;
            else
            {
                await new Promise((resolve) => this._unpauseQueue = resolve);
                this._unpauseQueue = null;
            }

            toNextItem: while (!this._released)
            {
                try
                {
                    if (this._cachedConfig === null)
                    {
                        await send(
                        {
                            type: "get-config",
                        });

                        continue toNextItem;
                    }

                    for (const key in this._getPinRequest)
                    {
                        delete this._getPinRequest[key];
                        const pin = Number(key);

                        await send(
                        {
                            type: "get-pin",
                            pin,
                        });

                        continue toNextItem;
                    }

                    for (const key in this._setPinRequest)
                    {
                        const isHigh = this._setPinRequest[key];
                        delete this._setPinRequest[key];

                        if (typeof isHigh !== "boolean")
                            continue;

                        const pin = Number(key);

                        await send(
                        {
                            type: "set-pin",
                            pin,
                            isHigh,
                        });

                        continue toNextItem;
                    }

                    for (const key in this._getPinModeRequest)
                    {
                        delete this._getPinModeRequest[key];
                        const pin = Number(key);

                        await send(
                        {
                            type: "get-pin-mode",
                            pin,
                        });

                        continue toNextItem;
                    }

                    for (const key in this._setPinModeRequest)
                    {
                        const mode = this._setPinModeRequest[key];
                        delete this._setPinModeRequest[key];

                        if (typeof mode !== "string")
                            continue;

                        const pin = Number(key);

                        await send(
                        {
                            type: "set-pin-mode",
                            pin,
                            mode,
                        });

                        continue toNextItem;
                    }
                }
                catch (error)
                {
                    console.error(error);
                }

                await new Promise((resolve) => this._unpauseQueue = resolve);
                this._unpauseQueue = null;
            }
        })

        /**
        @param {ArduinoResponse} response
        */ const onResponseReceived = (response) => // MARK: Response Reaction
        {
            switch (response.type)
            {
                case "+get-config":
                {
                    this._cachedConfig = response.config;
                    for (const { resolve } of this._getConfigResponders ?? [])
                        resolve(response.config);

                    this._getConfigResponders = [];

                    break;
                }
                case "+get-pin-ok":
                {
                    const responders = this._getPinResponders[response.pin];

                    if (responders !== undefined)
                    {
                        for (const { resolve } of responders)
                            resolve(response.isHigh);
    
                        responders.length = 0;
                    }

                    break;
                }
                case "+get-pin-error":
                {
                    const cause = new Error(
                        "ArduinoInterface received an error from Arduino " +
                        `'${response.message}'.`);
                    let errorUsed = false;

                    const responders = this._getPinResponders[response.pin];

                    if (responders !== undefined)
                    {
                        for (const { reject, stack } of responders)
                        {
                            const error = new Error(
                                `Error from Arduino: '${response.message}'.`);
                            error.stack = stack;
                            error.cause = cause;
                            reject(error);
                            errorUsed = true;
                        }

                        responders.length = 0;
                    }

                    if (!errorUsed)
                        console.warn(cause);

                    break;
                }
                case "+set-pin-ok":
                {
                    const responders = this._setPinResponders[response.pin];

                    if (responders !== undefined)
                    {
                        for (const { resolve } of responders)
                            resolve();

                        responders.length = 0;
                    }

                    break;
                }
                case "+set-pin-error":
                {
                    const cause = new Error(
                        "ArduinoInterface received an error from Arduino " +
                        `'${response.message}'.`);
                    let errorUsed = false;

                    const responders = this._setPinResponders[response.pin];

                    if (responders !== undefined)
                    {
                        for (const { reject, stack } of responders)
                        {
                            const error = new Error(
                                `Error from Arduino: '${response.message}'.`);
                            error.stack = stack;
                            error.cause = cause;
                            reject(error);
                            errorUsed = true;
                        }

                        responders.length = 0;
                    }

                    if (!errorUsed)
                        console.warn(cause);

                    break;
                }
                case "+get-pin-mode-ok":
                {
                    const responders = this._getPinModeResponders[response.pin];

                    if (responders !== undefined)
                    {
                        for (const { resolve } of responders)
                            resolve(response.mode);

                        responders.length = 0;
                    }

                    break;
                }
                case "+get-pin-mode-error":
                {
                    const cause = new Error(
                        "ArduinoInterface received an error from Arduino " +
                        `'${response.message}'.`);
                    let errorUsed = false;

                    const responders = this._getPinModeResponders[response.pin];

                    if (responders !== undefined)
                    {
                        for (const { reject, stack } of responders)
                        {
                            const error = new Error(
                                `Error from Arduino: '${response.message}'.`);
                            error.stack = stack;
                            error.cause = cause;
                            reject(error);
                            errorUsed = true;
                        }

                        responders.length = 0;
                    }

                    if (!errorUsed)
                        console.warn(cause);

                    break;
                }
                case "+set-pin-mode-ok":
                {
                    const responders = this._setPinModeResponders[response.pin];

                    if (responders !== undefined)
                    {
                        for (const { resolve } of responders)
                            resolve();

                        responders.length = 0;
                    }

                    break;
                }
                case "+set-pin-mode-error":
                {
                    const cause = new Error(
                        "ArduinoInterface received an error from Arduino " +
                        `'${response.message}'.`);
                    let errorUsed = false;

                    const responders = this._setPinModeResponders[response.pin];

                    if (responders !== undefined)
                    {
                        for (const { reject, stack } of responders)
                        {
                            const error = new Error(
                                `Error from Arduino: '${response.message}'.`);
                            error.stack = stack;
                            error.cause = cause;
                            reject(error);
                            errorUsed = true;
                        }

                        responders.length = 0;
                    }

                    if (!errorUsed)
                        console.warn(cause);

                    break;
                }
                case "pin-changed":
                {
                    const
                    {
                        pin: pinId,
                        isHigh: pinIsHigh,
                    }
                    = response;

                    this.getConfig().then((config) =>
                    {
                        let pinName;

                        for (const pin of config.pins)
                            if (pin.id === pinId)
                                pinName = pin.name;

                        if (pinName === undefined)
                            throw new Error(
                                `Invalid pin id '${pinId}' sent from Arduino`);

                        const event =
                        {
                            target: this,
                            pinId,
                            pinName,
                            pinIsHigh,
                        };

                        for (const listener of this._eventListeners["pin-change"])
                            setTimeout(listener, undefined, event);
                    })

                    break;
                }
                case "error":
                {
                    let error;

                    switch (response.error)
                    {
                        case "invalid-control-byte":
                            error = new Error(
                                "Arduino reported receiving invalid " +
                                "control byte.");
                            break;
                        case "collect-overflow":
                            error = new Error(
                                "Arduino reported overflowing a " +
                                "collection.");
                            break;
                        case "invalid-syntax":
                            error = new Error(
                                "Arduino reported receiving invalid " +
                                `syntax. 'Expected ${response.expected}'.`);
                            break;
                        case "invalid-value":
                            error = new Error(
                                "Arduino reported receiving invalid " +
                                `value: 'Expected ${response.expected}'.`);
                            break;
                        case "timed-out":
                            error = new Error(
                                "Arduino reported timing out.");
                            break;
                    }

                    this.rejectAll(
                        error,
                        "ArduinoInterface received error.");

                    break;
                }
            }

            responseUnpauseQueue?.();
        }

        setTimeout(async () => // MARK: Reader Loop
        {
            /**
            @type {Generator<void, never, number>}
            */ const sink = function*()
            {
                /**
                @type {Generator<void, ArduinoResponse, number>?}
                */ let deserializer = null;

                while (true)
                {
                    let byte;
                    escapeSequence: switch (byte = yield)
                    {
                        case _CONTROL_BYTE: switch (byte = yield)
                        {
                            case _START_TEXT_BYTE:
                            {
                                deserializer = arduinoResponseDeserializer();
                                deserializer.next();

                                break escapeSequence;
                            }
                            default:
                            {
                                deserializer = null;
                                console.warn(new SyntaxError(
                                    "Received invalid control byte from Arduino."));
                                break escapeSequence;
                            }
                            case _CONTROL_BYTE:
                        }
                        default:
                        {
                            if (deserializer !== null)
                            {
                                const { done, value } = deserializer.next(byte);
                                if (done)
                                {
                                    deserializer = null;
                                    onResponseReceived(value);
                                }
                            }

                            break escapeSequence;
                        }
                    }
                }
            }();
            sink.next();

            while (!this._released)
            {
                const { done, value } = await this._reader.read();
                if (done)
                    break;

                delayTimeout?.();

                for (const byte of value)
                    sink.next(byte);
            }
        })
    }

    // MARK: Interface
    /**
    @returns {SerialPort}
    @public @readonly*/ get port() { return this._port }

    /**
    @param {unknown} reason
    @param {string} [context]
    @returns {boolean}
    @public*/ rejectAll(reason, context)
    {
        let errorUsed = false;

        if (this._getConfigResponders !== null)
        {
            const responders = this._getConfigResponders;

            for (const { reject, stack } of responders ?? [])
            {
                const error = new Error(context
                    ?? `'rejectAllPromises()' called on ArduinoInterface.`);
                error.stack = stack;
                error.cause = reason;
                reject(error);
                errorUsed = true;
            }

            responders.length = 0;
        }

        for (const key in this._getPinResponders)
        {
            const pin = Number(key);
            const responders = this._getPinResponders[pin];

            if (responders !== undefined)
            {
                for (const { reject, stack } of responders)
                {
                    const error = new Error(context
                        ?? `'rejectAllPromises()' called on ArduinoInterface.`);
                    error.stack = stack;
                    error.cause = reason;
                    reject(error);
                    errorUsed = true;
                }

                responders.length = 0;
            }
        }

        for (const key in this._setPinResponders)
        {
            const pin = Number(key);
            const responders = this._setPinResponders[pin];

            if (responders !== undefined)
            {
                for (const { reject, stack } of responders)
                {
                    const error = new Error(context
                        ?? `'rejectAllPromises()' called on ArduinoInterface.`);
                    error.stack = stack;
                    error.cause = reason;
                    reject(error);
                    errorUsed = true;
                }

                responders.length = 0;
            }
        }

        for (const key in this._getPinModeResponders)
        {
            const pin = Number(key);
            const responders = this._getPinModeResponders[pin];

            if (responders !== undefined)
            {
                for (const { reject, stack } of responders)
                {
                    const error = new Error(context
                        ?? `'rejectAllPromises()' called on ArduinoInterface.`);
                    error.stack = stack;
                    error.cause = reason;
                    reject(error);
                    errorUsed = true;
                }

                responders.length = 0;
            }
        }

        for (const key in this._setPinModeResponders)
        {
            const pin = Number(key);
            const responders = this._setPinModeResponders[pin];

            if (responders !== undefined)
            {
                for (const { reject, stack } of responders)
                {
                    const error = new Error(context
                        ?? `'rejectAllPromises()' called on ArduinoInterface.`);
                    error.stack = stack;
                    error.cause = reason;
                    reject(error);
                    errorUsed = true;
                }

                responders.length = 0;
            }
        }

        return errorUsed;
    }

    /**
    @returns {Promise<ArduinoConfig>}
    @public*/ async getConfig()
    {
        return new Promise((resolve, reject) =>
        {
            if (this._released)
                return reject(new Error(
                    "ArduinoInterface is released."));

            if (this._cachedConfig !== null)
                return resolve(this._cachedConfig);

            (this._getConfigResponders ??= []).push(
            {
                resolve, reject, stack: new Error().stack,
            });

            this._unpauseQueue?.();
        });
    }

    /**
    @returns {boolean}
    @public @readonly*/ get released() { return this._released }

    /**
    @public*/ release()
    {
        if (this._released)
            return;

        this._released = true;

        const event = { target: this };

        for (const listener of this._eventListeners["release"])
            setTimeout(listener, undefined, event);

        this.rejectAll(
            new Error(
                "'release()' was called on ArduinoInterface."),
            "ArduinoInterface is released.");

        this._reader.releaseLock();
        this._writer.releaseLock();
    }

    /**
    @param {string} pinName
    @returns {Promise<number>}
    @public*/ async pinNameToId(pinName)
    {
        const config = await this.getConfig();

        let pinId;

        for (const pinConfig of config.pins)
            if (pinConfig.name === pinName)
                pinId = pinConfig.id;

        if (pinId === undefined)
            throw new Error(
                `Pin with name ${pinName} does not exist.`);

        return pinId;
    }

    /**
    @overload
    @param {number} pinId
    @returns {Promise<boolean>}
    *//**
    @overload
    @param {string} pinName
    @returns {Promise<boolean>}
    *//**
    @param {number | string} pin
    @returns {Promise<boolean>}
    @public*/ async getPin(pin)
    {
        return new Promise(async (resolve, reject) =>
        {
            if (this._released)
                return reject(new Error(
                    "ArduinoInterface is released."));

            if (typeof pin !== "number")
            {
                pin = await this.pinNameToId(pin);
            }

            (this._getPinResponders[pin] ??= []).push(
            {
                resolve, reject, stack: new Error().stack,
            });

            this._getPinRequest[pin] = true;

            this._unpauseQueue?.();
        });
    }

    /**
    @overload
    @param {number} pinId
    @param {boolean} isHigh
    @returns {Promise<void>}
    *//**
    @overload
    @param {string} pinName
    @param {boolean} isHigh
    @returns {Promise<void>}
    *//**
    @param {number | string} pin
    @param {boolean} isHigh
    @returns {Promise<void>}
    @public*/ async setPin(pin, isHigh)
    {
        return new Promise(async (resolve, reject) =>
        {
            if (this._released)
                return reject(new Error(
                    "ArduinoInterface is released."));

            if (typeof pin !== "number")
            {
                pin = await this.pinNameToId(pin);
            }

            (this._setPinResponders[pin] ??= []).push(
            {
                resolve, reject, stack: new Error().stack,
            });

            this._setPinRequest[pin] = isHigh;

            this._unpauseQueue?.();
        });
    }

    /**
    @overload
    @param {number} pinId
    @returns {Promise<ArduinoPinMode>}
    *//**
    @overload
    @param {string} pinName
    @returns {Promise<ArduinoPinMode>}
    *//**
    @param {number | string} pin
    @returns {Promise<ArduinoPinMode>}
    @public*/ async getPinMode(pin)
    {
        return new Promise(async (resolve, reject) =>
        {
            if (this._released)
                return reject(new Error(
                    "ArduinoInterface is released."));

            if (typeof pin !== "number")
            {
                pin = await this.pinNameToId(pin);
            }

            (this._getPinModeResponders[pin] ??= []).push(
            {
                resolve, reject, stack: new Error().stack,
            });

            this._getPinModeRequest[pin] = true;

            this._unpauseQueue?.();
        });
    }

    /**
    @overload
    @param {number} pinId
    @param {ArduinoPinMode} mode
    @returns {Promise<void>}
    *//**
    @overload
    @param {string} pinName
    @param {ArduinoPinMode} mode
    @returns {Promise<void>}
    *//**
    @param {number | string} pin
    @param {ArduinoPinMode} mode
    @returns {Promise<void>}
    @public*/ async setPinMode(pin, mode)
    {
        return new Promise(async (resolve, reject) =>
        {
            if (this._released)
                return reject(new Error(
                    "ArduinoInterface is released."));

            if (typeof pin !== "number")
            {
                pin = await this.pinNameToId(pin);
            }

            (this._setPinModeResponders[pin] ??= []).push(
            {
                resolve, reject, stack: new Error().stack,
            });

            this._setPinModeRequest[pin] = mode;

            this._unpauseQueue?.();
        });
    }

    /**
    @template {keyof ArduinoInterfaceEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: ArduinoInterfaceEventMap[K]) => void} listener
    @public*/ addEventListener(type, listener)
    {
        switch (type)
        {
            case "pin-change":
            case "release":
                this._eventListeners[type].push(listener);
                break;
            default:
                throw new TypeError(
                    `'${type}' is not a valid event type.`);
        }
    }

    /**
    @template {keyof ArduinoInterfaceEventMap} const K
    @param {K} type
    @param {(this: unknown, ...args: ArduinoInterfaceEventMap[K]) => void} listener
    @public*/ removeEventListener(type, listener)
    {
        switch (type)
        {
            case "pin-change":
            case "release":
            {
                const index = this._eventListeners[type].indexOf(listener);
                if (index !== -1)
                    this._eventListeners[type].splice(index, 1);

                break;
            }
            default:
                throw new TypeError(
                    `'${type}' is not a valid event type.`);
        }
    }
}
// @ts-ignore: Allow use in debug console.
window.ArduinoInterface = ArduinoInterface;