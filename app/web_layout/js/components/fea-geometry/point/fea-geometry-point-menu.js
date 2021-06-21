class FeaGeometryPointMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            menuNames: {
                "geometry-add-point-menu": "fea-geometry-add-point-menu",
                "geometry-update-point-menu": "fea-geometry-update-point-menu",
                "geometry-delete-point-menu": "fea-geometry-delete-point-menu",
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
                    padding: 0rem;
                }
            </style>

            <div class=wrapper>
                <fea-geometry-point-menu-buttons></fea-geometry-point-menu-buttons>
                <slot></slot>
            </div>
        `;

        this.addEventListener("activate-menu", (event) => this.activateMenu(event));

        this.addEventListener("deactivate-menu", (event) => this.deactivateMenu(event));
    }

    set selectPointInClient(pointNumber) {
        this.shadowRoot.querySelector("fea-geometry-point-menu-buttons").activateButton = "geometry-update-point-menu-button";
        this.querySelector("fea-geometry-update-point-menu").selectPointInClient = pointNumber;
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

export default FeaGeometryPointMenu;
