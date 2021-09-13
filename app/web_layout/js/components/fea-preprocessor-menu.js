class FeaPreprocessorMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            menuNames: {
                "geometry-menu": "fea-geometry-menu",
                "material-menu": "fea-material-menu",
                "section-menu": "fea-section-menu",
                "properties-menu": "fea-properties-menu",
                "load-menu": "fea-load-menu",
                "boundary-condition-menu": "fea-boundary-condition-menu",
            },
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

    delay(t, v) {
        return new Promise(function(resolve) { 
            setTimeout(resolve.bind(null, v), t)
        });
    }

    set selectPointInClient(pointNumber) {
        if (this.querySelector("fea-geometry-menu") === null) {
            this.delay(0)
                .then(() => { 
                    this.shadowRoot.querySelector("fea-preprocessor-menu-buttons").toggleButton = "geometry-menu-button";
                })
                .then(async () => { this.querySelector("fea-geometry-menu").selectPointInClient = pointNumber });
        } else {
            this.delay(0)
                .then(() => { 
                    this.querySelector("fea-geometry-menu").selectPointInClient = pointNumber;
                });
        }
    }

    set selectLineInClient(lineNumber) {
        if (this.querySelector("fea-geometry-menu") === null) {
            this.delay(0)
                .then(() => { 
                    this.shadowRoot.querySelector("fea-preprocessor-menu-buttons").toggleButton = "geometry-menu-button";
                })
                .then(async () => { this.querySelector("fea-geometry-menu").selectLineInClient = lineNumber });
        } else {
            this.delay(0)
                .then(() => { 
                    this.querySelector("fea-geometry-menu").selectLineInClient = lineNumber;
                });
        }
    }

    set selectConcentratedLoadInClient(concentratedLoadPointNumber) {
        if (this.querySelector("fea-load-menu") === null) {
            this.delay(0)
                .then(() => { 
                    this.shadowRoot.querySelector("fea-preprocessor-menu-buttons").toggleButton = "load-menu-button";
                })
                .then(async () => { this.querySelector("fea-load-menu").selectConcentratedLoadInClient = concentratedLoadPointNumber });
        } else {
            this.delay(0)
                .then(() => { 
                    this.querySelector("fea-load-menu").selectConcentratedLoadInClient = concentratedLoadPointNumber;
                });
        }
    }

    set selectDistributedLineLoadInClient(distributedLineLoadLineNumber) {
        if (this.querySelector("fea-load-menu") === null) {
            this.delay(0)
                .then(() => { 
                    this.shadowRoot.querySelector("fea-preprocessor-menu-buttons").toggleButton = "load-menu-button";
                })
                .then(async () => { this.querySelector("fea-load-menu").selectDistributedLineLoadInClient = distributedLineLoadLineNumber });
        } else {
            this.delay(0)
                .then(() => { 
                    this.querySelector("fea-load-menu").selectDistributedLineLoadInClient = distributedLineLoadLineNumber;
                });
        }
    }

    set selectBoundaryConditionInClient(boundaryConditionPointNumber) {
        if (this.querySelector("fea-boundary-condition-menu") === null) {
            this.delay(0)
                .then(() => { 
                    this.shadowRoot.querySelector("fea-preprocessor-menu-buttons").toggleButton = "boundary-condition-menu-button";
                })
                .then(async () => { this.querySelector("fea-boundary-condition-menu").selectBoundaryConditionInClient = 
                    boundaryConditionPointNumber });
        } else {
            this.delay(0)
                .then(() => { 
                    this.querySelector("fea-boundary-condition-menu").selectBoundaryConditionInClient = 
                        boundaryConditionPointNumber;
                });
        }
    }

    connectedCallback() {
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

    updateCanvasSize() {
        this.dispatchEvent(new CustomEvent("resize", {
            bubbles: true,
            composed: true,
        }));
    }

    activateMenu(event) {
        const menuName = event.detail.menuName;
        const menu = document.createElement(this.state.menuNames[menuName]);
        this.append(menu);
        event.stopPropagation();
        this.updateCanvasSize();
    }

    deactivateMenu(event) {
        const menuName = event.detail.menuName;
        this.querySelector(this.state.menuNames[menuName]).remove();
        event.stopPropagation();
        this.updateCanvasSize();
    }
}

export default FeaPreprocessorMenu;
