class FeaGeometryMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {};

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
        switch (event.detail.menuName) {
            case "geometry-point-menu":
                const feaGeometryPointMenu = document.createElement("fea-geometry-point-menu");
                this.append(feaGeometryPointMenu);
                event.stopPropagation();
                break;
            case "geometry-line-menu":
                const feaGeometryLineMenu = document.createElement("fea-geometry-line-menu");
                this.append(feaGeometryLineMenu);
                event.stopPropagation();
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "geometry-point-menu":
                this.querySelector("fea-geometry-point-menu").remove();
                event.stopPropagation();
                break;
            case "geometry-line-menu":
                this.querySelector("fea-geometry-line-menu").remove();
                event.stopPropagation();
                break;
        }
    }
}

export default FeaGeometryMenu;
