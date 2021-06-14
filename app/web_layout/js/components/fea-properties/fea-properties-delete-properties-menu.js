class FeaPropertiesDeletePropertiesMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,     // u32;
            properties: [],     // array of: [{ name: String, materialName: String, sectionName: String,
            //              sectionType: String, usedIn: [u32, ...] }];
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

                .properties-name-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin: 0rem;
                    align-items: center;
                }

                .properties-name-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .properties-name-select-filter-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .properties-name-filter-label {
                    position: relative;
                }
                  
                .properties-name-filter-label:before {
                    content: "";
                    position: absolute;
                    left: 0rem;
                    top: 0rem;
                    bottom: 0rem;
                    width: 0.8rem;
                    background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
                }

                .properties-name-filter {
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

                .properties-name-filter::placeholder {
                    font-size: 85%;
                }

                .properties-name-filter:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .properties-name-filter:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .properties-name {
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

                .properties-name option {
                    background-color: #484f60;
                }

                .properties-name:hover {
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

                <div class="properties-name-field-content">
                    <p class="properties-name-caption">Properties name</p>
                    <div class="properties-name-select-filter-content">
                        <label class="properties-name-filter-label">
                            <input class="properties-name-filter" type="text" placeholder="Filter..."/>
                        </label>
                        <select class="properties-name"></select>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.deleteProperties());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelPropertiesDelete());

        this.shadowRoot.querySelector(".properties-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".properties-name-filter").value,
                this.shadowRoot.querySelector(".properties-name"));
        });
    }

    set actionId(value) {
        this.props.actionId = value;
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

    definePropertiesNameOptions() {
        const propertiesDeleteNameSelect = this.shadowRoot.querySelector(".properties-name");
        for (let i = propertiesDeleteNameSelect.length - 1; i >= 0; i--) {
            propertiesDeleteNameSelect.options[i] = null;
        }
        for (let i = 0; i < this.props.properties.length; i++) {
            let deleteOption = document.createElement("option");
            deleteOption.value = this.props.properties[i].name.replace(/['"]+/g, "");
            deleteOption.innerHTML = this.props.properties[i].name.replace(/['"]+/g, "");
            propertiesDeleteNameSelect.appendChild(deleteOption);
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

    deleteProperties() {
        const selectedPropertiesNameField = this.shadowRoot.querySelector(".properties-name");
        if (selectedPropertiesNameField.value == "") {
            if (selectedPropertiesNameField.classList.contains("highlighted") === false) {
                selectedPropertiesNameField.classList.add("highlighted");
            }
        }

        if (selectedPropertiesNameField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const deletedPropertiesData = this.props.properties.find(properties => properties.name == `"${selectedPropertiesNameField.value}"`);
        const message = { "delete_properties": { "actionId": this.props.actionId, "name": deletedPropertiesData.name.replace(/['"]+/g, "") } };
        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".properties-name-filter").value = null;
    }

    cancelPropertiesDelete() {
        if (this.props.properties.length > 0) {
            this.definePropertiesNameOptions();
        }
        this.shadowRoot.querySelector(".properties-name-filter").value = null;
        const selectedPropertiesNameForDeleteField = this.shadowRoot.querySelector(".properties-name");
        this.dropHighlight(selectedPropertiesNameForDeleteField);
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }
}

export default FeaPropertiesDeletePropertiesMenu;
