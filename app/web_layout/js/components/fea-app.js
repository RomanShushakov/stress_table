import { initializeActionsRouter } from "../wasm_modules_initialization/actions_router_initialization.js";


class FeaApp extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            actionId: 1,                            // u32;
            actionsRouter: null,                    // wasm module "actions_router";
            isFEModelLoaded: false,                 // load status of wasm module "fe_model";
            isLinesSelectionModeEnabled: false,
            pointsDataDependentMenus: [
                "fea-geometry-add-point-menu",
                "fea-geometry-update-point-menu",
                "fea-geometry-delete-point-menu",
                "fea-geometry-add-line-menu",
                "fea-geometry-update-line-menu",
                "fea-load-add-concentrated-load-menu",
                "fea-boundary-condition-add-boundary-condition-menu",
            ],
            linesDataDependentMenus: [
                "fea-geometry-add-line-menu",
                "fea-geometry-update-line-menu",
                "fea-geometry-delete-line-menu",
                "fea-properties-assign-properties-menu",
                "fea-properties-beam-section-orientation-menu",
                "fea-load-add-distributed-line-load-menu",
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
            ],
            propertiesDataDependentMenus: [
                "fea-properties-add-properties-menu",
                "fea-properties-update-properties-menu",
                "fea-properties-delete-properties-menu",
                "fea-properties-assign-properties-menu",
                "fea-properties-beam-section-orientation-menu",
            ],
            linesSelectionDependentMenus: [
                "fea-properties-assign-properties-menu",
                "fea-properties-beam-section-orientation-menu",
            ],
            assignedPropertiesToLinesDataDependentMenus: [
                "fea-properties-assign-properties-menu",
                "fea-properties-beam-section-orientation-menu",
            ],
            beamSectionsOrientationsDependentMenus: [
                "fea-properties-beam-section-orientation-menu",
            ],
            concentratedLoadsDataDependentMenus: [
                "fea-load-add-concentrated-load-menu",
                "fea-load-update-concentrated-load-menu",
                "fea-load-delete-concentrated-load-menu",
            ],
            distributedLineLoadsDataDependentMenus: [
                "fea-load-add-distributed-line-load-menu",
                "fea-load-update-distributed-line-load-menu",
                "fea-load-delete-distributed-line-load-menu",
            ],
            boundaryConditionsDataDependentMenus: [
                "fea-boundary-condition-add-boundary-condition-menu",
                "fea-boundary-condition-update-boundary-condition-menu",
                "fea-boundary-condition-delete-boundary-condition-menu",
            ],
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

        window.addEventListener("feModelLoaded", (event) => {
            this.state.isFEModelLoaded = true;
            event.stopPropagation();
        });

        window.addEventListener("resize", () => this.updateCanvasSize());

        this.addEventListener("activatePreprocessorMenu", () => this.activatePreprocessorMenu());
        this.addEventListener("activate-postprocessor", () => this.activatePostprocessor());

        this.addEventListener("getActionId", (event) => this.getActionId(event));
        this.addEventListener("getActionIdForToolBar", (event) => this.getActionIdForToolBar(event));

        this.addEventListener("getFEModelLoadStatus", (event) => this.getFEModelLoadStatus(event));

        this.addEventListener("getPoints", (event) => this.getPoints(event));
        this.addEventListener("getLines", (event) => this.getLines(event));

        this.addEventListener("getMaterials", (event) => this.getMaterials(event));
        this.addEventListener("getTrussSections", (event) => this.getTrussSections(event));
        this.addEventListener("getBeamSections", (event) => this.getBeamSections(event));
        this.addEventListener("getProperties", (event) => this.getProperties(event));
        this.addEventListener("getAssignedPropertiesToLines", (event) => this.getAssignedPropertiesToLines(event));
        this.addEventListener("getBeamSectionsLocalAxis1Directions", (event) => this.getBeamSectionsLocalAxis1Directionsions(event));

        this.addEventListener("getConcentratedLoads", (event) => this.getConcentratedLoads(event));
        this.addEventListener("getDistributedLineLoads", (event) => this.getDistributedLineLoads(event));

        this.addEventListener("selected_points", (event) => this.handleSelectedPointsMessage(event));
        this.addEventListener("selected_nodes", (event) => this.handleSelectedNodesMessage(event));
        this.addEventListener("selected_lines", (event) => this.handleSelectedLinesMessage(event));
        this.addEventListener("selected_line_elements", (event) => this.handleSelectedLineElementsMessage(event));
        this.addEventListener("selected_concentrated_loads_points_numbers", 
            (event) => this.handleSelectedConcentratedLoadsPointsNumbersMessage(event));
        this.addEventListener("selected_distributed_line_loads_lines_numbers", 
            (event) => this.handleSelectedDistributedLineLoadsLinesNumbersMessage(event));

        this.addEventListener("toggleGeometryVisibility", (event) => this.handleToggleGeometryVisibilityMessage(event));
        this.addEventListener("toggleMeshVisibility", (event) => this.handleToggleMeshVisibilityMessage(event));
        this.addEventListener("changeView", (event) => this.handleChangeViewMessage(event));

        this.addEventListener("previewSelectedLineNumbers", (event) => this.handlePreviewSelectedLineNumbersMessage(event));
        this.addEventListener("previewBeamSectionOrientation", (event) => this.handlePreviewBeamSectionOrientationMessage(event));

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

        this.addEventListener("add_properties_server_message", (event) => this.handleAddPropertiesServerMessage(event));
        this.addEventListener("update_properties_server_message", (event) => this.handleUpdatePropertiesServerMessage(event));
        this.addEventListener("delete_properties_server_message", (event) => this.handleDeletePropertiesServerMessage(event));

        this.addEventListener("add_assigned_properties_to_lines_server_message", 
            (event) => this.handleAddAssignedPropertiesToLinesServerMessage(event));
        this.addEventListener("update_assigned_properties_to_lines_server_message", 
            (event) => this.handleUpdateAssignedPropertiesToLinesServerMessage(event));
        this.addEventListener("delete_assigned_properties_to_lines_server_message", 
            (event) => this.handleDeleteAssignedPropertiesToLinesServerMessage(event));

        this.addEventListener("add_beam_section_local_axis_1_direction_server_message", 
            (event) => this.handleAddBeamSectionLocalAxis1DirectionServerMessage(event));
        this.addEventListener("remove_beam_section_local_axis_1_direction_server_message",
            (event) => this.handleRemoveBeamSectionLocalAxis1DirectionServerMessage(event));
        this.addEventListener("update_beam_section_orientation_data_server_message",
            (event) => this.handleUpdateBeamSectionOrientationDataServerMessage(event));

        this.addEventListener("add_concentrated_load_server_message", (event) => this.handleAddConcentratedLoadServerMessage(event));
        this.addEventListener("update_concentrated_load_server_message", (event) => this.handleUpdateConcentratedLoadServerMessage(event));
        this.addEventListener("delete_concentrated_load_server_message", (event) => this.handleDeleteConcentratedLoadServerMessage(event));

        this.addEventListener("add_distributed_line_load_server_message", 
            (event) => this.handleAddDistributedLineLoadServerMessage(event));
        this.addEventListener("update_distributed_line_load_server_message", 
            (event) => this.handleUpdateDistributedLineLoadServerMessage(event));
        this.addEventListener("delete_distributed_line_load_server_message", 
            (event) => this.handleDeleteDistributedLineLoadServerMessage(event));

        this.addEventListener("add_node_server_message", (event) => this.handleAddNodeServerMessage(event));

        this.addEventListener("decreaseActionId", (_event) => this.handleDecreaseActionIdMessage());

        this.addEventListener("enableLinesSelectionMode", 
            (event) => this.handleEnableLinesSelectionModeMessage(event));

        this.addEventListener("disableLinesSelectionMode", 
            (event) => this.handleDisableLinesSelectionModeMessage(event));
    }

    set feModelError(error) {
        throw error;
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

    activatePostprocessor() {
        this.querySelector("fea-preprocessor-menu").remove();
        const feaPostprocessor = document.createElement("fea-postprocessor");
        this.append(feaPostprocessor);
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

    getFEModelLoadStatus(event) {
        this.querySelector(event.target.tagName.toLowerCase()).isFEModelLoaded = this.state.isFEModelLoaded;
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
                        "name": key, "area": value.area, "i11": value.i11, "i22": value.i22, "i12": value.i12, "it": value.it,
                        "shear_factor": value.shear_factor,
                    }));
                this.querySelector(event.target.tagName.toLowerCase()).beamSections = beamSections; 
            }
        );
        event.stopPropagation();
    }

    
    getProperties(event) {
        this.state.actionsRouter.extract_properties(
            (extractedPropertiesData) => { 
                const properties = Array.from(
                    Object.entries(extractedPropertiesData.extracted_properties),
                    ([key, value]) => ({
                        "name": key, "material_name": value.material_name,
                        "cross_section_name": value.cross_section_name,
                        "cross_section_type": `"${value.cross_section_type.toLowerCase()}"`,
                    }));
                this.querySelector(event.target.tagName.toLowerCase()).properties = properties; 
            }
        );
        event.stopPropagation();
    }

    getAssignedPropertiesToLines(event) {
        this.state.actionsRouter.extract_assigned_properties_to_lines(
            (extractedAssignedPropertiesToLinesData) => { 
                let assignedPropertiesToLines = new Array();
                const extractedAssignedPropertiesToLines = 
                    Object.entries(extractedAssignedPropertiesToLinesData.extracted_assigned_properties_to_lines);
                for (let i = 0; i < extractedAssignedPropertiesToLines.length; i++) {
                    const relatedLinesData = extractedAssignedPropertiesToLines[i][1].related_lines_data;
                    const assignedPropertyToLines = { 
                        "name": extractedAssignedPropertiesToLines[i][0], 
                        "related_lines_data": relatedLinesData,
                    }
                    assignedPropertiesToLines.push(assignedPropertyToLines)
                }
                this.querySelector(event.target.tagName.toLowerCase()).assignedPropertiesToLines = assignedPropertiesToLines; 
            }
        );
        event.stopPropagation();
    }

    getBeamSectionsLocalAxis1Directionsions(event) {
        this.state.actionsRouter.extract_beam_sections_local_axis_1_directions(
            (extractedBeamSectionsLocalAxis1DirectionsData) => {
                this.querySelector(event.target.tagName.toLowerCase()).beamSectionsLocalAxis1Directions = 
                    extractedBeamSectionsLocalAxis1DirectionsData.extracted_beam_sections_local_axis_1_directions; 
            }
        );
        event.stopPropagation();
    }

    getConcentratedLoads(event) {
        this.state.actionsRouter.extract_concentrated_loads(
            (extractedConcentratedLoadsData) => { 
                const concentratedLoads = new Map(Array.from(
                    Object.entries(extractedConcentratedLoadsData.extracted_concentrated_loads), 
                    ([key, value]) => [parseInt(key), value]
                ));
                this.querySelector(event.target.tagName.toLowerCase()).concentratedLoads = concentratedLoads; 
            }
        );
        event.stopPropagation();
    }

    getDistributedLineLoads(event) {
        this.state.actionsRouter.extract_distributed_line_loads(
            (extractedDistributedLineLoadsData) => { 
                const distributedLineLoads = new Map(Array.from(
                    Object.entries(extractedDistributedLineLoadsData.extracted_distributed_line_loads), 
                    ([key, value]) => [parseInt(key), value]
                ));
                this.querySelector(event.target.tagName.toLowerCase()).distributedLineLoads = distributedLineLoads; 
            }
        );
        event.stopPropagation();
    }

    showObjectInfoHandler(objectInfo) {
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

        } else if ("line_data_with_props" in objectInfo) {
            const lineNumber = objectInfo.line_data_with_props.number;
            const composedObjectInfo = `Line: 
                number: ${lineNumber},
                start point number: ${objectInfo.line_data_with_props.start_point_number},
                end point number: ${objectInfo.line_data_with_props.end_point_number},
                material name: ${objectInfo.line_data_with_props.material_name.replace(/['"]+/g, "")},
                cross section name: ${objectInfo.line_data_with_props.cross_section_name.replace(/['"]+/g, "")},
                cross section type: ${objectInfo.line_data_with_props.cross_section_type},`;
            this.shadowRoot.querySelector("fea-renderer").objectInfo = composedObjectInfo;    
            if (this.querySelector("fea-preprocessor-menu") !== null) {
                this.querySelector("fea-preprocessor-menu").selectLineInClient = lineNumber;
            }   
        } else if ("concentrated_load_data" in objectInfo) {
            const concentratedLoadPointNumber = objectInfo.concentrated_load_data.point_number;
            const composedObjectInfo = `Concentrated load: 
                applied to point: ${concentratedLoadPointNumber},
                Fx: ${objectInfo.concentrated_load_data.fx},
                Fy: ${objectInfo.concentrated_load_data.fy},
                Fz: ${objectInfo.concentrated_load_data.fz},
                Mx: ${objectInfo.concentrated_load_data.mx},
                My: ${objectInfo.concentrated_load_data.my},
                Mz: ${objectInfo.concentrated_load_data.mz}`;
            this.shadowRoot.querySelector("fea-renderer").objectInfo = composedObjectInfo;          
            if (this.querySelector("fea-preprocessor-menu") !== null) {
                this.querySelector("fea-preprocessor-menu").selectConcentratedLoadInClient = concentratedLoadPointNumber;
            }
        } else if ("distributed_line_load_data" in objectInfo) {
            const distributedLoadLineNumber = objectInfo.distributed_line_load_data.line_number;
            const composedObjectInfo = `Distributed line load: 
                applied to line: ${distributedLoadLineNumber},
                Qx: ${objectInfo.distributed_line_load_data.qx},
                Qy: ${objectInfo.distributed_line_load_data.qy},
                Qz: ${objectInfo.distributed_line_load_data.qz}`;
            this.shadowRoot.querySelector("fea-renderer").objectInfo = composedObjectInfo;          
            if (this.querySelector("fea-preprocessor-menu") !== null) {
                this.querySelector("fea-preprocessor-menu").selectDistributedLineLoadInClient = distributedLoadLineNumber;
            }
        } else {
            throw "Fea-app: Unknown object!";
        }
    }

    showObjectInfoWithoutMenuOpeningHandler(objectInfo) {
        if ("point_data" in objectInfo) {
            const pointNumber = objectInfo.point_data.number;
            const composedObjectInfo = `Point: 
                number: ${pointNumber},
                x: ${objectInfo.point_data.x},
                y: ${objectInfo.point_data.y},
                z: ${objectInfo.point_data.z}`;
            this.shadowRoot.querySelector("fea-renderer").objectInfo = composedObjectInfo;          
        } else if ("line_data" in objectInfo) {
            const lineNumber = objectInfo.line_data.number;
            const composedObjectInfo = `Line: 
                number: ${lineNumber},
                start point number: ${objectInfo.line_data.start_point_number},
                end point number: ${objectInfo.line_data.end_point_number}`;
            this.shadowRoot.querySelector("fea-renderer").objectInfo = composedObjectInfo;    
        } else if ("line_data_with_props" in objectInfo) {
            const lineNumber = objectInfo.line_data_with_props.number;
            const composedObjectInfo = `Line: 
                number: ${lineNumber},
                start point number: ${objectInfo.line_data_with_props.start_point_number},
                end point number: ${objectInfo.line_data_with_props.end_point_number},
                material name: ${objectInfo.line_data_with_props.material_name.replace(/['"]+/g, "")},
                cross section name: ${objectInfo.line_data_with_props.cross_section_name.replace(/['"]+/g, "")},
                cross section type: ${objectInfo.line_data_with_props.cross_section_type},`;
            this.shadowRoot.querySelector("fea-renderer").objectInfo = composedObjectInfo;    
        }
        else if ("concentrated_load_data" in objectInfo) {
            const concentratedLoadpointNumber = objectInfo.concentrated_load_data.point_number;
            const composedObjectInfo = `Concentrated load: 
                applied to point: ${concentratedLoadpointNumber},
                Fx: ${objectInfo.concentrated_load_data.fx},
                Fy: ${objectInfo.concentrated_load_data.fy},
                Fz: ${objectInfo.concentrated_load_data.fz},
                Mx: ${objectInfo.concentrated_load_data.mx},
                My: ${objectInfo.concentrated_load_data.my},
                Mz: ${objectInfo.concentrated_load_data.mz}`;
            this.shadowRoot.querySelector("fea-renderer").objectInfo = composedObjectInfo;          
        } else {
            throw "Fea-app: Unknown object!";
        }
    }

    selectLinesInClientForDataAssign(selectedLinesNumbers) {
        for (let i = 0; i < this.state.linesSelectionDependentMenus.length; i++) {
            if (this.querySelector(this.state.linesSelectionDependentMenus[i]) !== null) {
                if (Array.isArray(selectedLinesNumbers)) {
                    for (let j = 0; j < selectedLinesNumbers.length; j++) {
                        this.querySelector(this.state.linesSelectionDependentMenus[i])
                            .selectLineInClientForDataAssign = selectedLinesNumbers[j];
                    }
                } else if (Number.isInteger(selectedLinesNumbers)) {
                    this.querySelector(this.state.linesSelectionDependentMenus[i])
                        .selectLineInClientForDataAssign = selectedLinesNumbers;
                } else {
                    throw "Fea-app: Unknown object!";
                }
            }
        }  
    }

    handleSelectedPointsMessage(event) {
        const pointNumbers = event.detail.point_numbers;
        if (pointNumbers.length > 1) {
            console.log("Selected point numbers: ", pointNumbers);
        } else {
            const pointNumber = pointNumbers[0];
            if (this.state.isLinesSelectionModeEnabled === false) {
                this.state.actionsRouter.show_point_info(
                    BigInt(pointNumber),
                    (objectInfo) => this.showObjectInfoHandler(objectInfo),
                );
            } else {
                this.state.actionsRouter.show_point_info(
                    BigInt(pointNumber),
                    (objectInfo) => this.showObjectInfoWithoutMenuOpeningHandler(objectInfo),
                );
            }
        }
        event.stopPropagation();
    }

    handleSelectedNodesMessage(event) {
        const nodeNumbers = event.detail.node_numbers;
        if (nodeNumbers.length > 1) {
            console.log("Selected node numbers: ", nodeNumbers);
        } else {
            console.log("Selected node number: ", nodeNumbers[0]);
        }
        event.stopPropagation();
    }

    handleSelectedLinesMessage(event) {
        const lineNumbers = event.detail.line_numbers;
        if (lineNumbers.length > 1) {
            if (this.state.isLinesSelectionModeEnabled === false) {
                console.log("Selected line numbers: ", lineNumbers);
            } else {
                this.selectLinesInClientForDataAssign(lineNumbers);
            }
        } else {
            const lineNumber = lineNumbers[0];
            if (this.state.isLinesSelectionModeEnabled === false) {
                this.state.actionsRouter.show_line_info(
                    BigInt(lineNumber),
                    (objectInfo) => this.showObjectInfoHandler(objectInfo),
                );
            } else {
                this.selectLinesInClientForDataAssign(lineNumber);
                this.state.actionsRouter.show_line_info(
                    BigInt(lineNumber),
                    (objectInfo) => this.showObjectInfoWithoutMenuOpeningHandler(objectInfo),
                );
            }
        }
        event.stopPropagation();
    }

    handleSelectedLineElementsMessage(event) {
        const lineElementNumbers = event.detail.line_element_numbers;
        if (lineElementNumbers.length > 1) {
            console.log("Selected line element numbers: ", lineElementNumbers);
        } else {
            console.log("Selected line element number: ", lineElementNumbers[0]);
        }
        event.stopPropagation();
    }

    handleSelectedConcentratedLoadsPointsNumbersMessage(event) {
        const concentratedLoadsPointsNumbers = event.detail.concentrated_loads_points_numbers;
        if (concentratedLoadsPointsNumbers.length > 1) {
            console.log("Selected concentrated loads points numbers: ", concentratedLoadsPointsNumbers);
        } else {
            const concentratedLoadPointNumber = concentratedLoadsPointsNumbers[0];
            if (this.state.isLinesSelectionModeEnabled === false) {
                this.state.actionsRouter.show_concentrated_load_info(
                    BigInt(concentratedLoadPointNumber),
                    (objectInfo) => this.showObjectInfoHandler(objectInfo),
                );
            } else {
                this.state.actionsRouter.show_concentrated_load_info(
                    BigInt(concentratedLoadPointNumber),
                    (objectInfo) => this.showObjectInfoWithoutMenuOpeningHandler(objectInfo),
                );
            }
        }
        event.stopPropagation();
    }

    handleSelectedDistributedLineLoadsLinesNumbersMessage(event) {
        const distributedLineLoadsLinesNumbers = event.detail.distributed_line_loads_lines_numbers;
        if (distributedLineLoadsLinesNumbers.length > 1) {
            console.log("Selected distributed line loads lines numbers: ", distributedLineLoadsLinesNumbers);
        } else {
            const distributedLineLoadLineNumber = distributedLineLoadsLinesNumbers[0];
            if (this.state.isLinesSelectionModeEnabled === false) {
                this.state.actionsRouter.show_distributed_line_load_info(
                    BigInt(distributedLineLoadLineNumber),
                    (objectInfo) => this.showObjectInfoHandler(objectInfo),
                );
            } else {
                this.state.actionsRouter.show_distributed_line_load_info(
                    BigInt(distributedLineLoadLineNumber),
                    (objectInfo) => this.showObjectInfoWithoutMenuOpeningHandler(objectInfo),
                );
            }
        }
        event.stopPropagation();
    }

    handleToggleGeometryVisibilityMessage(event) {
        const data = true;
        this.shadowRoot.querySelector("fea-renderer").toggleGeometryVisibility = data;
        event.stopPropagation();
    }

    handleToggleMeshVisibilityMessage(event) {
        const data = true;
        this.shadowRoot.querySelector("fea-renderer").toggleMeshVisibility = data;
        event.stopPropagation();
    }

    handleChangeViewMessage(event) {
        const selectedView = event.detail.selectedView;
        this.shadowRoot.querySelector("fea-renderer").selectedView = selectedView;
        event.stopPropagation();
    }

    handlePreviewSelectedLineNumbersMessage(event) {
        const selectedLineNumbersObject = event.detail;
        try {
            this.shadowRoot.querySelector("fea-renderer").previewSelectedLineNumbers = selectedLineNumbersObject;
        } catch (error) {
            this.querySelector(event.target.tagName.toLowerCase()).rendererError = error;
            throw error;
        }
        event.stopPropagation();
    }

    handlePreviewBeamSectionOrientationMessage(event) {
        const beamSectionOrientationObject = event.detail;
        try {
            this.shadowRoot.querySelector("fea-renderer").previewBeamSectionOrientation = beamSectionOrientationObject;
        } catch (error) {
            this.querySelector(event.target.tagName.toLowerCase()).rendererError = error;
            throw error;
        }
        event.stopPropagation();
    }

    handleEnableLinesSelectionModeMessage(event) {
        this.state.isLinesSelectionModeEnabled = true;
        event.stopPropagation();
    }

    handleDisableLinesSelectionModeMessage(event) {
        this.state.isLinesSelectionModeEnabled = false;
        event.stopPropagation();
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
                                this.state.actionsRouter.handle_message(currentMessage, toCache);
                            } 
                        })
                }
        }); 
    }

    handleClientMessage(event) {
        const toCache = true;
        const message = event.detail.message;
        try {
            this.state.actionsRouter.handle_message(message, toCache);
        } catch (error) {
            if (event.target.tagName.toLowerCase() == "fea-properties-beam-section-orientation-menu") {
                const errorData = { "message": message, "error": error };
                this.querySelector(event.target.tagName.toLowerCase()).feModelError = errorData;
            } else {
                console.log("Error!!!", error);
                this.querySelector(event.target.tagName.toLowerCase()).feModelError = error;
            }
            throw error;
        }
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
            start_point_number: event.detail.line_data.start_point_number,
            end_point_number: event.detail.line_data.end_point_number };
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
            start_point_number: event.detail.line_data.start_point_number,
            end_point_number: event.detail.line_data.end_point_number };
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
            it: event.detail.beam_section_data.it,
            shear_factor: event.detail.beam_section_data.shear_factor };
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
            it: event.detail.beam_section_data.it,
            shear_factor: event.detail.beam_section_data.shear_factor };
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

    handleAddPropertiesServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;   
        }
        const properties = { 
            name: event.detail.properties_data.name,
            material_name: event.detail.properties_data.material_name,
            cross_section_name: event.detail.properties_data.cross_section_name,
            cross_section_type: event.detail.properties_data.cross_section_type,
        };
        for (let i = 0; i < this.state.propertiesDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.propertiesDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.propertiesDataDependentMenus[i]).addPropertiesToClient = properties;
            }
        } 
        event.stopPropagation();
    }

    handleUpdatePropertiesServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;   
        }
        const properties = { 
            name: event.detail.properties_data.name,
            material_name: event.detail.properties_data.material_name,
            cross_section_name: event.detail.properties_data.cross_section_name,
            cross_section_type: event.detail.properties_data.cross_section_type,
        };
        for (let i = 0; i < this.state.propertiesDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.propertiesDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.propertiesDataDependentMenus[i]).updatePropertiesInClient = properties;
            }
        } 
        event.stopPropagation();
    }

    handleDeletePropertiesServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;           
        }
        const properties = { name: event.detail.properties_data.name };
        for (let i = 0; i < this.state.propertiesDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.propertiesDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.propertiesDataDependentMenus[i]).deletePropertiesFromClient = properties;
            }
        } 
        event.stopPropagation();
    }

    handleAddAssignedPropertiesToLinesServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;   
        }
        const assignedPropertiesToLines = { 
            name: event.detail.assigned_properties_to_lines_data.name,
            related_lines_data: event.detail.assigned_properties_to_lines_data.related_lines_data,
        };
        for (let i = 0; i < this.state.assignedPropertiesToLinesDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.assignedPropertiesToLinesDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.assignedPropertiesToLinesDataDependentMenus[i])
                    .addAssignedPropertiesToLinesToClient = assignedPropertiesToLines;
            }
        } 
        const linesColorSchemeData = {
            line_numbers: event.detail.assigned_properties_to_lines_data.line_numbers,
            old_line_numbers: event.detail.assigned_properties_to_lines_data.old_line_numbers,
            cross_section_type: event.detail.assigned_properties_to_lines_data.cross_section_type,
        }
        this.shadowRoot.querySelector("fea-renderer").updateLinesColorScheme = linesColorSchemeData;
        event.stopPropagation();
    }

    handleUpdateAssignedPropertiesToLinesServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;   
        }
        const assignedPropertiesToLines = { 
            name: event.detail.assigned_properties_to_lines_data.name,
            related_lines_data: event.detail.assigned_properties_to_lines_data.related_lines_data,
        };
        for (let i = 0; i < this.state.assignedPropertiesToLinesDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.assignedPropertiesToLinesDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.assignedPropertiesToLinesDataDependentMenus[i])
                    .updateAssignedPropertiesToLinesInClient = assignedPropertiesToLines;
            }
        } 
        const linesColorSchemeData = {
            line_numbers: event.detail.assigned_properties_to_lines_data.line_numbers,
            old_line_numbers: event.detail.assigned_properties_to_lines_data.old_line_numbers,
            cross_section_type: event.detail.assigned_properties_to_lines_data.cross_section_type,
        }
        this.shadowRoot.querySelector("fea-renderer").updateLinesColorScheme = linesColorSchemeData;
        event.stopPropagation();
    }

    handleDeleteAssignedPropertiesToLinesServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;           
        }
        const assignedPropertiesToLines = { name: event.detail.assigned_properties_to_lines_data.name };
        for (let i = 0; i < this.state.assignedPropertiesToLinesDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.assignedPropertiesToLinesDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.assignedPropertiesToLinesDataDependentMenus[i])
                    .deleteAssignedPropertiesToLinesFromClient = assignedPropertiesToLines;
            }
        } 
        const linesColorSchemeData = {
            line_numbers: event.detail.assigned_properties_to_lines_data.line_numbers,
            old_line_numbers: event.detail.assigned_properties_to_lines_data.old_line_numbers,
            cross_section_type: event.detail.assigned_properties_to_lines_data.cross_section_type,
        }
        this.shadowRoot.querySelector("fea-renderer").updateLinesColorScheme = linesColorSchemeData;
        event.stopPropagation();
    }

    handleAddBeamSectionLocalAxis1DirectionServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;           
        }
        const beamSectionLocalAxis1DirectionData = event.detail.local_axis_1_direction_data.local_axis_1_direction;
        for (let i = 0; i < this.state.beamSectionsOrientationsDependentMenus.length; i++) {
            if (this.querySelector(this.state.beamSectionsOrientationsDependentMenus[i]) !== null) {
                this.querySelector(this.state.beamSectionsOrientationsDependentMenus[i])
                    .addBeamSectionLocalAxis1DirectionToClient = beamSectionLocalAxis1DirectionData;
            }
        } 
        event.stopPropagation();
    }

    handleRemoveBeamSectionLocalAxis1DirectionServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;           
        }
        const beamSectionLocalAxis1DirectionData = event.detail.local_axis_1_direction_data.local_axis_1_direction;
        for (let i = 0; i < this.state.beamSectionsOrientationsDependentMenus.length; i++) {
            if (this.querySelector(this.state.beamSectionsOrientationsDependentMenus[i]) !== null) {
                this.querySelector(this.state.beamSectionsOrientationsDependentMenus[i])
                    .removeBeamSectionLocalAxis1DirectionFromClient = beamSectionLocalAxis1DirectionData;
            }
        } 
        event.stopPropagation();
    }

    handleUpdateBeamSectionOrientationDataServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;   
        }
        const beamSectionOrientationData = { 
            local_axis_1_direction: event.detail.beam_section_orientation_data.local_axis_1_direction,
            line_numbers: event.detail.beam_section_orientation_data.line_numbers,
        };
        for (let i = 0; i < this.state.beamSectionsOrientationsDependentMenus.length; i++) {
            if (this.querySelector(this.state.beamSectionsOrientationsDependentMenus[i]) !== null) {
                this.querySelector(this.state.beamSectionsOrientationsDependentMenus[i])
                    .updateBeamSectionOrientationDataInClient = beamSectionOrientationData;
            }
        } 
        event.stopPropagation();
    }

    handleAddConcentratedLoadServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const concentratedLoad = { 
            point_number: event.detail.concentrated_load_data.point_number, 
            fx: event.detail.concentrated_load_data.fx,
            fy: event.detail.concentrated_load_data.fy, 
            fz: event.detail.concentrated_load_data.fz,
            mx: event.detail.concentrated_load_data.mx, 
            my: event.detail.concentrated_load_data.my,
            mz: event.detail.concentrated_load_data.mz };
        for (let i = 0; i < this.state.concentratedLoadsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.concentratedLoadsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.concentratedLoadsDataDependentMenus[i])
                    .addConcentratedLoadToClient = concentratedLoad;
            }
        } 
        this.shadowRoot.querySelector("fea-renderer").addConcentratedLoadToRenderer = concentratedLoad;
        event.stopPropagation();
    }

    handleUpdateConcentratedLoadServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const concentratedLoad = { 
            point_number: event.detail.concentrated_load_data.point_number, 
            fx: event.detail.concentrated_load_data.fx,
            fy: event.detail.concentrated_load_data.fy, 
            fz: event.detail.concentrated_load_data.fz,
            mx: event.detail.concentrated_load_data.mx, 
            my: event.detail.concentrated_load_data.my,
            mz: event.detail.concentrated_load_data.mz };
        for (let i = 0; i < this.state.concentratedLoadsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.concentratedLoadsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.concentratedLoadsDataDependentMenus[i])
                    .updateConcentratedLoadInClient = concentratedLoad;
            }
        } 
        this.shadowRoot.querySelector("fea-renderer").updateConcentratedLoadInRenderer = concentratedLoad;
        event.stopPropagation();
    }

    handleDeleteConcentratedLoadServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const concentratedLoad = { point_number: event.detail.concentrated_load_data.point_number };
        for (let i = 0; i < this.state.concentratedLoadsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.concentratedLoadsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.concentratedLoadsDataDependentMenus[i])
                    .deleteConcentratedLoadFromClient = concentratedLoad;
            }
        } 
        this.shadowRoot.querySelector("fea-renderer").deleteConcentratedLoadFromRenderer = concentratedLoad;
        event.stopPropagation();
    }

    handleAddDistributedLineLoadServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const distributedLineLoad = { 
            line_number: event.detail.distributed_line_load_data.line_number, 
            qx: event.detail.distributed_line_load_data.qx,
            qy: event.detail.distributed_line_load_data.qy, 
            qz: event.detail.distributed_line_load_data.qz };
        for (let i = 0; i < this.state.distributedLineLoadsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.distributedLineLoadsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.distributedLineLoadsDataDependentMenus[i])
                    .addDistributedLineLoadToClient = distributedLineLoad;
            }
        } 
        this.shadowRoot.querySelector("fea-renderer").addDistributedLineLoadToRenderer = distributedLineLoad;
        event.stopPropagation();
    }

    handleUpdateDistributedLineLoadServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const distributedLineLoad = { 
            line_number: event.detail.distributed_line_load_data.line_number, 
            qx: event.detail.distributed_line_load_data.qx,
            qy: event.detail.distributed_line_load_data.qy, 
            qz: event.detail.distributed_line_load_data.qz };
        for (let i = 0; i < this.state.distributedLineLoadsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.distributedLineLoadsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.distributedLineLoadsDataDependentMenus[i])
                    .updateDistributedLineLoadInClient = distributedLineLoad;
            }
        } 
        this.shadowRoot.querySelector("fea-renderer").updateDistributedLineLoadInRenderer = distributedLineLoad;
        event.stopPropagation();
    }

    handleDeleteDistributedLineLoadServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const distributedLineLoad = { line_number: event.detail.distributed_line_load_data.line_number };
        for (let i = 0; i < this.state.distributedLineLoadsDataDependentMenus.length; i++) {
            if (this.querySelector(this.state.distributedLineLoadsDataDependentMenus[i]) !== null) {
                this.querySelector(this.state.distributedLineLoadsDataDependentMenus[i])
                    .deleteDistributedLineLoadFromClient = distributedLineLoad;
            }
        } 
        this.shadowRoot.querySelector("fea-renderer").deleteDistributedLineLoadFromRenderer = distributedLineLoad;
        event.stopPropagation();
    }

    handleAddNodeServerMessage(event) {
        if (event.detail.is_action_id_should_be_increased === true) {
            this.state.actionId += 1;
        }
        const node = { 
            number: event.detail.node_data.number, x: event.detail.node_data.x,
            y: event.detail.node_data.y, z: event.detail.node_data.z };
        this.shadowRoot.querySelector("fea-renderer").addNodeToRenderer = node;
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
