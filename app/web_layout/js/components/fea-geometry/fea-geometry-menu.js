class FeaGeometryMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
            points: [],
        };

        this.state = {
            childrenNamesForActionIdUpdate: [
                "fea-geometry-point-menu",
                "fea-geometry-line-menu"
            ],

            childrenNamesForPointCrud: [
                "fea-geometry-point-menu",
                "fea-geometry-line-menu",
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

                .geometry-menu-caption {
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
                <p class="geometry-menu-caption">Geometry</p>
                <fea-geometry-menu-buttons></fea-geometry-menu-buttons>
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
            case "geometry-point-menu":
                const feaGeometryPointMenu = document.createElement("fea-geometry-point-menu");
                this.append(feaGeometryPointMenu);
                event.stopPropagation();
                this.querySelector("fea-geometry-point-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.points.length; i++) {
                    const point = this.props.points[i];
                    this.querySelector("fea-geometry-point-menu").addPointToClient = point;
                } 
                break;
            case "geometry-line-menu":
                const feaGeometryLineMenu = document.createElement("fea-geometry-line-menu");
                this.append(feaGeometryLineMenu);
                event.stopPropagation();
                this.querySelector("fea-geometry-line-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.points.length; i++) {
                    const point = this.props.points[i];
                    this.querySelector("fea-geometry-line-menu").addPointToClient = point;
                } 
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "geometry-point-menu":
                this.querySelector("fea-geometry-point-menu").remove();
                event.stopPropagation();
                break;
            case "geometry-line-menu":
                this.querySelector("fea-geometry-line-menu").remove();
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
}

export default FeaGeometryMenu;
