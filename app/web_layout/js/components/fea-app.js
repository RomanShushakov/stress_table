class FeaApp extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            actionId: 1,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                }
            </style>
            <div>
                <fea-app-title-bar></fea-app-title-bar>
                <slot></slot>
            </div>
        `;

        this.addEventListener("activate-postprocessor", () => this.activatePostprocessor());
        this.addEventListener("activate-preprocessor", () => this.activatePreprocessor());
    }

    async connectedCallback() {
        this.activatePreprocessor();
        this.updatePreprocessor();
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
        if (this.querySelector("fea-postprocessor") !== null) {
            this.querySelector("fea-postprocessor").remove();
        }
        let feaPreprocessor = document.createElement("fea-preprocessor");
        this.append(feaPreprocessor);
        this.updatePreprocessor();
    }


    updatePreprocessor() {
        this.querySelector("fea-preprocessor").actionId = this.state.actionId;
    }

    activatePostprocessor() {
        this.querySelector("fea-preprocessor").remove();
        let feaPostprocessor = document.createElement("fea-postprocessor");
        this.append(feaPostprocessor);
    }

}

export default FeaApp;