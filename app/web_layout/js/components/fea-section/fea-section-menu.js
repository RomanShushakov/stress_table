class FeaSectionMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,     // u32;
            trussSections: [],  // array of: [{ name: String, area: f64, area2: f64 or null }];
        };

        this.state = {
            childrenNamesForActionIdUpdate: [
                "fea-section-truss-menu",
                "fea-section-beam-menu"
            ],

            childrenNamesForTrussSectionCrud: [
                "fea-section-truss-menu",
            ],

            // childrenNamesForLineCrud: [
            //     "fea-geometry-line-menu",
            // ],
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
                    this.querySelector("fea-section-menu").addTrussSectionToClient = trussSection;
                }
                break;
            case "section-beam-menu":
                const feaSectionBeamMenu = document.createElement("fea-section-beam-menu");
                this.append(feaSectionBeamMenu);
                event.stopPropagation();
                this.querySelector("fea-section-beam-menu").actionId = this.props.actionId;
                // for (let [pointNumber, coordinates] of this.props.points) {
                //     const point = { "number": pointNumber, "x": coordinates.x, "y": coordinates.y, "z": coordinates.z };
                //     this.querySelector("fea-geometry-line-menu").addPointToClient = point;
                // }
                // for (let [lineNumber, linePointsNumbers] of this.props.lines) {
                //     const line = { "number": lineNumber,
                //         "startPointNumber": linePointsNumbers.startPointNumber,
                //         "endPointNumber": linePointsNumbers.endPointNumber };
                //     this.querySelector("fea-geometry-line-menu").addLineToClient = line;
                // }
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
}

export default FeaSectionMenu;
