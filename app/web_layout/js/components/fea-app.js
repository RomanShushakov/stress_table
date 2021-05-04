import { initializeActionsRouter } from "../wasm_js_interface_modules/actions_router_initialization.js";


class FeaApp extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            actionId: 1,
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
        this.addEventListener("add point", (event) => this.addPoint(event.detail.pointData));
        this.addEventListener("update point", (event) => this.updatePoint(event.detail.pointData));
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
        this.updatePreprocessorActionId();
        this.state.actionsRouter.add_geometry_to_activated_preprocessor();
    }


    updatePreprocessorActionId() {
        this.querySelector("fea-preprocessor").actionId = this.state.actionId;
    }

    activatePostprocessor() {
        this.querySelector("fea-preprocessor").remove();
        const feaPostprocessor = document.createElement("fea-postprocessor");
        this.append(feaPostprocessor);
    }

    handleMessage(message) {
        this.state.actionsRouter.handle_message(message);
    }

    addPoint(pointData) {
        if (pointData[4] === false) {
            this.state.actionId += 1;
            this.updatePreprocessorActionId();
        }
        const point = { number: pointData[0], x: pointData[1], y: pointData[2], z: pointData[3] };
        this.querySelector("fea-preprocessor").addPointFromModule = point;
    }

    updatePoint(pointData) {
        this.state.actionId += 1;
        this.updatePreprocessorActionId();
        const point = { number: pointData[0], x: pointData[1], y: pointData[2], z: pointData[3] };
        this.querySelector("fea-preprocessor").updatePointFromModule = point;
    }
}

export default FeaApp;
