class FeaSectionUpdateBeamMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,             // u32;
            isPropertiesLoaded: false,  // load status of wasm module "properties";
            beamSections: [],           // array of: [{ name: String, area: f64, i11: f64, i22: f64, i12: f64, it: f64 }];
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

                .I11-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .I11-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .I11 {
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

                .I11[type=number]::-webkit-outer-spin-button,
                .I11[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .I11[type=number] {
                    -moz-appearance: textfield;
                }

                .I11:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .I11:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .I22-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .I22-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .I22 {
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

                .I22[type=number]::-webkit-outer-spin-button,
                .I22[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .I22[type=number] {
                    -moz-appearance: textfield;
                }

                .I22:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .I22:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .I12-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .I12-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .I12 {
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

                .I12[type=number]::-webkit-outer-spin-button,
                .I12[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .I12[type=number] {
                    -moz-appearance: textfield;
                }

                .I12:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .I12:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .It-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .It-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .It {
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

                .It[type=number]::-webkit-outer-spin-button,
                .It[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .It[type=number] {
                    -moz-appearance: textfield;
                }

                .It:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .It:focus {
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

                <div class="area-field-content">
                    <p class="area-caption">Area</p>
                    <input class="area" type="number"/>
                </div>

                <div class="I11-field-content">
                    <p class="I11-caption">I11</p>
                    <input class="I11" type="number"/>
                </div>

                <div class="I22-field-content">
                    <p class="I22-caption">I22</p>
                    <input class="I22" type="number"/>
                </div>

                <div class="I12-field-content">
                    <p class="I12-caption">I12</p>
                    <input class="I12" type="number"/>
                </div>

                <div class="It-field-content">
                    <p class="It-caption">It</p>
                    <input class="It" type="number"/>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.updateBeamSection());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelBeamSectionUpdate());

        this.shadowRoot.querySelector(".beam-section-name").addEventListener("change", () => this.updateBeamSectionData());

        this.shadowRoot.querySelector(".beam-section-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".beam-section-name-filter").value,
                this.shadowRoot.querySelector(".beam-section-name"));
        });

        this.shadowRoot.querySelector(".area").addEventListener("click", () => {
            const inputtedArea = this.shadowRoot.querySelector(".area");
            this.dropHighlight(inputtedArea);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".I11").addEventListener("click", () => {
            const inputtedI11 = this.shadowRoot.querySelector(".I11");
            this.dropHighlight(inputtedI11);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".I22").addEventListener("click", () => {
            const inputtedI22 = this.shadowRoot.querySelector(".I22");
            this.dropHighlight(inputtedI22);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".I12").addEventListener("click", () => {
            const inputtedI12 = this.shadowRoot.querySelector(".I12");
            this.dropHighlight(inputtedI12);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".It").addEventListener("click", () => {
            const inputtedIt = this.shadowRoot.querySelector(".It");
            this.dropHighlight(inputtedIt);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set isPropertiesLoaded(value) {
        this.props.isPropertiesLoaded = value;
    }

    set beamSections(value) {
        this.props.beamSections = value;
    }

    set addBeamSectionToClient(beamSection) {
        this.props.beamSections.push(beamSection);
        this.props.beamSections.sort((a, b) => a.name - b.name);
        this.defineBeamSectionNameOptions();
    }

    set updateBeamSectionInClient(beamSection) {
        let beamSectionInProps = this.props.beamSections
            .find(existedBeamSection => existedBeamSection.name == beamSection.name);
        beamSectionInProps.area = beamSection.area;
        beamSectionInProps.i11 = beamSection.i11;
        beamSectionInProps.i22 = beamSection.i22;
        beamSectionInProps.i12 = beamSection.i12;
        beamSectionInProps.it = beamSection.it;
        this.defineBeamSectionNameOptions();
    }

    set deleteBeamSectionFromClient(beamSection) {
        let beamSectionIndexInProps = this.props.beamSections
            .findIndex(existedBeamSection => existedBeamSection.name == beamSection.name);
        this.props.beamSections.splice(beamSectionIndexInProps, 1);
        this.props.beamSections.sort((a, b) => a.name - b.name);
        this.defineBeamSectionNameOptions();
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
            this.getPropertiesLoadStatus();
            if (this.props.isPropertiesLoaded === true) {
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

    getPropertiesLoadStatus() {
        this.dispatchEvent(new CustomEvent("getPropertiesLoadStatus", {
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
        const updateBeamSectionNameSelect = this.shadowRoot.querySelector(".beam-section-name");
        for (let i = updateBeamSectionNameSelect.length - 1; i >= 0; i--) {
            updateBeamSectionNameSelect.options[i] = null;
        }
        if (this.props.beamSections.length > 0) {
            for (let i = 0; i < this.props.beamSections.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = this.props.beamSections[i].name.replace(/['"]+/g, "");
                updateOption.innerHTML = this.props.beamSections[i].name.replace(/['"]+/g, "");;
                updateBeamSectionNameSelect.appendChild(updateOption);
            }
            this.shadowRoot.querySelector(".area").value = this.props.beamSections[0].area;
            this.shadowRoot.querySelector(".I11").value = this.props.beamSections[0].i11;
            this.shadowRoot.querySelector(".I22").value = this.props.beamSections[0].i22;
            this.shadowRoot.querySelector(".I12").value = this.props.beamSections[0].i12;
            this.shadowRoot.querySelector(".It").value = this.props.beamSections[0].it;
        } else {
            this.shadowRoot.querySelector(".area").value = "";
            this.shadowRoot.querySelector(".I11").value = "";
            this.shadowRoot.querySelector(".I22").value = "";
            this.shadowRoot.querySelector(".I12").value = "";
            this.shadowRoot.querySelector(".It").value = "";
        }
    }

    updateBeamSectionData() {
        const selectedBeamSectionName = this.shadowRoot.querySelector(".beam-section-name").value;
        const beamSectionInProps = this.props.beamSections
            .find(beamSection => beamSection.name == `"${selectedBeamSectionName}"`);
        this.shadowRoot.querySelector(".area").value = beamSectionInProps.area;
        this.dropHighlight(this.shadowRoot.querySelector(".area"));
        this.shadowRoot.querySelector(".I11").value = beamSectionInProps.i11;
        this.dropHighlight(this.shadowRoot.querySelector(".I11"));
        this.shadowRoot.querySelector(".I22").value = beamSectionInProps.i22;
        this.dropHighlight(this.shadowRoot.querySelector(".I22"));
        this.shadowRoot.querySelector(".I12").value = beamSectionInProps.i12;
        this.dropHighlight(this.shadowRoot.querySelector(".I12"));
        this.shadowRoot.querySelector(".It").value = beamSectionInProps.it;
        this.dropHighlight(this.shadowRoot.querySelector(".It"));
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

    updateBeamSection() {
        const selectedBeamSectionNameField = this.shadowRoot.querySelector(".beam-section-name");
        if (selectedBeamSectionNameField.value == "") {
            if (selectedBeamSectionNameField.classList.contains("highlighted") === false) {
                selectedBeamSectionNameField.classList.add("highlighted");
            }
        }

        const inputtedAreaField = this.shadowRoot.querySelector(".area");
        if (inputtedAreaField.value == "") {
            if (inputtedAreaField.classList.contains("highlighted") === false) {
                inputtedAreaField.classList.add("highlighted");
            }
        }

        const inputtedI11Field = this.shadowRoot.querySelector(".I11");
        if (inputtedI11Field.value == "") {
            if (inputtedI11Field.classList.contains("highlighted") === false) {
                inputtedI11Field.classList.add("highlighted");
            }
        }

        const inputtedI22Field = this.shadowRoot.querySelector(".I22");
        if (inputtedI22Field.value == "") {
            if (inputtedI22Field.classList.contains("highlighted") === false) {
                inputtedI22Field.classList.add("highlighted");
            }
        }

        const inputtedI12Field = this.shadowRoot.querySelector(".I12");
        if (inputtedI12Field.value == "") {
            if (inputtedI12Field.classList.contains("highlighted") === false) {
                inputtedI12Field.classList.add("highlighted");
            }
        }

        const inputtedItField = this.shadowRoot.querySelector(".It");
        if (inputtedItField.value == "") {
            if (inputtedItField.classList.contains("highlighted") === false) {
                inputtedItField.classList.add("highlighted");
            }
        }

        if (selectedBeamSectionNameField.value === "" || inputtedAreaField.value === "" ||
            inputtedI11Field.value === "" || inputtedI22Field.value === "" ||
            inputtedI12Field.value === "" || inputtedItField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const beamSectionDataInProps = this.props.beamSections
            .find(beamSection => beamSection.area == inputtedAreaField.value && 
                beamSection.i11 == inputtedI11Field.value && beamSection.i22 == inputtedI22Field.value &&
                beamSection.i12 == inputtedI12Field.value && beamSection.it == inputtedItField.value);
        if (beamSectionDataInProps != null) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The beam section with the same data does already exist!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(inputtedAreaField.value) === false || this.isNumeric(inputtedI11Field.value) === false ||
            this.isNumeric(inputtedI22Field.value) === false || this.isNumeric(inputtedI12Field.value) === false ||
            this.isNumeric(inputtedItField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values for Area, I11, I22, I12 and It!";
                return;
            } else {
                return;
            }
        }

        const oldBeamSectionValues = this.props.beamSections
            .find(beamSection => beamSection.name == `"${selectedBeamSectionNameField.value}"`);

        this.getActionId();
        
        const message = { "update_beam_section": {
            "actionId": this.props.actionId,
            "name": selectedBeamSectionNameField.value, 
            "old_beam_section_values": { 
                "area":  oldBeamSectionValues.area,
                "i11":  oldBeamSectionValues.i11,
                "i22":  oldBeamSectionValues.i22,
                "i12":  oldBeamSectionValues.i12,
                "it":  oldBeamSectionValues.it },
            "new_beam_section_values": { 
                "area": inputtedAreaField.value,
                "i11": inputtedI11Field.value,
                "i22": inputtedI22Field.value,
                "i12": inputtedI12Field.value,
                "it": inputtedItField.value }
        }};
        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".beam-section-name-filter").value = null;
    }

    cancelBeamSectionUpdate() {
        if (this.props.beamSections.length > 0) {
            this.defineBeamSectionNameOptions();
        }
        this.shadowRoot.querySelector(".beam-section-name-filter").value = null;
        const selectedBeamSectionNameForUpdateField = this.shadowRoot.querySelector(".beam-section-name");
        this.dropHighlight(selectedBeamSectionNameForUpdateField);
        const inputtedAreaField = this.shadowRoot.querySelector(".area");
        this.dropHighlight(inputtedAreaField);
        const inputtedI11Field = this.shadowRoot.querySelector(".I11");
        this.dropHighlight(inputtedI11Field);
        const inputtedI22Field = this.shadowRoot.querySelector(".I22");
        this.dropHighlight(inputtedI22Field);
        const inputtedI12Field = this.shadowRoot.querySelector(".I12");
        this.dropHighlight(inputtedI12Field);
        const inputtedItField = this.shadowRoot.querySelector(".It");
        this.dropHighlight(inputtedItField);
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

export default FeaSectionUpdateBeamMenu;
