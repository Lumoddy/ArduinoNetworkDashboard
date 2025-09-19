
/**
*/ export class EdgeResizer extends HTMLElement
{
    /**
    @public*/ constructor()
    {
        super();

        this.addEventListener("pointerdown", (e) =>
        {
            const parentElement = this.parentElement;
            const previousElement = this.previousElementSibling;
            const nextElement = this.nextElementSibling;
            if (parentElement === null
                || !(previousElement instanceof HTMLElement)
                || !(nextElement instanceof HTMLElement))
                return;

            switch (window.getComputedStyle(parentElement).flexDirection)
            {
                case "row":
                case "column":
                    break;
                default:
                    return;
            }

            this.setPointerCapture(e.pointerId);
        });

        this.addEventListener("pointerup", (e) =>
        {
            this.releasePointerCapture(e.pointerId);
        });

        this.addEventListener("pointermove", (e) =>
        {
            if (!this.hasPointerCapture(e.pointerId))
                return;

            const parentElement = this.parentElement;
            const previousElement = this.previousElementSibling;
            const nextElement = this.nextElementSibling;
            if (parentElement === null
                || !(previousElement instanceof HTMLElement)
                || !(nextElement instanceof HTMLElement))
                return;

            switch (window.getComputedStyle(parentElement).flexDirection)
            {
                case "row":
                {
                    const totalFlex
                        = Number(window.getComputedStyle(previousElement).flexGrow)
                        + Number(window.getComputedStyle(nextElement).flexGrow);
                    const startingPreviousElementEdge = previousElement.getBoundingClientRect().left;
                    const startingNextElementEdge = nextElement.getBoundingClientRect().right;
                    let rect = this.getBoundingClientRect();
                    let edge = rect.x + (rect.width * 0.5);
                    const target = e.clientX;

                    let lower = 0;
                    let upper = totalFlex;

                    const tolerance = totalFlex / (startingNextElementEdge - startingPreviousElementEdge);

                    /**
                    @type {{ previousFlexGrow: string, nextFlexGrow: string }?}
                    */ let undoIfExitingLoop = null;
                    while (Math.abs(edge - target) && upper > lower + tolerance)
                    {
                        const middle = (lower + upper) * 0.5;
                        previousElement.style.flexGrow = `${middle}`;
                        nextElement.style.flexGrow = `${totalFlex - middle}`;

                        rect = this.getBoundingClientRect();
                        edge = rect.x + (rect.width * 0.5);

                        const previousEdgeMoved =
                            previousElement.getBoundingClientRect().left <
                            startingPreviousElementEdge - 0.1;
                        const nextEdgeMoved =
                            nextElement.getBoundingClientRect().right >
                            startingNextElementEdge + 0.1;

                        undoIfExitingLoop = null;

                        if (nextEdgeMoved)
                        {
                            undoIfExitingLoop =
                            {
                                previousFlexGrow: previousElement.style.flexGrow,
                                nextFlexGrow: nextElement.style.flexGrow,
                            }

                            if (previousEdgeMoved)
                                break;

                            upper = middle;
                        }
                        else if (previousEdgeMoved)
                        {
                            undoIfExitingLoop =
                            {
                                previousFlexGrow: previousElement.style.flexGrow,
                                nextFlexGrow: nextElement.style.flexGrow,
                            }

                            lower = middle;
                        }
                        else if (target < edge)
                            upper = middle;
                        else
                            lower = middle;
                    }

                    if (undoIfExitingLoop !== null)
                    {
                        ({
                            previousFlexGrow: previousElement.style.flexGrow,
                            nextFlexGrow: nextElement.style.flexGrow,
                        }
                        = undoIfExitingLoop);
                    }

                    break;
                }
                case "column":
                {
                    const totalFlex
                        = Number(window.getComputedStyle(previousElement).flexGrow)
                        + Number(window.getComputedStyle(nextElement).flexGrow);
                    const startingPreviousElementEdge = previousElement.getBoundingClientRect().top;
                    const startingNextElementEdge = nextElement.getBoundingClientRect().bottom;
                    let rect = this.getBoundingClientRect();
                    let edge = rect.y + (rect.height * 0.5);
                    const target = e.clientY;

                    let lower = 0;
                    let upper = totalFlex;

                    const tolerance = totalFlex / (startingNextElementEdge - startingPreviousElementEdge);

                    /**
                    @type {{ previousFlexGrow: string, nextFlexGrow: string }?}
                    */ let undoIfExitingLoop = null;
                    while (Math.abs(edge - target) && upper > lower + tolerance)
                    {
                        const middle = (lower + upper) * 0.5;
                        previousElement.style.flexGrow = `${middle}`;
                        nextElement.style.flexGrow = `${totalFlex - middle}`;

                        rect = this.getBoundingClientRect();
                        edge = rect.y + (rect.height * 0.5);

                        const previousEdgeMoved =
                            previousElement.getBoundingClientRect().top <
                            startingPreviousElementEdge - 0.1;
                        const nextEdgeMoved =
                            nextElement.getBoundingClientRect().bottom >
                            startingNextElementEdge + 0.1;

                        undoIfExitingLoop = null;

                        if (nextEdgeMoved)
                        {
                            undoIfExitingLoop =
                            {
                                previousFlexGrow: previousElement.style.flexGrow,
                                nextFlexGrow: nextElement.style.flexGrow,
                            }

                            if (previousEdgeMoved)
                                break;

                            upper = middle;
                        }
                        else if (previousEdgeMoved)
                        {
                            undoIfExitingLoop =
                            {
                                previousFlexGrow: previousElement.style.flexGrow,
                                nextFlexGrow: nextElement.style.flexGrow,
                            }

                            lower = middle;
                        }
                        else if (target < edge)
                            upper = middle;
                        else
                            lower = middle;
                    }

                    if (undoIfExitingLoop !== null)
                    {
                        ({
                            previousFlexGrow: previousElement.style.flexGrow,
                            nextFlexGrow: nextElement.style.flexGrow,
                        }
                        = undoIfExitingLoop);
                    }

                    break;
                }
                default:
                    break;
            }
        });
    }
}
customElements.define("edge-resizer", EdgeResizer);
