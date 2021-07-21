class FeaSectionAddTrussMenu extends HTMLElement {
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
                }

                .truss-section-name-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .truss-section-name {
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

                .truss-section-name:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .truss-section-name:focus {
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
                    <input class="truss-section-name" type="text"/>
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

         this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.addTrussSection());

         this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelTrussSectionAddition());

         this.shadowRoot.querySelector(".truss-section-name").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".truss-section-name");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
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
        this.defineNewTrussSectionName();
    }

    set updateTrussSectionInClient(trussSection) {
        let trussSectionInProps = this.props.trussSections
            .find(existedTrussSection => existedTrussSection.name == trussSection.name);
        trussSectionInProps.area = trussSection.area;
        trussSectionInProps.area2 = trussSection.area2;
    }

    set deleteTrussSectionFromClient(trussSection) {
        let trussSectionIndexInProps = this.props.trussSections
            .findIndex(existedTrussSection => existedTrussSection.name == trussSection.name);
        this.props.trussSections.splice(trussSectionIndexInProps, 1);
        this.props.trussSections.sort((a, b) => a.name - b.name);
        this.defineNewTrussSectionName();
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
                this.defineNewTrussSectionName();
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

    defineNewTrussSectionName() {
        const newTrussSectionName = "truss1";
        this.shadowRoot.querySelector(".truss-section-name").value = newTrussSectionName;
        this.shadowRoot.querySelector(".area").value = 1;
        this.shadowRoot.querySelector(".area2").value = null;
    }


    addTrussSection() {
        const newTrussSectionNameField = this.shadowRoot.querySelector(".truss-section-name");
        if (newTrussSectionNameField.value === "") {
            if (newTrussSectionNameField.classList.contains("highlighted") === false) {
                newTrussSectionNameField.classList.add("highlighted");
            }
        }

        const areaField = this.shadowRoot.querySelector(".area");
        if (areaField.value === "") {
            if (areaField.classList.contains("highlighted") === false) {
                areaField.classList.add("highlighted");
            }
        }
        
        const area2Field = this.shadowRoot.querySelector(".area2");

        if (newTrussSectionNameField.value === "" || areaField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(areaField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values for Area!";
                return;
            } else {
                return;
            }
        }

        this.getActionId();

        const message = {"add_truss_section": {
            "actionId": this.props.actionId,
            "name": newTrussSectionNameField.value, 
            "area": areaField.value, "area2":  area2Field.value,
        }};

        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }

    cancelTrussSectionAddition() {
        this.defineNewTrussSectionName();
        const newTrussSectionNameField = this.shadowRoot.querySelector(".truss-section-name");
        this.dropHighlight(newTrussSectionNameField);
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

export default FeaSectionAddTrussMenu;
