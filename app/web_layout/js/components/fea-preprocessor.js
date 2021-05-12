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
                    background-color: #2e3440;
                }
            </style>
            <div class=wrapper>
                <fea-preprocessor-menu>
                    <fea-geometry></fea-geometry>
                    <fea-properties></fea-properties>
                    <fea-node></fea-node>
                    <fea-element></fea-element>
                    <fea-displacement></fea-displacement>
                    <fea-load></fea-load>
                </fea-preprocessor-menu>
            </div>
        `;
        this.addEventListener("menu open", (event) => this.closeOtherMenus(event));
    }

    set actionId(value) {
        this.props.actionId = value;
        this.updateChildrenActionId();
    }

    set addPointToClient(point) {
        this.shadowRoot.querySelector("fea-geometry").addPointToClient = point;
    }

    set updatePointInClient(point) {
        this.shadowRoot.querySelector("fea-geometry").updatePointInClient = point;
    }

    set deletePointFromClient(point) {
        this.shadowRoot.querySelector("fea-geometry").deletePointFromClient = point;
    }

    set addLineToClient(line) {
        this.shadowRoot.querySelector("fea-geometry").addLineToClient = line;
    }

    set updateLineInClient(line) {
        this.shadowRoot.querySelector("fea-geometry").updateLineInClient = line;
    }

    set deleteLineFromClient(line) {
        this.shadowRoot.querySelector("fea-geometry").deleteLineFromClient = line;
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

    closeOtherMenus(event) {
        switch (event.detail.from) {
            case "geometry":
                this.shadowRoot.querySelector("fea-properties").close = "_empty";
                event.stopPropagation();
                break;
            case "properties":
                this.shadowRoot.querySelector("fea-geometry").close = "_empty";
                event.stopPropagation();
                break;
        }
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
