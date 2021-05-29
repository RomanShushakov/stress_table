class FeaSectionMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,     // u32;
            // points: new Map(),  // map: { number: u32, { x: f64, y: f64, z: f64}, ... };
            // lines: new Map(),   // map: { number: u32, startPointNumber: u32, endPointNumber: u32 }, ...};
        };

        this.state = {
            childrenNamesForActionIdUpdate: [
                "fea-section-truss-menu",
                "fea-section-beam-menu"
            ],

            // childrenNamesForPointCrud: [
            //     "fea-geometry-point-menu",
            //     "fea-geometry-line-menu",
            // ],

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

    // set addPointToClient(point) {
    //     this.props.points.set(point.number, {"x": point.x, "y": point.y, "z": point.z});
    //     this.addPointToChildren(point);
    // }

    // set updatePointInClient(point) {
    //     this.props.points.set(point.number, {"x": point.x, "y": point.y, "z": point.z});
    //     this.updatePointInChildren(point);
    // }

    // set deletePointFromClient(point) {
    //     this.props.points.delete(point.number);
    //     this.deletePointFromChildren(point);
    // }

    // set addLineToClient(line) {
    //     this.props.lines.set(line.number, { "startPointNumber": line.startPointNumber, "endPointNumber": line.endPointNumber });
    //     this.addLineToChildren(line);
    // }

    // set updateLineInClient(line) {
    //     this.props.lines.set(line.number, { "startPointNumber": line.startPointNumber, "endPointNumber": line.endPointNumber });
    //     this.updateLineInChildren(line);
    // }

    // set deleteLineFromClient(line) {
    //     this.props.lines.delete(line.number);
    //     this.deleteLineFromChildren(line);
    // }

    // set selectPointInClient(pointNumber) {
    //     this.shadowRoot.querySelector("fea-geometry-menu-buttons").activateButton = "geometry-point-menu-button";
    //     this.querySelector("fea-geometry-point-menu").selectPointInClient = pointNumber;
    // }

    // set selectLineInClient(lineNumber) {
    //     this.shadowRoot.querySelector("fea-geometry-menu-buttons").activateButton = "geometry-line-menu-button";
    //     this.querySelector("fea-geometry-line-menu").selectLineInClient = lineNumber;
    // }

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
                // for (let [pointNumber, coordinates] of this.props.points) {
                //     const point = { "number": pointNumber, "x": coordinates.x, "y": coordinates.y, "z": coordinates.z };
                //     this.querySelector("fea-geometry-point-menu").addPointToClient = point;
                // }
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

    // updateChildrenActionId() {
    //     for (let i = 0; i < this.state.childrenNamesForActionIdUpdate.length; i++) {
    //         if (this.querySelector(this.state.childrenNamesForActionIdUpdate[i]) !== null) {
    //             this.querySelector(this.state.childrenNamesForActionIdUpdate[i]).actionId = this.props.actionId;
    //         }
    //     } 
    // }

    // addPointToChildren(point) {
    //     for (let i = 0; i < this.state.childrenNamesForPointCrud.length; i++) {
    //         if (this.querySelector(this.state.childrenNamesForPointCrud[i]) !== null) {
    //             this.querySelector(this.state.childrenNamesForPointCrud[i]).addPointToClient = point;
    //         }
    //     } 
    // }

    // updatePointInChildren(point) {
    //     for (let i = 0; i < this.state.childrenNamesForPointCrud.length; i++) {
    //         if (this.querySelector(this.state.childrenNamesForPointCrud[i]) !== null) {
    //             this.querySelector(this.state.childrenNamesForPointCrud[i]).updatePointInClient = point;
    //         }
    //     } 
    // }

    // deletePointFromChildren(point) {
    //     for (let i = 0; i < this.state.childrenNamesForPointCrud.length; i++) {
    //         if (this.querySelector(this.state.childrenNamesForPointCrud[i]) !== null) {
    //             this.querySelector(this.state.childrenNamesForPointCrud[i]).deletePointFromClient = point;
    //         }
    //     } 
    // }

    // addLineToChildren(line) {
    //     for (let i = 0; i < this.state.childrenNamesForLineCrud.length; i++) {
    //         if (this.querySelector(this.state.childrenNamesForLineCrud[i]) !== null) {
    //             this.querySelector(this.state.childrenNamesForLineCrud[i]).addLineToClient = line;
    //         }
    //     } 
    // }

    // updateLineInChildren(line) {
    //     for (let i = 0; i < this.state.childrenNamesForLineCrud.length; i++) {
    //         if (this.querySelector(this.state.childrenNamesForLineCrud[i]) !== null) {
    //             this.querySelector(this.state.childrenNamesForLineCrud[i]).updateLineInClient = line;
    //         }
    //     } 
    // }

    // deleteLineFromChildren(line) {
    //     for (let i = 0; i < this.state.childrenNamesForLineCrud.length; i++) {
    //         if (this.querySelector(this.state.childrenNamesForLineCrud[i]) !== null) {
    //             this.querySelector(this.state.childrenNamesForLineCrud[i]).deleteLineFromClient = line;
    //         }
    //     } 
    // }
}

export default FeaSectionMenu;
