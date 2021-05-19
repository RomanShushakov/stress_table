class FeaPreprocessorMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
            points: [],
            lines: [],
        };

        this.state = {
            childrenNamesForActionIdUpdate: [
                "fea-geometry-menu",
            ],

            childrenNamesForPointCrud: [
                "fea-geometry-menu",
            ],

            childrenNamesForLineCrud: [
                "fea-geometry-menu",
            ],

            menuNames: [
                "geometry-menu", "material-menu", "properties-menu", "mesh-menu", "load-menu",
                "boundary-condition-menu", "analysis-menu",
            ]
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
        this.props.points.push(point);
        this.props.points.sort((a, b) => a.number - b.number);
        this.addPointToChildren(point);
    }

    set updatePointInClient(point) {
        let pointInProps = this.props.points.find(existedPoint => existedPoint.number == point.number);
        pointInProps.x = point.x;
        pointInProps.y = point.y;
        pointInProps.z = point.z;
        this.updatePointInChildren(point);
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
            this.querySelector("fea-geometry-menu").selectPointInClient = pointNumber;
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
            this.querySelector("fea-geometry-menu").selectLineInClient = lineNumber;
        }
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
                for (let i = 0; i < this.props.points.length; i++) {
                    const point = this.props.points[i];
                    this.querySelector("fea-geometry-menu").addPointToClient = point;
                }
                for (let i = 0; i < this.props.lines.length; i++) {
                    const line = this.props.lines[i];
                    this.querySelector("fea-geometry-menu").addLineToClient = line;
                }  
                break;
            case "material-menu":
                const feaMaterialMenu = document.createElement("fea-material-menu");
                this.append(feaMaterialMenu);
                event.stopPropagation();
                this.updateCanvasSize();
                this.querySelector("fea-material-menu").actionId = this.props.actionId;
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
}

export default FeaPreprocessorMenu;
