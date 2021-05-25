class FeaPreprocessorMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,     // u32;
            points: new Map(),  // map: { number: u32, { x: f64, y: f64, z: f64}, ... };
            lines: new Map(),   // map: { number: u32, startPointNumber: u32, endPointNumber: u32 }, ...};
            materials: [],      // array of: [{ name: String, youngModulus: f64, poissonRatio: f64 }, ...];
        };

        this.state = {
            childrenNamesForActionIdUpdate: [
                "fea-geometry-menu",
                "fea-material-menu",
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
            ],

            childrenNamesForMaterialCrud: [
                "fea-material-menu",
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
}

export default FeaPreprocessorMenu;
