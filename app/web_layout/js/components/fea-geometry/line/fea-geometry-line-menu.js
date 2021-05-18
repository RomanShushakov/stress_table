class FeaGeometryLineMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
            points: [],
            lines: [],
        };

        this.state = {
            childrenNamesForActionIdUpdate: [
                "fea-geometry-add-line-menu",
                "fea-geometry-update-line-menu",
                "fea-geometry-delete-line-menu",
            ],

            childrenNamesForPointCrud: [
                "fea-geometry-add-line-menu",
                "fea-geometry-update-line-menu",
            ],

            childrenNamesForLineCrud: [
                "fea-geometry-add-line-menu",
                "fea-geometry-update-line-menu",
                "fea-geometry-delete-line-menu",
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
                <fea-geometry-line-menu-buttons></fea-geometry-line-menu-buttons>
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
        this.props.points.push(point);
        this.props.points.sort((a, b) => a.number - b.number);
        this.addPointToChildren(point);
    }

    set updatePointInClient(_point) {
    }

    set deletePointFromClient(point) {
        let pointIndexInProps = this.props.points.findIndex(existedPoint => existedPoint.number == point.number);
        this.props.points.splice(pointIndexInProps, 1);
        this.props.points.sort((a, b) => a.number - b.number);
        this.deletePointFromChildren(point);
    }

    set addLineToClient(line) {
        this.props.lines.push(line);
        this.props.lines.sort((a, b) => a.number - b.number);
        this.addLineToChildren(line);
    }

    set updateLineInClient(line) {
        let lineInProps = this.props.lines.find(existedLine => existedLine.number == line.number);
        lineInProps.startPointNumber = line.startPointNumber;
        lineInProps.endPointNumber = line.endPointNumber;
        this.updateLineInChildren(line);
    }

    set deleteLineFromClient(line) {
        let lineIndexInProps = this.props.lines.findIndex(existedLine => existedLine.number == line.number);
        this.props.lines.splice(lineIndexInProps, 1);
        this.props.lines.sort((a, b) => a.number - b.number);
        this.deleteLineFromChildren(line);
    }

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

    activateMenu(event) {
        switch (event.detail.menuName) {
            case "geometry-add-line-menu":
                const feaGeometryAddLineMenu = document.createElement("fea-geometry-add-line-menu");
                this.append(feaGeometryAddLineMenu);
                event.stopPropagation();
                this.querySelector("fea-geometry-add-line-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.points.length; i++) {
                    const point = this.props.points[i];
                    this.querySelector("fea-geometry-add-line-menu").addPointToClient = point;
                }
                for (let i = 0; i < this.props.lines.length; i++) {
                    const line = this.props.lines[i];
                    this.querySelector("fea-geometry-add-line-menu").addLineToClient = line;
                }  
                break;
            case "geometry-update-line-menu":
                const feaGeometryUpdateLineMenu = document.createElement("fea-geometry-update-line-menu");
                this.append(feaGeometryUpdateLineMenu);
                event.stopPropagation();
                this.querySelector("fea-geometry-update-line-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.points.length; i++) {
                    const point = this.props.points[i];
                    this.querySelector("fea-geometry-update-line-menu").addPointToClient = point;
                }
                for (let i = 0; i < this.props.lines.length; i++) {
                    const line = this.props.lines[i];
                    this.querySelector("fea-geometry-update-line-menu").addLineToClient = line;
                }   
                break;
            case "geometry-delete-line-menu":
                const feaGeometryDeleteLineMenu = document.createElement("fea-geometry-delete-line-menu");
                this.append(feaGeometryDeleteLineMenu);
                event.stopPropagation();
                this.querySelector("fea-geometry-delete-line-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.lines.length; i++) {
                    const line = this.props.lines[i];
                    this.querySelector("fea-geometry-delete-line-menu").addLineToClient = line;
                }  
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "geometry-add-line-menu":
                this.querySelector("fea-geometry-add-line-menu").remove();
                event.stopPropagation();
                break;
            case "geometry-update-line-menu":
                this.querySelector("fea-geometry-update-line-menu").remove();
                event.stopPropagation();
                break;
            case "geometry-delete-line-menu":
                this.querySelector("fea-geometry-delete-line-menu").remove();
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

    addPointToChildren(point) {
        for (let i = 0; i < this.state.childrenNamesForPointCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForPointCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForPointCrud[i]).addPointToClient = point;
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
}

export default FeaGeometryLineMenu;
