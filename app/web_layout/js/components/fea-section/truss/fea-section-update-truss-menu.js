class FeaSectionUpdateTrussMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,             // u32;
            isFEModelLoaded: false,     // load status of wasm module "fe_model";
            trussSections: [],          // array of: [{ name: String, area: f64, area2: f64 or null }];
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

                .truss-section-name-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin: 0rem;
                    align-items: center;
                }

                .truss-section-name-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .truss-section-name-select-filter-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .truss-section-name-filter-label {
                    position: relative;
                }
                  
                .truss-section-name-filter-label:before {
                    content: "";
                    position: absolute;
                    left: 0rem;
                    top: 0rem;
                    bottom: 0rem;
                    width: 0.8rem;
                    background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
                }

                .truss-section-name-filter {
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

                .truss-section-name-filter::placeholder {
                    font-size: 85%;
                }

                .truss-section-name-filter:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .truss-section-name-filter:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .truss-section-name {
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

                .truss-section-name option {
                    background-color: #484f60;
                }

                .truss-section-name:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .area-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .area-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .area {
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

                .area[type=number]::-webkit-outer-spin-button,
                .area[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .area[type=number] {
                    -moz-appearance: textfield;
                }

                .area:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .area:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .area2-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .area2-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .area2 {
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

                .area2[type=number]::-webkit-outer-spin-button,
                .area2[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .area2[type=number] {
                    -moz-appearance: textfield;
                }

                .area2:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .area2:focus {
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

                <div class="truss-section-name-field-content">
                    <p class="truss-section-name-caption">Section name</p>
                    <div class="truss-section-name-select-filter-content">
                        <label class="truss-section-name-filter-label">
                            <input class="truss-section-name-filter" type="text" placeholder="Filter..."/>
                        </label>
                        <select class="truss-section-name"></select>
                    </div>
                </div>

                <div class="area-field-content">
                    <p class="area-caption">Area</p>
                    <input class="area" type="number"/>
                </div>

                <div class="area2-field-content">
                    <p class="area2-caption">Area 2 (optional)</p>
                    <input class="area2" type="number"/>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.updateTrussSection());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelTrussSectionUpdate());

        this.shadowRoot.querySelector(".truss-section-name").addEventListener("change", () => this.updateTrussSectionData());

        this.shadowRoot.querySelector(".truss-section-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".truss-section-name-filter").value,
                this.shadowRoot.querySelector(".truss-section-name"));
        });

        this.shadowRoot.querySelector(".area").addEventListener("click", () => {
            const inputtedArea = this.shadowRoot.querySelector(".area");
            this.dropHighlight(inputtedArea);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".area2").addEventListener("click", () => {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set isFEModelLoaded(value) {
        this.props.isFEModelLoaded = value;
    }

    set trussSections(value) {
        this.props.trussSections = value;
    }

    set addTrussSectionToClient(trussSection) {
        this.props.trussSections.push(trussSection);
        this.props.trussSections.sort((a, b) => a.name - b.name);
        this.defineTrussSectionNameOptions();
    }

    set updateTrussSectionInClient(trussSection) {
        let trussSectionInProps = this.props.trussSections
            .find(existedTrussSection => existedTrussSection.name == trussSection.name);
        trussSectionInProps.area = trussSection.area;
        trussSectionInProps.area2 = trussSection.area2;
        this.defineTrussSectionNameOptions();
    }

    set deleteTrussSectionFromClient(trussSection) {
        let trussSectionIndexInProps = this.props.trussSections
            .findIndex(existedTrussSection => existedTrussSection.name == trussSection.name);
        this.props.trussSections.splice(trussSectionIndexInProps, 1);
        this.props.trussSections.sort((a, b) => a.name - b.name);
        this.defineTrussSectionNameOptions();
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
                this.getTrussSections();
                this.defineTrussSectionNameOptions();
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

    getTrussSections() {
        this.dispatchEvent(new CustomEvent("getTrussSections", {
            bubbles: true,
            composed: true,
        }));
    }

    defineTrussSectionNameOptions() {
        const updateTrussSectionNameSelect = this.shadowRoot.querySelector(".truss-section-name");
        for (let i = updateTrussSectionNameSelect.length - 1; i >= 0; i--) {
            updateTrussSectionNameSelect.options[i] = null;
        }
        if (this.props.trussSections.length > 0) {
            for (let i = 0; i < this.props.trussSections.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = this.props.trussSections[i].name.replace(/['"]+/g, "");
                updateOption.innerHTML = this.props.trussSections[i].name.replace(/['"]+/g, "");
                updateTrussSectionNameSelect.appendChild(updateOption);
            }
            this.shadowRoot.querySelector(".area").value = this.props.trussSections[0].area;
            this.shadowRoot.querySelector(".area2").value = this.props.trussSections[0].area2;
        } else {
            this.shadowRoot.querySelector(".area").value = "";
            this.shadowRoot.querySelector(".area2").value = null;
        }
    }

    updateTrussSectionData() {
        const selectedTrussSectionName = this.shadowRoot.querySelector(".truss-section-name").value;
        const trussSectionInProps = this.props.trussSections
            .find(trussSection => trussSection.name == `"${selectedTrussSectionName}"`);
        this.shadowRoot.querySelector(".area").value = trussSectionInProps.area;
        this.dropHighlight(this.shadowRoot.querySelector(".area"));
        this.shadowRoot.querySelector(".area2").value = trussSectionInProps.area2;
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

    updateTrussSection() {
        const selectedTrussSectionNameField = this.shadowRoot.querySelector(".truss-section-name");
        if (selectedTrussSectionNameField.value == "") {
            if (selectedTrussSectionNameField.classList.contains("highlighted") === false) {
                selectedTrussSectionNameField.classList.add("highlighted");
            }
        }

        const inputtedAreaField = this.shadowRoot.querySelector(".area");
        if (inputtedAreaField.value == "") {
            if (inputtedAreaField.classList.contains("highlighted") === false) {
                inputtedAreaField.classList.add("highlighted");
            }
        }

        const inputtedArea2Field = this.shadowRoot.querySelector(".area2");

        if (selectedTrussSectionNameField.value === "" || inputtedAreaField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(inputtedAreaField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values for Area!";
                return;
            } else {
                return;
            }
        }

        const oldTrussSectionValues = this.props.trussSections
            .find(trussSection => trussSection.name == `"${selectedTrussSectionNameField.value}"`);

        this.getActionId();
        
        const message = { "update_truss_section": {
            "actionId": this.props.actionId,
            "name": selectedTrussSectionNameField.value, 
            "old_truss_section_values": { 
                "area":  oldTrussSectionValues.area,
                "area2": oldTrussSectionValues.area2 },
            "new_truss_section_values": { 
                "area": inputtedAreaField.value,
                "area2": inputtedArea2Field.value }
        }};
        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".truss-section-name-filter").value = null;
    }

    cancelTrussSectionUpdate() {
        if (this.props.trussSections.length > 0) {
            this.defineTrussSectionNameOptions();
        }
        this.shadowRoot.querySelector(".truss-section-name-filter").value = null;
        const selectedTrussSectionNameForUpdateField = this.shadowRoot.querySelector(".truss-section-name");
        this.dropHighlight(selectedTrussSectionNameForUpdateField);
        const inputtedAreaField = this.shadowRoot.querySelector(".area");
        this.dropHighlight(inputtedAreaField);
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

export default FeaSectionUpdateTrussMenu;
