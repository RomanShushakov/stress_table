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
            buttonNames: {
                geometry: "Geometry", point:  "Point", line: "Line",
                pointAdd: "Add", pointUpdate: "Update", pointDelete: "Delete",
                lineAdd: "Add", lineUpdate: "Update", lineDelete: "Delete",
            },
            buttonFullNames: {
                geometry: "geometry", point:  "point", line: "line",
                pointAdd: "pointAdd", pointUpdate: "pointUpdate", pointDelete: "pointDelete",
                lineAdd: "lineAdd", lineUpdate: "lineUpdate", lineDelete: "lineDelete",
            }
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


                .add-action-over-point-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
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

                .update-action-over-point-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .update-action-over-point-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                }

                .search-point-number-for-update {
                    width: 10rem;
                    margin-bottom: 0.5rem;
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

                .delete-action-over-point-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .delete-action-over-point-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                    width: 10.5rem;
                }

                .search-point-number-for-delete {
                    width: 10rem;
                    margin-bottom: 0.5rem;
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

                .search-start-point-number-for-line-addition {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .search-end-point-number-for-line-addition {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .add-line-number {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .search-line-number-for-update {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .selected-start-point-number-for-line-addition {
                    margin-bottom: 0.62rem;
                }

                .selected-end-point-number-for-line-addition {
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

                .search-start-point-number-for-line-update {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .search-end-point-number-for-line-update {
                    width: 10rem;
                    margin-bottom: 0.5rem;
                }

                .updated-line-number {
                    margin-bottom: 0.62rem;
                }

                .selected-start-point-number-for-line-update {
                    margin-bottom: 0.62rem;
                }

                .selected-end-point-number-for-line-delete {
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
            </style>

            <div class="wrapper">
                <hiding-content-button 
                    class=geometry
                    name=${this.state.buttonNames.geometry}
                    full-name=${this.state.buttonFullNames.geometry}
                    content-position=relative
                    content-direction=row
                    button-width=12rem
                    button-font-size=100%
                >
                    <hiding-content-button 
                        class=point
                        name=${this.state.buttonNames.point} 
                        full-name=${this.state.buttonFullNames.point}
                        content-position=absolute
                        content-direction=row
                        content-left=0rem
                        button-width=5.85rem
                        button-font-size=100%
                        button-margin-right=0.25rem
                    >
                        <hiding-content-button 
                            class=point-add
                            name=${this.state.buttonNames.pointAdd}
                            full-name=${this.state.buttonFullNames.pointAdd}
                            content-position=absolute
                            content-direction=column
                            content-left=0rem
                            button-width=3rem
                            button-font-size=85%
                            button-margin-right=0.25rem
                            content-background=#adadad
                            content-border="2px solid #737373"
                            content-padding=0.5rem
                        >
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
                        </hiding-content-button>
                        <hiding-content-button 
                            class=point-update
                            name=${this.state.buttonNames.pointUpdate} 
                            full-name=${this.state.buttonFullNames.pointUpdate}
                            content-position=absolute
                            content-direction=column
                            content-left=0rem
                            button-width=4.5rem
                            button-font-size=85%
                            button-margin-right=0.25rem
                            content-background=#adadad
                            content-border="2px solid #737373"
                            content-padding=0.5rem
                        >
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
                        </hiding-content-button>
                        <hiding-content-button
                            class=point-delete
                            name=${this.state.buttonNames.pointDelete} 
                            full-name=${this.state.buttonFullNames.pointDelete}
                            content-position=absolute 
                            content-direction=column
                            content-left=0rem
                            button-width=4rem
                            button-font-size=85%
                            content-background=#adadad
                            content-border="2px solid #737373"
                            content-padding=0.5rem
                        >
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
                        </hiding-content-button>
                    </hiding-content-button>
                    <hiding-content-button
                        class=line
                        name=${this.state.buttonNames.line}
                        full-name=${this.state.buttonFullNames.line}
                        content-position=absolute
                        content-direction=row
                        content-left=0rem
                        button-width=5.85rem
                        button-font-size=100%
                    >
                        <hiding-content-button 
                            class=line-add
                            name=${this.state.buttonNames.lineAdd}
                            full-name=${this.state.buttonFullNames.lineAdd}
                            content-position=absolute
                            content-direction=column
                            content-left=0rem
                            button-width=3rem
                            button-font-size=85%
                            button-margin-right=0.25rem
                            content-background=#adadad
                            content-border="2px solid #737373"
                            content-padding=0.5rem
                        >
                            <ul class="add-action-over-line-fields">
                                <li>
                                    <p class="add-action-over-line-fields-description">New line number:</p>
                                    <input class="add-line-number" type="number" step="1"/>
                                </li>
                                <li>
                                    <input class="search-start-point-number-for-line-addition" type="number" placeholder="Search for start point..."/>
                                    <p class="add-action-over-line-fields-description">Select start point:</p>
                                    <select class="selected-start-point-number-for-line-addition" size="3"></select>
                                </li>
                                <li>
                                    <input class="search-end-point-number-for-line-addition" type="number" placeholder="Search for end point..."/>
                                    <p class="add-action-over-line-fields-description">Select end point:</p>
                                    <select class="selected-end-point-number-for-line-addition" size="3"></select>
                                </li>
                            </ul>
                            <div class="add-action-over-line-apply-cancel-buttons-container">
                                <button class="line-add-action-apply">Apply</button>
                                <button class="line-add-action-cancel">Cancel</button>
                            </div> 
                        </hiding-content-button>
                        <hiding-content-button 
                            class=line-update
                            name=${this.state.buttonNames.lineUpdate}
                            full-name=${this.state.buttonFullNames.lineUpdate}
                            content-position=absolute 
                            content-direction=column
                            content-left=0rem
                            button-width=4.5rem
                            button-font-size=85%
                            button-margin-right=0.25rem
                            content-background=#adadad
                            content-border="2px solid #737373"
                            content-padding=0.5rem
                        >
                            <ul class="update-action-over-line-fields">
                                <li>
                                    <input class="search-line-number-for-update" type="number" placeholder="Search for numbers..."/>
                                    <p class="update-action-over-line-fields-description">Select line number:</p>
                                    <select class="updated-line-number" size="3"></select>
                                </li>
                                <li>
                                    <input class="search-start-point-number-for-line-update" type="number" placeholder="Search for start point..."/>
                                    <p class="update-action-over-line-fields-description">Change line start point:</p>
                                    <select class="selected-start-point-number-for-line-update" size="3"></select>
                                </li>
                                <li>
                                    <input class="search-end-point-number-for-line-update" type="number" placeholder="Search for start point..."/>
                                    <p class="update-action-over-line-fields-description">Change line end point:</p>
                                    <select class="selected-end-point-number-for-line-update" size="3"></select>
                                </li>
                            </ul>
                            <div class="update-action-over-line-apply-cancel-buttons-container">
                                <button class="line-update-action-apply">Apply</button>
                                <button class="line-update-action-cancel">Cancel</button>
                            </div> 
                        </hiding-content-button>
                        <hiding-content-button 
                            class=line-delete
                            name=${this.state.buttonNames.lineDelete}
                            full-name=${this.state.buttonFullNames.lineDelete}
                            content-position=absolute
                            content-direction=column 
                            content-left=0rem
                            button-width=4rem
                            button-font-size=85%
                            content-background=#adadad
                            content-border="2px solid #737373"
                            content-padding=0.5rem
                        >
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
                        </hiding-content-button>
                    </hiding-content-button>
                </hiding-content-button> 
            </div>
        `;
        this.addEventListener("hide siblings", (event) => this.hideSiblings(event));

        this.addEventListener("update content height", (event) => this.updateContentHeight(event));

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

        this.shadowRoot.querySelector(".search-start-point-number-for-line-addition").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".search-start-point-number-for-line-addition").value,
                this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition"));
        });

        this.shadowRoot.querySelector(".search-end-point-number-for-line-addition").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".search-end-point-number-for-line-addition").value,
                this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition"));
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
        this.defineNewPointNumber();
        if (this.props.points.length !== 0) {
            this.defineUpdateAndDeletePointNumbers();
            if (this.props.points.length < 2) {
                // this.shadowRoot.querySelector(".line").disabled = true;
            } else {
                this.defineNewLineNumber();
                const lineStartPointNumberForAddition = this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition");
                const lineEndPointNumberForAddition = this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition");
                this.definePointNumbersForLine(lineStartPointNumberForAddition, lineEndPointNumberForAddition);
                if (this.props.lines.length !== 0) {
                    this.defineUpdateAndDeleteLineNumbers();
                } else {
                    // this.shadowRoot.querySelector(".select-update-action-over-line").disabled = true;
                    // this.shadowRoot.querySelector(".select-delete-action-over-line").disabled = true;
                }
            }
        } else {
            // this.shadowRoot.querySelector(".select-update-action-over-point").disabled = true;
            // this.shadowRoot.querySelector(".select-delete-action-over-point").disabled = true;
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

    hideSiblings(event) {
        switch (event.detail.from) {
        case "geometry":
            this.shadowRoot.querySelector(".point").deactivate = "";
            this.shadowRoot.querySelector(".point-add").deactivate = "";
            this.shadowRoot.querySelector(".point-update").deactivate = "";
            this.shadowRoot.querySelector(".point-delete").deactivate = "";
            this.shadowRoot.querySelector(".line").deactivate = "";
            this.shadowRoot.querySelector(".line-add").deactivate = "";
            this.shadowRoot.querySelector(".line-update").deactivate = "";
            this.shadowRoot.querySelector(".line-delete").deactivate = "";
            break;
        case "point":
            this.shadowRoot.querySelector(".line").deactivate = "";
            this.shadowRoot.querySelector(".line-add").deactivate = "";
            this.shadowRoot.querySelector(".line-update").deactivate = "";
            this.shadowRoot.querySelector(".line-delete").deactivate = "";
            break;
        case "pointAdd":
            this.shadowRoot.querySelector(".point-update").deactivate = "";
            this.shadowRoot.querySelector(".point-delete").deactivate = "";
            break;
        case "pointUpdate":
            this.shadowRoot.querySelector(".point-add").deactivate = "";
            this.shadowRoot.querySelector(".point-delete").deactivate = "";
            break;
        case "pointDelete":
            this.shadowRoot.querySelector(".point-add").deactivate = "";
            this.shadowRoot.querySelector(".point-update").deactivate = "";
            break;
        case "line":
            this.shadowRoot.querySelector(".point").deactivate = "";
            this.shadowRoot.querySelector(".point-add").deactivate = "";
            this.shadowRoot.querySelector(".point-update").deactivate = "";
            this.shadowRoot.querySelector(".point-delete").deactivate = "";
            break;
        case "lineAdd":
            this.shadowRoot.querySelector(".line-update").deactivate = "";
            this.shadowRoot.querySelector(".line-delete").deactivate = "";
            break;
        case "lineUpdate":
            this.shadowRoot.querySelector(".line-add").deactivate = "";
            this.shadowRoot.querySelector(".line-delete").deactivate = "";
            break;
        case "lineDelete":
            this.shadowRoot.querySelector(".line-add").deactivate = "";
            this.shadowRoot.querySelector(".line-update").deactivate = "";
            break;
        default:
            console.log("Sorry, we are out of button full names.");
        }
    }

    findAdditionalHeight(event) {
        switch (event.detail.from) {
            case "pointAdd":
            case "pointUpdate":
            case "pointDelete":
            case "lineAdd":
            case "lineUpdate":
            case "lineDelete":
                return 25;
        }
        return 0;
    }

    updateContentHeight(event) {
        const additionalHeight = this.findAdditionalHeight(event);
        const contentTotalHeight = 
            // event.detail.height + 
            this.shadowRoot.querySelector(".geometry").offsetHeight +
            this.shadowRoot.querySelector(".point-add").offsetHeight + 
            this.shadowRoot.querySelector(".line-add").offsetHeight + 
            this.shadowRoot.querySelector(".add-action-over-point-fields").offsetHeight + 
            this.shadowRoot.querySelector(".add-action-over-point-apply-cancel-buttons-container").offsetHeight +
            this.shadowRoot.querySelector(".update-action-over-point-fields").offsetHeight + 
            this.shadowRoot.querySelector(".update-action-over-point-apply-cancel-buttons-container").offsetHeight +
            this.shadowRoot.querySelector(".delete-action-over-point-fields").offsetHeight +
            this.shadowRoot.querySelector(".delete-action-over-point-apply-cancel-buttons-container").offsetHeight +
            this.shadowRoot.querySelector(".add-action-over-line-fields").offsetHeight + 
            this.shadowRoot.querySelector(".add-action-over-line-apply-cancel-buttons-container").offsetHeight +
            this.shadowRoot.querySelector(".update-action-over-line-fields").offsetHeight + 
            this.shadowRoot.querySelector(".update-action-over-line-apply-cancel-buttons-container").offsetHeight +
            this.shadowRoot.querySelector(".delete-action-over-line-fields").offsetHeight +
            this.shadowRoot.querySelector(".delete-action-over-line-apply-cancel-buttons-container").offsetHeight + 
            additionalHeight;
        this.shadowRoot.querySelector(".wrapper").setAttribute("style", `height: ${contentTotalHeight}px;`);
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
        for (let i = pointUpdateNumberSelect.length - 1; i >= 0; i--) {
            pointUpdateNumberSelect.options[i] = null;
        }
        for (let i = pointDeleteNumberSelect.length - 1; i >= 0; i--) {
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

    defineNewLineNumber() {
        let newLineNumber = 0;
        const isLineNumberInArray = (line) => line.number === newLineNumber;
        do {
            newLineNumber += 1;
        } while (this.props.points.some(isLineNumberInArray));
        this.shadowRoot.querySelector(".add-line-number").value = newLineNumber;
        this.shadowRoot.querySelector(".add-line-number").min = newLineNumber;
    }

    definePointNumbersForLine(startPointNumberSelector, endPointNumberSelector) {
        for (let i = startPointNumberSelector.length - 1; i >= 0; i--) {
            startPointNumberSelector.options[i] = null;
        }
        for (let i = endPointNumberSelector.length - 1; i >= 0; i--) {
            endPointNumberSelector.options[i] = null;
        }
        for (let i = 0; i < this.props.points.length; i++) {
            let updateOption = document.createElement("option");
            let deleteOption = document.createElement("option");
            updateOption.value = this.props.points[i].number;
            deleteOption.value = this.props.points[i].number;
            updateOption.innerHTML = `#${this.props.points[i].number}`;
            deleteOption.innerHTML = `#${this.props.points[i].number}`;
            startPointNumberSelector.appendChild(updateOption);
            endPointNumberSelector.appendChild(deleteOption);  
        }
    }

    defineUpdateAndDeleteLineNumbers() {
        const lineUpdateNumberSelect = this.shadowRoot.querySelector(".updated-line-number");
        const lineDeleteNumberSelect = this.shadowRoot.querySelector(".deleted-line-number");
        for (let i = lineUpdateNumberSelect.length - 1; i >= 0; i--) {
            lineUpdateNumberSelect.options[i] = null;
        }
        for (let i = lineDeleteNumberSelect.length - 1; i >= 0; i--) {
            lineDeleteNumberSelect.options[i] = null;
        }
        for (let i = 0; i < this.props.lines.length; i++) {
            let updateOption = document.createElement("option");
            let deleteOption = document.createElement("option");
            updateOption.value = this.props.lines[i].number;
            deleteOption.value = this.props.lines[i].number;
            updateOption.innerHTML = `#${this.props.lines[i].number}`;
            deleteOption.innerHTML = `#${this.props.lines[i].number}`;
            lineUpdateNumberSelect.appendChild(updateOption);
            lineDeleteNumberSelect.appendChild(deleteOption);  
        }
        // this.shadowRoot.querySelector(".update-x-coord").value = this.props.points[0].x;
        // this.shadowRoot.querySelector(".update-y-coord").value = this.props.points[0].y;
        // this.shadowRoot.querySelector(".update-z-coord").value = this.props.points[0].z;
    }
}

export default FeaGeometry;