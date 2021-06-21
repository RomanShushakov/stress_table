class FeaGeometryMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            menuNames: {
                "geometry-point-menu": "fea-geometry-point-menu",
                "geometry-line-menu": "fea-geometry-line-menu",
            },
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: flex;
                }

                .wrapper {
                    display: flex;
                    flex-direction: column;
                    background-color: #3b4453;
                    padding: 1rem;
                }

                .geometry-menu-caption {
                    margin: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0.3rem;
                    padding-left: 0rem;
                    padding-right: 0rem;
                    color: #D9D9D9;
                    border-bottom: 0.1rem solid #4a5060;
                    font-size: 85%;
                }
            </style>

            <div class=wrapper>
                <p class="geometry-menu-caption">Geometry</p>
                <fea-geometry-menu-buttons></fea-geometry-menu-buttons>
                <slot></slot>
            </div>
        `;

        this.addEventListener("activate-menu", (event) => this.activateMenu(event));

        this.addEventListener("deactivate-menu", (event) => this.deactivateMenu(event));
    }

    set selectPointInClient(pointNumber) {
        this.shadowRoot.querySelector("fea-geometry-menu-buttons").activateButton = "geometry-point-menu-button";
        this.querySelector("fea-geometry-point-menu").selectPointInClient = pointNumber;
    }

    set selectLineInClient(lineNumber) {
        this.shadowRoot.querySelector("fea-geometry-menu-buttons").activateButton = "geometry-line-menu-button";
        this.querySelector("fea-geometry-line-menu").selectLineInClient = lineNumber;
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

    activateMenu(event) {
        const menuName = event.detail.menuName;
        const menu = document.createElement(this.state.menuNames[menuName]);
        this.append(menu);
        event.stopPropagation();
    }

    deactivateMenu(event) {
        const menuName = event.detail.menuName;
        this.querySelector(this.state.menuNames[menuName]).remove();
        event.stopPropagation();
    }
}

export default FeaGeometryMenu;
