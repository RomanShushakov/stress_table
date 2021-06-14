class FeaPropertiesMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,     // u32;
            lines: new Map(),   // map: { number: u32, startPointNumber: u32, endPointNumber: u32 }, ...};
            materials: [],      // array of: [{ name: String, youngModulus: f64, poissonRatio: f64 }, ...];
            trussSections: [],  // array of: [{ name: String, area: f64, area2: f64 or null }];
            beamSections: [],   // array of: [{ name: String, area: f64, I11: f64, I22: f64, I12: f64, It: f64 }];
            properties: [],     // array of: [{ name: String, materialName: String, sectionName: String,
                                //              sectionType: String, usedIn: [u32, ...] }];
        };

        this.state = {
            childrenNamesForActionIdUpdate: [
                "fea-properties-add-properties-menu",
                "fea-properties-update-properties-menu",
                "fea-properties-delete-properties-menu",
                "fea-properties-assign-properties-menu",
            ],

            childrenNamesForLineCrud: [
                "fea-properties-assign-properties-menu",
            ],

            childrenNamesForMaterialCrud: [
                "fea-properties-add-properties-menu",
                "fea-properties-update-properties-menu",
            ],

            childrenNamesForTrussSectionCrud: [
                "fea-properties-add-properties-menu",
                "fea-properties-update-properties-menu",
            ],

            childrenNamesForBeamSectionCrud: [
                "fea-properties-add-properties-menu",
                "fea-properties-update-properties-menu",
            ],

            childrenNamesForPropertiesCrud: [
                "fea-properties-add-properties-menu",
                "fea-properties-update-properties-menu",
                "fea-properties-delete-properties-menu",
                "fea-properties-assign-properties-menu",
            ],
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: flex;
                }

                .wrapper {
                    display: flex;
                    flex-direction: column;
                    background-color: #3b4453;
                    padding: 1rem;
                }

                .properties-menu-caption {
                    margin: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0.3rem;
                    padding-left: 0rem;
                    padding-right: 0rem;
                    color: #D9D9D9;
                    border-bottom: 0.1rem solid #4a5060;
                    font-size: 85%;
                }
            </style>

            <div class=wrapper>
                <p class="properties-menu-caption">Properties</p>
                <fea-properties-menu-buttons></fea-properties-menu-buttons>
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

    set addMaterialToClient(material) {
        this.props.materials.push(material);
        this.props.materials.sort((a, b) => a.name - b.name);
        this.addMaterialToChildren(material);
    }

    set updateMaterialInClient(material) {
        let materialInProps = this.props.materials.find(existedMaterial => existedMaterial.name == material.name);
        materialInProps.youngModulus = material.youngModulus;
        materialInProps.poissonRatio = material.poissonRatio;
        this.updateMaterialInChildren(material);
    }

    set deleteMaterialFromClient(material) {
        let materialIndexInProps = this.props.materials.findIndex(existedMaterial => existedMaterial.name == material.name);
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
        this.updatePropertiesInChildren(properties);
    }

    set deletePropertiesFromClient(properties) {
        let propertiesIndexInProps = this.props.properties
            .findIndex(existedProperties => existedProperties.name == existedProperties.name);
        this.props.properties.splice(propertiesIndexInProps, 1);
        this.props.properties.sort((a, b) => a.name - b.name);
        this.deletePropertiesFromChildren(properties);
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
            case "properties-add-properties-menu":
                const feaPropertiesAddPropertiesMenu = document.createElement("fea-properties-add-properties-menu");
                this.append(feaPropertiesAddPropertiesMenu);
                event.stopPropagation();
                this.querySelector("fea-properties-add-properties-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.materials.length; i++) {
                    const material = this.props.materials[i];
                    this.querySelector("fea-properties-add-properties-menu").addMaterialToClient = material;
                }
                for (let i = 0; i < this.props.trussSections.length; i++) {
                    const trussSection = this.props.trussSections[i];
                    this.querySelector("fea-properties-add-properties-menu").addTrussSectionToClient = trussSection;
                }
                for (let i = 0; i < this.props.beamSections.length; i++) {
                    const beamSection = this.props.beamSections[i];
                    this.querySelector("fea-properties-add-properties-menu").addBeamSectionToClient = beamSection;
                }
                for (let i = 0; i < this.props.properties.length; i++) {
                    const properties = this.props.properties[i];
                    this.querySelector("fea-properties-add-properties-menu").addPropertiesToClient = properties;
                }
                break;
            case "properties-update-properties-menu":
                const feaPropertiesUpdatePropertiesMenu = document.createElement("fea-properties-update-properties-menu");
                this.append(feaPropertiesUpdatePropertiesMenu);
                event.stopPropagation();
                this.querySelector("fea-properties-update-properties-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.materials.length; i++) {
                    const material = this.props.materials[i];
                    this.querySelector("fea-properties-update-properties-menu").addMaterialToClient = material;
                }
                for (let i = 0; i < this.props.trussSections.length; i++) {
                    const trussSection = this.props.trussSections[i];
                    this.querySelector("fea-properties-update-properties-menu").addTrussSectionToClient = trussSection;
                }
                for (let i = 0; i < this.props.beamSections.length; i++) {
                    const beamSection = this.props.beamSections[i];
                    this.querySelector("fea-properties-update-properties-menu").addBeamSectionToClient = beamSection;
                }
                for (let i = 0; i < this.props.properties.length; i++) {
                    const properties = this.props.properties[i];
                    this.querySelector("fea-properties-update-properties-menu").addPropertiesToClient = properties;
                }
                break;
            case "properties-delete-properties-menu":
                const feaPropertiesDeletePropertiesMenu = document.createElement("fea-properties-delete-properties-menu");
                this.append(feaPropertiesDeletePropertiesMenu);
                event.stopPropagation();
                this.querySelector("fea-properties-delete-properties-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.properties.length; i++) {
                    const properties = this.props.properties[i];
                    this.querySelector("fea-properties-delete-properties-menu").addPropertiesToClient = properties;
                }
            case "properties-assign-properties-menu":
                const feaPropertiesAssignPropertiesMenu = document.createElement("fea-properties-assign-properties-menu");
                this.append(feaPropertiesAssignPropertiesMenu);
                event.stopPropagation();
                this.querySelector("fea-properties-assign-properties-menu").actionId = this.props.actionId;
                for (let [lineNumber, linePointsNumbers] of this.props.lines) {
                    const line = { "number": lineNumber,
                        "startPointNumber": linePointsNumbers.startPointNumber,
                        "endPointNumber": linePointsNumbers.endPointNumber };
                    this.querySelector("fea-properties-assign-properties-menu").addLineToClient = line;
                }
                for (let i = 0; i < this.props.properties.length; i++) {
                    const properties = this.props.properties[i];
                    this.querySelector("fea-properties-assign-properties-menu").addPropertiesToClient = properties;
                }
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "properties-add-properties-menu":
                this.querySelector("fea-properties-add-properties-menu").remove();
                event.stopPropagation();
                break;
            case "properties-update-properties-menu":
                this.querySelector("fea-properties-update-properties-menu").remove();
                event.stopPropagation();
                break;
            case "properties-delete-properties-menu":
                this.querySelector("fea-properties-delete-properties-menu").remove();
                event.stopPropagation();
                break;
            case "properties-assign-properties-menu":
                this.querySelector("fea-properties-assign-properties-menu").remove();
                event.stopPropagation();
                break;
        }
    }

    updateChildrenActionId() {
        for (let i = 0; i < this.state.childrenNamesForActionIdUpdate.length; i++) {
            if (this.querySelector(this.state.childrenNamesForActionIdUpdate[i]) !== null) {
                this.querySelector(this.state.childrenNamesForActionIdUpdate[i]).actionId = this.props.actionId;
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
                this.querySelector(this.state.childrenNamesForPropertiesCrud[i]).addPointToChildren = properties;
            }
        } 
    }

    updatePropertiesInChildren(properties) {
        for (let i = 0; i < this.state.childrenNamesForPropertiesCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForPropertiesCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForPropertiesCrud[i]).updatePropertiesInChildren = properties;
            }
        } 
    }

    deletePropertiesFromChildren(properties) {
        for (let i = 0; i < this.state.childrenNamesForPropertiesCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForPropertiesCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForPropertiesCrud[i]).deletePropertiesFromChildren = properties;
            }
        } 
    }
}

export default FeaPropertiesMenu;
