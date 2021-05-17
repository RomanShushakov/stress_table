class FeaGeometryPointMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
            points: [],
        };

        this.state = {
            childrenNamesForActionIdUpdate: [
                "fea-geometry-add-point-menu",
                "fea-geometry-update-point-menu",
                "fea-geometry-delete-point-menu",
            ],

            childrenNamesForPointCrud: [
                "fea-geometry-add-point-menu",
                "fea-geometry-update-point-menu",
                "fea-geometry-delete-point-menu",
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
                <fea-geometry-point-menu-buttons></fea-geometry-point-menu-buttons>
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
            case "geometry-add-point-menu":
                const feaGeometryAddPointMenu = document.createElement("fea-geometry-add-point-menu");
                this.append(feaGeometryAddPointMenu);
                event.stopPropagation();
                this.querySelector("fea-geometry-add-point-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.points.length; i++) {
                    const point = this.props.points[i];
                    this.querySelector("fea-geometry-add-point-menu").addPointToClient = point;
                } 
                break;
            case "geometry-update-point-menu":
                const feaGeometryUpdatePointMenu = document.createElement("fea-geometry-update-point-menu");
                this.append(feaGeometryUpdatePointMenu);
                event.stopPropagation();
                this.querySelector("fea-geometry-update-point-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.points.length; i++) {
                    const point = this.props.points[i];
                    this.querySelector("fea-geometry-update-point-menu").addPointToClient = point;
                } 
                break;
            case "geometry-delete-point-menu":
                const feaGeometryDeletePointMenu = document.createElement("fea-geometry-delete-point-menu");
                this.append(feaGeometryDeletePointMenu);
                event.stopPropagation();
                this.querySelector("fea-geometry-delete-point-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.points.length; i++) {
                    const point = this.props.points[i];
                    this.querySelector("fea-geometry-delete-point-menu").addPointToClient = point;
                } 
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "geometry-add-point-menu":
                this.querySelector("fea-geometry-add-point-menu").remove();
                event.stopPropagation();
                break;
            case "geometry-update-point-menu":
                this.querySelector("fea-geometry-update-point-menu").remove();
                event.stopPropagation();
                break;
            case "geometry-delete-point-menu":
                this.querySelector("fea-geometry-delete-point-menu").remove();
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
}

export default FeaGeometryPointMenu;
