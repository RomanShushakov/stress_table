import { initializeActionsRouter } from "../wasm_modules_initialization/actions_router_initialization.js";


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
                    display: flex;
                }

                .wrapper {
                    display: flex;
                    align-items: start;
                    flex-direction: row-reverse;
                    box-sizing: content-box;
                }
            </style>
            <div class="main-window">
                <fea-app-title-bar></fea-app-title-bar>
                <div class="wrapper">
                    <fea-renderer></fea-renderer>
                    <slot></slot>
                </div>
            </div>
        `;

        this.addEventListener("activate-postprocessor", () => this.activatePostprocessor());
        this.addEventListener("activate-preprocessor", () => this.activatePreprocessor());

        this.addEventListener("client message", (event) => this.handleClientMessage(event));

        this.addEventListener("add point server message", (event) => this.handleAddPointServerMessage(event));
        this.addEventListener("update point server message", (event) => this.handleUpdatePointServerMessage(event));
        this.addEventListener("delete point server message", (event) => this.handleDeletePointServerMessage(event));

        this.addEventListener("add line server message", (event) => this.handleAddLineServerMessage(event));
        this.addEventListener("update line server message", (event) => this.handleUpdateLineServerMessage(event));
        this.addEventListener("delete line server message", (event) => this.handleDeleteLineServerMessage(event));

        this.addEventListener("decrease action id", (event) => this.handleDecreaseActionIdMessage(event));
    }

    async connectedCallback() {
        this.state.actionsRouter = await initializeActionsRouter(this.greeting);
        this.activatePreprocessor();
        this.updateTitleBarActionId();
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

    updateTitleBarActionId() {
        this.shadowRoot.querySelector("fea-app-title-bar").actionId = this.state.actionId;
    }

    activatePostprocessor() {
        this.querySelector("fea-preprocessor").remove();
        const feaPostprocessor = document.createElement("fea-postprocessor");
        this.append(feaPostprocessor);
    }

    handleClientMessage(event) {
        this.state.actionsRouter.handle_message(event.detail.message, (objectInfo) => this.showObjectInfo(objectInfo));
        event.stopPropagation();
    }

    showObjectInfo(objectInfo) {
        this.shadowRoot.querySelector("fea-renderer").objectInfo = objectInfo;
    }

    handleAddPointServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
            this.updatePreprocessorActionId();
            this.updateTitleBarActionId();
        }
        const point = { 
            number: event.detail.point_data.number, x: event.detail.point_data.x,
            y: event.detail.point_data.y, z: event.detail.point_data.z };
        this.querySelector("fea-preprocessor").addPointToClient = point;
        this.shadowRoot.querySelector("fea-renderer").addPointToRenderer = point;
        event.stopPropagation();
    }

    handleUpdatePointServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
            this.updatePreprocessorActionId();
            this.updateTitleBarActionId();
        }
        const point = { number: event.detail.point_data.number, x: event.detail.point_data.x,
            y: event.detail.point_data.y, z: event.detail.point_data.z };
        this.querySelector("fea-preprocessor").updatePointInClient = point;
        this.shadowRoot.querySelector("fea-renderer").updatePointInRenderer = point;
        event.stopPropagation();
    }

    handleDeletePointServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
            this.updatePreprocessorActionId();
            this.updateTitleBarActionId();
        }
        const point = { number: event.detail.point_data.number };
        this.querySelector("fea-preprocessor").deletePointFromClient = point;
        this.shadowRoot.querySelector("fea-renderer").deletePointFromRenderer = point;
        event.stopPropagation();
    }

    handleAddLineServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
            this.updatePreprocessorActionId();
            this.updateTitleBarActionId();
        }
        const line = { 
            number: event.detail.line_data.number,
            startPointNumber: event.detail.line_data.start_point_number,
            endPointNumber: event.detail.line_data.end_point_number };
        this.querySelector("fea-preprocessor").addLineToClient = line;
        this.shadowRoot.querySelector("fea-renderer").addLineToRenderer = line;
        event.stopPropagation();
    }

    handleUpdateLineServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
            this.updatePreprocessorActionId();
            this.updateTitleBarActionId();
        }
        const line = { 
            number: event.detail.line_data.number,
            startPointNumber: event.detail.line_data.start_point_number,
            endPointNumber: event.detail.line_data.end_point_number };
        this.querySelector("fea-preprocessor").updateLineInClient = line;
        this.shadowRoot.querySelector("fea-renderer").updateLineInRenderer = line;
        event.stopPropagation();
    }

    handleDeleteLineServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
            this.updatePreprocessorActionId();
            this.updateTitleBarActionId();
        }
        const line = { number: event.detail.line_data.number };
        this.querySelector("fea-preprocessor").deleteLineFromClient = line;
        this.shadowRoot.querySelector("fea-renderer").deleteLineFromRenderer = line;
        event.stopPropagation();
    }

    handleDecreaseActionIdMessage() {
        this.state.actionId -= 1;
        if (this.querySelector("fea-preprocessor") !== null) {
            this.updatePreprocessorActionId();
        }
        this.updateTitleBarActionId();
    }
}

export default FeaApp;
