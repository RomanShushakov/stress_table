class FeaSectionBeamMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,     // u32;
            beamSections: [],   // array of: [{ name: String, area: f64, I11: f64, I22: f64, I12: f64, It: f64 }];
        };

        this.state = {
            childrenNamesForActionIdUpdate: [
                "fea-section-add-beam-menu",
                "fea-section-update-beam-menu",
                "fea-section-delete-beam-menu",
            ],

            childrenNamesForBeamSectionCrud: [
                "fea-section-add-beam-menu",
                "fea-section-update-beam-menu",
                "fea-section-delete-beam-menu",
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
                    padding: 0rem;
                }
            </style>

            <div class=wrapper>
                <fea-section-beam-menu-buttons></fea-section-beam-menu-buttons>
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
            case "section-add-beam-menu":
                const feaSectionAddBeamMenu = document.createElement("fea-section-add-beam-menu");
                this.append(feaSectionAddBeamMenu);
                event.stopPropagation();
                this.querySelector("fea-section-add-beam-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.beamSections.length; i++) {
                    const beamSection = this.props.beamSections[i];
                    this.querySelector("fea-section-add-beam-menu").addBeamSectionToClient = beamSection;
                }
                break;
            case "section-update-beam-menu":
                const feaSectionUpdateBeamMenu = document.createElement("fea-section-update-beam-menu");
                this.append(feaSectionUpdateBeamMenu);
                event.stopPropagation();
                this.querySelector("fea-section-update-beam-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.beamSections.length; i++) {
                    const beamSection = this.props.beamSections[i];
                    this.querySelector("fea-section-update-beam-menu").addBeamSectionToClient = beamSection;
                }
                break;
            case "section-delete-beam-menu":
                const feaSectionDeleteBeamMenu = document.createElement("fea-section-delete-beam-menu");
                this.append(feaSectionDeleteBeamMenu);
                event.stopPropagation();
                this.querySelector("fea-section-delete-beam-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.beamSections.length; i++) {
                    const beamSection = this.props.beamSections[i];
                    this.querySelector("fea-section-delete-beam-menu").addBeamSectionToClient = beamSection;
                }
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "section-add-beam-menu":
                this.querySelector("fea-section-add-beam-menu").remove();
                event.stopPropagation();
                break;
            case "section-update-beam-menu":
                this.querySelector("fea-section-update-beam-menu").remove();
                event.stopPropagation();
                break;
            case "section-delete-beam-menu":
                this.querySelector("fea-section-delete-beam-menu").remove();
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

export default FeaSectionBeamMenu;
