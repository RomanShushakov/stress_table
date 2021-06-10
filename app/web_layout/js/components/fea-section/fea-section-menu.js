class FeaSectionMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,     // u32;
            trussSections: [],  // array of: [{ name: String, area: f64, area2: f64 or null }];
            beamSections: [],   // array of: [{ name: String, area: f64, I11: f64, I22: f64, I12: f64, It: f64 }];
        };

        this.state = {
            childrenNamesForActionIdUpdate: [
                "fea-section-truss-menu",
                "fea-section-beam-menu"
            ],

            childrenNamesForTrussSectionCrud: [
                "fea-section-truss-menu",
            ],

            childrenNamesForBeamSectionCrud: [
                "fea-section-beam-menu",
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

                .section-menu-caption {
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
                <p class="section-menu-caption">Section</p>
                <fea-section-menu-buttons></fea-section-menu-buttons>
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
            case "section-truss-menu":
                const feaSectionTrussMenu = document.createElement("fea-section-truss-menu");
                this.append(feaSectionTrussMenu);
                event.stopPropagation();
                this.querySelector("fea-section-truss-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.trussSections.length; i++) {
                    const trussSection = this.props.trussSections[i];
                    this.querySelector("fea-section-truss-menu").addTrussSectionToClient = trussSection;
                }
                break;
            case "section-beam-menu":
                const feaSectionBeamMenu = document.createElement("fea-section-beam-menu");
                this.append(feaSectionBeamMenu);
                event.stopPropagation();
                this.querySelector("fea-section-beam-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.beamSections.length; i++) {
                    const beamSection = this.props.beamSections[i];
                    this.querySelector("fea-section-beam-menu").addBeamSectionToClient = beamSection;
                }
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "section-truss-menu":
                this.querySelector("fea-section-truss-menu").remove();
                event.stopPropagation();
                break;
            case "section-beam-menu":
                this.querySelector("fea-section-beam-menu").remove();
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
}

export default FeaSectionMenu;
