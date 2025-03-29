/**
@template {{ [N in string]: readonly unknown[] }} M
*/ export class ReadonlyCustomEventHandler
{
    /**
    @protected*/ constructor()
    {
        /**
        @type {{ [N in keyof M]?: ((...args: M[N]) => void)[] }}
        @protected*/ this.listenerLists = {};
    }

    /**
    @template {keyof M} N
    @param {N} type
    @param {(...args: M[N]) => void} listener
    @public*/ addListener(type, listener)
    {
        const listeners = this.listenerLists[type];

        if (listeners === undefined)
            this.listenerLists[type] = [listener];
        else
            listeners.push(listener);
    }

    /**
    @template {keyof M} N
    @param {N} type
    @param {(...args: M[N]) => void} listener
    @public*/ removeListener(type, listener)
    {
        const listeners = this.listenerLists[type];

        if (listeners !== undefined)
        {
            const index = listeners.indexOf(listener);
            if (index > 0)
                listeners.splice(index, 1);
        }
    }
}

/**
@extends {ReadonlyCustomEventHandler<M>}
@template {{ [N in string]: readonly unknown[] }} M
*/ export class CustomEventHandler extends ReadonlyCustomEventHandler
{
    /**
    @public*/ constructor()
    {
        super();
    }

    /**
    @template {keyof M} N
    @param {N} type
    @param {M[N]} args
    @public*/ dispatch(type, ...args)
    {
        const listeners = this.listenerLists[type];

        if (listeners !== undefined)
            for (const listener of listeners)
                listener(...args);
    }
}