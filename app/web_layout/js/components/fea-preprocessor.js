class FeaPreprocessor extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
        };

        this.state = {};

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                }

                .wrapper {
                    background-color: #eee;
                }
            </style>
            <div class="wrapper">
                <fea-preprocessor-menu>
                    <fea-geometry></fea-geometry>
                    <fea-properties></fea-properties>
                    <fea-node></fea-node>
                    <fea-element></fea-element>
                    <fea-displacement></fea-displacement>
                    <fea-load></fea-load>
                </fea-preprocessor-menu>
                <fea-preprocessor-canvas></fea-preprocessor-canvas>
            </div>
        `;
    }


    set actionId(value) {
        this.props.actionId = value;
        this.updateChildrenActionId();
    }

    set addPointToClient(point) {
        this.shadowRoot.querySelector("fea-geometry").addPointToClient = point;
    }

    set updatePointOnClient(point) {
        this.shadowRoot.querySelector("fea-geometry").updatePointOnClient = point;
    }

    set deletePointFromClient(point) {
        this.shadowRoot.querySelector("fea-geometry").deletePointFromClient = point;
    }

    set addLineToClient(line) {
        this.shadowRoot.querySelector("fea-geometry").addLineToClient = line;
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updateChildrenActionId();
    }

    disconnectedCallback() {
    }
    
    static get observedAttributes() {
        return [];
    }
    
    attributeChangedCallback(name, oldValue, newValue) {
    }
    
    adoptedCallback() {
    }

    updateChildrenActionId() {
        this.shadowRoot.querySelector("fea-geometry").actionId = this.props.actionId;
    }

}

export default FeaPreprocessor;
