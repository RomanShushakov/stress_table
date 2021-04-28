class FeaPreprocessorMenu extends HTMLElement {
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
                <p>Hello from fea-preprocessor-menu</p>
                <slot></slot>
                <button>Submit</button>
                <button>Edit Model</button>
                <button class="result">Result</button>
            </div>
        `;

        this.shadowRoot.querySelector(".result").addEventListener("click", () => this.activatePostprocessor());
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

    activatePostprocessor() {
        this.dispatchEvent(new CustomEvent("activate-postprocessor", {
            bubbles: true,
            composed: true,
        }));

    }

}

export default FeaPreprocessorMenu;