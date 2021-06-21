class FeaMaterialMenu extends HTMLElement {
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
                .material-menu-caption {
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
                <p class="material-menu-caption">Material</p>
                <fea-material-menu-buttons></fea-material-menu-buttons>
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
            case "material-add-material-menu":
                const feaMaterialAddMaterialMenu = document.createElement("fea-material-add-material-menu");
                this.append(feaMaterialAddMaterialMenu);
                event.stopPropagation();
                break;
            case "material-update-material-menu":
                const feaMaterialUpdateMaterialMenu = document.createElement("fea-material-update-material-menu");
                this.append(feaMaterialUpdateMaterialMenu);
                event.stopPropagation();
                break;
            case "material-delete-material-menu":
                const feaMaterialDeleteMaterialMenu = document.createElement("fea-material-delete-material-menu");
                this.append(feaMaterialDeleteMaterialMenu);
                event.stopPropagation();
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "material-add-material-menu":
                this.querySelector("fea-material-add-material-menu").remove();
                event.stopPropagation();
                break;
            case "material-update-material-menu":
                this.querySelector("fea-material-update-material-menu").remove();
                event.stopPropagation();
                break;
            case "material-delete-material-menu":
                this.querySelector("fea-material-delete-material-menu").remove();
                event.stopPropagation();
                break;
        }
    }
}

export default FeaMaterialMenu;
