import { initializeActionsRouter } from "../wasm_modules_initialization/actions_router_initialization.js";


class FeaApp extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            actionId: 1,                // u32;
            actionsRouter: null,        // wasm module "actions_router";
            isGeometryLoaded: false,    // load status of wasm module "geometry";
            isPropertiesLoaded: false,  // load status of wasm module "properties";
            linesMultipleSelectionModeEnabled: false,
            pointsDataDependentMenus: [
                "fea-geometry-add-point-menu",
                "fea-geometry-update-point-menu",
                "fea-geometry-delete-point-menu",
                "fea-geometry-add-line-menu",
                "fea-geometry-update-line-menu",
            ],
            linesDataDependentMenus: [
                "fea-geometry-add-line-menu",
                "fea-geometry-update-line-menu",
                "fea-geometry-delete-line-menu",
                "fea-properties-assign-properties-menu",
                "fea-properties-beam-section-orientation-menu",
            ],
            materialsDataDependentMenus: [
                "fea-material-add-material-menu",
                "fea-material-update-material-menu",
                "fea-material-delete-material-menu",
                "fea-properties-add-properties-menu",
                "fea-properties-update-properties-menu",
            ],
            trussSectionsDataDependentMenus: [
                "fea-section-add-truss-menu",
                "fea-section-update-truss-menu",
                "fea-section-delete-truss-menu",
                "fea-properties-add-properties-menu",
                "fea-properties-update-properties-menu",
            ],
            beamSectionsDataDependentMenus: [
                "fea-section-add-beam-menu",
                "fea-section-update-beam-menu",
                "fea-section-delete-beam-menu",
                "fea-properties-add-properties-menu",
                "fea-properties-update-properties-menu",
            ]
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: flex;
                }

                .main-window {
                    padding: 0rem;
                    margin: 0rem;
                    display: block;
                }

                .wrapper {
                    display: flex;
                    align-items: start;
                    flex-direction: row-reverse;
                    box-sizing: content-box;
                }
            </style>
            <div class="main-window">
                <fea-app-menu-bar username="${this.getAttribute("username")}"></fea-app-menu-bar>
                <fea-app-tool-bar></fea-app-tool-bar>
                <div class="wrapper">
                    <fea-renderer></fea-renderer>
                    <slot></slot>
                </div>
            </div>
        `;

        window.addEventListener("geometryLoaded", (event) => {
            this.state.isGeometryLoaded = true;
            event.stopPropagation();
        });

        window.addEventListener("propertiesLoaded", (event) => {
            this.state.isPropertiesLoaded = true;
            event.stopPropagation();
        });

        window.addEventListener("resize", () => this.updateCanvasSize());

        this.addEventListener("activatePreprocessorMenu", () => this.activatePreprocessorMenu());
        this.addEventListener("activate-postprocessor", () => this.activatePostprocessor());

        this.addEventListener("getActionId", (event) => this.getActionId(event));
        this.addEventListener("getActionIdForToolBar", (event) => this.getActionIdForToolBar(event));

        this.addEventListener("getGeometryLoadStatus", (event) => this.getGeometryLoadStatus(event));

        this.addEventListener("getPoints", (event) => this.getPoints(event));
        this.addEventListener("getLines", (event) => this.getLines(event));

        this.addEventListener("getPropertiesLoadStatus", (event) => this.getPropertiesLoadStatus(event));

        this.addEventListener("getMaterials", (event) => this.getMaterials(event));
        this.addEventListener("getTrussSections", (event) => this.getTrussSections(event));
        this.addEventListener("getBeamSections", (event) => this.getBeamSections(event));

        this.addEventListener("clientMessage", (event) => this.handleClientMessage(event));

        this.addEventListener("add_point_server_message", (event) => this.handleAddPointServerMessage(event));
        this.addEventListener("update_point_server_message", (event) => this.handleUpdatePointServerMessage(event));
        this.addEventListener("delete_point_server_message", (event) => this.handleDeletePointServerMessage(event));

        this.addEventListener("add_line_server_message", (event) => this.handleAddLineServerMessage(event));
        this.addEventListener("update_line_server_message", (event) => this.handleUpdateLineServerMessage(event));
        this.addEventListener("delete_line_server_message", (event) => this.handleDeleteLineServerMessage(event));

        this.addEventListener("add_material_server_message", (event) => this.handleAddMaterialServerMessage(event));
        this.addEventListener("update_material_server_message", (event) => this.handleUpdateMaterialServerMessage(event));
        this.addEventListener("delete_material_server_message", (event) => this.handleDeleteMaterialServerMessage(event));

        this.addEventListener("add_truss_section_server_message", (event) => this.handleAddTrussSectionServerMessage(event));
        this.addEventListener("update_truss_section_server_message", (event) => this.handleUpdateTrussSectionServerMessage(event));
        this.addEventListener("delete_truss_section_server_message", (event) => this.handleDeleteTrussSectionServerMessage(event));

        this.addEventListener("add_beam_section_server_message", (event) => this.handleAddBeamSectionServerMessage(event));
        this.addEventListener("update_beam_section_server_message", (event) => this.handleUpdateBeamSectionServerMessage(event));
        this.addEventListener("delete_beam_section_server_message", (event) => this.handleDeleteBeamSectionServerMessage(event));

        this.addEventListener("decreaseActionId", (_event) => this.handleDecreaseActionIdMessage());

        this.addEventListener("enableLinesMultipleSelectionMode", 
            (event) => this.handleEnableLinesMultipleSelectionModeMessage(event));

        this.addEventListener("disableLinesMultipleSelectionMode", 
            (event) => this.handleDisableLinesMultipleSelectionModeMessage(event));

        this.addEventListener("refreshPointsData", (event) => this.refreshPointsData(event));
    }

    async connectedCallback() {
        this.state.actionsRouter = await initializeActionsRouter();
        this.activatePreprocessorMenu();
        // this.handleLoadCache();
    }

    disconnectedCallback() {
    }
    
    static get observedAttributes() {
        return ["username"];
    }
    
    attributeChangedCallback(name, oldValue, newValue) {
    }
    
    adoptedCallback() {
    }

    activatePreprocessorMenu() {
        if (this.querySelector("fea-postprocessor-menu") !== null) {
            this.querySelector("fea-postprocessor").remove();
        }
        const feaPreprocessorMenu = document.createElement("fea-preprocessor-menu");
        this.append(feaPreprocessorMenu);
        this.updateCanvasSize();
    }

    getActionId(event) {
        this.querySelector(event.target.tagName.toLowerCase()).actionId = this.state.actionId;
        event.stopPropagation();
    }

    getActionIdForToolBar(event) {
        this.shadowRoot.querySelector("fea-app-tool-bar").actionId = this.state.actionId;
        event.stopPropagation();
    }

    getGeometryLoadStatus(event) {
        this.querySelector(event.target.tagName.toLowerCase()).isGeometryLoaded = this.state.isGeometryLoaded;
        event.stopPropagation();
    }

    getPoints(event) {
        this.state.actionsRouter.extract_points(
            (extractedPointsData) => { 
                const points = new Map(Array.from(
                    Object.entries(extractedPointsData.extracted_points), 
                    ([key, value]) => [parseInt(key), value]
                ));
                this.querySelector(event.target.tagName.toLowerCase()).points = points; 
            }
        );
        event.stopPropagation();
    }

    getLines(event) {
        this.state.actionsRouter.extract_lines(
            (extractedLinesData) => { 
                const lines = new Map(Array.from(
                    Object.entries(extractedLinesData.extracted_lines), 
                    ([key, value]) => [parseInt(key), value]
                ));
                this.querySelector(event.target.tagName.toLowerCase()).lines = lines; 
            }
        );
        event.stopPropagation();
    }

    getPropertiesLoadStatus(event) {
        this.querySelector(event.target.tagName.toLowerCase()).isPropertiesLoaded = this.state.isPropertiesLoaded;
        event.stopPropagation();
    }

    getMaterials(event) {
        this.state.actionsRouter.extract_materials(
            (extractedMaterialsData) => { 
                const materials = Array.from(
                    Object.entries(extractedMaterialsData.extracted_materials),
                    ([key, value]) => ({ 
                        "name": key, "young_modulus": value.young_modulus, "poisson_ratio": value.poisson_ratio, 
                    }));
                this.querySelector(event.target.tagName.toLowerCase()).materials = materials; 
            }
        );
        event.stopPropagation();
    }

    getTrussSections(event) {
        this.state.actionsRouter.extract_truss_sections(
            (extractedTrussSectionsData) => { 
                const trussSections = Array.from(
                    Object.entries(extractedTrussSectionsData.extracted_truss_sections),
                    ([key, value]) => ({
                        "name": key, "area": value.area, "area2": value.area2,
                    }));
                this.querySelector(event.target.tagName.toLowerCase()).trussSections = trussSections; 
            }
        );
        event.stopPropagation();
    }

    getBeamSections(event) {
        this.state.actionsRouter.extract_beam_sections(
            (extractedBeamSectionsData) => { 
                const beamSections = Array.from(
                    Object.entries(extractedBeamSectionsData.extracted_beam_sections),
                    ([key, value]) => ({
                        "name": key, "area": value.area, "i11": value.i11, "i22": value.i22, "i12": value.i12, "it": value.it
                    }));
                this.querySelector(event.target.tagName.toLowerCase()).beamSections = beamSections; 
            }
        );
        event.stopPropagation();
    }

    activatePostprocessor() {
        this.querySelector("fea-preprocessor-menu").remove();
        const feaPostprocessor = document.createElement("fea-postprocessor");
        this.append(feaPostprocessor);
        this.updateCanvasSize();
    }

    async getData(url = "") {
        const response = await fetch(url, {
            method: "get"
        });
        return response;
    }

    handleLoadCache() {       
        this.getData("/cache/load")
            .then(response => {
                if (response.ok) {
                    response.json()
                        .then(data => {
                            for (let i = 0; i < data.messages.length; i++) {
                                const toCache = false;
                                const currentMessage = JSON.parse(data.messages[i]);
                                if ("undo" in currentMessage) {
                                    this.handleDecreaseActionIdMessage();
                                } 
                                this.state.actionsRouter.handle_message(
                                    currentMessage,
                                    (objectInfo) => this.showObjectInfo(objectInfo),
                                    (selectedView) => this.changeView(selectedView),
                                    toCache);
                            } 
                        })
                }
        }); 
    }

    handleClientMessage(event) {
        const toCache = true;
        const message = event.detail.message;
        this.state.actionsRouter.handle_message(
            message,
            (objectInfo) => this.showObjectInfo(objectInfo),
            (selectedView) => this.changeView(selectedView),
            toCache);
        event.stopPropagation();
    }

    showObjectInfo(objectInfo) {
        if (this.state.linesMultipleSelectionModeEnabled === false) {
            if ("point_data" in objectInfo) {
                const pointNumber = objectInfo.point_data.number;
                const composedObjectInfo = `Point: 
                    number: ${pointNumber},
                    x: ${objectInfo.point_data.x},
                    y: ${objectInfo.point_data.y},
                    z: ${objectInfo.point_data.z}`;
                this.shadowRoot.querySelector("fea-renderer").objectInfo = composedObjectInfo;          
                if (this.querySelector("fea-preprocessor-menu") !== null) {
                    this.querySelector("fea-preprocessor-menu").selectPointInClient = pointNumber;
                }
            } else if ("line_data" in objectInfo) {
                const lineNumber = objectInfo.line_data.number;
                const composedObjectInfo = `Line: 
                    number: ${lineNumber},
                    start point number: ${objectInfo.line_data.start_point_number},
                    end point number: ${objectInfo.line_data.end_point_number}`;
                this.shadowRoot.querySelector("fea-renderer").objectInfo = composedObjectInfo;    
                if (this.querySelector("fea-preprocessor-menu") !== null) {
                    this.querySelector("fea-preprocessor-menu").selectLineInClient = lineNumber;
                }   
    
            } else {
                throw "Fea-app: Unknown object!";
            }
        } else {
            if ("line_data" in objectInfo) {
                const lineNumber = objectInfo.line_data.number;
                if (this.querySelector("fea-preprocessor-menu") !== null) {
                    this.querySelector("fea-preprocessor-menu").selectLineInClientForDataAssign = lineNumber;
                }
            } else {
                if (this.querySelector("fea-preprocessor-menu") !== null) {
                    for (let i = 0; i < objectInfo.length; i++) {
                        this.querySelector("fea-preprocessor-menu").selectLineInClientForDataAssign = objectInfo[i];
                    }
                }   
            }
        }
    }

    changeView(selectedView) {
        this.shadowRoot.querySelector("fea-renderer").selectedView = selectedView;
    }

    handleEnableLinesMultipleSelectionModeMessage(event) {
        this.state.linesMultipleSelectionModeEnabled = true;
        event.stopPropagation();
    }

    handleDisableLinesMultipleSelectionModeMessage(event) {
        this.state.linesMultipleSelectionModeEnabled = false;
        event.stopPropagation();
    }

    handleAddPointServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const point = { 
            number: event.detail.point_data.number, x: event.detail.point_data.x,
            y: event.detail.point_data.y, z: event.detail.point_data.z };
        for (let i = 0; i < this.state.pointsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.pointsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.pointsDataDependentMenus[i]).addPointToClient = point;
            }
        } 
        this.shadowRoot.querySelector("fea-renderer").addPointToRenderer = point;
        event.stopPropagation();
    }

    handleUpdatePointServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const point = { number: event.detail.point_data.number, x: event.detail.point_data.x,
            y: event.detail.point_data.y, z: event.detail.point_data.z };
        for (let i = 0; i < this.state.pointsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.pointsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.pointsDataDependentMenus[i]).updatePointInClient = point;
            }
        } 
        this.shadowRoot.querySelector("fea-renderer").updatePointInRenderer = point;
        event.stopPropagation();
    }

    handleDeletePointServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const point = { number: event.detail.point_data.number };
        for (let i = 0; i < this.state.pointsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.pointsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.pointsDataDependentMenus[i]).deletePointFromClient = point;
            }
        } 
        this.shadowRoot.querySelector("fea-renderer").deletePointFromRenderer = point;
        event.stopPropagation();
    }

    handleAddLineServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const line = { 
            number: event.detail.line_data.number,
            startPointNumber: event.detail.line_data.start_point_number,
            endPointNumber: event.detail.line_data.end_point_number };
        for (let i = 0; i < this.state.linesDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.linesDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.linesDataDependentMenus[i]).addLineToClient = line;
            }
        } 
        this.shadowRoot.querySelector("fea-renderer").addLineToRenderer = line;
        event.stopPropagation();
    }

    handleUpdateLineServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const line = { 
            number: event.detail.line_data.number,
            startPointNumber: event.detail.line_data.start_point_number,
            endPointNumber: event.detail.line_data.end_point_number };
        for (let i = 0; i < this.state.linesDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.linesDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.linesDataDependentMenus[i]).updateLineInClient = line;
            }
        } 
        this.shadowRoot.querySelector("fea-renderer").updateLineInRenderer = line;
        event.stopPropagation();
    }

    handleDeleteLineServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const line = { number: event.detail.line_data.number };
        for (let i = 0; i < this.state.linesDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.linesDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.linesDataDependentMenus[i]).deleteLineFromClient = line;
            }
        } 
        this.shadowRoot.querySelector("fea-renderer").deleteLineFromRenderer = line;
        event.stopPropagation();
    }

    handleAddMaterialServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const material = { 
            name: event.detail.material_data.name,
            young_modulus: event.detail.material_data.young_modulus,
            poisson_ratio: event.detail.material_data.poisson_ratio };
        for (let i = 0; i < this.state.materialsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.materialsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.materialsDataDependentMenus[i]).addMaterialToClient = material;
            }
        } 
        event.stopPropagation();
    }


    handleUpdateMaterialServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const material = { 
            name: event.detail.material_data.name,
            young_modulus: event.detail.material_data.young_modulus,
            poisson_ratio: event.detail.material_data.poisson_ratio };
        for (let i = 0; i < this.state.materialsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.materialsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.materialsDataDependentMenus[i]).updateMaterialInClient = material;
            }
        } 
        event.stopPropagation();
    }

    handleDeleteMaterialServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const material = { number: event.detail.material_data.name };
        for (let i = 0; i < this.state.materialsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.materialsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.materialsDataDependentMenus[i]).deleteMaterialFromClient = material;
            }
        } 
        event.stopPropagation();
    }

    handleAddTrussSectionServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;   
        }
        const trussSection = { 
            name: event.detail.truss_section_data.name,
            area: event.detail.truss_section_data.area,
            area2: event.detail.truss_section_data.area2 };
        for (let i = 0; i < this.state.trussSectionsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.trussSectionsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.trussSectionsDataDependentMenus[i]).addTrussSectionToClient = trussSection;
            }
        } 
        event.stopPropagation();
    }

    handleUpdateTrussSectionServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const trussSection = { 
            name: event.detail.truss_section_data.name,
            area: event.detail.truss_section_data.area,
            area2: event.detail.truss_section_data.area2 };
        for (let i = 0; i < this.state.trussSectionsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.trussSectionsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.trussSectionsDataDependentMenus[i]).updateTrussSectionInClient = trussSection;
            }
        } 
        event.stopPropagation();
    }

    handleDeleteTrussSectionServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;    
        }
        const trussSection = { name: event.detail.truss_section_data.name };
        for (let i = 0; i < this.state.trussSectionsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.trussSectionsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.trussSectionsDataDependentMenus[i]).deleteTrussSectionFromClient = trussSection;
            }
        } 
        event.stopPropagation();
    }

    handleAddBeamSectionServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;  
        }
        const beamSection = { 
            name: event.detail.beam_section_data.name,
            area: event.detail.beam_section_data.area,
            i11: event.detail.beam_section_data.i11,
            i22: event.detail.beam_section_data.i22,
            i12: event.detail.beam_section_data.i12,
            it: event.detail.beam_section_data.it };
        for (let i = 0; i < this.state.beamSectionsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.beamSectionsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.beamSectionsDataDependentMenus[i]).addBeamSectionToClient = beamSection;
            }
        } 
        event.stopPropagation();
    }

    handleUpdateBeamSectionServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;           
        }
        const beamSection = { 
            name: event.detail.beam_section_data.name,
            area: event.detail.beam_section_data.area,
            i11: event.detail.beam_section_data.i11,
            i22: event.detail.beam_section_data.i22,
            i12: event.detail.beam_section_data.i12,
            it: event.detail.beam_section_data.it };
        for (let i = 0; i < this.state.beamSectionsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.beamSectionsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.beamSectionsDataDependentMenus[i]).updateBeamSectionInClient = beamSection;
            }
        } 
        event.stopPropagation();
    }

    handleDeleteBeamSectionServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;           
        }
        const beamSection = { name: event.detail.beam_section_data.name };
        for (let i = 0; i < this.state.beamSectionsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.beamSectionsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.beamSectionsDataDependentMenus[i]).deleteBeamSectionFromClient = beamSection;
            }
        } 
        event.stopPropagation();
    }

    handleDecreaseActionIdMessage() {
        this.state.actionId -= 1;
    }

    updateCanvasSize() {
        if (this.querySelector("fea-postprocessor") !== null) {
            const canvasWidth = window.innerWidth - this.querySelector("fea-postprocessor").offsetWidth - 15;
            const canvasHeight = window.innerHeight - this.shadowRoot.querySelector("fea-app-menu-bar").offsetHeight - 
                this.shadowRoot.querySelector("fea-app-tool-bar").offsetHeight - 40;
            this.shadowRoot.querySelector("fea-renderer").canvasSize = { "width": canvasWidth, "height": canvasHeight };
        } else if (this.querySelector("fea-preprocessor-menu") !== null) {
            const canvasWidth = window.innerWidth - this.querySelector("fea-preprocessor-menu").offsetWidth - 15;
            const canvasHeight = window.innerHeight - this.shadowRoot.querySelector("fea-app-menu-bar").offsetHeight - 
                this.shadowRoot.querySelector("fea-app-tool-bar").offsetHeight - 40;
            this.shadowRoot.querySelector("fea-renderer").canvasSize = { "width": canvasWidth, "height": canvasHeight };
        } else {
            this.shadowRoot.querySelector("fea-renderer").canvasSize = { "width":  window.innerWidth, "height": window.innerHeight };
        }
    }
}

export default FeaApp;
