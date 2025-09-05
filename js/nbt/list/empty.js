import { ListTag } from "./index.js";

/**
 * A {@linkcode TagList} that stores bytes (`Uint8[]`).
*/ export class EmptyListTag extends ListTag
{
    /**
    @public*/ constructor()
    {
        super();
    }

    /**
    @returns {number}
    @public @override*/ static elementTypeId() { return 0 }

    /**
    @returns {number}
    @public @override*/ elementTypeId() { return 0 }

    /**
    @returns {number}
    @public @override*/ get length() { return 0 }

    /**
    @returns {ArrayIterator<never>}
    @public*/ [Symbol.iterator]() { return [][Symbol.iterator]() }
}