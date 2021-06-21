class FeaGeometryLineMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            menuNames: {
                "geometry-add-line-menu": "fea-geometry-add-line-menu",
                "geometry-update-line-menu": "fea-geometry-update-line-menu",
                "geometry-delete-line-menu": "fea-geometry-delete-line-menu",
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
                <fea-geometry-line-menu-buttons></fea-geometry-line-menu-buttons>
                <slot></slot>
            </div>
        `;

        this.addEventListener("activate-menu", (event) => this.activateMenu(event));

        this.addEventListener("deactivate-menu", (event) => this.deactivateMenu(event));
    }

    set selectLineInClient(lineNumber) {
        this.shadowRoot.querySelector("fea-geometry-line-menu-buttons").activateButton = "geometry-update-line-menu-button";
        this.querySelector("fea-geometry-update-line-menu").selectLineInClient = lineNumber;
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

export default FeaGeometryLineMenu;
