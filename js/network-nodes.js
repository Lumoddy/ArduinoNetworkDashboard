import { LoudArray, ReadonlyLoudArray } from "./loud";

/**
*/ export class NetworkNode
{
    /**
    @public*/ constructor()
    {
        /**
        @type {LoudArray<NetworkNode | NetworkRootNode>}
        @private*/ this._connectedTo = new LoudArray();
    }
}

/**
*/ export class NetworkRootNode
{
    /**
    @public */ constructor()
    {

    }
}