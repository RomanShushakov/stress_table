class FeaMaterialMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,     // u32;
            materials: [],      // array of: [{ name: String, youngModulus: f64, poissonRatio: f64 }, ...];
        };

        this.state = {
            childrenNamesForActionIdUpdate: [
                "fea-material-add-material-menu",
                "fea-material-update-material-menu",
                "fea-material-delete-material-menu",
            ],

            childrenNamesForMaterialCrud: [
                "fea-material-add-material-menu",
                "fea-material-update-material-menu",
                "fea-material-delete-material-menu",
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

                .material-menu-caption {
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
                <p class="material-menu-caption">Material</p>
                <fea-material-menu-buttons></fea-material-menu-buttons>
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

    set addMaterialToClient(material) {
        this.props.materials.push(material);
        this.props.materials.sort((a, b) => a.name - b.name);
        this.addMaterialToChildren(material);
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
            case "material-add-material-menu":
                const feaMaterialAddMaterialMenu = document.createElement("fea-material-add-material-menu");
                this.append(feaMaterialAddMaterialMenu);
                event.stopPropagation();
                this.querySelector("fea-material-add-material-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.materials.length; i++) {
                    const material = this.props.materials[i];
                    this.querySelector("fea-material-add-material-menu").addMaterialToClient = material;
                }
                break;
            case "material-update-material-menu":
                const feaMaterialUpdateMaterialMenu = document.createElement("fea-material-update-material-menu");
                this.append(feaMaterialUpdateMaterialMenu);
                event.stopPropagation();
                this.querySelector("fea-material-update-material-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.materials.length; i++) {
                    const material = this.props.materials[i];
                    this.querySelector("fea-material-update-material-menu").addMaterialToClient = material;
                }
                break;
            case "material-delete-material-menu":
                const feaMaterialDeleteMaterialMenu = document.createElement("fea-material-delete-material-menu");
                this.append(feaMaterialDeleteMaterialMenu);
                event.stopPropagation();
                this.querySelector("fea-material-delete-material-menu").actionId = this.props.actionId;
                for (let i = 0; i < this.props.materials.length; i++) {
                    const material = this.props.materials[i];
                    this.querySelector("fea-material-delete-material-menu").addMaterialToClient = material;
                }
                break;
        }
    }

    deactivateMenu(event) {
        switch (event.detail.menuName) {
            case "material-add-material-menu":
                this.querySelector("fea-material-add-material-menu").remove();
                event.stopPropagation();
                break;
            case "material-update-material-menu":
                this.querySelector("fea-material-update-material-menu").remove();
                event.stopPropagation();
                break;
            case "material-delete-material-menu":
                this.querySelector("fea-material-delete-material-menu").remove();
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

    addMaterialToChildren(material) {
        for (let i = 0; i < this.state.childrenNamesForMaterialCrud.length; i++) {
            if (this.querySelector(this.state.childrenNamesForMaterialCrud[i]) !== null) {
                this.querySelector(this.state.childrenNamesForMaterialCrud[i]).addMaterialToClient = material;
            }
        } 
    }
}

export default FeaMaterialMenu;
