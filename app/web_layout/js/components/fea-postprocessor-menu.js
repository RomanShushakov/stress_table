class FeaPostprocessorMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {};

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                }
            </style>
            <div>
                <p>Hello from fea-postprocessor-menu</p>
                <button class="fem">FEM</button>
                <button>Plot Reactions</button>
                <slot></slot>
            </div>
        `;

        this.shadowRoot.querySelector(".fem").addEventListener("click", () => this.activatePreprocessor());
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

    activatePreprocessor() {
        this.dispatchEvent(new CustomEvent("activate-preprocessor", {
            bubbles: true,
            composed: true,
        }));

    }

}

export default FeaPostprocessorMenu;