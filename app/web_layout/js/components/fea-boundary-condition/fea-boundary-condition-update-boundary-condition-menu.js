class FeaBoundaryConditionUpdateBoundaryConditionMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,                 // u32;
            isFEModelLoaded: false,         // load status of wasm module "fe_model";
            boundaryConditions: new Map(),  // map: { point_number: u32, { ux: f64, uy: f64, uz: f64, rx: f64, ry: f64, rz: f64 }, ... };
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

                .ux-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .ux-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .ux {
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

                .ux[type=number]::-webkit-outer-spin-button,
                .ux[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .ux[type=number] {
                    -moz-appearance: textfield;
                }

                .ux:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .ux:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .uy-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .uy-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .uy {
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

                .uy[type=number]::-webkit-outer-spin-button,
                .uy[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .uy[type=number] {
                    -moz-appearance: textfield;
                }

                .uy:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .uy:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .uz-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .uz-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .uz {
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

                .uz[type=number]::-webkit-outer-spin-button,
                .uz[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .uz[type=number] {
                    -moz-appearance: textfield;
                }

                .uz:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .uz:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .rx-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .rx-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .rx {
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

                .rx[type=number]::-webkit-outer-spin-button,
                .rx[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .rx[type=number] {
                    -moz-appearance: textfield;
                }

                .rx:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .rx:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .ry-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .ry-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .ry {
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

                .ry[type=number]::-webkit-outer-spin-button,
                .ry[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .ry[type=number] {
                    -moz-appearance: textfield;
                }

                .ry:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .ry:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .rz-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .rz-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .rz {
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

                .rz[type=number]::-webkit-outer-spin-button,
                .rz[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .rz[type=number] {
                    -moz-appearance: textfield;
                }

                .rz:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .rz:focus {
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

                <div class="ux-field-content">
                    <p class="ux-caption">Ux</p>
                    <input class="ux" type="number"/>
                </div>

                <div class="uy-field-content">
                    <p class="uy-caption">Uy</p>
                    <input class="uy" type="number"/>
                </div>

                <div class="uz-field-content">
                    <p class="uz-caption">Uz</p>
                    <input class="uz" type="number"/>
                </div>

                <div class="rx-field-content">
                    <p class="rx-caption">Rx</p>
                    <input class="rx" type="number"/>
                </div>

                <div class="ry-field-content">
                    <p class="ry-caption">Ry</p>
                    <input class="ry" type="number"/>
                </div>

                <div class="rz-field-content">
                    <p class="rz-caption">Rz</p>
                    <input class="rz" type="number"/>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.updateBoundaryCondition());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelBoundaryConditionUpdate());

        this.shadowRoot.querySelector(".point-number").addEventListener("change", () => this.updateBoundaryConditionValues());

        this.shadowRoot.querySelector(".point-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".point-number-filter").value,
                this.shadowRoot.querySelector(".point-number"));
        });

        this.shadowRoot.querySelector(".ux").addEventListener("click", () => {
            const inputtedUXField = this.shadowRoot.querySelector(".ux");
            this.dropHighlight(inputtedUXField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".uy").addEventListener("click", () => {
            const inputtedUYField = this.shadowRoot.querySelector(".uy");
            this.dropHighlight(inputtedUYField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".uz").addEventListener("click", () => {
            const inputtedUZField = this.shadowRoot.querySelector(".uz");
            this.dropHighlight(inputtedUZField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".rx").addEventListener("click", () => {
            const inputtedRXField = this.shadowRoot.querySelector(".rx");
            this.dropHighlight(inputtedRXField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".ry").addEventListener("click", () => {
            const inputtedRYField = this.shadowRoot.querySelector(".ry");
            this.dropHighlight(inputtedRYField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".rz").addEventListener("click", () => {
            const inputtedRZField = this.shadowRoot.querySelector(".rz");
            this.dropHighlight(inputtedRZField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set isFEModelLoaded(value) {
        this.props.isFEModelLoaded = value;
    }

    set boundaryConditions(value) {
        this.props.boundaryConditions = value;
    }

    set addBoundaryConditionToClient(boundaryCondition) {
        this.props.boundaryConditions.set(boundaryCondition.point_number, 
            {
                "ux": boundaryCondition.ux, "uy": boundaryCondition.uy, "uz": boundaryCondition.uz,
                "rx": boundaryCondition.rx, "ry": boundaryCondition.ry, "rz": boundaryCondition.rz,
            });
        this.defineBoundaryConditionOptions();
    }

    set updateBoundaryConditionInClient(boundaryCondition) {
        this.props.boundaryConditions.set(boundaryCondition.point_number, 
            {
                "ux": boundaryCondition.ux, "uy": boundaryCondition.uy, "uz": boundaryCondition.uz,
                "rx": boundaryCondition.rx, "ry": boundaryCondition.ry, "rz": boundaryCondition.rz,
            });
        this.defineBoundaryConditionOptions();
    }

    set deleteBoundaryConditionFromClient(boundaryCondition) {
        this.props.boundaryConditions.delete(boundaryCondition.point_number);
        this.defineBoundaryConditionOptions();
    }

    set feModelError(error) {
        if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = error;
        }
    }

    set selectBoundaryConditionInClient(pointNumber) {
        const frame = () => {
            if (this.props.isFEModelLoaded === true) {
                clearInterval(id);
                const pointNumberSelect = this.shadowRoot.querySelector(".point-number");
                const pointNumberOptions = pointNumberSelect.options;
                for (let option, i = 0; option = pointNumberOptions[i]; i++) {
                    if (option.value == pointNumber) {
                        pointNumberSelect.selectedIndex = i;
                        break;
                    }
                }
                this.updateBoundaryConditionValues();
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
                this.getBoundaryConditions();
                this.defineBoundaryConditionOptions();
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

    getBoundaryConditions() {
        this.dispatchEvent(new CustomEvent("getBoundaryConditions", {
            bubbles: true,
            composed: true,
        }));
    }

    defineBoundaryConditionOptions() {
        const pointNumberSelect = this.shadowRoot.querySelector(".point-number");
        for (let i = pointNumberSelect.length - 1; i >= 0; i--) {
            pointNumberSelect.options[i] = null;
        }
        if (this.props.boundaryConditions.size > 0) {
            const pointsNumbers = Array.from(this.props.boundaryConditions.keys()).sort((a, b) => a - b);
            for (let i = 0; i < pointsNumbers.length; i++) {
                let pointNumberOption = document.createElement("option");
                pointNumberOption.value = pointsNumbers[i];
                pointNumberOption.innerHTML = pointsNumbers[i];
                pointNumberSelect.appendChild(pointNumberOption);
            }

            if (this.props.boundaryConditions.get(pointsNumbers[0]) !== undefined) {
                this.shadowRoot.querySelector(".ux").value = this.props.boundaryConditions.get(pointsNumbers[0]).ux;
                this.shadowRoot.querySelector(".uy").value = this.props.boundaryConditions.get(pointsNumbers[0]).uy;
                this.shadowRoot.querySelector(".uz").value = this.props.boundaryConditions.get(pointsNumbers[0]).uz;
                this.shadowRoot.querySelector(".rx").value = this.props.boundaryConditions.get(pointsNumbers[0]).rx;
                this.shadowRoot.querySelector(".ry").value = this.props.boundaryConditions.get(pointsNumbers[0]).ry;
                this.shadowRoot.querySelector(".rz").value = this.props.boundaryConditions.get(pointsNumbers[0]).rz;
            } else {
                this.shadowRoot.querySelector(".ux").value = "";
                this.shadowRoot.querySelector(".uy").value = "";
                this.shadowRoot.querySelector(".uz").value = "";
                this.shadowRoot.querySelector(".rx").value = "";
                this.shadowRoot.querySelector(".ry").value = "";
                this.shadowRoot.querySelector(".rz").value = "";
            }

        } else {
            this.shadowRoot.querySelector(".ux").value = "";
            this.shadowRoot.querySelector(".uy").value = "";
            this.shadowRoot.querySelector(".uz").value = "";
            this.shadowRoot.querySelector(".rx").value = "";
            this.shadowRoot.querySelector(".ry").value = "";
            this.shadowRoot.querySelector(".rz").value = "";
        }
    }

    updateBoundaryConditionValues() {
        const selectedPointNumber = this.shadowRoot.querySelector(".point-number").value;
        this.shadowRoot.querySelector(".ux").value = this.props.boundaryConditions.get(parseInt(selectedPointNumber)).ux;
        this.dropHighlight(this.shadowRoot.querySelector(".ux"));
        this.shadowRoot.querySelector(".uy").value = this.props.boundaryConditions.get(parseInt(selectedPointNumber)).uy;
        this.dropHighlight(this.shadowRoot.querySelector(".uy"));
        this.shadowRoot.querySelector(".uz").value = this.props.boundaryConditions.get(parseInt(selectedPointNumber)).uz;
        this.dropHighlight(this.shadowRoot.querySelector(".uz"));
        this.shadowRoot.querySelector(".rx").value = this.props.boundaryConditions.get(parseInt(selectedPointNumber)).rx;
        this.dropHighlight(this.shadowRoot.querySelector(".rx"));
        this.shadowRoot.querySelector(".ry").value = this.props.boundaryConditions.get(parseInt(selectedPointNumber)).ry;
        this.dropHighlight(this.shadowRoot.querySelector(".ry"));
        this.shadowRoot.querySelector(".rz").value = this.props.boundaryConditions.get(parseInt(selectedPointNumber)).rz;
        this.dropHighlight(this.shadowRoot.querySelector(".rz"));
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

    updateBoundaryCondition() {
        const selectedPointNumberField = this.shadowRoot.querySelector(".point-number");
        if (selectedPointNumberField.value == "") {
            if (selectedPointNumberField.classList.contains("highlighted") === false) {
                selectedPointNumberField.classList.add("highlighted");
            }
        }

        const inputtedUXField = this.shadowRoot.querySelector(".ux");
        const inputtedUYField = this.shadowRoot.querySelector(".uy");
        const inputtedUZField = this.shadowRoot.querySelector(".uz");
        const inputtedRXField = this.shadowRoot.querySelector(".rx");
        const inputtedRYField = this.shadowRoot.querySelector(".ry");
        const inputtedRZField = this.shadowRoot.querySelector(".rz");

        if (selectedPointNumberField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The point number field should be filled!";
                return;
            } else {
                return;
            }
        }

        if (inputtedUXField.value === "" && inputtedUYField.value === "" && inputtedUZField.value === "" &&
            inputtedRXField.value === "" && inputtedRYField.value === "" && inputtedRZField.value === "") {
            if (inputtedUXField.classList.contains("highlighted") === false) {
                inputtedUXField.classList.add("highlighted");
            }
            if (inputtedUYField.classList.contains("highlighted") === false) {
                inputtedUYField.classList.add("highlighted");
            }
            if (inputtedUZField.classList.contains("highlighted") === false) {
                inputtedUZField.classList.add("highlighted");
            }
            if (inputtedRXField.classList.contains("highlighted") === false) {
                inputtedRXField.classList.add("highlighted");
            }
            if (inputtedRYField.classList.contains("highlighted") === false) {
                inputtedRYField.classList.add("highlighted");
            }
            if (inputtedRZField.classList.contains("highlighted") === false) {
                inputtedRZField.classList.add("highlighted");
            }
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: At least one of the highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if ((selectedPointNumberField.value != "" && this.isNumeric(selectedPointNumberField.value) === false) || 
            (inputtedUXField.value != "" && this.isNumeric(inputtedUXField.value) === false) || 
            (inputtedUYField.value != "" && this.isNumeric(inputtedUYField.value) === false) || 
            (inputtedUZField.value != "" && this.isNumeric(inputtedUZField.value) === false) ||
            (inputtedRXField.value != "" && this.isNumeric(inputtedRXField.value) === false) || 
            (inputtedRYField.value != "" && this.isNumeric(inputtedRYField.value) === false) || 
            (inputtedRZField.value != "" && this.isNumeric(inputtedRZField.value) === false)) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values!";
                return;
            } else {
                return;
            }
        }

        this.getActionId();

        const oldBoundaryConditionValues = this.props.boundaryConditions.get(parseInt(selectedPointNumberField.value));

        const message = {"update_boundary_condition": {
            "actionId": this.props.actionId,
            "point_number": selectedPointNumberField.value, 
            "old_boundary_condition_values": 
                { 
                    "ux": oldBoundaryConditionValues.ux, "uy": oldBoundaryConditionValues.uy,
                    "uz": oldBoundaryConditionValues.uz, "rx": oldBoundaryConditionValues.rx,
                    "ry": oldBoundaryConditionValues.ry, "rz": oldBoundaryConditionValues.rz,   
                },
            "new_boundary_condition_values": 
                { 
                    "ux": inputtedUXField.value != "" ? inputtedUXField.value : null,
                    "uy": inputtedUYField.value != "" ? inputtedUYField.value : null,
                    "uz": inputtedUZField.value != "" ? inputtedUZField.value : null,
                    "rx": inputtedRXField.value != "" ? inputtedRXField.value : null,
                    "ry": inputtedRYField.value != "" ? inputtedRYField.value : null,
                    "rz": inputtedRZField.value != "" ? inputtedRZField.value : null,
                }
        }};

        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".point-number-filter").value = null;
    }

    cancelBoundaryConditionUpdate() {
        if (this.props.boundaryConditions.size > 0) {
            this.defineBoundaryConditionOptions();
        }
        this.shadowRoot.querySelector(".point-number-filter").value = null;
        const selectedPointNumberForUpdateField = this.shadowRoot.querySelector(".point-number");
        this.dropHighlight(selectedPointNumberForUpdateField);
        const inputtedUXField = this.shadowRoot.querySelector(".ux");
        this.dropHighlight(inputtedUXField);
        const inputtedUYField = this.shadowRoot.querySelector(".uy");
        this.dropHighlight(inputtedUYField);
        const inputtedUZField = this.shadowRoot.querySelector(".uz");
        this.dropHighlight(inputtedUZField);
        const inputtedRXField = this.shadowRoot.querySelector(".rx");
        this.dropHighlight(inputtedRXField);
        const inputtedRYField = this.shadowRoot.querySelector(".ry");
        this.dropHighlight(inputtedRYField);
        const inputtedRZField = this.shadowRoot.querySelector(".rz");
        this.dropHighlight(inputtedRZField);
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

export default FeaBoundaryConditionUpdateBoundaryConditionMenu;
