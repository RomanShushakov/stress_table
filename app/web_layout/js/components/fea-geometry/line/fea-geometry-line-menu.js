class FeaGeometryLineMenu extends HTMLElement {
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

    set actionId(value) {
        this.props.actionId = value;
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
            case "geometry-add-line-menu":
                const feaGeometryAddLineMenu = document.createElement("fea-geometry-add-line-menu");
                this.append(feaGeometryAddLineMenu);
                event.stopPropagation();
                break;
            case "geometry-update-line-menu":
                const feaGeometryUpdateLineMenu = document.createElement("fea-geometry-update-line-menu");
                this.append(feaGeometryUpdateLineMenu);
                event.stopPropagation();
                break;
            case "geometry-delete-line-menu":
                const feaGeometryDeleteLineMenu = document.createElement("fea-geometry-delete-line-menu");
                this.append(feaGeometryDeleteLineMenu);
                event.stopPropagation();
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "geometry-add-line-menu":
                this.querySelector("fea-geometry-add-line-menu").remove();
                event.stopPropagation();
                break;
            case "geometry-update-line-menu":
                this.querySelector("fea-geometry-update-line-menu").remove();
                event.stopPropagation();
                break;
            case "geometry-delete-line-menu":
                this.querySelector("fea-geometry-delete-line-menu").remove();
                event.stopPropagation();
                break;
        }
    }
}

export default FeaGeometryLineMenu;
