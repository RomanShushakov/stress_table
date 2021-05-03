import { initializeActionsRouter } from "../wasm_js_interface_modules/actions_router_initialization.js";
import { initializeGeometry } from "../wasm_js_interface_modules/geometry_initialization.js";


class FeaApp extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            actionsRouter: null,
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
        this.addEventListener("client message", (event) => this.handleMessage(event.detail.message));
        this.addEventListener("add point", (event) => this.addPoint(event.detail.message));
    }

    async connectedCallback() {
        this.state.actionsRouter = await initializeActionsRouter(this.greeting);
        this.activatePreprocessor();
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
        const feaPreprocessor = document.createElement("fea-preprocessor");
        this.append(feaPreprocessor);
        this.updatePreprocessor();
    }


    updatePreprocessor() {
        this.querySelector("fea-preprocessor").actionId = this.state.actionsRouter.get_action_id();
    }

    activatePostprocessor() {
        this.querySelector("fea-preprocessor").remove();
        const feaPostprocessor = document.createElement("fea-postprocessor");
        this.append(feaPostprocessor);
    }

    handleMessage(message) {
        this.state.actionsRouter.handle_message(message);
    }

    addPoint(message) {
        console.log(message);
    }
}

export default FeaApp;