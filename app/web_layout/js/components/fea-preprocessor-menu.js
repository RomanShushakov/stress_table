class FeaPreprocessorMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,                 // u32;
            points: new Map(),              // map: { number: u32, { x: f64, y: f64, z: f64}, ... };
            lines: new Map(),               // map: { number: u32, startPointNumber: u32, endPointNumber: u32 }, ...};
            materials: [],                  // array of: [{ name: String, youngModulus: f64, poissonRatio: f64 }, ...];
            trussSections: [],              // array of: [{ name: String, area: f64, area2: f64 or null }];
            beamSections: [],               // array of: [{ name: String, area: f64, I11: f64, I22: f64, I12: f64, It: f64 }];
            properties: [],                 // array of: [{ name: String, materialName: String, sectionName: String,
                                            //              sectionType: String }];
            assignedProperties: [],         // array of: [{ name: String, lineNumbers: [u32...] }];
            beamSectionsOrientations: [],   // array of: [{ propertiesName: String, localAxis1Direction: [f64; 3],
                                            //              lineNumbers: [u32...] }];
        };

        this.state = {

            childrenNamesForActionIdUpdate: [
                "fea-geometry-menu",
                "fea-material-menu",
                "fea-section-menu",
                "fea-properties-menu",
            ],

            childrenNamesForPointCrud: [
                "fea-geometry-menu",
            ],

            childrenNamesForLineCrud: [
                "fea-geometry-menu",
                "fea-properties-menu",
            ],

            childrenNamesForMaterialCrud: [
                "fea-material-menu",
                "fea-properties-menu",
            ],

            childrenNamesForTrussSectionCrud: [
                "fea-section-menu",
                "fea-properties-menu",
            ],

            childrenNamesForBeamSectionCrud: [
                "fea-section-menu",
                "fea-properties-menu",
            ],

            childrenNamesForPropertiesCrud: [
                "fea-properties-menu",
            ],

            childrenNamesForAssignedPropertiesCrud: [
                "fea-properties-menu",
            ],

            childrenNamesForBeamSectionsOrientationsCrud: [
                "fea-properties-menu",
            ],
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                }

                .wrapper {
                    background-color: #2e3440;
                    display: flex;
                    flex-direction: row;
                }
            </style>
            <div class=wrapper>
                <fea-preprocessor-menu-buttons></fea-preprocessor-menu-buttons>
                <slot></slot>
            </div>
        `;
        
        this.addEventListener("activate-menu", (event) => this.activateMenu(event));

        this.addEventListener("deactivate-menu", (event) => this.deactivateMenu(event));
    }

    set actionId(value) {
        this.props.actionId = value;
        this.updateChildrenActionId();
    }

    set addPointToClient(point) {
        this.props.points.set(point.number, {"x": point.x, "y": point.y, "z": point.z});
        this.addPointToChildren(point);
    }

    set updatePointInClient(point) {
        this.props.points.set(point.number, {"x": point.x, "y": point.y, "z": point.z});
        this.updatePointInChildren(point);
    }

    set deletePointFromClient(point) {
        this.props.points.delete(point.number);
        this.deletePointFromChildren(point);
    }

    set addLineToClient(line) {
        this.props.lines.set(line.number, { "startPointNumber": line.startPointNumber, "endPointNumber": line.endPointNumber });
        this.addLineToChildren(line);
    }

    set updateLineInClient(line) {
        this.props.lines.set(line.number, { "startPointNumber": line.startPointNumber, "endPointNumber": line.endPointNumber });
        this.updateLineInChildren(line);
    }

    set deleteLineFromClient(line) {
        this.props.lines.delete(line.number);
        this.deleteLineFromChildren(line);
    }

    delay(t, v) {
        return new Promise(function(resolve) { 
            setTimeout(resolve.bind(null, v), t)
        });
    }

    set selectPointInClient(pointNumber) {
        if (this.querySelector("fea-geometry-menu") === null) {
            this.delay(0)
                .then(() => { 
                    this.shadowRoot.querySelector("fea-preprocessor-menu-buttons").toggleButton = "geometry-menu-button";
                })
                .then(async () => { this.querySelector("fea-geometry-menu").selectPointInClient = pointNumber });
        } else {
            this.delay(0)
                .then(() => { 
                    this.querySelector("fea-geometry-menu").selectPointInClient = pointNumber;
                });
        }
    }

    set selectLineInClient(lineNumber) {
        if (this.querySelector("fea-geometry-menu") === null) {
            this.delay(0)
                .then(() => { 
                    this.shadowRoot.querySelector("fea-preprocessor-menu-buttons").toggleButton = "geometry-menu-button";
                })
                .then(async () => { this.querySelector("fea-geometry-menu").selectLineInClient = lineNumber });
        } else {
            this.delay(0)
                .then(() => { 
                    this.querySelector("fea-geometry-menu").selectLineInClient = lineNumber;
                });
        }
    }

    set selectLineInClientForDataAssign(lineNumber) {
        if (this.querySelector("fea-properties-menu") !== null) {
            this.querySelector("fea-properties-menu").selectLineInClientForDataAssign = lineNumber;
        }
    }

    set addMaterialToClient(material) {
        this.props.materials.push(material);
        this.props.materials.sort((a, b) => a.name - b.name);
        this.addMaterialToChildren(material);
    }

    set updateMaterialInClient(material) {
        let materialInProps = this.props.materials
            .find(existedMaterial => existedMaterial.name == material.name);
        materialInProps.youngModulus = material.youngModulus;
        materialInProps.poissonRatio = material.poissonRatio;
        this.updateMaterialInChildren(material);
    }

    set deleteMaterialFromClient(material) {
        let materialIndexInProps = this.props.materials
            .findIndex(existedMaterial => existedMaterial.name == material.name);
        this.props.materials.splice(materialIndexInProps, 1);
        this.props.materials.sort((a, b) => a.name - b.name);
        this.deleteMaterialFromChildren(material);
    }

    set addTrussSectionToClient(trussSection) {
        this.props.trussSections.push(trussSection);
        this.props.trussSections.sort((a, b) => a.name - b.name);
        this.addTrussSectionToChildren(trussSection);
    }

    set updateTrussSectionInClient(trussSection) {
        let trussSectionInProps = this.props.trussSections
            .find(existedTrussSection => existedTrussSection.name == trussSection.name);
        trussSectionInProps.area = trussSection.area;
        trussSectionInProps.area2 = trussSection.area2;
        this.updateTrussSectionInChildren(trussSection);
    }

    set deleteTrussSectionFromClient(trussSection) {
        let trussSectionIndexInProps = this.props.trussSections
            .findIndex(existedTrussSection => existedTrussSection.name == trussSection.name);
        this.props.trussSections.splice(trussSectionIndexInProps, 1);
        this.props.trussSections.sort((a, b) => a.name - b.name);
        this.deleteTrussSectionFromChildren(trussSection);
    }

    set addBeamSectionToClient(beamSection) {
        this.props.beamSections.push(beamSection);
        this.props.beamSections.sort((a, b) => a.name - b.name);
        this.addBeamSectionToChildren(beamSection);
    }

    set updateBeamSectionInClient(beamSection) {
        let beamSectionInProps = this.props.beamSections
            .find(existedBeamSection => existedBeamSection.name == beamSection.name);
        beamSectionInProps.area = beamSection.area;
        beamSectionInProps.I11 = beamSection.I11;
        beamSectionInProps.I22 = beamSection.I22;
        beamSectionInProps.I12 = beamSection.I12;
        beamSectionInProps.It = beamSection.It;
        this.updateBeamSectionInChildren(beamSection);
    }

    set deleteBeamSectionFromClient(beamSection) {
        let beamSectionIndexInProps = this.props.beamSections
            .findIndex(existedBeamSection => existedBeamSection.name == beamSection.name);
        this.props.beamSections.splice(beamSectionIndexInProps, 1);
        this.props.beamSections.sort((a, b) => a.name - b.name);
        this.deleteBeamSectionFromChildren(beamSection);
    }

    set addPropertiesToClient(properties) {
        this.props.properties.push(properties);
        this.props.properties.sort((a, b) => a.name - b.name);
        this.addPropertiesToChildren(properties);
    }

    set updatePropertiesInClient(properties) {
        let propertiesInProps = this.props.properties
            .find(existedProperties => existedProperties.name == properties.name);
        propertiesInProps.materialName = properties.materialName;
        propertiesInProps.sectionName = properties.sectionName;
        propertiesInProps.setionType = properties.sectionType;
        propertiesInProps.usedIn = properties.usedIn;
        this.updatepropertiesInChildren(properties);
    }

    set deletePropertiesFromClient(properties) {
        let propertiesIndexInProps = this.props.properties
            .findIndex(existedProperties => existedProperties.name == properties.name);
        this.props.properties.splice(propertiesIndexInProps, 1);
        this.props.properties.sort((a, b) => a.name - b.name);
        this.deletePropertiesFromChildren(properties);
    }

    set addAssignedPropertiesToClient(assignedProperties) {
        this.props.assignedProperties.push(assignedProperties);
        this.props.assignedProperties.sort((a, b) => a.name - b.name);
        this.addAssignedPropertiesToChildren(assignedProperties);
    }

    set updateAssignedPropertiesInClient(assignedProperties) {
        let assignedPropertiesInProps = this.props.assignedProperties
            .find(existedAssignedProperties => existedAssignedProperties.name == assignedProperties.name);
        assignedPropertiesInProps.lineNumbers = assignedProperties.lineNumbers;
        this.updateAssignedPropertiesInChildren(assignedProperties);
    }

    set deleteAssignedPropertiesFromClient(assignedProperties) {
        let assignedPropertiesIndexInProps = this.props.assignedProperties
            .findIndex(existedAssignedProperties => existedAssignedProperties.name == assignedProperties.name);
        this.props.assignedProperties.splice(assignedPropertiesIndexInProps, 1);
        this.props.assignedProperties.sort((a, b) => a.name - b.name);
        this.deleteAssignedPropertiesFromChildren(assignedProperties);
    }

    set addBeamSectionOrientationToClient(beamSectionOrientation) {
        this.props.beamSectionsOrientations.push(beamSectionOrientation);
        this.addBeamSectionOrientationToChildren(beamSectionOrientation);
    }

    set updateBeamSectionOrientationInClient(beamSectionOrientation) {
        let beamSectionOrientationInProps = this.props.beamSectionsOrientations
            .find(existedBeamSectionOrientation => 
                existedBeamSectionOrientation.propertiesName == beamSectionOrientation.propertiesName &&
                existedBeamSectionOrientation.localAxis1Direction == beamSectionOrientation.localAxis1Direction);
            beamSectionOrientationInProps.lineNumbers = beamSectionOrientation.lineNumbers;
        this.updateBeamSectionOrientationInChildren(beamSectionOrientation);
    }

    set deleteBeamSectionOrientationFromClient(beamSectionOrientation) {
        let beamSectionOrientationIndexInProps = this.props.beamSectionsOrientations
            .findIndex(existedBeamSectionOrientation => 
                existedBeamSectionOrientation.propertiesName == beamSectionOrientation.propertiesName);
        this.props.beamSectionsOrientations.splice(beamSectionOrientationIndexInProps, 1);
        this.deleteBeamSectionOrientationFromChildren(beamSectionOrientation);
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
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
            case "geometry-menu":
                const feaGeometryMenu = document.createElement("fea-geometry-menu");
                this.append(feaGeometryMenu);
                event.stopPropagation();
                this.updateCanvasSize();
                this.querySelector("fea-geometry-menu").actionId = this.props.actionId;
                for (let [pointNumber, coordinates] of this.props.points) {
                    const point = { "number": pointNumber, "x": coordinates.x, "y": coordinates.y, "z": coordinates.z };
                    this.querySelector("fea-geometry-menu").addPointToClient = point;
                }
                for (let [lineNumber, linePointsNumbers] of this.props.lines) {
                    const line = { "number": lineNumber,
                        "startPointNumber": linePointsNumbers.startPointNumber,
                        "endPointNumber": linePointsNumbers.endPointNumber };
                    this.querySelector("fea-geometry-menu").addLineToClient = line;
                }
                break;
            case "material-menu":
                const feaMaterialMenu = document.createElement("fea-material-menu");
                this.append(feaMaterialMenu);
                event.stopPropagation();
                this.updateCanvasSize();
                this.querySelector("fea-material-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.materials.length; i++) {
                    const material = this.props.materials[i];
                    this.querySelector("fea-material-menu").addMaterialToClient = material;
                }
                break;
            case "section-menu":
                const feaSectionMenu = document.createElement("fea-section-menu");
                this.append(feaSectionMenu);
                event.stopPropagation();
                this.updateCanvasSize();
                this.querySelector("fea-section-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.trussSections.length; i++) {
                    const trussSection = this.props.trussSections[i];
                    this.querySelector("fea-section-menu").addTrussSectionToClient = trussSection;
                }
                for (let i = 0; i < this.props.beamSections.length; i++) {
                    const beamSection = this.props.beamSections[i];
                    this.querySelector("fea-section-menu").addBeamSectionToClient = beamSection;
                }
                break;
            case "properties-menu":
                const feaPropertiesMenu = document.createElement("fea-properties-menu");
                this.append(feaPropertiesMenu);
                event.stopPropagation();
                this.updateCanvasSize();
                this.querySelector("fea-properties-menu").actionId = this.props.actionId;
                for (let [lineNumber, linePointsNumbers] of this.props.lines) {
                    const line = { "number": lineNumber,
                        "startPointNumber": linePointsNumbers.startPointNumber,
                        "endPointNumber": linePointsNumbers.endPointNumber };
                    this.querySelector("fea-properties-menu").addLineToClient = line;
                }
                for (let i = 0; i < this.props.materials.length; i++) {
                    const material = this.props.materials[i];
                    this.querySelector("fea-properties-menu").addMaterialToClient = material;
                }
                for (let i = 0; i < this.props.trussSections.length; i++) {
                    const trussSection = this.props.trussSections[i];
                    this.querySelector("fea-properties-menu").addTrussSectionToClient = trussSection;
                }
                for (let i = 0; i < this.props.beamSections.length; i++) {
                    const beamSection = this.props.beamSections[i];
                    this.querySelector("fea-properties-menu").addBeamSectionToClient = beamSection;
                }
                for (let i = 0; i < this.props.properties.length; i++) {
                    const property = this.props.properties[i];
                    this.querySelector("fea-properties-menu").addPropertiesToClient = property;
                }
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "geometry-menu":
                this.querySelector("fea-geometry-menu").remove();
                event.stopPropagation();
                this.updateCanvasSize();
                break;
            case "material-menu":
                this.querySelector("fea-material-menu").remove();
                event.stopPropagation();
                this.updateCanvasSize();
                break;
            case "section-menu":
                this.querySelector("fea-section-menu").remove();
                event.stopPropagation();
                this.updateCanvasSize();
                break;
            case "properties-menu":
                this.querySelector("fea-properties-menu").remove();
                event.stopPropagation();
                this.updateCanvasSize();
                break;
        }
    }

    updateCanvasSize() {
        this.dispatchEvent(new CustomEvent("resize", {
            bubbles: true,
            composed: true,
        }));
    }

    updateChildrenActionId() {
        for (let i = 0; i < this.state.childrenNamesForActionIdUpdate.length; i++) {
            if (this.querySelector(this.state.childrenNamesForActionIdUpdate[i]) !== null) {
                this.querySelector(this.state.childrenNamesForActionIdUpdate[i]).actionId = this.props.actionId;
            }
        } 
    }

    addPointToChildren(point) {
        for (let i = 0; i < this.state.childrenNamesForPointCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForPointCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForPointCrud[i]).addPointToClient = point;
            }
        } 
    }

    updatePointInChildren(point) {
        for (let i = 0; i < this.state.childrenNamesForPointCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForPointCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForPointCrud[i]).updatePointInClient = point;
            }
        } 
    }

    deletePointFromChildren(point) {
        for (let i = 0; i < this.state.childrenNamesForPointCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForPointCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForPointCrud[i]).deletePointFromClient = point;
            }
        } 
    }

    addLineToChildren(line) {
        for (let i = 0; i < this.state.childrenNamesForLineCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForLineCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForLineCrud[i]).addLineToClient = line;
            }
        } 
    }

    updateLineInChildren(line) {
        for (let i = 0; i < this.state.childrenNamesForLineCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForLineCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForLineCrud[i]).updateLineInClient = line;
            }
        } 
    }

    deleteLineFromChildren(line) {
        for (let i = 0; i < this.state.childrenNamesForLineCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForLineCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForLineCrud[i]).deleteLineFromClient = line;
            }
        } 
    }

    addMaterialToChildren(material) {
        for (let i = 0; i < this.state.childrenNamesForMaterialCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForMaterialCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForMaterialCrud[i]).addMaterialToClient = material;
            }
        } 
    }

    updateMaterialInChildren(material) {
        for (let i = 0; i < this.state.childrenNamesForMaterialCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForMaterialCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForMaterialCrud[i]).updateMaterialInClient = material;
            }
        } 
    }

    deleteMaterialFromChildren(material) {
        for (let i = 0; i < this.state.childrenNamesForMaterialCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForMaterialCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForMaterialCrud[i]).deleteMaterialFromClient = material;
            }
        } 
    }

    addTrussSectionToChildren(trussSection) {
        for (let i = 0; i < this.state.childrenNamesForTrussSectionCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForTrussSectionCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForTrussSectionCrud[i]).addTrussSectionToClient = trussSection;
            }
        } 
    }

    updateTrussSectionInChildren(trussSection) {
        for (let i = 0; i < this.state.childrenNamesForTrussSectionCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForTrussSectionCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForTrussSectionCrud[i]).updateTrussSectionInClient = trussSection;
            }
        } 
    }

    deleteTrussSectionFromChildren(trussSection) {
        for (let i = 0; i < this.state.childrenNamesForTrussSectionCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForTrussSectionCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForTrussSectionCrud[i]).deleteTrussSectionFromClient = trussSection;
            }
        } 
    }

    addBeamSectionToChildren(beamSection) {
        for (let i = 0; i < this.state.childrenNamesForBeamSectionCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForBeamSectionCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForBeamSectionCrud[i]).addBeamSectionToClient = beamSection;
            }
        } 
    }

    updateBeamSectionInChildren(beamSection) {
        for (let i = 0; i < this.state.childrenNamesForBeamSectionCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForBeamSectionCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForBeamSectionCrud[i]).updateBeamSectionInClient = beamSection;
            }
        } 
    }

    deleteBeamSectionFromChildren(beamSection) {
        for (let i = 0; i < this.state.childrenNamesForBeamSectionCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForBeamSectionCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForBeamSectionCrud[i]).deleteBeamSectionFromClient = beamSection;
            }
        } 
    }

    addPropertiesToChildren(properties) {
        for (let i = 0; i < this.state.childrenNamesForPropertiesCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForPropertiesCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForPropertiesCrud[i]).addPropertiesToClient = properties;
            }
        } 
    }

    updatePropertiesInChildren(properties) {
        for (let i = 0; i < this.state.childrenNamesForPropertiesCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForPropertiesCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForPropertiesCrud[i]).updatePropertiesInClient = properties;
            }
        } 
    }

    deletePropertiesFromChildren(properties) {
        for (let i = 0; i < this.state.childrenNamesForPropertiesCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForPropertiesCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForPropertiesCrud[i]).deletePropertiesFromClient = properties;
            }
        } 
    }

    addAssignedPropertiesToChildren(assignedProperties) {
        for (let i = 0; i < this.state.childrenNamesForAssignedPropertiesCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForAssignedPropertiesCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForAssignedPropertiesCrud[i]).addAssignedPropertiesToClient = assignedProperties;
            }
        } 
    }

    updateAssignedPropertiesInChildren(assignedProperties) {
        for (let i = 0; i < this.state.childrenNamesForAssignedPropertiesCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForAssignedPropertiesCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForAssignedPropertiesCrud[i]).updateAssignedPropertiesInClient = assignedProperties;
            }
        } 
    }

    deleteAssignedPropertiesFromChildren(assignedProperties) {
        for (let i = 0; i < this.state.childrenNamesForAssignedPropertiesCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForAssignedPropertiesCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForAssignedPropertiesCrud[i]).deleteAssignedPropertiesFromClient = assignedProperties;
            }
        } 
    }
}

export default FeaPreprocessorMenu;
