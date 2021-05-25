class FeaMaterialAddMaterialMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,     // u32;
            materials: [],      // array of: [{ name: String, youngModulus: f64, poissonRatio: f64 }, ...];
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

                .material-name-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin: 0rem;
                }

                .material-name-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .material-name {
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

                .material-name:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .material-name:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .young-modulus-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .young-modulus-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .young-modulus {
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

                .young-modulus[type=number]::-webkit-outer-spin-button,
                .young-modulus[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .young-modulus[type=number] {
                    -moz-appearance: textfield;
                }

                .young-modulus:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .young-modulus:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .poisson-ratio-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .poisson-ratio-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .poisson-ratio {
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

                .poisson-ratio[type=number]::-webkit-outer-spin-button,
                .poisson-ratio[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .poisson-ratio[type=number] {
                    -moz-appearance: textfield;
                }

                .poisson-ratio:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .poisson-ratio:focus {
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
                <div class="material-name-field-content">
                    <p class="material-name-caption">Material name</p>
                    <input class="material-name" type="text"/>
                </div>

                <div class="young-modulus-field-content">
                    <p class="young-modulus-caption">Young's modulus</p>
                    <input class="young-modulus" type="number"/>
                </div>

                <div class="poisson-ratio-field-content">
                    <p class="poisson-ratio-caption">Poisson's ratio</p>
                    <input class="poisson-ratio" type="number"/>
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

         this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.addMaterial());

         this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelMaterialAddition());

         this.shadowRoot.querySelector(".material-name").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".material-name");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".young-modulus").addEventListener("click", () => {
            const inputtedYoungModulus = this.shadowRoot.querySelector(".young-modulus");
            this.dropHighlight(inputtedYoungModulus);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".poisson-ratio").addEventListener("click", () => {
            const inputtedPoissonRatio = this.shadowRoot.querySelector(".poisson-ratio");
            this.dropHighlight(inputtedPoissonRatio);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set addMaterialToClient(material) {
        this.props.materials.push(material);
        this.props.materials.sort((a, b) => a.name - b.name);
        this.defineNewMaterialName();
    }

    set updateMaterialInClient(material) {
        let materialInProps = this.props.materials.find(existedMaterial => existedMaterial.name == material.name);
        materialInProps.youngModulus = material.youngModulus;
        materialInProps.poissonRatio = material.poissonRatio;
    }

    set deleteMaterialFromClient(material) {
        let materialIndexInProps = this.props.materials.findIndex(existedMaterial => existedMaterial.name == material.name);
        this.props.materials.splice(materialIndexInProps, 1);
        this.props.materials.sort((a, b) => a.name - b.name);
        this.defineNewMaterialName();
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.defineNewMaterialName();
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

    defineNewMaterialName() {
        const newMaterialName = "mat1";
        this.shadowRoot.querySelector(".material-name").value = newMaterialName;
        this.shadowRoot.querySelector(".young-modulus").value = 10000000;
        this.shadowRoot.querySelector(".poisson-ratio").value = 0.3;
    }


    addMaterial() {
        const newMaterialNameField = this.shadowRoot.querySelector(".material-name");
        if (newMaterialNameField.value === "") {
            if (newMaterialNameField.classList.contains("highlighted") === false) {
                newMaterialNameField.classList.add("highlighted");
            }
        }
        const youngModulusField = this.shadowRoot.querySelector(".young-modulus");
        if (youngModulusField.value === "") {
            if (youngModulusField.classList.contains("highlighted") === false) {
                youngModulusField.classList.add("highlighted");
            }
        }
        const poissonRationField = this.shadowRoot.querySelector(".poisson-ratio");
        if (poissonRationField.value === "") {
            if (poissonRationField.classList.contains("highlighted") === false) {
                poissonRationField.classList.add("highlighted");
            }
        }

        if (newMaterialNameField.value === "" || youngModulusField.value === "" || 
            poissonRationField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const materialNameInProps = this.props.materials.find(material => material.name == `"${newMaterialNameField.value}"`);
        if (materialNameInProps != null) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The material with the same name does already exist!";
                return;
            } else {
                return;
            }
        }

        const materialDataInProps = this.props.materials.find(material => material.youngModulus == youngModulusField.value && 
            material.poissonRatio == poissonRationField.value);
        if (materialDataInProps != null) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The material with the same data does already exist!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(youngModulusField.value) === false || this.isNumeric(poissonRationField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values for Young's modulus and Poisson's ratio!";
                return;
            } else {
                return;
            }
        }

        const message = {"add_material": {
            "actionId": this.props.actionId,
            "name": newMaterialNameField.value, 
            "young_modulus":  youngModulusField.value, "poisson_ratio":  poissonRationField.value,
        }};

        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }

    cancelMaterialAddition() {
        this.defineNewMaterialName();
        const newPointNumberField = this.shadowRoot.querySelector(".material-name");
        this.dropHighlight(newPointNumberField);
        const inputtedXField = this.shadowRoot.querySelector(".young-modulus");
        this.dropHighlight(inputtedXField);
        const inputtedYField = this.shadowRoot.querySelector(".poisson-ratio");
        this.dropHighlight(inputtedYField);
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

export default FeaMaterialAddMaterialMenu;
