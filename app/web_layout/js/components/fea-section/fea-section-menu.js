class FeaSectionMenu extends HTMLElement {
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

                .section-menu-caption {
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
                <p class="section-menu-caption">Section</p>
                <fea-section-menu-buttons></fea-section-menu-buttons>
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
            case "section-truss-menu":
                const feaSectionTrussMenu = document.createElement("fea-section-truss-menu");
                this.append(feaSectionTrussMenu);
                event.stopPropagation();
                break;
            case "section-beam-menu":
                const feaSectionBeamMenu = document.createElement("fea-section-beam-menu");
                this.append(feaSectionBeamMenu);
                event.stopPropagation();
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "section-truss-menu":
                this.querySelector("fea-section-truss-menu").remove();
                event.stopPropagation();
                break;
            case "section-beam-menu":
                this.querySelector("fea-section-beam-menu").remove();
                event.stopPropagation();
                break;
        }
    }
}

export default FeaSectionMenu;
