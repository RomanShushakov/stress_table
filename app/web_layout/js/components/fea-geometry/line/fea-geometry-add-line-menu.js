class FeaGeometryAddLineMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,             // u32;
            isGeometryLoaded: false,    // load status of wasm module "geometry";
            points: new Map(),          // map: { number: u32, { x: f64, y: f64, z: f64}, ... };
            lines: new Map(),           // map: { number: u32, start_point_number: u32, end_point_number: u32 }, ...};
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

                .line-number-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin: 0rem;
                }

                .line-number-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .line-number {
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

                .line-number::-webkit-outer-spin-button,
                .line-number::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .line-number[type=number] {
                    -moz-appearance: textfield;
                }

                .line-number:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .line-number:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .start-point-number-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    align-items: center;
                }

                .start-point-number-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .start-point-number-select-filter-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .start-point-number-filter-label {
                    position: relative;
                }
                  
                .start-point-number-filter-label:before {
                    content: "";
                    position: absolute;
                    left: 0rem;
                    top: 0rem;
                    bottom: 0rem;
                    width: 0.8rem;
                    background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
                }

                .start-point-number-filter {
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

                .start-point-number-filter::placeholder {
                    font-size: 85%;
                }

                .start-point-number-filter::-webkit-outer-spin-button,
                .start-point-number-filter::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .start-point-number-filter[type=number] {
                    -moz-appearance: textfield;
                }

                .start-point-number-filter:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .start-point-number-filter:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .start-point-number {
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

                .start-point-number option {
                    background-color: #484f60;
                }

                .start-point-number:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .end-point-number-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    align-items: center;
                }

                .end-point-number-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .end-point-number-select-filter-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .end-point-number-filter-label {
                    position: relative;
                }
                  
                .end-point-number-filter-label:before {
                    content: "";
                    position: absolute;
                    left: 0rem;
                    top: 0rem;
                    bottom: 0rem;
                    width: 0.8rem;
                    background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
                }

                .end-point-number-filter {
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

                .end-point-number-filter::placeholder {
                    font-size: 85%;
                }

                .end-point-number-filter::-webkit-outer-spin-button,
                .end-point-number-filter::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .end-point-number-filter[type=number] {
                    -moz-appearance: textfield;
                }

                .end-point-number-filter:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .end-point-number-filter:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .end-point-number {
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

                .end-point-number option {
                    background-color: #484f60;
                }

                .end-point-number:hover {
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
                <div class="line-number-field-content">
                    <p class="line-number-caption">Line number</p>
                    <input class="line-number" type="number" step="1"/>
                </div>

                <div class="start-point-number-field-content">
                    <p class="start-point-number-caption">Start point number</p>
                    <div class="start-point-number-select-filter-content">
                        <label class="start-point-number-filter-label">
                            <input class="start-point-number-filter" type="number" placeholder="Filter..."/>
                        </label>
                        <select class="start-point-number"></select>
                    </div>
                </div>

                <div class="end-point-number-field-content">
                    <p class="end-point-number-caption">End point number</p>
                    <div class="end-point-number-select-filter-content">
                        <label class="end-point-number-filter-label">
                            <input class="end-point-number-filter" type="number" placeholder="Filter..."/>
                        </label>
                        <select class="end-point-number"></select>
                    </div>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.addLine());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelLineAddition());

        this.shadowRoot.querySelector(".line-number").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".line-number");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".start-point-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".start-point-number-filter").value,
                this.shadowRoot.querySelector(".start-point-number"));
        });

        this.shadowRoot.querySelector(".start-point-number").addEventListener("change", 
            () => this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "");

        this.shadowRoot.querySelector(".end-point-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".end-point-number-filter").value,
                this.shadowRoot.querySelector(".end-point-number"));
        });

        this.shadowRoot.querySelector(".end-point-number").addEventListener("change", 
            () => this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "");
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set isGeometryLoaded(value) {
        this.props.isGeometryLoaded = value;
    }

    set points(value) {
        this.props.points = value;
    }

    set lines(value) {
        this.props.lines = value;
    }

    set addPointToClient(point) {
        this.props.points.set(point.number, {"x": point.x, "y": point.y, "z": point.z});
        this.defineStartPointNumberOptions();
        this.defineEndPointNumberOptions();
    }

    set deletePointFromClient(point) {
        this.props.points.delete(point.number);
        this.defineStartPointNumberOptions();
        this.defineEndPointNumberOptions();
    }

    set addLineToClient(line) {
        this.props.lines.set(line.number, { "start_point_number": line.start_point_number, "end_point_number": line.end_point_number });
        this.defineNewLineNumber();
        this.defineStartPointNumberOptions();
        this.defineEndPointNumberOptions();
    }

    set updateLineInClient(line) {
        this.props.lines.set(line.number, { "start_point_number": line.start_point_number, "end_point_number": line.end_point_number });
    }

    set deleteLineFromClient(line) {
        this.props.lines.delete(line.number);
        this.defineNewLineNumber();
        this.defineStartPointNumberOptions();
        this.defineEndPointNumberOptions();
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
            this.getGeometryLoadStatus();
            if (this.props.isGeometryLoaded === true) {
                clearInterval(id);
                this.getPoints();
                this.getLines();
                this.defineStartPointNumberOptions();
                this.defineEndPointNumberOptions();
                this.defineNewLineNumber();
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

    getGeometryLoadStatus() {
        this.dispatchEvent(new CustomEvent("getGeometryLoadStatus", {
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

    getLines() {
        this.dispatchEvent(new CustomEvent("getLines", {
            bubbles: true,
            composed: true,
        }));
    }

    defineNewLineNumber() {

        let newLineNumber = 0;
        const isLineNumberInArray = (number) => number === newLineNumber;
        const sortedLinesNumbers = Array.from(this.props.lines.keys()).sort((a, b) => a - b);
        do {
            newLineNumber += 1;
        } while (sortedLinesNumbers.some(isLineNumberInArray));
        this.shadowRoot.querySelector(".line-number").value = newLineNumber;
        this.shadowRoot.querySelector(".line-number").min = newLineNumber;
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

    defineStartPointNumberOptions() {
        const startPointNumberSelect = this.shadowRoot.querySelector(".start-point-number");
        for (let i = startPointNumberSelect.length - 1; i >= 0; i--) {
            startPointNumberSelect.options[i] = null;
        }
        if (this.props.points.size > 0) {
            const pointsNumbers = Array.from(this.props.points.keys()).sort((a, b) => a - b);
            for (let i = 0; i < pointsNumbers.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = pointsNumbers[i];
                updateOption.innerHTML = pointsNumbers[i];
                startPointNumberSelect.appendChild(updateOption);
            }
        }
    }

    defineEndPointNumberOptions() {
        const endPointNumberSelect = this.shadowRoot.querySelector(".end-point-number");
        for (let i = endPointNumberSelect.length - 1; i >= 0; i--) {
            endPointNumberSelect.options[i] = null;
        }
        if (this.props.points.size > 0) {
            const pointsNumbers = Array.from(this.props.points.keys()).sort((a, b) => a - b);
            for (let i = 0; i < pointsNumbers.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = pointsNumbers[i];
                updateOption.innerHTML = pointsNumbers[i];
                endPointNumberSelect.appendChild(updateOption);
            }
        }
    }

    addLine() {
        const newLineNumberField = this.shadowRoot.querySelector(".line-number");
        if (newLineNumberField.value == "") {
            if (newLineNumberField.classList.contains("highlighted") === false) {
                newLineNumberField.classList.add("highlighted");
            }
        }
        const startPointField = this.shadowRoot.querySelector(".start-point-number");
        if (startPointField.value == "") {
            if (startPointField.classList.contains("highlighted") === false) {
                startPointField.classList.add("highlighted");
            }
        }
        const endPointField = this.shadowRoot.querySelector(".end-point-number");
        if (endPointField.value == "") {
            if (endPointField.classList.contains("highlighted") === false) {
                endPointField.classList.add("highlighted");
            }
        }
        if (newLineNumberField.value == "" || startPointField.value == "" || endPointField.value == "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }
        if (this.props.lines.has(parseInt(newLineNumberField.value)) === true) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The line with the same number does already exist!";
                return;
            } else {
                return;
            }
        }
        const linePointNumbersInProps = Array.from(this.props.lines.values()).find(lineNumbers => 
            (lineNumbers.start_point_number == startPointField.value && lineNumbers.end_point_number == endPointField.value) ||
            (lineNumbers.start_point_number == endPointField.value && lineNumbers.end_point_number == startPointField.value));
        if (linePointNumbersInProps != null) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The line with the same start and end points does already exist!";
                return;
            } else {
                return;
            }
        }

        if (startPointField.value === endPointField.value) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The start and the end point numbers should not be the same!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(newLineNumberField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values!";
                return;
            } else {
                return;
            }
        }

        this.getActionId();

        const message = {"add_line": {
            "actionId": this.props.actionId,
            "number": newLineNumberField.value, 
            "start_point_number": startPointField.value, "end_point_number": endPointField.value
        }};

        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".start-point-number-filter").value = null;
        this.shadowRoot.querySelector(".end-point-number-filter").value = null;
    }

    cancelLineAddition() {
        this.defineNewLineNumber();
        this.defineStartPointNumberOptions();
        this.defineEndPointNumberOptions();
        this.shadowRoot.querySelector(".start-point-number-filter").value = null;
        this.shadowRoot.querySelector(".end-point-number-filter").value = null;
        const newLineNumberField = this.shadowRoot.querySelector(".line-number");
        this.dropHighlight(newLineNumberField);
        const startPointField = this.shadowRoot.querySelector(".start-point-number");
        this.dropHighlight(startPointField);
        const endPointField = this.shadowRoot.querySelector(".end-point-number");
        this.dropHighlight(endPointField);
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

export default FeaGeometryAddLineMenu;
