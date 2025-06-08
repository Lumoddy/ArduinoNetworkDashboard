import { GraphView } from "../graphics/graph-view.js";

/**
*/ export class NetworkGraphView extends GraphView
{
    /**
    @override
    @protected @readonly*/ static observedAttributes =
    [
        ...GraphView.observedAttributes
    ];

    /**
    @public*/ constructor()
    {
        super();

        const node = super.createGraphNode();
        node.appendChild(document.crea);
    }

    /**
    @override
    @protected*/ connectedCallback()
    {
        super.connectedCallback();
    }

    /**
    @override
    @protected*/ disconnectedCallback()
    {
        super.disconnectedCallback();
    }

    /**
    @param {typeof NetworkGraphView["observedAttributes"][number]} attributeName
    @param {string | null} oldValue
    @param {string | null} newValue
    @override
    @protected*/ attributeChangedCallback(attributeName, oldValue, newValue)
    {
        switch (attributeName)
        {
            default:
                super.attributeChangedCallback(attributeName, oldValue, newValue);
                break;
        }
    }
}
customElements.define("network-graph-view", NetworkGraphView);