class FeaGeometry extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
            points: [
                { number: 1, x: 5.9, y: 0, z: 0, },
                { number: 3, x: 5.9, y: 8.4, z: 3.2, },
                { number: 12, x: 100, y: 0, z: 0, },
            ],
            lines: [
                { number: 1, startPoint: 1, endPoint: 12, },
                { number: 5, startPoint: 3, endPoint: 1, },
            ]
        };

        this.state = {
            isGeometryActive: false,
            isPointActive: false,
            isAddPointActive: false,
            isUpdatePointActive: false,
            isDeletePointActive: false,
            isLineActive: false,
            isAddLineActive: false,
            isUpdateLineActive: false,
            isDeleteLineActive: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: flex;
                }

                .wrapper {
                    background-color: #eee;
                    display: flex;
                    align-items: center;
                    box-sizing: content-box;
                    flex-direction: column;
                    border-right: 1px solid #9a9a9a;
                    border-left: 1px solid #9a9a9a;

                }

                .geometry-container {
                    margin: 0rem;
                }

                .geometry {
                    width: 12.0rem;
                    font-family: inherit;
                    font-size: 100%;
                    line-height: 1.15;
                    margin-bottom: 0.25rem;
                    border-radius: 5px;
                    border: 2px solid #737373;
                }

                .geometry:hover {
                    background: #d2d2d2;
                }

                .point-container {
                    margin: 0rem;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }

                .actions-over-point {
                    margin: 0rem;
                }

                .add-action-over-point {
                    margin: 0rem;
                    background: #adadad;
                    border: 2px solid #737373;
                    border-radius: 5px;
                    padding: 0.5rem;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }

                .add-action-over-point-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .point {
                    width: 5.85rem;
                    font-family: inherit;
                    font-size: 100%;
                    line-height: 1.15;
                    margin-bottom: 0.25rem;
                    border-radius: 5px;
                    border: 2px solid #737373;
                }

                .point:hover {
                    background: #d2d2d2;
                }

                .select-add-action-over-point {
                    width: 3rem;
                    font-family: inherit;
                    font-size: 85%;
                    line-height: 1.15;
                    margin-bottom: 0.25rem;
                    border-radius: 5px;
                    border: 2px solid #737373;
                }

                .select-add-action-over-point:hover {
                    background: #d2d2d2;
                }

                .select-update-action-over-point {
                    width: 4.5rem;
                    font-family: inherit;
                    font-size: 85%;
                    line-height: 1.15;
                    margin-bottom: 0.25rem;
                    border-radius: 5px;
                    border: 2px solid #737373;
                }

                .select-update-action-over-point:hover {
                    background: #d2d2d2;
                }

                .select-delete-action-over-point {
                    width: 4rem;
                    font-family: inherit;
                    font-size: 85%;
                    line-height: 1.15;
                    margin-bottom: 0.25rem;
                    border-radius: 5px;
                    border: 2px solid #737373;
                }

                .select-delete-action-over-point:hover {
                    background: #d2d2d2;
                }

                .add-action-over-point-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                }

                .add-point-number {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .add-x-coord {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .add-y-coord {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .add-z-coord {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .add-action-over-point-apply-cancel-buttons-container {
                    margin: 0rem;
                }

                .point-add-action-apply {
                    width: 5rem;
                    padding: 0rem;
                }

                .point-add-action-cancel {
                    width: 5rem;
                    padding: 0rem;
                }

                .update-action-over-point {
                    margin: 0rem;
                    background: #adadad;
                    border: 2px solid #737373;
                    border-radius: 5px;
                    padding: 0.5rem;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }

                .update-action-over-point-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .search-point-number-for-update {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .update-action-over-point-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                }

                .updated-point-number {
                    margin-bottom: 0.62rem;
                }

                .update-x-coord {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .update-y-coord {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .update-z-coord {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .update-action-over-point-apply-cancel-buttons-container {
                    margin: 0rem;
                }

                .point-update-action-apply {
                    width: 5rem;
                    padding: 0rem;
                }

                .point-update-action-cancel {
                    width: 5rem;
                    padding: 0rem;
                }

                .delete-action-over-point {
                    margin: 0rem;
                    background: #adadad;
                    border: 2px solid #737373;
                    border-radius: 5px;
                    padding: 0.5rem;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }

                .delete-action-over-point-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .search-point-number-for-delete {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .delete-action-over-point-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                    width: 10.5rem;
                }

                .deleted-point-number {
                    margin-bottom: 0.62rem;
                }

                .delete-action-over-point-apply-cancel-buttons-container {
                    margin: 0rem;
                }

                .point-delete-action-apply {
                    width: 5rem;
                    padding: 0rem;
                }

                .point-delete-action-cancel {
                    width: 5rem;
                    padding: 0rem;
                }

                .line {
                    width: 5.85rem;
                    font-family: inherit;
                    font-size: 100%;
                    line-height: 1.15;
                    margin-bottom: 0.25rem;
                    border-radius: 5px;
                    border: 2px solid #737373;
                }

                .line:hover {
                    background: #d2d2d2;
                }

                .line-container {
                    margin: 0rem;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }

                .select-add-action-over-line {
                    width: 3rem;
                    font-family: inherit;
                    font-size: 85%;
                    line-height: 1.15;
                    margin-bottom: 0.25rem;
                    border-radius: 5px;
                    border: 2px solid #737373;
                }

                .select-add-action-over-line:hover {
                    background: #d2d2d2;
                }

                .select-update-action-over-line {
                    width: 4.5rem;
                    font-family: inherit;
                    font-size: 85%;
                    line-height: 1.15;
                    margin-bottom: 0.25rem;
                    border-radius: 5px;
                    border: 2px solid #737373;
                }

                .select-update-action-over-line:hover {
                    background: #d2d2d2;
                }

                .select-delete-action-over-line {
                    width: 4rem;
                    font-family: inherit;
                    font-size: 85%;
                    line-height: 1.15;
                    margin-bottom: 0.25rem;
                    border-radius: 5px;
                    border: 2px solid #737373;
                }

                .select-delete-action-over-line:hover {
                    background: #d2d2d2;
                }

                .add-action-over-line {
                    margin: 0rem;
                    background: #adadad;
                    border: 2px solid #737373;
                    border-radius: 5px;
                    padding: 0.5rem;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }

                .add-action-over-line-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .add-action-over-line-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                }

                .add-line-number {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .selected-point-number {
                    margin-bottom: 0.62rem;
                }

                .add-action-over-line-apply-cancel-buttons-container {
                    margin: 0rem;
                }

                .line-add-action-apply {
                    width: 5rem;
                    padding: 0rem;
                }

                .line-add-action-cancel {
                    width: 5rem;
                    padding: 0rem;
                }

                .update-action-over-line {
                    margin: 0rem;
                    background: #adadad;
                    border: 2px solid #737373;
                    border-radius: 5px;
                    padding: 0.5rem;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }

                .update-action-over-line-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .update-action-over-line-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                    width: 10.5rem;
                }

                .updated-line-number {
                    margin-bottom: 0.62rem;
                }

                .update-action-over-line-apply-cancel-buttons-container {
                    margin: 0rem;
                }

                .line-update-action-apply {
                    width: 5rem;
                    padding: 0rem;
                }

                .line-update-action-cancel {
                    width: 5rem;
                    padding: 0rem;
                }

                .delete-action-over-line {
                    margin: 0rem;
                    background: #adadad;
                    border: 2px solid #737373;
                    border-radius: 5px;
                    padding: 0.5rem;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }

                .delete-action-over-line-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .delete-action-over-line-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                    width: 10.5rem;
                }

                .deleted-line-number {
                    margin-bottom: 0.62rem;
                }

                .delete-action-over-line-apply-cancel-buttons-container {
                    margin: 0rem;
                }

                .line-delete-action-apply {
                    width: 5rem;
                    padding: 0rem;
                }

                .line-delete-action-cancel {
                    width: 5rem;
                    padding: 0rem;
                }

                .active {
                    background: #adadad;
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
                                <p class="add-action-over-point-fields-description">New point number:</p>
                                <input class="add-point-number" type="number" step="1"/>
                            </li>
                            <li>
                                <p class="add-action-over-point-fields-description">X coordinate:</p>
                                <input class="add-x-coord" type="number"/>
                            </li>
                            <li>
                                <p class="add-action-over-point-fields-description">Y coordinate:</p>
                                <input class="add-y-coord" type="number"/>
                            </li>
                            <li>
                                <p class="add-action-over-point-fields-description">Z coordinate:</p>
                                <input class="add-z-coord" type="number"/>
                            </li>
                        </ul>

                        <div class="add-action-over-point-apply-cancel-buttons-container">
                            <button class="point-add-action-apply">Apply</button>
                            <button class="point-add-action-cancel">Cancel</button>
                        </div> 
                        

                    </div>

                    <div class="update-action-over-point hidden">

                        <ul class="update-action-over-point-fields">
                            <li>
                                <input class="search-point-number-for-update" type="number" placeholder="Search for numbers..."/>
                                <p class="update-action-over-point-fields-description">Select point number:</p>
                                <select class="updated-point-number" size="3"></select>                          
                            </li>
                            <li>
                                <p class="update-action-over-point-fields-description">X coordinate:</p>
                                <input class="update-x-coord" type="number"/>
                            </li>
                            <li>
                                <p class="update-action-over-point-fields-description">Y coordinate:</p>
                                <input class="update-y-coord" type="number"/>
                            </li>
                            <li>
                                <p class="update-action-over-point-fields-description">Z coordinate:</p>
                                <input class="update-z-coord" type="number"/>
                            </li>
                        </ul>

                        <div class="update-action-over-point-apply-cancel-buttons-container">
                            <button class="point-update-action-apply">Apply</button>
                            <button class="point-update-action-cancel">Cancel</button>
                        </div> 

                    </div>

                    <div class="delete-action-over-point hidden">

                        <ul class="delete-action-over-point-fields">
                            <li>
                                <input class="search-point-number-for-delete" type="number" placeholder="Search for numbers..."/>
                                <p class="delete-action-over-point-fields-description">Select point number:</p>
                                <select class="deleted-point-number" size="3"></select>
                            </li>
                        </ul>

                        <div class="delete-action-over-point-apply-cancel-buttons-container">
                            <button class="point-delete-action-apply">Apply</button>
                            <button class="point-delete-action-cancel">Cancel</button>
                        </div>                       

                    </div>
                </div>

                <div class="line-container hidden">

                    <div class="actions-over-line hidden">
                        <button class="select-add-action-over-line">Add</button>
                        <button class="select-update-action-over-line">Update</button>
                        <button class="select-delete-action-over-line">Delete</button>
                    </div>

                    <div class="add-action-over-line hidden">

                        <ul class="add-action-over-line-fields">
                            <li>
                                <p class="add-action-over-line-fields-description">New line number:</p>
                                <input class="add-line-number" type="number" step="1"/>
                            </li>
                            <li>
                                <p class="add-action-over-line-fields-description">Select start point:</p>
                                <select class="selected-point-number"></select>
                            </li>
                            <li>
                                <p class="add-action-over-line-fields-description">Select end point:</p>
                                <select class="selected-point-number"></select>
                            </li>
                        </ul>

                        <div class="add-action-over-line-apply-cancel-buttons-container">
                            <button class="line-add-action-apply">Apply</button>
                            <button class="line-add-action-cancel">Cancel</button>
                        </div> 

                    </div>

                    <div class="update-action-over-line hidden">

                        <ul class="update-action-over-line-fields">
                            <li>
                                <p class="update-action-over-line-fields-description">Select line number:</p>
                                <select class="updated-line-number"></select>
                            </li>
                            <li>
                                <p class="update-action-over-line-fields-description">Change line start point:</p>
                                <select class="updated-line-number"></select>
                            </li>
                            <li>
                                <p class="update-action-over-line-fields-description">Change line end point:</p>
                                <select class="selected-point-number"></select>
                            </li>
                        </ul>

                        <div class="update-action-over-line-apply-cancel-buttons-container">
                            <button class="line-update-action-apply">Apply</button>
                            <button class="line-update-action-cancel">Cancel</button>
                        </div> 

                    </div>

                    <div class="delete-action-over-line hidden">

                        <ul class="delete-action-over-line-fields">
                            <li>
                                <p class="delete-action-over-line-fields-description">Select line number:</p>
                                <select class="deleted-line-number"></select>
                            </li>
                        </ul>

                        <div class="delete-action-over-line-apply-cancel-buttons-container">
                            <button class="line-delete-action-apply">Apply</button>
                            <button class="line-delete-action-cancel">Cancel</button>
                        </div>

                    </div>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".geometry").addEventListener("click", () => {
            this.toggleGeometry();
        });

        this.shadowRoot.querySelector(".point").addEventListener("click", () => {
            if (this.state.isPointActive === false) {
                this.isPointActive = true;
                this.isLineActive = false;
                this.activatePoint();
            } else {
                this.isPointActive = false;
                this.deactivatePoint();
            }
        });

        this.shadowRoot.querySelector(".select-add-action-over-point").addEventListener("click", () => {
            if (this.state.isAddPointActive === false) {
                this.isAddPointActive = true;
                this.isUpdatePointActive = false;
                this.isDeletePointActive = false;
                this.activateAddPoint();
            } else {
                this.isAddPointActive = false;
                this.deactivateAddPoint();
            }
        });

        this.shadowRoot.querySelector(".select-update-action-over-point").addEventListener("click", () => {
            if (this.state.isUpdatePointActive === false) {
                this.isAddPointActive = false;
                this.isUpdatePointActive = true;
                this.isDeletePointActive = false;
                this.activateUpdatePoint();
            } else {
                this.isUpdatePointActive = false;
                this.deactivateUpdatePoint();
            }
        });

        this.shadowRoot.querySelector(".select-delete-action-over-point").addEventListener("click", () => {
            if (this.state.isDeletePointActive === false) {
                this.isAddPointActive = false;
                this.isUpdatePointActive = false;
                this.isDeletePointActive = true;
                this.activateDeletePoint();
            } else {
                this.isDeletePointActive = false;
                this.deactivateDeletePoint();
            }
        });

        this.shadowRoot.querySelector(".line").addEventListener("click", () => {
            if (this.state.isLineActive === false) {
                this.isPointActive = false;
                this.isLineActive = true;
                this.activateLine();
            } else {
                this.isLineActive = false;
                this.deactivateLine();
            }
        });

        this.shadowRoot.querySelector(".select-add-action-over-line").addEventListener("click", () => {
            if (this.state.isAddLineActive === false) {
                this.isAddLineActive = true;
                this.isUpdateLineActive = false;
                this.isDeleteLineActive = false;
                this.activateAddLine();
            } else {
                this.isAddLineActive = false;
                this.deactivateAddLine();
            }
        });

        this.shadowRoot.querySelector(".select-update-action-over-line").addEventListener("click", () => {
            if (this.state.isUpdateLineActive === false) {
                this.isAddLineActive = false;
                this.isUpdateLineActive = true;
                this.isDeleteLineActive = false;
                this.activateUpdateLine();
            } else {
                this.isUpdateLineActive = false;
                this.deactivateUpdateLine();
            }
        });

        this.shadowRoot.querySelector(".select-delete-action-over-line").addEventListener("click", () => {
            if (this.state.isDeleteLineActive === false) {
                this.isAddLineActive = false;
                this.isUpdateLineActive = false;
                this.isDeleteLineActive = true;
                this.activateDeleteLine();
            } else {
                this.isDeleteLineActive = false;
                this.deactivateDeleteLine();
            }
        });

        this.shadowRoot.querySelector(".point-add-action-apply").addEventListener("click", () => this.addPoint());

        this.shadowRoot.querySelector(".point-add-action-cancel").addEventListener("click", () => this.cancelPointAddition());

        this.shadowRoot.querySelector(".updated-point-number").addEventListener("change", () => this.updatePointCoordinates());

        this.shadowRoot.querySelector(".point-update-action-apply").addEventListener("click", () => this.updatePoint());

        this.shadowRoot.querySelector(".point-update-action-cancel").addEventListener("click", () => this.cancelPointUpdateOrDelete());

        this.shadowRoot.querySelector(".point-delete-action-apply").addEventListener("click", () => this.deletePoint());

        this.shadowRoot.querySelector(".point-delete-action-cancel").addEventListener("click", () => this.cancelPointUpdateOrDelete());

        this.shadowRoot.querySelector(".search-point-number-for-update").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".search-point-number-for-update").value,
                this.shadowRoot.querySelector(".updated-point-number"));
        });

        this.shadowRoot.querySelector(".search-point-number-for-delete").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".search-point-number-for-delete").value,
                this.shadowRoot.querySelector(".deleted-point-number"));
        });
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set isGeometryActive(value) {
        this.state.isGeometryActive = value;
        const geometryButton = this.shadowRoot.querySelector(".geometry");
        this.changeButtonColor(geometryButton, this.state.isGeometryActive);

    }

    set isPointActive(value) {
        this.state.isPointActive = value;
        const pointButton = this.shadowRoot.querySelector(".point");
        this.changeButtonColor(pointButton, this.state.isPointActive);
    }

    set isAddPointActive(value) {
        this.state.isAddPointActive = value;
        const addPointButton = this.shadowRoot.querySelector(".select-add-action-over-point");
        this.changeButtonColor(addPointButton, this.state.isAddPointActive);
    }

    set isUpdatePointActive(value) {
        this.state.isUpdatePointActive = value;
        const updatePointButton = this.shadowRoot.querySelector(".select-update-action-over-point");
        this.changeButtonColor(updatePointButton, this.state.isUpdatePointActive);
    }

    set isDeletePointActive(value) {
        this.state.isDeletePointActive = value;
        const deletePointButton = this.shadowRoot.querySelector(".select-delete-action-over-point");
        this.changeButtonColor(deletePointButton, this.state.isDeletePointActive);
    }

    set isLineActive(value) {
        this.state.isLineActive = value;
        const lineButton = this.shadowRoot.querySelector(".line");
        this.changeButtonColor(lineButton, this.state.isLineActive);
    }

    set isAddLineActive(value) {
        this.state.isAddLineActive = value;
        const addLineButton = this.shadowRoot.querySelector(".select-add-action-over-line");
        this.changeButtonColor(addLineButton, this.state.isAddLineActive);
    }

    set isUpdateLineActive(value) {
        this.state.isUpdateLineActive = value;
        const updateLineButton = this.shadowRoot.querySelector(".select-update-action-over-line");
        this.changeButtonColor(updateLineButton, this.state.isUpdateLineActive);
    }

    set isDeleteLineActive(value) {
        this.state.isDeleteLineActive = value;
        const deleteLineButton = this.shadowRoot.querySelector(".select-delete-action-over-line");
        this.changeButtonColor(deleteLineButton, this.state.isDeleteLineActive);
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.defineNewPointNumber();
        if (this.props.points.length !== 0) {
            this.defineUpdateAndDeletePointNumbers();
        } else {
            this.shadowRoot.querySelector(".select-update-action-over-point").disabled = true;
            this.shadowRoot.querySelector(".select-delete-action-over-point").disabled = true;
        }
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
            this.isGeometryActive = true;
        } else {
            geometryContainer.classList.add("hidden");
            this.isGeometryActive = false;
        }
        this.deactivateGeometry();
    }

    deactivateGeometry() {
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
        const actionsOverLine = this.shadowRoot.querySelector(".actions-over-line");
        if (actionsOverLine.classList.contains("hidden") === false) {
            actionsOverLine.classList.add("hidden");
        }
        const addActionOverLine = this.shadowRoot.querySelector(".add-action-over-line");
        if (addActionOverLine.classList.contains("hidden") === false) {
            addActionOverLine.classList.add("hidden");
        }
        const updateActionOverLine = this.shadowRoot.querySelector(".update-action-over-line");
        if (updateActionOverLine.classList.contains("hidden") === false) {
            updateActionOverLine.classList.add("hidden");
        }
        const deleteActionOverLine = this.shadowRoot.querySelector(".delete-action-over-line");
        if (deleteActionOverLine.classList.contains("hidden") === false) {
            deleteActionOverLine.classList.add("hidden");
        }
        this.isPointActive = false;
        this.isAddPointActive = false;
        this.isUpdatePointActive = false;
        this.isDeletePointActive = false;
        this.isLineActive = false;
        this.isAddLineActive = false;
        this.isUpdateLineActive = false;
        this.isDeleteLineActive = false;
    }

    activatePoint() {
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
        this.isLineActive = false;
        this.isAddLineActive = false;
        this.isUpdateLineActive = false;
        this.isDeleteLineActive = false;
        this.deactivateLine();
    }

    deactivatePoint() {
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

    activateLine() {
        const lineContainer = this.shadowRoot.querySelector(".line-container");
        if (lineContainer.classList.contains("hidden") === true) {
            lineContainer.classList.remove("hidden");
        } 
        const actionsOverLine = this.shadowRoot.querySelector(".actions-over-line");
        if (actionsOverLine.classList.contains("hidden") === true) {
            actionsOverLine.classList.remove("hidden");
        } else {
            actionsOverLine.classList.add("hidden");
        }
        const addActionOverLine = this.shadowRoot.querySelector(".add-action-over-line");
        if (addActionOverLine.classList.contains("hidden") === false) {
            addActionOverLine.classList.add("hidden");
        }
        const updateActionOverLine = this.shadowRoot.querySelector(".update-action-over-line");
        if (updateActionOverLine.classList.contains("hidden") === false) {
            updateActionOverLine.classList.add("hidden");
        }
        const deleteActionOverLine = this.shadowRoot.querySelector(".delete-action-over-line");
        if (deleteActionOverLine.classList.contains("hidden") === false) {
            deleteActionOverLine.classList.add("hidden");
        }
        this.isPointActive = false;
        this.isAddPointActive = false;
        this.isUpdatePointActive = false;
        this.isDeletePointActive = false;
        this.deactivatePoint();
    }

    deactivateLine() {
        const lineContainer = this.shadowRoot.querySelector(".line-container");
        if (lineContainer.classList.contains("hidden") === false) {
            lineContainer.classList.add("hidden");
        } 
        const actionsOverLine = this.shadowRoot.querySelector(".actions-over-line");
        if (actionsOverLine.classList.contains("hidden") === false) {
            actionsOverLine.classList.add("hidden");
        }
        const addActionOverLine = this.shadowRoot.querySelector(".add-action-over-line");
        if (addActionOverLine.classList.contains("hidden") === false) {
            addActionOverLine.classList.add("hidden");
        }
        const updateActionOverLine = this.shadowRoot.querySelector(".update-action-over-line");
        if (updateActionOverLine.classList.contains("hidden") === false) {
            updateActionOverLine.classList.add("hidden");
        }
        const deleteActionOverLine = this.shadowRoot.querySelector(".delete-action-over-line");
        if (deleteActionOverLine.classList.contains("hidden") === false) {
            deleteActionOverLine.classList.add("hidden");
        }
    }

    activateAddLine() {
        const addLineContainer = this.shadowRoot.querySelector(".add-action-over-line");
        if (addLineContainer.classList.contains("hidden") === true) {
            addLineContainer.classList.remove("hidden");
        }
        const updateLineContainer = this.shadowRoot.querySelector(".update-action-over-line");
        if (updateLineContainer.classList.contains("hidden") === false) {
            updateLineContainer.classList.add("hidden");
        }
        const deleteLineContainer = this.shadowRoot.querySelector(".delete-action-over-line");
        if (deleteLineContainer.classList.contains("hidden") === false) {
            deleteLineContainer.classList.add("hidden");
        }
    }

    deactivateAddLine() {
        const addLineContainer = this.shadowRoot.querySelector(".add-action-over-line");
        if (addLineContainer.classList.contains("hidden") === false) {
            addLineContainer.classList.add("hidden");
        }
    }

    activateUpdateLine() {
        const updateLineContainer = this.shadowRoot.querySelector(".update-action-over-line");
        if (updateLineContainer.classList.contains("hidden") === true) {
            updateLineContainer.classList.remove("hidden");
        }
        const addLineContainer = this.shadowRoot.querySelector(".add-action-over-line");
        if (addLineContainer.classList.contains("hidden") === false) {
            addLineContainer.classList.add("hidden");
        }
        const deleteLineContainer = this.shadowRoot.querySelector(".delete-action-over-line");
        if (deleteLineContainer.classList.contains("hidden") === false) {
            deleteLineContainer.classList.add("hidden");
        }
    }

    deactivateUpdateLine() {
        const updateLineContainer = this.shadowRoot.querySelector(".update-action-over-line");
        if (updateLineContainer.classList.contains("hidden") === false) {
            updateLineContainer.classList.add("hidden");
        }
    }

    activateDeleteLine() {
        const deleteLineContainer = this.shadowRoot.querySelector(".delete-action-over-line");
        if (deleteLineContainer.classList.contains("hidden") === true) {
            deleteLineContainer.classList.remove("hidden");
        }
        const addLineContainer = this.shadowRoot.querySelector(".add-action-over-line");
        if (addLineContainer.classList.contains("hidden") === false) {
            addLineContainer.classList.add("hidden");
        }
        const updateLineContainer = this.shadowRoot.querySelector(".update-action-over-line");
        if (updateLineContainer.classList.contains("hidden") === false) {
            updateLineContainer.classList.add("hidden");
        }
    }

    deactivateDeleteLine() {
        const deleteLineContainer = this.shadowRoot.querySelector(".delete-action-over-line");
        if (deleteLineContainer.classList.contains("hidden") === false) {
            deleteLineContainer.classList.add("hidden");
        }
    }

    changeButtonColor(button, isActive) {
        if (isActive === true) {
            if (button.classList.contains("active") === false) {
                button.classList.add("active");
            }
        } else {
            if (button.classList.contains("active") === true) {
                button.classList.remove("active");
            }
        }
    }

    defineNewPointNumber() {
        let newPointNumber = 0;
        const isPointNumberInArray = (point) => point.number === newPointNumber;
        do {
            newPointNumber += 1;
        } while (this.props.points.some(isPointNumberInArray));
        this.shadowRoot.querySelector(".add-point-number").value = newPointNumber;
        this.shadowRoot.querySelector(".add-point-number").min = newPointNumber;
        this.shadowRoot.querySelector(".add-x-coord").value = 0.0;
        this.shadowRoot.querySelector(".add-y-coord").value = 0.0;
        this.shadowRoot.querySelector(".add-z-coord").value = 0.0;
    }

    defineUpdateAndDeletePointNumbers() {
        const pointUpdateNumberSelect = this.shadowRoot.querySelector(".updated-point-number");
        const pointDeleteNumberSelect = this.shadowRoot.querySelector(".deleted-point-number");
        for (let i = pointUpdateNumberSelect.length-1; i >= 0; i--) {
            pointUpdateNumberSelect.options[i] = null;
        }
        for (let i = pointDeleteNumberSelect.length-1; i >= 0; i--) {
            pointDeleteNumberSelect.options[i] = null;
        }
        for (let i = 0; i < this.props.points.length; i++) {
            let updateOption = document.createElement("option");
            let deleteOption = document.createElement("option");
            updateOption.value = this.props.points[i].number;
            deleteOption.value = this.props.points[i].number;
            updateOption.innerHTML = `#${this.props.points[i].number}`;
            deleteOption.innerHTML = `#${this.props.points[i].number}`;
            pointUpdateNumberSelect.appendChild(updateOption);
            pointDeleteNumberSelect.appendChild(deleteOption);  
        }
        this.shadowRoot.querySelector(".update-x-coord").value = this.props.points[0].x;
        this.shadowRoot.querySelector(".update-y-coord").value = this.props.points[0].y;
        this.shadowRoot.querySelector(".update-z-coord").value = this.props.points[0].z;
    }

    filter(keywordField, selectField) {
        for (let i = 0; i < selectField.length; i++) {
            let txt = selectField.options[i].value;
            if (txt.substring(0, keywordField.length).toLowerCase() !== keywordField.toLowerCase() && keywordField.trim() !== "") {
                selectField.options[i].style.display = "none";
            } else {
                selectField.options[i].style.display = "list-item";
            }
        }
    }

    updatePointCoordinates() {
        const selectedPointNumber = this.shadowRoot.querySelector(".updated-point-number").value;
        const pointInProps = this.props.points.find(point => point.number == selectedPointNumber);
        this.shadowRoot.querySelector(".update-x-coord").value = pointInProps.x;
        this.shadowRoot.querySelector(".update-y-coord").value = pointInProps.y;
        this.shadowRoot.querySelector(".update-z-coord").value = pointInProps.z;
    }

    addPoint() {
        const newPointNumber = this.shadowRoot.querySelector(".add-point-number").value;
        const inputtedX = this.shadowRoot.querySelector(".add-x-coord").value;
        const inputtedY = this.shadowRoot.querySelector(".add-y-coord").value;
        const inputtedZ = this.shadowRoot.querySelector(".add-z-coord").value;
        console.log(newPointNumber, inputtedX, inputtedY, inputtedZ);
    }

    cancelPointAddition() {
        this.defineNewPointNumber();
    }

    updatePoint() {
        const selectedPointNumber = this.shadowRoot.querySelector(".updated-point-number").value;
        const inputtedX = this.shadowRoot.querySelector(".update-x-coord").value;
        const inputtedY = this.shadowRoot.querySelector(".update-y-coord").value;
        const inputtedZ = this.shadowRoot.querySelector(".update-z-coord").value;
        console.log(selectedPointNumber, inputtedX, inputtedY, inputtedZ);
    }

    deletePoint() {
        const selectedPointNumber = this.shadowRoot.querySelector(".deleted-point-number").value;
        console.log(selectedPointNumber);
    }

    cancelPointUpdateOrDelete() {
        this.defineUpdateAndDeletePointNumbers();
    }
}

export default FeaGeometry;