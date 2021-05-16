class FeaPreprocessor extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
        };

        this.state = {
            childrenNames: [
                "fea-geometry-menu",
            ]
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                }

                .wrapper {
                    background-color: #2e3440;
                    display: flex;
                    flex-direction: row;
                }
            </style>
            <div class=wrapper>
                <fea-preprocessor-menu-buttons></fea-preprocessor-menu-buttons>
                <slot></slot>
            </div>
        `;
        
        this.addEventListener("activate-menu", (event) => this.activateMenu(event));

        this.addEventListener("deactivate-menu", (event) => this.deactivateMenu(event));
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

    activateMenu(event) {
        switch (event.detail.menuName) {
            case "geometry-menu":
                const feaGeometryMenu = document.createElement("fea-geometry-menu");
                this.append(feaGeometryMenu);
                event.stopPropagation();
                this.updateCanvasSize();
                this.updateChildrenActionId();
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "geometry-menu":
                this.querySelector("fea-geometry-menu").remove();
                event.stopPropagation();
                this.updateCanvasSize();
                break;
        }
    }

    updateChildrenActionId() {
        for (let i = 0; i < this.state.childrenNames.length; i ++) {
            if (this.querySelector(this.state.childrenNames[i]) !== null) {
                console.log(this.state.childrenNames[i]);
            }
        } 
        // this.querySelector("fea-geometry").actionId = this.props.actionId;
    }

    updateCanvasSize() {
        this.dispatchEvent(new CustomEvent("resize", {
            bubbles: true,
            composed: true,
        }));
    }
}

export default FeaPreprocessor;
