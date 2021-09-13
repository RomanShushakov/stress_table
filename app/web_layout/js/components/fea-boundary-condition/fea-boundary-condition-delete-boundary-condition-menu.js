class FeaBoundaryConditionDeleteBoundaryConditionMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,                 // u32;
            isFEModelLoaded: false,         // load status of wasm module "fe_model";
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

                <div class="apply-cancel-buttons">
                    <button class="apply-button">Apply</button>
                    <button class="cancel-button">Cancel</button>
                </div>

                <div class="analysis-info">
                    <p class="analysis-info-message"></p>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.deleteBoundaryCondition());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelBoundaryConditionDelete());

        this.shadowRoot.querySelector(".point-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".point-number-filter").value,
                this.shadowRoot.querySelector(".point-number"));
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
                "optional_ux": boundaryCondition.optional_ux, "optional_uy": boundaryCondition.optional_uy, 
                "optional_uz": boundaryCondition.optional_uz, "optional_rx": boundaryCondition.optional_rx, 
                "optional_ry": boundaryCondition.optional_ry, "optional_rz": boundaryCondition.optional_rz,
            });
        this.defineBoundaryConditionOptions();
    }

    set updateBoundaryConditionInClient(_boundaryCondition) {
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
        const pointsNumbers = Array.from(this.props.boundaryConditions.keys()).sort((a, b) => a - b);
        for (let i = 0; i < pointsNumbers.length; i++) {
            let pointNumberOption = document.createElement("option");
            pointNumberOption.value = pointsNumbers[i];
            pointNumberOption.innerHTML = pointsNumbers[i];
            pointNumberSelect.appendChild(pointNumberOption);
        }
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

    deleteBoundaryCondition() {
        const selectedPointNumberField = this.shadowRoot.querySelector(".point-number");
        if (selectedPointNumberField.value == "") {
            if (selectedPointNumberField.classList.contains("highlighted") === false) {
                selectedPointNumberField.classList.add("highlighted");
            }
        }

        if (selectedPointNumberField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        this.getActionId();
        
        const message = {"delete_boundary_condition": 
            { "actionId": this.props.actionId, "point_number": parseInt(selectedPointNumberField.value) }};

        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
        this.shadowRoot.querySelector(".point-number-filter").value = null;
    }

    cancelBoundaryConditionDelete() {
        if (this.props.boundaryConditions.size > 0) {
            this.defineBoundaryConditionOptions();
        }
        this.shadowRoot.querySelector(".point-number-filter").value = null;
        const selectedPointNumberForDeleteField = this.shadowRoot.querySelector(".point-number");
        this.dropHighlight(selectedPointNumberForDeleteField);
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }
}

export default FeaBoundaryConditionDeleteBoundaryConditionMenu;
