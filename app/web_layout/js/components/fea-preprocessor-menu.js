class FeaPreprocessorMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            menuNames: {
                "geometry-menu": "fea-geometry-menu",
                "material-menu": "fea-material-menu",
                "section-menu": "fea-section-menu",
                "properties-menu": "fea-properties-menu",
            },
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

    // set addAssignedPropertiesToClient(assignedProperties) {
    //     this.props.assignedProperties.push(assignedProperties);
    //     this.props.assignedProperties.sort((a, b) => a.name - b.name);
    //     this.addAssignedPropertiesToChildren(assignedProperties);
    // }

    // set updateAssignedPropertiesInClient(assignedProperties) {
    //     let assignedPropertiesInProps = this.props.assignedProperties
    //         .find(existedAssignedProperties => existedAssignedProperties.name == assignedProperties.name);
    //     assignedPropertiesInProps.lineNumbers = assignedProperties.lineNumbers;
    //     this.updateAssignedPropertiesInChildren(assignedProperties);
    // }

    // set deleteAssignedPropertiesFromClient(assignedProperties) {
    //     let assignedPropertiesIndexInProps = this.props.assignedProperties
    //         .findIndex(existedAssignedProperties => existedAssignedProperties.name == assignedProperties.name);
    //     this.props.assignedProperties.splice(assignedPropertiesIndexInProps, 1);
    //     this.props.assignedProperties.sort((a, b) => a.name - b.name);
    //     this.deleteAssignedPropertiesFromChildren(assignedProperties);
    // }

    // set addBeamSectionOrientationToClient(beamSectionOrientation) {
    //     this.props.beamSectionsOrientations.push(beamSectionOrientation);
    //     this.addBeamSectionOrientationToChildren(beamSectionOrientation);
    // }

    // set updateBeamSectionOrientationInClient(beamSectionOrientation) {
    //     let beamSectionOrientationInProps = this.props.beamSectionsOrientations
    //         .find(existedBeamSectionOrientation => 
    //             existedBeamSectionOrientation.propertiesName == beamSectionOrientation.propertiesName &&
    //             existedBeamSectionOrientation.localAxis1Direction == beamSectionOrientation.localAxis1Direction);
    //         beamSectionOrientationInProps.lineNumbers = beamSectionOrientation.lineNumbers;
    //     this.updateBeamSectionOrientationInChildren(beamSectionOrientation);
    // }

    // set deleteBeamSectionOrientationFromClient(beamSectionOrientation) {
    //     let beamSectionOrientationIndexInProps = this.props.beamSectionsOrientations
    //         .findIndex(existedBeamSectionOrientation => 
    //             existedBeamSectionOrientation.propertiesName == beamSectionOrientation.propertiesName);
    //     this.props.beamSectionsOrientations.splice(beamSectionOrientationIndexInProps, 1);
    //     this.deleteBeamSectionOrientationFromChildren(beamSectionOrientation);
    // }

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

    updateCanvasSize() {
        this.dispatchEvent(new CustomEvent("resize", {
            bubbles: true,
            composed: true,
        }));
    }

    activateMenu(event) {
        const menuName = event.detail.menuName;
        const menu = document.createElement(this.state.menuNames[menuName]);
        this.append(menu);
        event.stopPropagation();
        this.updateCanvasSize();
    }

    deactivateMenu(event) {
        const menuName = event.detail.menuName;
        this.querySelector(this.state.menuNames[menuName]).remove();
        event.stopPropagation();
        this.updateCanvasSize();
    }
}

export default FeaPreprocessorMenu;
