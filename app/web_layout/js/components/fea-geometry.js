class FeaGeometry extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
        };

        this.state = {
            isAddPointActive: false,
            isUpdatePointActive: false,
            isDeletePointActive: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                }

                .hidden {
                    display: none;
                }
            </style>
            <div class="wrapper">

                <button class="geometry">Geometry</button>

                <div class="geometry-container hidden">
                    <button class="point">Point</button>
                    <button class="line">Line</button>
                </div>

                <div class="point-container hidden">

                    <div class="actions-over-point hidden">
                        <button class="select-add-action-over-point">Add</button>
                        <button class="select-update-action-over-point">Update</button>
                        <button class="select-delete-action-over-point">Delete</button>
                    </div>

                    <div class="add-action-over-point hidden">

                        <ul class="add-action-over-point-fields">
                            <li>
                                <p class="added-point-number">#1 New</p>
                            </li>
                            <li>
                                <p class="add-action-over-point-fields-description">X coordinate:</p>
                                <input class="add-x-coord"/>
                            </li>
                            <li>
                                <p class="add-action-over-point-fields-description">Y coordinate:</p>
                                <input class="add-y-coord"/>
                            </li>
                            <li>
                                <p class="add-action-over-point-fields-description">Z coordinate:</p>
                                <input class="add-z-coord"/>
                            </li>
                        </ul>

                        <button class="point-add-action-apply">Apply</button>

                    </div>

                    <div class="update-action-over-point hidden">

                        <ul class="update-action-over-point-fields">
                            <li>
                                <select class="updated-point-number">
                                    <option>#1</option>
                                </select>
                            </li>
                            <li>
                                <p class="update-action-over-point-fields-description">X coordinate:</p>
                                <input class="update-x-coord"/>
                            </li>
                            <li>
                                <p class="update-action-over-point-fields-description">Y coordinate:</p>
                                <input class="update-y-coord"/>
                            </li>
                            <li>
                                <p class="update-action-over-point-fields-description">Z coordinate:</p>
                                <input class="update-z-coord"/>
                            </li>
                        </ul>
                        
                        <button class="point-update-action-apply">Apply</button>

                    </div>

                    <div class="delete-action-over-point hidden">

                        <ul class="delete-action-over-point-fields">
                            <li>
                                <select class="deleted-point-number">
                                    <option>#1</option>
                                </select>
                            </li>
                        </ul>
                        
                        <button class="point-delete-action-apply">Apply</button>

                    </div>
                </div>

                <div class="line-container hidden">

                    <div class="actions-over-line">
                        <button class="select-add-action-over-line">Add</button>
                        <button class="select-update-action-over-line">Update</button>
                        <button class="select-delete-action-over-line">Delete</button>
                    </div>

                    <div class="add-action-over-line">

                        <ul class="add-action-over-line-fields">
                            <li>
                                <p class="added-line-number">#1 New</p>
                            </li>
                            <li>
                                <p class="add-action-over-line-fields-description">Select start point:</p>
                                <select class="selected-point-number">
                                    <option>#1</option>
                                </select>
                            </li>
                            <li>
                                <p class="add-action-over-line-fields-description">Select end point:</p>
                                <select class="selected-point-number">
                                    <option>#2</option>
                                </select>
                            </li>
                        </ul>

                        <button class="line-add-action-apply">Apply</button>

                    </div>

                    <div class="update-action-over-line">

                        <ul class="update-action-over-line-fields">
                            <li>
                                <select class="updated-line-number">
                                    <option>#1</option>
                                </select>
                            </li>
                            <li>
                                <p class="update-action-over-line-fields-description">Change line start point:</p>
                                <select class="selected-point-number">
                                    <option>#2</option>
                                </select>
                            </li>
                        </ul>
                        
                        <button class="line-update-action-apply">Apply</button>

                    </div>

                    <div class="delete-action-over-line">

                        <ul class="delete-action-over-line-fields">
                            <li>
                                <select class="deleted-line-number">
                                    <option>#1</option>
                                </select>
                            </li>
                        </ul>
                        
                        <button class="line-delete-action-apply">Apply</button>

                    </div>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".geometry").addEventListener("click", () => this.toggleGeometry());
        this.shadowRoot.querySelector(".point").addEventListener("click", () => this.togglePoint());
        this.shadowRoot.querySelector(".select-add-action-over-point").addEventListener("click", () => {
            if (this.state.isAddPointActive === false) {
                this.state.isAddPointActive = true;
                this.state.isUpdatePointActive = false;
                this.state.isDeletePointActive = false;
                this.activateAddPoint();
            } else {
                this.state.isAddPointActive = false;
                this.deactivateAddPoint();
            }
        });
        this.shadowRoot.querySelector(".select-update-action-over-point").addEventListener("click", () => {
            if (this.state.isUpdatePointActive === false) {
                this.state.isAddPointActive = false;
                this.state.isUpdatePointActive = true;
                this.state.isDeletePointActive = false;
                this.activateUpdatePoint();
            } else {
                this.state.isUpdatePointActive = false;
                this.deactivateUpdatePoint();
            }
        });
        this.shadowRoot.querySelector(".select-delete-action-over-point").addEventListener("click", () => {
            if (this.state.isDeletePointActive === false) {
                this.state.isAddPointActive = false;
                this.state.isUpdatePointActive = false;
                this.state.isDeletePointActive = true;
                this.activateDeletePoint();
            } else {
                this.state.isDeletePointActive = false;
                this.deactivateDeletePoint();
            }
        });
    }

    set actionId(value) {
        this.props.actionId = value;
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

    toggleGeometry() {
        const geometryContainer = this.shadowRoot.querySelector(".geometry-container");
        if (geometryContainer.classList.contains("hidden") === true) {
            geometryContainer.classList.remove("hidden");
        } else {
            geometryContainer.classList.add("hidden");
        }
        const pointContainer = this.shadowRoot.querySelector(".point-container");
        if (pointContainer.classList.contains("hidden") === false) {
            pointContainer.classList.add("hidden");
        }
        const actionsOverPoint = this.shadowRoot.querySelector(".actions-over-point");
        if (actionsOverPoint.classList.contains("hidden") === false) {
            actionsOverPoint.classList.add("hidden");
        }
        const addActionOverPoint = this.shadowRoot.querySelector(".add-action-over-point");
        if (addActionOverPoint.classList.contains("hidden") === false) {
            addActionOverPoint.classList.add("hidden");
        }
        const updateActionOverPoint = this.shadowRoot.querySelector(".update-action-over-point");
        if (updateActionOverPoint.classList.contains("hidden") === false) {
            updateActionOverPoint.classList.add("hidden");
        }
        const deleteActionOverPoint = this.shadowRoot.querySelector(".delete-action-over-point");
        if (deleteActionOverPoint.classList.contains("hidden") === false) {
            deleteActionOverPoint.classList.add("hidden");
        }
        const lineContainer = this.shadowRoot.querySelector(".line-container");
        if (lineContainer.classList.contains("hidden") === false) {
            lineContainer.classList.add("hidden");
        }
    }

    togglePoint() {
        const pointContainer = this.shadowRoot.querySelector(".point-container");
        if (pointContainer.classList.contains("hidden") === true) {
            pointContainer.classList.remove("hidden");
        } 
        const actionsOverPoint = this.shadowRoot.querySelector(".actions-over-point");
        if (actionsOverPoint.classList.contains("hidden") === true) {
            actionsOverPoint.classList.remove("hidden");
        } else {
            actionsOverPoint.classList.add("hidden");
        }
        const addActionOverPoint = this.shadowRoot.querySelector(".add-action-over-point");
        if (addActionOverPoint.classList.contains("hidden") === false) {
            addActionOverPoint.classList.add("hidden");
        }
        const updateActionOverPoint = this.shadowRoot.querySelector(".update-action-over-point");
        if (updateActionOverPoint.classList.contains("hidden") === false) {
            updateActionOverPoint.classList.add("hidden");
        }
        const deleteActionOverPoint = this.shadowRoot.querySelector(".delete-action-over-point");
        if (deleteActionOverPoint.classList.contains("hidden") === false) {
            deleteActionOverPoint.classList.add("hidden");
        }

    }

    activateAddPoint() {
        const addPointContainer = this.shadowRoot.querySelector(".add-action-over-point");
        if (addPointContainer.classList.contains("hidden") === true) {
            addPointContainer.classList.remove("hidden");
        }
        const updatePointContainer = this.shadowRoot.querySelector(".update-action-over-point");
        if (updatePointContainer.classList.contains("hidden") === false) {
            updatePointContainer.classList.add("hidden");
        }
        const deletePointContainer = this.shadowRoot.querySelector(".delete-action-over-point");
        if (deletePointContainer.classList.contains("hidden") === false) {
            deletePointContainer.classList.add("hidden");
        }
    }

    deactivateAddPoint() {
        const addPointContainer = this.shadowRoot.querySelector(".add-action-over-point");
        if (addPointContainer.classList.contains("hidden") === false) {
            addPointContainer.classList.add("hidden");
        }
    }

    activateUpdatePoint() {
        const updatePointContainer = this.shadowRoot.querySelector(".update-action-over-point");
        if (updatePointContainer.classList.contains("hidden") === true) {
            updatePointContainer.classList.remove("hidden");
        }
        const addPointContainer = this.shadowRoot.querySelector(".add-action-over-point");
        if (addPointContainer.classList.contains("hidden") === false) {
            addPointContainer.classList.add("hidden");
        }
        const deletePointContainer = this.shadowRoot.querySelector(".delete-action-over-point");
        if (deletePointContainer.classList.contains("hidden") === false) {
            deletePointContainer.classList.add("hidden");
        }
    }

    deactivateUpdatePoint() {
        const updatePointContainer = this.shadowRoot.querySelector(".update-action-over-point");
        if (updatePointContainer.classList.contains("hidden") === false) {
            updatePointContainer.classList.add("hidden");
        }
    }

    activateDeletePoint() {
        const deletePointContainer = this.shadowRoot.querySelector(".delete-action-over-point");
        if (deletePointContainer.classList.contains("hidden") === true) {
            deletePointContainer.classList.remove("hidden");
        }
        const addPointContainer = this.shadowRoot.querySelector(".add-action-over-point");
        if (addPointContainer.classList.contains("hidden") === false) {
            addPointContainer.classList.add("hidden");
        }
        const updatePointContainer = this.shadowRoot.querySelector(".update-action-over-point");
        if (updatePointContainer.classList.contains("hidden") === false) {
            updatePointContainer.classList.add("hidden");
        }
    }

    deactivateDeletePoint() {
        const deletePointContainer = this.shadowRoot.querySelector(".delete-action-over-point");
        if (deletePointContainer.classList.contains("hidden") === false) {
            deletePointContainer.classList.add("hidden");
        }
    }
}

export default FeaGeometry;