class FeaBoundaryConditionAddBoundaryConditionMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,                 // u32;
            isFEModelLoaded: false,         // load status of wasm module "fe_model";
            points: new Map(),              // map: { number: u32, { x: f64, y: f64, z: f64}, ... };
            boundaryConditions: new Map(),  // map: { point_number: u32, 
                                            //      { optional_ux: f64 or null, optional_uy: f64 or null, 
                                            //        optional_uz: f64 or null, optional_rx: f64 or null, 
                                            //        optional_ry: f64 or null, optional_rz: f64 or null }, ... };
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

                .optional-ux-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .optional-ux-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .optional-ux {
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

                .optional-ux[type=number]::-webkit-outer-spin-button,
                .optional-ux[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .optional-ux[type=number] {
                    -moz-appearance: textfield;
                }

                .optional-ux:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .optional-ux:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .optional-uy-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .optional-uy-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .optional-uy {
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

                .optional-uy[type=number]::-webkit-outer-spin-button,
                .optional-uy[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .optional-uy[type=number] {
                    -moz-appearance: textfield;
                }

                .optional-uy:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .optional-uy:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .optional-uz-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .optional-uz-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .optional-uz {
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

                .optional-uz[type=number]::-webkit-outer-spin-button,
                .optional-uz[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .optional-uz[type=number] {
                    -moz-appearance: textfield;
                }

                .optional-uz:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .optional-uz:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .optional-rx-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .optional-rx-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .optional-rx {
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

                .optional-rx[type=number]::-webkit-outer-spin-button,
                .optional-rx[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .optional-rx[type=number] {
                    -moz-appearance: textfield;
                }

                .optional-rx:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .optional-rx:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .optional-ry-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .optional-ry-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .optional-ry {
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

                .optional-ry[type=number]::-webkit-outer-spin-button,
                .optional-ry[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .optional-ry[type=number] {
                    -moz-appearance: textfield;
                }

                .optional-ry:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .optional-ry:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .optional-rz-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .optional-rz-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .optional-rz {
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

                .optional-rz[type=number]::-webkit-outer-spin-button,
                .optional-rz[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .optional-rz[type=number] {
                    -moz-appearance: textfield;
                }

                .optional-rz:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .optional-rz:focus {
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

                <div class="optional-ux-field-content">
                    <p class="optional-ux-caption">Ux</p>
                    <input class="optional-ux" type="number"/>
                </div>

                <div class="optional-uy-field-content">
                    <p class="optional-uy-caption">Uy</p>
                    <input class="optional-uy" type="number"/>
                </div>

                <div class="optional-uz-field-content">
                    <p class="optional-uz-caption">Uz</p>
                    <input class="optional-uz" type="number"/>
                </div>

                <div class="optional-rx-field-content">
                    <p class="optional-rx-caption">Rx</p>
                    <input class="optional-rx" type="number"/>
                </div>

                <div class="optional-ry-field-content">
                    <p class="optional-ry-caption">Ry</p>
                    <input class="optional-ry" type="number"/>
                </div>

                <div class="optional-rz-field-content">
                    <p class="optional-rz-caption">Rz</p>
                    <input class="optional-rz" type="number"/>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.addBoundaryCondition());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelBoundaryConditionAddition());

        this.shadowRoot.querySelector(".point-number").addEventListener("change", () => this.updateBoundaryConditionValues());

        this.shadowRoot.querySelector(".point-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".point-number-filter").value,
                this.shadowRoot.querySelector(".point-number"));
        });

        this.shadowRoot.querySelector(".optional-ux").addEventListener("click", () => {
            const inputtedUXField = this.shadowRoot.querySelector(".optional-ux");
            this.dropHighlight(inputtedUXField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".optional-uy").addEventListener("click", () => {
            const inputtedUYField = this.shadowRoot.querySelector(".optional-uy");
            this.dropHighlight(inputtedUYField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".optional-uz").addEventListener("click", () => {
            const inputtedUZField = this.shadowRoot.querySelector(".optional-uz");
            this.dropHighlight(inputtedUZField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".optional-rx").addEventListener("click", () => {
            const inputtedRXField = this.shadowRoot.querySelector(".optional-rx");
            this.dropHighlight(inputtedRXField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".optional-ry").addEventListener("click", () => {
            const inputtedRYField = this.shadowRoot.querySelector(".optional-ry");
            this.dropHighlight(inputtedRYField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".optional-rz").addEventListener("click", () => {
            const inputtedRZField = this.shadowRoot.querySelector(".optional-rz");
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

    set points(value) {
        this.props.points = value;
    }

    set boundaryConditions(value) {
        this.props.boundaryConditions = value;
    }

    set addPointToClient(point) {
        this.props.points.set(point.number, {"x": point.x, "y": point.y, "z": point.z});
        this.defineBoundaryConditionOptions();
    }

    set updatePointInClient(point) {
        this.props.points.set(point.number, {"x": point.x, "y": point.y, "z": point.z});
        this.defineBoundaryConditionOptions();
    }

    set deletePointFromClient(point) {
        this.props.points.delete(point.number);
        this.defineBoundaryConditionOptions();
    }

    set addBoundaryConditionToClient(boundaryCondition) {
        this.props.boundaryConditions.set(boundaryCondition.point_number, 
            {
                "optional_ux": boundaryCondition.optional_ux, "optional_uy": boundaryCondition.optional_uy, 
                "optional_uz": boundaryCondition.optional_uz, "optional_rx": boundaryCondition.optional_rx, 
                "optional_ry": boundaryCondition.optional_ry, "optional_rz": boundaryCondition.optional_rz,
            });
        this.defineBoundaryConditionOptions();
    }

    set updateBoundaryConditionInClient(boundaryCondition) {
        this.props.boundaryConditions.set(boundaryCondition.point_number, 
            {
                "optional_ux": boundaryCondition.optional_ux, "optional_uy": boundaryCondition.optional_uy, 
                "optional_uz": boundaryCondition.optional_uz, "optional_rx": boundaryCondition.optional_rx, 
                "optional_ry": boundaryCondition.optional_ry, "optional_rz": boundaryCondition.optional_rz,
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

    getPoints() {
        this.dispatchEvent(new CustomEvent("getPoints", {
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
        if (this.props.points.size > 0) {
            const pointsNumbers = Array.from(this.props.points.keys()).sort((a, b) => a - b);
            for (let i = 0; i < pointsNumbers.length; i++) {
                let pointNumberOption = document.createElement("option");
                pointNumberOption.value = pointsNumbers[i];
                pointNumberOption.innerHTML = pointsNumbers[i];
                pointNumberSelect.appendChild(pointNumberOption);
            }

            if (this.props.boundaryConditions.get(pointsNumbers[0]) !== undefined) {
                this.shadowRoot.querySelector(".optional-ux").value = this.props.boundaryConditions.get(pointsNumbers[0]).optional_ux;
                this.shadowRoot.querySelector(".optional-uy").value = this.props.boundaryConditions.get(pointsNumbers[0]).optional_uy;
                this.shadowRoot.querySelector(".optional-uz").value = this.props.boundaryConditions.get(pointsNumbers[0]).optional_uz;
                this.shadowRoot.querySelector(".optional-rx").value = this.props.boundaryConditions.get(pointsNumbers[0]).optional_rx;
                this.shadowRoot.querySelector(".optional-ry").value = this.props.boundaryConditions.get(pointsNumbers[0]).optional_ry;
                this.shadowRoot.querySelector(".optional-rz").value = this.props.boundaryConditions.get(pointsNumbers[0]).optional_rz;
            } else {
                this.shadowRoot.querySelector(".optional-ux").value = "";
                this.shadowRoot.querySelector(".optional-uy").value = "";
                this.shadowRoot.querySelector(".optional-uz").value = "";
                this.shadowRoot.querySelector(".optional-rx").value = "";
                this.shadowRoot.querySelector(".optional-ry").value = "";
                this.shadowRoot.querySelector(".optional-rz").value = "";
            }

        } else {
            this.shadowRoot.querySelector(".optional-ux").value = "";
            this.shadowRoot.querySelector(".optional-uy").value = "";
            this.shadowRoot.querySelector(".optional-uz").value = "";
            this.shadowRoot.querySelector(".optional-rx").value = "";
            this.shadowRoot.querySelector(".optional-ry").value = "";
            this.shadowRoot.querySelector(".optional-rz").value = "";
        }
    }

    updateBoundaryConditionValues() {
        const selectedPointNumber = this.shadowRoot.querySelector(".point-number").value;
        if (this.props.boundaryConditions.get(parseInt(selectedPointNumber)) !== undefined) {
            this.shadowRoot.querySelector(".optional-ux").value = this.props.boundaryConditions.get(parseInt(selectedPointNumber)).optional_ux;
            this.dropHighlight(this.shadowRoot.querySelector(".optional-ux"));
            this.shadowRoot.querySelector(".optional-uy").value = this.props.boundaryConditions.get(parseInt(selectedPointNumber)).optional_uy;
            this.dropHighlight(this.shadowRoot.querySelector(".optional-uy"));
            this.shadowRoot.querySelector(".optional-uz").value = this.props.boundaryConditions.get(parseInt(selectedPointNumber)).optional_uz;
            this.dropHighlight(this.shadowRoot.querySelector(".optional-uz"));
            this.shadowRoot.querySelector(".optional-rx").value = this.props.boundaryConditions.get(parseInt(selectedPointNumber)).optional_rx;
            this.dropHighlight(this.shadowRoot.querySelector(".optional-rx"));
            this.shadowRoot.querySelector(".optional-ry").value = this.props.boundaryConditions.get(parseInt(selectedPointNumber)).optional_ry;
            this.dropHighlight(this.shadowRoot.querySelector(".optional-ry"));
            this.shadowRoot.querySelector(".optional-rz").value = this.props.boundaryConditions.get(parseInt(selectedPointNumber)).optional_rz;
            this.dropHighlight(this.shadowRoot.querySelector(".optional-rz"));
        } else {
            this.shadowRoot.querySelector(".optional-ux").value = "";
            this.dropHighlight(this.shadowRoot.querySelector(".optional-ux"));
            this.shadowRoot.querySelector(".optional-uy").value = "";
            this.dropHighlight(this.shadowRoot.querySelector(".optional-uy"));
            this.shadowRoot.querySelector(".optional-uz").value = "";
            this.dropHighlight(this.shadowRoot.querySelector(".optional-uz"));
            this.shadowRoot.querySelector(".optional-rx").value = "";
            this.dropHighlight(this.shadowRoot.querySelector(".optional-rx"));
            this.shadowRoot.querySelector(".optional-ry").value = "";
            this.dropHighlight(this.shadowRoot.querySelector(".optional-ry"));
            this.shadowRoot.querySelector(".optional-rz").value = "";
            this.dropHighlight(this.shadowRoot.querySelector(".optional-rz"));
        }
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

    addBoundaryCondition() {
        const selectedPointNumberField = this.shadowRoot.querySelector(".point-number");
        if (selectedPointNumberField.value == "") {
            if (selectedPointNumberField.classList.contains("highlighted") === false) {
                selectedPointNumberField.classList.add("highlighted");
            }
        }

        const inputtedUXField = this.shadowRoot.querySelector(".optional-ux");
        const inputtedUYField = this.shadowRoot.querySelector(".optional-uy");
        const inputtedUZField = this.shadowRoot.querySelector(".optional-uz");
        const inputtedRXField = this.shadowRoot.querySelector(".optional-rx");
        const inputtedRYField = this.shadowRoot.querySelector(".optional-ry");
        const inputtedRZField = this.shadowRoot.querySelector(".optional-rz");

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

        const message = {"add_boundary_condition": {
            "actionId": this.props.actionId,
            "point_number": selectedPointNumberField.value, 
            "optional_ux": inputtedUXField.value != "" ? inputtedUXField.value : null,
            "optional_uy": inputtedUYField.value != "" ? inputtedUYField.value : null,
            "optional_uz": inputtedUZField.value != "" ? inputtedUZField.value : null,
            "optional_rx": inputtedRXField.value != "" ? inputtedRXField.value : null,
            "optional_ry": inputtedRYField.value != "" ? inputtedRYField.value : null,
            "optional_rz": inputtedRZField.value != "" ? inputtedRZField.value : null,
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

    cancelBoundaryConditionAddition() {
        if (this.props.points.size > 0) {
            this.defineBoundaryConditionOptions();
        }
        this.shadowRoot.querySelector(".point-number-filter").value = null;
        const selectedPointNumberField = this.shadowRoot.querySelector(".point-number");
        this.dropHighlight(selectedPointNumberField);
        const inputtedUXField = this.shadowRoot.querySelector(".optional-ux");
        this.dropHighlight(inputtedUXField);
        const inputtedUYField = this.shadowRoot.querySelector(".optional-uy");
        this.dropHighlight(inputtedUYField);
        const inputtedUZField = this.shadowRoot.querySelector(".optional-uz");
        this.dropHighlight(inputtedUZField);
        const inputtedRXField = this.shadowRoot.querySelector(".optional-rx");
        this.dropHighlight(inputtedRXField);
        const inputtedRYField = this.shadowRoot.querySelector(".optional-ry");
        this.dropHighlight(inputtedRYField);
        const inputtedRZField = this.shadowRoot.querySelector(".optional-rz");
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

export default FeaBoundaryConditionAddBoundaryConditionMenu;
