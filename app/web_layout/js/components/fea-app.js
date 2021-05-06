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

                .wrapper {
                    display: flex;
                    align-items: start;
                    flex-direction: row;
                    box-sizing: content-box;
                }
            </style>
            <div class="main-window">
                <fea-app-title-bar></fea-app-title-bar>
                <div class="wrapper">
                    <!-- <preprocessor-canvas></preprocessor-canvas>  -->
                    <slot></slot>
                </div>
            </div>
        `;

        this.addEventListener("activate-postprocessor", () => this.activatePostprocessor());
        this.addEventListener("activate-preprocessor", () => this.activatePreprocessor());
        this.addEventListener("client message", (event) => this.handleClientMessage(event));
        this.addEventListener("add point server message", (event) => this.handleAddPointServerMessage(event));
        this.addEventListener("update point server message", (event) => this.handleUpdatePointServerMessage(event));
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
        if (this.state.actionId !== 1) {
            this.state.actionsRouter.add_whole_geometry_to_preprocessor();
        }
    }

    updatePreprocessorActionId() {
        this.querySelector("fea-preprocessor").actionId = this.state.actionId;
    }

    activatePostprocessor() {
        this.querySelector("fea-preprocessor").remove();
        const feaPostprocessor = document.createElement("fea-postprocessor");
        this.append(feaPostprocessor);
    }

    handleClientMessage(event) {
        this.state.actionsRouter.handle_message(event.detail.message);
        event.stopPropagation();
    }

    handleAddPointServerMessage(event) {
        if (event.detail.is_preprocessor_request === false) {
            this.state.actionId += 1;
            this.updatePreprocessorActionId();
        }
        const point = { 
            number: event.detail.point_data.number, x: event.detail.point_data.x,
            y: event.detail.point_data.y, z: event.detail.point_data.z };
        this.querySelector("fea-preprocessor").addPointFromServer = point;
        event.stopPropagation();
    }

    handleUpdatePointServerMessage(event) {
        this.state.actionId += 1;
        this.updatePreprocessorActionId();
        const point = { number: event.detail.point_data.number, x: event.detail.point_data.x,
            y: event.detail.point_data.y, z: event.detail.point_data.z };
        this.querySelector("fea-preprocessor").updatePointFromServer = point;
        event.stopPropagation();
    }
}

export default FeaApp;
