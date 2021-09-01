class FeaSectionDeleteBeamMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,             // u32;
            isFEModelLoaded: false,     // load status of wasm module "fe_model";
            beamSections: [],           // array of: [{ name: String, area: f64, i11: f64, i22: f64, i12: f64, it: f64, shear_factor: f64 }];
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

                .beam-section-name-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin: 0rem;
                    align-items: center;
                }

                .beam-section-name-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .beam-section-name-select-filter-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .beam-section-name-filter-label {
                    position: relative;
                }
                  
                .beam-section-name-filter-label:before {
                    content: "";
                    position: absolute;
                    left: 0rem;
                    top: 0rem;
                    bottom: 0rem;
                    width: 0.8rem;
                    background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
                }

                .beam-section-name-filter {
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

                .beam-section-name-filter::placeholder {
                    font-size: 85%;
                }

                .beam-section-name-filter:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .beam-section-name-filter:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .beam-section-name {
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

                .beam-section-name option {
                    background-color: #484f60;
                }

                .beam-section-name:hover {
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

                <div class="beam-section-name-field-content">
                    <p class="beam-section-name-caption">Section name</p>
                    <div class="beam-section-name-select-filter-content">
                        <label class="beam-section-name-filter-label">
                            <input class="beam-section-name-filter" type="text" placeholder="Filter..."/>
                        </label>
                        <select class="beam-section-name"></select>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.deleteBeamSection());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelBeamSectionDelete());

        this.shadowRoot.querySelector(".beam-section-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".beam-section-name-filter").value,
                this.shadowRoot.querySelector(".beam-section-name"));
        });
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set isFEModelLoaded(value) {
        this.props.isFEModelLoaded = value;
    }

    set beamSections(value) {
        this.props.beamSections = value;
    }

    set addBeamSectionToClient(beamSection) {
        this.props.beamSections.push(beamSection);
        this.props.beamSections.sort((a, b) => a.name - b.name);
        this.defineBeamSectionNameOptions();
    }

    set updateBeamSectionInClient(_beamSection) {
    }

    set deleteBeamSectionFromClient(beamSection) {
        let beamSectionIndexInProps = this.props.beamSections
            .findIndex(existedBeamSection => existedBeamSection.name == beamSection.name);
        this.props.beamSections.splice(beamSectionIndexInProps, 1);
        this.props.beamSections.sort((a, b) => a.name - b.name);
        this.defineBeamSectionNameOptions();
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
                this.getBeamSections();
                this.defineBeamSectionNameOptions();
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

    getBeamSections() {
        this.dispatchEvent(new CustomEvent("getBeamSections", {
            bubbles: true,
            composed: true,
        }));
    }

    defineBeamSectionNameOptions() {
        const beamSectionDeleteNameSelect = this.shadowRoot.querySelector(".beam-section-name");
        for (let i = beamSectionDeleteNameSelect.length - 1; i >= 0; i--) {
            beamSectionDeleteNameSelect.options[i] = null;
        }
        for (let i = 0; i < this.props.beamSections.length; i++) {
            let deleteOption = document.createElement("option");
            deleteOption.value = this.props.beamSections[i].name.replace(/['"]+/g, "");
            deleteOption.innerHTML = this.props.beamSections[i].name.replace(/['"]+/g, "");
            beamSectionDeleteNameSelect.appendChild(deleteOption);
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

    deleteBeamSection() {
        const selectedBeamSectionNameField = this.shadowRoot.querySelector(".beam-section-name");
        if (selectedBeamSectionNameField.value == "") {
            if (selectedBeamSectionNameField.classList.contains("highlighted") === false) {
                selectedBeamSectionNameField.classList.add("highlighted");
            }
        }

        if (selectedBeamSectionNameField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const deletedBeamSectionData = this.props.beamSections
            .find(beamSection => beamSection.name == `"${selectedBeamSectionNameField.value}"`);

        this.getActionId();
        
        const message = {"delete_beam_section": { "actionId": this.props.actionId, "name": deletedBeamSectionData.name.replace(/['"]+/g, "") }};
        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".beam-section-name-filter").value = null;
    }

    cancelBeamSectionDelete() {
        if (this.props.beamSections.length > 0) {
            this.defineBeamSectionNameOptions();
        }
        this.shadowRoot.querySelector(".beam-section-name-filter").value = null;
        const selectedBeamSectionNameForDeleteField = this.shadowRoot.querySelector(".beam-section-name");
        this.dropHighlight(selectedBeamSectionNameForDeleteField);
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }
}

export default FeaSectionDeleteBeamMenu;
