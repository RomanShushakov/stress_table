class FeaPreprocessor extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
            points: [],
        };

        this.state = {
            childrenNamesForActionIdUpdate: [
                "fea-geometry-menu",
            ],

            childrenNamesForPointCrud: [
                "fea-geometry-menu",
            ],
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
        this.props.points.push(point);
        this.props.points.sort((a, b) => a.number - b.number);
        this.addPointToChildren(point);
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
                this.querySelector("fea-geometry-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.points.length; i++) {
                    const point = this.props.points[i];
                    this.querySelector("fea-geometry-menu").addPointToClient = point;
                } 
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

    updateCanvasSize() {
        this.dispatchEvent(new CustomEvent("resize", {
            bubbles: true,
            composed: true,
        }));
    }

    updateChildrenActionId() {
        for (let i = 0; i < this.state.childrenNamesForActionIdUpdate.length; i++) {
            if (this.querySelector(this.state.childrenNamesForActionIdUpdate[i]) !== null) {
                this.querySelector(this.state.childrenNamesForActionIdUpdate[i]).actionId = this.props.actionId;
            }
        } 
    }

    addPointToChildren(point) {
        for (let i = 0; i < this.state.childrenNamesForPointCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForPointCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForPointCrud[i]).addPointToClient = point;
            }
        } 
    }
}

export default FeaPreprocessor;
