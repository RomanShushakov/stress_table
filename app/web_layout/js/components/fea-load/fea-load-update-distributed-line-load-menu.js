class FeaLoadUpdateDistributedLineLoadMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,                     // u32;
            isFEModelLoaded: false,             // load status of wasm module "fe_model";
            distributedLineLoads: new Map(),    // map: { line_number: u32, { qx: f64, qy: f64, qz: f64 }, ... };
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
                    align-items: center;
                }

                .line-number-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .line-number-select-filter-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .line-number-filter-label {
                    position: relative;
                }
                  
                .line-number-filter-label:before {
                    content: "";
                    position: absolute;
                    left: 0rem;
                    top: 0rem;
                    bottom: 0rem;
                    width: 0.8rem;
                    background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
                }

                .line-number-filter {
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

                .line-number-filter::placeholder {
                    font-size: 85%;
                }

                .line-number-filter::-webkit-outer-spin-button,
                .line-number-filter::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .line-number-filter[type=number] {
                    -moz-appearance: textfield;
                }

                .line-number-filter:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .line-number-filter:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .line-number {
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

                .line-number option {
                    background-color: #484f60;
                }

                .line-number:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .qx-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .qx-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .qx {
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

                .qx[type=number]::-webkit-outer-spin-button,
                .qx[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .qx[type=number] {
                    -moz-appearance: textfield;
                }

                .qx:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .qx:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .qy-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .qy-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .qy {
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

                .qy[type=number]::-webkit-outer-spin-button,
                .qy[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .qy[type=number] {
                    -moz-appearance: textfield;
                }

                .qy:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .qy:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .qz-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .qz-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .qz {
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

                .qz[type=number]::-webkit-outer-spin-button,
                .qz[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .qz[type=number] {
                    -moz-appearance: textfield;
                }

                .qz:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .qz:focus {
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
                    <div class="line-number-select-filter-content">
                        <label class="line-number-filter-label">
                            <input class="line-number-filter" type="number" placeholder="Filter..."/>
                        </label>
                        <select class="line-number"></select>
                    </div>
                </div>

                <div class="qx-field-content">
                    <p class="qx-caption">Qx</p>
                    <input class="qx" type="number"/>
                </div>

                <div class="qy-field-content">
                    <p class="qy-caption">Qy</p>
                    <input class="qy" type="number"/>
                </div>

                <div class="qz-field-content">
                    <p class="qz-caption">Qz</p>
                    <input class="qz" type="number"/>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.updateDistributedLineLoad());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelDistributedLineLoadUpdate());

        this.shadowRoot.querySelector(".line-number").addEventListener("change", () => this.updateDistributedLineLoadValues());

        this.shadowRoot.querySelector(".line-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".line-number-filter").value,
                this.shadowRoot.querySelector(".line-number"));
        });

        this.shadowRoot.querySelector(".qx").addEventListener("click", () => {
            const inputtedQXField = this.shadowRoot.querySelector(".qx");
            this.dropHighlight(inputtedQXField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".qy").addEventListener("click", () => {
            const inputtedQYField = this.shadowRoot.querySelector(".qy");
            this.dropHighlight(inputtedQYField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".qz").addEventListener("click", () => {
            const inputtedQZField = this.shadowRoot.querySelector(".qz");
            this.dropHighlight(inputtedQZField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set isFEModelLoaded(value) {
        this.props.isFEModelLoaded = value;
    }

    set distributedLineLoads(value) {
        this.props.distributedLineLoads = value;
    }

    set addDistributedLineLoadToClient(distributedLineLoad) {
        this.props.distributedLineLoads.set(distributedLineLoad.line_number, 
            {
                "qx": distributedLineLoad.qx, "qy": distributedLineLoad.qy, "qz": distributedLineLoad.qz
            });
        this.defineDistributedLineLoadOptions();
    }

    set updateDistributedLineLoadInClient(distributedLineLoad) {
        this.props.distributedLineLoads.set(distributedLineLoad.line_number, 
            {
                "qx": distributedLineLoad.qx, "qy": distributedLineLoad.qy, "qz": distributedLineLoad.qz
            });
        this.defineDistributedLineLoadOptions();
    }

    set deleteDistributedLineLoadFromClient(distributedLineLoad) {
        this.props.distributedLineLoads.delete(distributedLineLoad.line_number);
        this.defineDistributedLineLoadOptions();
    }

    set feModelError(error) {
        if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = error;
        }
    }

    set selectDistributedLineLoadInClient(lineNumber) {
        const frame = () => {
            if (this.props.isFEModelLoaded === true) {
                clearInterval(id);
                const lineNumberSelect = this.shadowRoot.querySelector(".line-number");
                const lineNumberOptions = lineNumberSelect.options;
                for (let option, i = 0; option = lineNumberOptions[i]; i++) {
                    if (option.value == lineNumber) {
                        lineNumberSelect.selectedIndex = i;
                        break;
                    }
                }
                this.updateDistributedLineLoadValues();
            }
        }
        const id = setInterval(frame, 10);
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
                this.getDistributedLineLoads();
                this.defineDistributedLineLoadOptions();
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

    getDistributedLineLoads() {
        this.dispatchEvent(new CustomEvent("getDistributedLineLoads", {
            bubbles: true,
            composed: true,
        }));
    }

    defineDistributedLineLoadOptions() {
        const lineNumberSelect = this.shadowRoot.querySelector(".line-number");
        for (let i = lineNumberSelect.length - 1; i >= 0; i--) {
            lineNumberSelect.options[i] = null;
        }
        if (this.props.distributedLineLoads.size > 0) {
            const linesNumbers = Array.from(this.props.distributedLineLoads.keys()).sort((a, b) => a - b);
            for (let i = 0; i < linesNumbers.length; i++) {
                let lineNumberOption = document.createElement("option");
                lineNumberOption.value = linesNumbers[i];
                lineNumberOption.innerHTML = linesNumbers[i];
                lineNumberSelect.appendChild(lineNumberOption);
            }

            if (this.props.distributedLineLoads.get(linesNumbers[0]) !== undefined) {
                this.shadowRoot.querySelector(".qx").value = this.props.distributedLineLoads.get(linesNumbers[0]).qx;
                this.shadowRoot.querySelector(".qy").value = this.props.distributedLineLoads.get(linesNumbers[0]).qy;
                this.shadowRoot.querySelector(".qz").value = this.props.distributedLineLoads.get(linesNumbers[0]).qz;
            } else {
                this.shadowRoot.querySelector(".qx").value = "";
                this.shadowRoot.querySelector(".qy").value = "";
                this.shadowRoot.querySelector(".qz").value = "";
            }

        } else {
            this.shadowRoot.querySelector(".qx").value = "";
            this.shadowRoot.querySelector(".qy").value = "";
            this.shadowRoot.querySelector(".qz").value = "";
        }
    }

    updateDistributedLineLoadValues() {
        const selectedLineNumber = this.shadowRoot.querySelector(".point-number").value;
        this.shadowRoot.querySelector(".qx").value = this.props.distributedLineLoads.get(parseInt(selectedLineNumber)).qx;
        this.dropHighlight(this.shadowRoot.querySelector(".qx"));
        this.shadowRoot.querySelector(".qy").value = this.props.distributedLineLoads.get(parseInt(selectedLineNumber)).qy;
        this.dropHighlight(this.shadowRoot.querySelector(".qy"));
        this.shadowRoot.querySelector(".qz").value = this.props.distributedLineLoads.get(parseInt(selectedLineNumber)).qz;
        this.dropHighlight(this.shadowRoot.querySelector(".qz"));
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

    updateDistributedLineLoad() {
        const selectedLineNumberField = this.shadowRoot.querySelector(".line-number");
        if (selectedLineNumberField.value == "") {
            if (selectedLineNumberField.classList.contains("highlighted") === false) {
                selectedLineNumberField.classList.add("highlighted");
            }
        }

        const inputtedQXField = this.shadowRoot.querySelector(".qx");
        if (inputtedQXField.value == "") {
            if (inputtedQXField.classList.contains("highlighted") === false) {
                inputtedQXField.classList.add("highlighted");
            }
        }
        const inputtedQYField = this.shadowRoot.querySelector(".qy");
        if (inputtedQYField.value == "") {
            if (inputtedQYField.classList.contains("highlighted") === false) {
                inputtedQYField.classList.add("highlighted");
            }
        }

        const inputtedQZField = this.shadowRoot.querySelector(".qz");
        if (inputtedQZField.value == "") {
            if (inputtedQZField.classList.contains("highlighted") === false) {
                inputtedQZField.classList.add("highlighted");
            }
        }

        if (selectedLineNumberField.value === "" || 
            inputtedQXField.value === "" || inputtedQYField.value === "" || inputtedQZField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(selectedLineNumberField.value) === false || 
            this.isNumeric(inputtedQXField.value) === false || 
            this.isNumeric(inputtedQYField.value) === false || 
            this.isNumeric(inputtedQZField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values!";
                return;
            } else {
                return;
            }
        }

        this.getActionId();

        const oldDistributedLineLoadValues = this.props.distributedLineLoads.get(parseInt(selectedLineNumberField.value));

        const message = {"update_distributed_line_load": {
            "actionId": this.props.actionId,
            "line_number": selectedLineNumberField.value, 
            "old_distributed_line_load_values": 
                { 
                    "qx": oldDistributedLineLoadValues.qx, "qy": oldDistributedLineLoadValues.qy,
                    "qz": oldDistributedLineLoadValues.qz,   
                },
            "new_distributed_line_load_values": 
                { 
                    "qx": inputtedQXField.value, "qy": inputtedQYField.value,
                    "qz": inputtedQZField.value,
                }
        }};

        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".line-number-filter").value = null;
    }

    cancelDistributedLineLoadUpdate() {
        if (this.props.distributedLineLoads.size > 0) {
            this.defineDistributedLineLoadOptions();
        }
        this.shadowRoot.querySelector(".line-number-filter").value = null;
        const selectedLineNumberForUpdateField = this.shadowRoot.querySelector(".line-number");
        this.dropHighlight(selectedLineNumberForUpdateField);
        const inputtedQXField = this.shadowRoot.querySelector(".qx");
        this.dropHighlight(inputtedQXField);
        const inputtedQYField = this.shadowRoot.querySelector(".qy");
        this.dropHighlight(inputtedQYField);
        const inputtedQZField = this.shadowRoot.querySelector(".z");
        this.dropHighlight(inputtedQZField);
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

export default FeaLoadUpdateDistributedLineLoadMenu;
