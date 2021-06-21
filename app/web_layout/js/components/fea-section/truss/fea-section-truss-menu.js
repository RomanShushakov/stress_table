class FeaSectionTrussMenu extends HTMLElement {
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
                <fea-section-truss-menu-buttons></fea-section-truss-menu-buttons>
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
            case "section-add-truss-menu":
                const feaSectionAddTrussMenu = document.createElement("fea-section-add-truss-menu");
                this.append(feaSectionAddTrussMenu);
                event.stopPropagation();
                break;
            case "section-update-truss-menu":
                const feaSectionUpdateTrussMenu = document.createElement("fea-section-update-truss-menu");
                this.append(feaSectionUpdateTrussMenu);
                event.stopPropagation();
                break;
            case "section-delete-truss-menu":
                const feaSectionDeleteTrussMenu = document.createElement("fea-section-delete-truss-menu");
                this.append(feaSectionDeleteTrussMenu);
                event.stopPropagation();
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "section-add-truss-menu":
                this.querySelector("fea-section-add-truss-menu").remove();
                event.stopPropagation();
                break;
            case "section-update-truss-menu":
                this.querySelector("fea-section-update-truss-menu").remove();
                event.stopPropagation();
                break;
            case "section-delete-truss-menu":
                this.querySelector("fea-section-delete-truss-menu").remove();
                event.stopPropagation();
                break;
        }
    }
}

export default FeaSectionTrussMenu;
