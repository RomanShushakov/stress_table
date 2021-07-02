class FeaGeometryAddPointMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,             // u32;
            isFEModelLoaded: false,     // load status of wasm module "fe_model";
            points: new Map(),          // map: { number: u32, { x: f64, y: f64, z: f64}, ... };
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
                }

                .point-number-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .point-number {
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

                .point-number::-webkit-outer-spin-button,
                .point-number::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .point-number[type=number] {
                    -moz-appearance: textfield;
                }

                .point-number:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-number:focus {
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
                    <input class="point-number" type="number" step="1"/>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.addPoint());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelPointAddition());

        this.shadowRoot.querySelector(".point-number").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".point-number");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
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

    set isFEModelLoaded(value) {
        this.props.isFEModelLoaded = value;
    }

    set points(value) {
        this.props.points = value;
    }

    set addPointToClient(point) {
        this.props.points.set(point.number, {"x": point.x, "y": point.y, "z": point.z});
        this.defineNewPointNumber();
    }

    set updatePointInClient(point) {
        this.props.points.set(point.number, {"x": point.x, "y": point.y, "z": point.z});
    }

    set deletePointFromClient(point) {
        this.props.points.delete(point.number);
        this.defineNewPointNumber();
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        const frame = () => {
            this.getFEModelLoadStatus();
            if (this.props.isFEModelLoaded === true) {
                clearInterval(id);
                this.getPoints();
                this.defineNewPointNumber();
            }
        }
        const id = setInterval(frame, 10);
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

    getActionId() {
        this.dispatchEvent(new CustomEvent("getActionId", {
            bubbles: true,
            composed: true,
        }));
    }

    getFEModelLoadStatus() {
        this.dispatchEvent(new CustomEvent("getFEModelLoadStatus", {
            bubbles: true,
            composed: true,
        }));
    }

    getPoints() {
        this.dispatchEvent(new CustomEvent("getPoints", {
            bubbles: true,
            composed: true,
        }));
    }

    defineNewPointNumber() {
        let newPointNumber = 0;
        const isPointNumberInArray = (number) => number === newPointNumber;
        const sortedPointsNumbers = Array.from(this.props.points.keys()).sort((a, b) => a - b);
        do {
            newPointNumber += 1;
        } while (sortedPointsNumbers.some(isPointNumberInArray));
        this.shadowRoot.querySelector(".point-number").value = newPointNumber;
        this.shadowRoot.querySelector(".point-number").min = newPointNumber;
        this.shadowRoot.querySelector(".point-x-coord").value = 0.0;
        this.shadowRoot.querySelector(".point-y-coord").value = 0.0;
        this.shadowRoot.querySelector(".point-z-coord").value = 0.0;
    }


    addPoint() {
        const newPointNumberField = this.shadowRoot.querySelector(".point-number");
        if (newPointNumberField.value === "") {
            if (newPointNumberField.classList.contains("highlighted") === false) {
                newPointNumberField.classList.add("highlighted");
            }
        }
        const inputtedXField = this.shadowRoot.querySelector(".point-x-coord");
        if (inputtedXField.value === "") {
            if (inputtedXField.classList.contains("highlighted") === false) {
                inputtedXField.classList.add("highlighted");
            }
        }
        const inputtedYField = this.shadowRoot.querySelector(".point-y-coord");
        if (inputtedYField.value === "") {
            if (inputtedYField.classList.contains("highlighted") === false) {
                inputtedYField.classList.add("highlighted");
            }
        }
        const inputtedZField = this.shadowRoot.querySelector(".point-z-coord");
        if (inputtedZField.value === "") {
            if (inputtedZField.classList.contains("highlighted") === false) {
                inputtedZField.classList.add("highlighted");
            }
        }

        if (newPointNumberField.value === "" || inputtedXField.value === "" || 
            inputtedYField.value === "" || inputtedZField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if (this.props.points.has(parseInt(newPointNumberField.value)) === true) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The point with the same number does already exist!";
                return;
            } else {
                return;
            }
        }

        const pointCoordinatesInProps = Array.from(this.props.points.values()).find(coordinates => 
            coordinates.x == inputtedXField.value && 
            coordinates.y == inputtedYField.value && 
            coordinates.z == inputtedZField.value);
        if (pointCoordinatesInProps != null) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The point with the same coordinates does already exist!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(newPointNumberField.value) === false || this.isNumeric(inputtedXField.value) === false ||
            this.isNumeric(inputtedYField.value) === false || this.isNumeric(inputtedZField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values!";
                return;
            } else {
                return;
            }
        }

        this.getActionId();

        const message = {"add_point": {
            "actionId": this.props.actionId,
            "number": newPointNumberField.value, 
            "x":  inputtedXField.value, "y":  inputtedYField.value, "z": inputtedZField.value
        }};

        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }

    cancelPointAddition() {
        this.defineNewPointNumber();
        const newPointNumberField = this.shadowRoot.querySelector(".point-number");
        this.dropHighlight(newPointNumberField);
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

export default FeaGeometryAddPointMenu;
