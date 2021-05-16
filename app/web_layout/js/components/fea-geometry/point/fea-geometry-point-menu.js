class FeaGeometryPointMenu extends HTMLElement {
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
            case "geometry-add-point-menu":
                const feaGeometryAddPointMenu = document.createElement("fea-geometry-add-point-menu");
                this.append(feaGeometryAddPointMenu);
                event.stopPropagation();
                break;
            case "geometry-update-point-menu":
                const feaGeometryUpdatePointMenu = document.createElement("fea-geometry-update-point-menu");
                this.append(feaGeometryUpdatePointMenu);
                event.stopPropagation();
                break;
            case "geometry-delete-point-menu":
                const feaGeometryDeletePointMenu = document.createElement("fea-geometry-delete-point-menu");
                this.append(feaGeometryDeletePointMenu);
                event.stopPropagation();
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "geometry-add-point-menu":
                this.querySelector("fea-geometry-add-point-menu").remove();
                event.stopPropagation();
                break;
            case "geometry-update-point-menu":
                this.querySelector("fea-geometry-update-point-menu").remove();
                event.stopPropagation();
                break;
            case "geometry-delete-point-menu":
                this.querySelector("fea-geometry-delete-point-menu").remove();
                event.stopPropagation();
                break;
        }
    }
}

export default FeaGeometryPointMenu;
