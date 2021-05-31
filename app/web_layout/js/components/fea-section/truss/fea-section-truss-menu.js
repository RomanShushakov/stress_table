class FeaSectionTrussMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,     // u32;
            trussSections: [],  // array of: [{ name: String, area: f64, area2: f64 or null }];
        };

        this.state = {
            childrenNamesForActionIdUpdate: [
                "fea-section-add-truss-menu",
                "fea-section-update-truss-menu",
                "fea-section-delete-truss-menu",
            ],

            childrenNamesForTrussSectionCrud: [
                "fea-section-add-truss-menu",
                "fea-section-update-truss-menu",
                "fea-section-delete-truss-menu",
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
                <fea-section-truss-menu-buttons></fea-section-truss-menu-buttons>
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
            case "section-add-truss-menu":
                const feaSectionAddTrussMenu = document.createElement("fea-section-add-truss-menu");
                this.append(feaSectionAddTrussMenu);
                event.stopPropagation();
                this.querySelector("fea-section-add-truss-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.trussSections.length; i++) {
                    const trussSection = this.props.trussSections[i];
                    this.querySelector("fea-section-add-truss-menu").addTrussSectionToClient = trussSection;
                }
                break;
            case "section-update-truss-menu":
                const feaSectionUpdateTrussMenu = document.createElement("fea-section-update-truss-menu");
                this.append(feaSectionUpdateTrussMenu);
                event.stopPropagation();
                this.querySelector("fea-section-update-truss-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.trussSections.length; i++) {
                    const trussSection = this.props.trussSections[i];
                    this.querySelector("fea-section-update-truss-menu").addTrussSectionToClient = trussSection;
                }
                break;
            case "section-delete-truss-menu":
                const feaSectionDeleteTrussMenu = document.createElement("fea-section-delete-truss-menu");
                this.append(feaSectionDeleteTrussMenu);
                event.stopPropagation();
                this.querySelector("fea-section-delete-truss-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.trussSections.length; i++) {
                    const trussSection = this.props.trussSections[i];
                    this.querySelector("fea-section-delete-truss-menu").addTrussSectionToClient = trussSection;
                }
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "section-add-truss-menu":
                this.querySelector("fea-section-add-truss-menu").remove();
                event.stopPropagation();
                break;
            case "section-update-truss-menu":
                this.querySelector("fea-section-update-truss-menu").remove();
                event.stopPropagation();
                break;
            case "section-delete-truss-menu":
                this.querySelector("fea-section-delete-truss-menu").remove();
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
                this.querySelector(this.state.childrenNamesForTrussSectionCrud[i]).updateTrussSectionToClient = trussSection;
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
}

export default FeaSectionTrussMenu;
