class FeaSectionBeamMenu extends HTMLElement {
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
                <fea-section-beam-menu-buttons></fea-section-beam-menu-buttons>
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
            case "section-add-beam-menu":
                const feaSectionAddBeamMenu = document.createElement("fea-section-add-beam-menu");
                this.append(feaSectionAddBeamMenu);
                event.stopPropagation();
                break;
            case "section-update-beam-menu":
                const feaSectionUpdateBeamMenu = document.createElement("fea-section-update-beam-menu");
                this.append(feaSectionUpdateBeamMenu);
                event.stopPropagation();
                break;
            case "section-delete-beam-menu":
                const feaSectionDeleteBeamMenu = document.createElement("fea-section-delete-beam-menu");
                this.append(feaSectionDeleteBeamMenu);
                event.stopPropagation();
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "section-add-beam-menu":
                this.querySelector("fea-section-add-beam-menu").remove();
                event.stopPropagation();
                break;
            case "section-update-beam-menu":
                this.querySelector("fea-section-update-beam-menu").remove();
                event.stopPropagation();
                break;
            case "section-delete-beam-menu":
                this.querySelector("fea-section-delete-beam-menu").remove();
                event.stopPropagation();
                break;
        }
    }
}

export default FeaSectionBeamMenu;
