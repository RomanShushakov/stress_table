class FeaGeometryUpdatePointMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
            points: [],
        };

        this.state = {};

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
                    margin-top: 1rem;
                    align-items: center;
                }

                .point-number-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin: 0rem;
                    align-items: center;
                }

                .point-number-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .point-number-select-filter-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .point-number-filter-label {
                    position: relative;
                }
                  
                .point-number-filter-label:before {
                    content: "";
                    position: absolute;
                    left: 0rem;
                    top: 0rem;
                    bottom: 0rem;
                    width: 0.8rem;
                    background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
                }

                .point-number-filter {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding-left: 1.3rem;
                    width: 3.5rem;
                    background-color: #3b4453;
                    border: #4a5060;
                    border-bottom: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                }

                .point-number-filter::placeholder {
                    font-size: 85%;
                }

                .point-number-filter::-webkit-outer-spin-button,
                .point-number-filter::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .point-number-filter[type=number] {
                    -moz-appearance: textfield;
                }

                .point-number-filter:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-number-filter:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-number {
                    width: 5rem;
                    margin-top: 0.5rem;
                    background-color: #3b4453;
                    border: #4a5060;
                    border-bottom: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                    -webkit-appearance: none;
                    -moz-appearance: none;
                    background: url('data:image/svg+xml,<svg width="4" height="4" viewBox="0 0 4 4" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M1 1L2 2L3 1" stroke="rgb(112, 112, 114)" stroke-width="0.5"/></svg>') right / contain no-repeat;
                }

                .point-number option {
                    background-color: #484f60;
                }

                .point-number:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-x-coord-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .point-x-coord-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .point-x-coord {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    width: 5rem;
                    background-color: #3b4453;
                    border: #4a5060;
                    border-bottom: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                }

                .point-x-coord[type=number]::-webkit-outer-spin-button,
                .point-x-coord[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .point-x-coord[type=number] {
                    -moz-appearance: textfield;
                }

                .point-x-coord:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-x-coord:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-y-coord-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .point-y-coord-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .point-y-coord {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    width: 5rem;
                    background-color: #3b4453;
                    border: #4a5060;
                    border-bottom: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                }

                .point-y-coord[type=number]::-webkit-outer-spin-button,
                .point-y-coord[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .point-y-coord[type=number] {
                    -moz-appearance: textfield;
                }

                .point-y-coord:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-y-coord:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-z-coord-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .point-z-coord-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .point-z-coord {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    width: 5rem;
                    background-color: #3b4453;
                    border: #4a5060;
                    border-bottom: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                }

                .point-z-coord[type=number]::-webkit-outer-spin-button,
                .point-z-coord[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .point-z-coord[type=number] {
                    -moz-appearance: textfield;
                }

                .point-z-coord:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-z-coord:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .apply-cancel-buttons {
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                }

                .apply-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 4rem;
                    height: 1.7rem;
                }

                .apply-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .cancel-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 4rem;
                    height: 1.7rem;
                }

                .cancel-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .analysis-info {
                    display: flex;
                    margin: 0rem;
                    padding: 0rem;
                }

                .analysis-info-message {
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 80%;
                    width: 12rem;
                }

                .highlighted {
                    box-shadow: 0rem 0.1rem 0rem #72C5FF;
                }
            </style>

            <div class=wrapper>

                <div class="point-number-field-content">
                    <p class="point-number-caption">Point number</p>
                    <div class="point-number-select-filter-content">
                        <label class="point-number-filter-label">
                            <input class="point-number-filter" type="number" placeholder="Filter..."/>
                        </label>
                        <select class="point-number"></select>
                    </div>
                </div>

                <div class="point-x-coord-field-content">
                    <p class="point-x-coord-caption">X-coordinate</p>
                    <input class="point-x-coord" type="number"/>
                </div>

                <div class="point-y-coord-field-content">
                    <p class="point-y-coord-caption">Y-coordinate</p>
                    <input class="point-y-coord" type="number"/>
                </div>

                <div class="point-z-coord-field-content">
                    <p class="point-z-coord-caption">Z-coordinate</p>
                    <input class="point-z-coord" type="number"/>
                </div>
                
                <div class="apply-cancel-buttons">
                    <button class="apply-button">Apply</button>
                    <button class="cancel-button">Cancel</button>
                </div>

                <div class="analysis-info">
                    <p class="analysis-info-message"></p>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.updatePoint());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelPointUpdate());

        this.shadowRoot.querySelector(".point-number").addEventListener("change", () => this.updatePointCoordinates());

        this.shadowRoot.querySelector(".point-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".point-number-filter").value,
                this.shadowRoot.querySelector(".point-number"));
        });

        this.shadowRoot.querySelector(".point-x-coord").addEventListener("click", () => {
            const inputtedXField = this.shadowRoot.querySelector(".point-x-coord");
            this.dropHighlight(inputtedXField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".point-y-coord").addEventListener("click", () => {
            const inputtedYField = this.shadowRoot.querySelector(".point-y-coord");
            this.dropHighlight(inputtedYField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".point-z-coord").addEventListener("click", () => {
            const inputtedZField = this.shadowRoot.querySelector(".point-z-coord");
            this.dropHighlight(inputtedZField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set addPointToClient(point) {
        this.props.points.push(point);
        this.props.points.sort((a, b) => a.number - b.number);
        this.definePointNumberOptions();
    }

    set updatePointInClient(point) {
        let pointInProps = this.props.points.find(existedPoint => existedPoint.number == point.number);
        pointInProps.x = point.x;
        pointInProps.y = point.y;
        pointInProps.z = point.z;
        this.definePointNumberOptions();
    }

    set deletePointFromClient(point) {
        let pointIndexInProps = this.props.points.findIndex(existedPoint => existedPoint.number == point.number);
        this.props.points.splice(pointIndexInProps, 1);
        this.props.points.sort((a, b) => a.number - b.number);
        this.definePointNumberOptions();
    }

    set selectPointInClient(pointNumber) {
        const pointInProps = this.props.points.find(existedPoint => existedPoint.number == pointNumber);
        const pointNumberSelect = this.shadowRoot.querySelector(".point-number");
        const pointNumberOptions = pointNumberSelect.options;
        for (let option, i = 0; option = pointNumberOptions[i]; i++) {
            if (option.value == pointNumber) {
                pointNumberSelect.selectedIndex = i;
                break;
            }
        }
        this.updatePointCoordinates();
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

    definePointNumberOptions() {
        const pointUpdateNumberSelect = this.shadowRoot.querySelector(".point-number");
        for (let i = pointUpdateNumberSelect.length - 1; i >= 0; i--) {
            pointUpdateNumberSelect.options[i] = null;
        }
        if (this.props.points.length > 0) {
            for (let i = 0; i < this.props.points.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = this.props.points[i].number;
                updateOption.innerHTML = this.props.points[i].number;
                pointUpdateNumberSelect.appendChild(updateOption);
            }
            this.shadowRoot.querySelector(".point-x-coord").value = this.props.points[0].x;
            this.shadowRoot.querySelector(".point-y-coord").value = this.props.points[0].y;
            this.shadowRoot.querySelector(".point-z-coord").value = this.props.points[0].z;
        } else {
            this.shadowRoot.querySelector(".point-x-coord").value = "";
            this.shadowRoot.querySelector(".point-y-coord").value = "";
            this.shadowRoot.querySelector(".point-z-coord").value = "";
        }
    }

    updatePointCoordinates() {
        const selectedPointNumber = this.shadowRoot.querySelector(".point-number").value;
        const pointInProps = this.props.points.find(point => point.number == selectedPointNumber);
        this.shadowRoot.querySelector(".point-x-coord").value = pointInProps.x;
        this.dropHighlight(this.shadowRoot.querySelector(".point-x-coord"));
        this.shadowRoot.querySelector(".point-y-coord").value = pointInProps.y;
        this.dropHighlight(this.shadowRoot.querySelector(".point-y-coord"));
        this.shadowRoot.querySelector(".point-z-coord").value = pointInProps.z;
        this.dropHighlight(this.shadowRoot.querySelector(".point-z-coord"));
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    filter(keywordField, selectField) {
        for (let i = 0; i < selectField.length; i++) {
            let txt = selectField.options[i].value;
            if (txt.substring(0, keywordField.length).toLowerCase() !== keywordField.toLowerCase() && 
                keywordField.trim() !== "") {
                selectField.options[i].style.display = "none";
            } else {
                selectField.options[i].style.display = "list-item";
            }
        }
    }

    updatePoint() {
        const selectedPointNumberField = this.shadowRoot.querySelector(".point-number");
        if (selectedPointNumberField.value == "") {
            if (selectedPointNumberField.classList.contains("highlighted") === false) {
                selectedPointNumberField.classList.add("highlighted");
            }
        }

        const inputtedXField = this.shadowRoot.querySelector(".point-x-coord");
        if (inputtedXField.value == "") {
            if (inputtedXField.classList.contains("highlighted") === false) {
                inputtedXField.classList.add("highlighted");
            }
        }
        const inputtedYField = this.shadowRoot.querySelector(".point-y-coord");
        if (inputtedYField.value == "") {
            if (inputtedYField.classList.contains("highlighted") === false) {
                inputtedYField.classList.add("highlighted");
            }
        }

        const inputtedZField = this.shadowRoot.querySelector(".point-z-coord");
        if (inputtedZField.value == "") {
            if (inputtedZField.classList.contains("highlighted") === false) {
                inputtedZField.classList.add("highlighted");
            }
        }

        if (selectedPointNumberField.value === "" || inputtedXField.value === "" || 
            inputtedYField.value === "" || inputtedZField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const pointCoordinatesInProps = this.props.points.find(point => point.x == inputtedXField.value && 
            point.y == inputtedYField.value && point.z == inputtedZField.value);
        if (pointCoordinatesInProps != null) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The point with the same coordinates does already exist!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(selectedPointNumberField.value) === false || this.isNumeric(inputtedXField.value) === false ||
            this.isNumeric(inputtedYField.value) === false || this.isNumeric(inputtedZField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values!";
                return;
            } else {
                return;
            }
        }

        const oldPointValues = this.props.points.find(point => point.number == selectedPointNumberField.value);
        const message = {"update_point": {
            "actionId": this.props.actionId,
            "number": selectedPointNumberField.value, 
            "old_point_values": { "x":  oldPointValues.x, "y": oldPointValues.y, "z": oldPointValues.z },
            "new_point_values": { "x": inputtedXField.value, "y": inputtedYField.value, "z": inputtedZField.value }
        }};
        this.dispatchEvent(new CustomEvent("client message", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".point-number-filter").value = null;
    }

    cancelPointUpdate() {
        if (this.props.points.length > 0) {
            this.definePointNumberOptions();
        }
        this.shadowRoot.querySelector(".point-number-filter").value = null;
        const selectedPointNumberForUpdateField = this.shadowRoot.querySelector(".point-number");
        this.dropHighlight(selectedPointNumberForUpdateField);
        const inputtedXField = this.shadowRoot.querySelector(".point-x-coord");
        this.dropHighlight(inputtedXField);
        const inputtedYField = this.shadowRoot.querySelector(".point-y-coord");
        this.dropHighlight(inputtedYField);
        const inputtedZField = this.shadowRoot.querySelector(".point-z-coord");
        this.dropHighlight(inputtedZField);
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }

    isNumeric(str) {
        if (typeof str != "string") {
            return false;
        }
        return !isNaN(str) && !isNaN(parseFloat(str));
      }
}

export default FeaGeometryUpdatePointMenu;
