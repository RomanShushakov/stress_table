class FeaPropertiesUpdatePropertiesMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,     // u32;
            materials: [],      // array of: [{ name: String, youngModulus: f64, poissonRatio: f64 }, ...];
            trussSections: [],  // array of: [{ name: String, area: f64, area2: f64 or null }];
            beamSections: [],   // array of: [{ name: String, area: f64, I11: f64, I22: f64, I12: f64, It: f64 }];
            properties: [],     // array of: [{ name: String, materialName: String, sectionName: String,
                                //              sectionType: String }];
        };

        this.state = {
            sectionTypes: ["truss", "beam"],
        };

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

                .section-type-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0;
                    margin-left: 0;
                    margin-right: 0;
                    align-items: center;
                }

                .section-type-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .section-type-select-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .section-type {
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

                .section-type option {
                    background-color: #484f60;
                }

                .section-type:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .material-name-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    align-items: center;
                }

                .material-name-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .material-name-select-filter-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .material-name-filter-label {
                    position: relative;
                }
                  
                .material-name-filter-label:before {
                    content: "";
                    position: absolute;
                    left: 0rem;
                    top: 0rem;
                    bottom: 0rem;
                    width: 0.8rem;
                    background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
                }

                .material-name-filter {
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

                .material-name-filter::placeholder {
                    font-size: 85%;
                }

                .material-name-filter:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .material-name-filter:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .material-name {
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

                .material-name option {
                    background-color: #484f60;
                }

                .material-name:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .section-name-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    align-items: center;
                }

                .section-name-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .section-name-select-filter-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .section-name-filter-label {
                    position: relative;
                }
                  
                .section-name-filter-label:before {
                    content: "";
                    position: absolute;
                    left: 0rem;
                    top: 0rem;
                    bottom: 0rem;
                    width: 0.8rem;
                    background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
                }

                .section-name-filter {
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

                .section-name-filter::placeholder {
                    font-size: 85%;
                }

                .section-name-filter:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .section-name-filter:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .section-name {
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

                .section-name option {
                    background-color: #484f60;
                }

                .section-name:hover {
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

                <div class="section-type-field-content">
                    <p class="section-type-caption">Section type</p>
                    <div class="section-type-select-content">
                        <select class="section-type" disabled></select>
                    </div>
                </div>

                <div class="material-name-field-content">
                    <p class="material-name-caption">Material name</p>
                    <div class="material-name-select-filter-content">
                        <label class="material-name-filter-label">
                            <input class="material-name-filter" type="text" placeholder="Filter..."/>
                        </label>
                        <select class="material-name"></select>
                    </div>
                </div>

                <div class="section-name-field-content">
                    <p class="section-name-caption">Section name</p>
                    <div class="section-name-select-filter-content">
                        <label class="section-name-filter-label">
                            <input class="section-name-filter" type="text" placeholder="Filter..."/>
                        </label>
                        <select class="section-name"></select>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.updateProperties());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelPropertiesUpdate());

        this.shadowRoot.querySelector(".properties-name").addEventListener("change", 
            (event) => this.updateSelectedPropertiesData(event.target.value));

        this.shadowRoot.querySelector(".properties-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".properties-name-filter").value,
                this.shadowRoot.querySelector(".properties-name"));
        });

        this.shadowRoot.querySelector(".material-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".material-name-filter").value,
                this.shadowRoot.querySelector(".material-name"));
        });

        this.shadowRoot.querySelector(".material-name").addEventListener("change", 
            () => this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "");

        this.shadowRoot.querySelector(".section-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".section-name-filter").value,
                this.shadowRoot.querySelector(".section-name"));
        });

        this.shadowRoot.querySelector(".section-name").addEventListener("change", 
            () => this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "");
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set addMaterialToClient(material) {
        this.props.materials.push(material);
        this.props.materials.sort((a, b) => a.name - b.name);
        this.definePropertiesNameOptions();
    }

    set updateMaterialInClient(_material) {
    }

    set deleteMaterialFromClient(material) {
        let materialIndexInProps = this.props.materials.findIndex(existedMaterial => existedMaterial.name == material.name);
        this.props.materials.splice(materialIndexInProps, 1);
        this.props.materials.sort((a, b) => a.name - b.name);
        this.definePropertiesNameOptions();
    }

    set addTrussSectionToClient(trussSection) {
        this.props.trussSections.push(trussSection);
        this.props.trussSections.sort((a, b) => a.name - b.name);
        this.definePropertiesNameOptions();
    }

    set updateTrussSectionInClient(_trussSection) {
    }

    set deleteTrussSectionFromClient(trussSection) {
        let trussSectionIndexInProps = this.props.trussSections
            .findIndex(existedTrussSection => existedTrussSection.name == trussSection.name);
        this.props.trussSections.splice(trussSectionIndexInProps, 1);
        this.props.trussSections.sort((a, b) => a.name - b.name);
        this.definePropertiesNameOptions();
    }

    set addBeamSectionToClient(beamSection) {
        this.props.beamSections.push(beamSection);
        this.props.beamSections.sort((a, b) => a.name - b.name);
        this.definePropertiesNameOptions();
    }

    set updateBeamSectionInClient(_beamSection) {
    }

    set deleteBeamSectionFromClient(beamSection) {
        let beamSectionIndexInProps = this.props.beamSections
            .findIndex(existedBeamSection => existedBeamSection.name == beamSection.name);
        this.props.beamSections.splice(beamSectionIndexInProps, 1);
        this.props.beamSections.sort((a, b) => a.name - b.name);
        this.definePropertiesNameOptions();
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.definePropertiesNameOptions();
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

    updateSelectedPropertiesData(selectedPropertiesName) {
        const selectedPropertiesInProps = this.props.properties
            .findIndex(existedProperties => existedProperties.name == `"${selectedPropertiesName}"`);
        const selectedPropertiesSectionType = selectedPropertiesInProps.sectionType;
        const selectedPropertiesMaterialName = selectedPropertiesInProps.materialName;
        const selectedPropertiesSectionName = selectedPropertiesInProps.sectionName;
        const sectionTypeSelect = this.shadowRoot.querySelector(".section-type");
        const sectionTypeOptions =  sectionTypeSelect.options;
        for (let option, i = 0; option = sectionTypeOptions[i]; i++) {
            if (option.value == selectedPropertiesSectionType.replace(/['"]+/g, "")) {
                sectionTypeSelect.selectedIndex = i;
                break;
            }
        }       
        const materialNameSelect =  this.shadowRoot.querySelector(".material-name");
        const materialNameOptions =  materialNameSelect.options;
        for (let option, i = 0; option = materialNameOptions[i]; i++) {
            if (option.value == selectedPropertiesMaterialName.replace(/['"]+/g, "")) {
                materialNameSelect.selectedIndex = i;
                break;
            }
        }
        const sectionNameSelect =  this.shadowRoot.querySelector(".section-name");
        const sectionNameOptions =  sectionNameSelect.options;
        for (let option, i = 0; option = sectionNameOptions[i]; i++) {
            if (option.value == selectedPropertiesSectionName.replace(/['"]+/g, "")) {
                sectionNameSelect.selectedIndex = i;
                break;
            }
        }
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    definePropertiesNameOptions() {
        const propertiesUpdateNameSelect = this.shadowRoot.querySelector(".properties-name");
        for (let i = propertiesUpdateNameSelect.length - 1; i >= 0; i--) {
            propertiesUpdateNameSelect.options[i] = null;
        }
        this.defineSectionTypeOptions();
        this.defineMaterialNameOptions();
        this.defineSectionNameOptions();
        if (this.props.properties.length > 0) {
            for (let i = 0; i < this.props.properties.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = this.props.properties[i];
                updateOption.innerHTML = this.props.properties[i];
                propertiesUpdateNameSelect.appendChild(updateOption);
            }
            const selectedSectionType = this.props.properties[0].sectionType;
            const selectedMaterialName = this.props.properties[0].materialName;
            const selectedSectionName = this.props.properties[0].sectionName;
            const sectionTypeSelect = this.shadowRoot.querySelector(".section-type");
            const sectionTypeSelectOptions = sectionTypeSelect.options;
            for (let option, i = 0; option = sectionTypeSelectOptions[i]; i++) {
                if (option.value == selectedSectionType.replace(/['"]+/g, "")) {
                    sectionTypeSelect.selectedIndex = i;
                    break;
                }
            }
            const materialNameSelect = this.shadowRoot.querySelector(".material-name");
            const materialNameSelectOptions = materialNameSelect.options;
            for (let option, i = 0; option = materialNameSelectOptions[i]; i++) {
                if (option.value == selectedMaterialName.replace(/['"]+/g, "")) {
                    materialNameSelect.selectedIndex = i;
                    break;
                }
            }
            const sectionNameSelect = this.shadowRoot.querySelector(".section-name");
            const sectionNameSelectOptions = sectionNameSelect.options;
            for (let option, i = 0; option = sectionNameSelectOptions[i]; i++) {
                if (option.value == selectedSectionName.replace(/['"]+/g, "")) {
                    sectionNameSelect.selectedIndex = i;
                    break;
                }
            }
        }
    }

    defineSectionTypeOptions() {
        const sectionTypeSelect = this.shadowRoot.querySelector(".section-type");
        for (let i = sectionTypeSelect.length - 1; i >= 0; i--) {
            sectionTypeSelect.options[i] = null;
        }
        for (let i = 0; i < this.state.sectionTypes.length; i++) {
            let updateOption = document.createElement("option");
            updateOption.value = this.state.sectionTypes[i];
            updateOption.innerHTML = this.state.sectionTypes[i];
            sectionTypeSelect.appendChild(updateOption);
        }
    }

    defineNewPropertiesName() {
        const newPropertiesName = "prop1";
        this.shadowRoot.querySelector(".properties-name").value = newPropertiesName;
        const sectionTypeSelect = this.shadowRoot.querySelector(".section-type");
        for (let i = sectionTypeSelect.length - 1; i >= 0; i--) {
            sectionTypeSelect.options[i] = null;
        }
        for (let i = 0; i < this.state.sectionTypes.length; i++) {
            let updateOption = document.createElement("option");
            updateOption.value = this.state.sectionTypes[i];
            updateOption.innerHTML = this.state.sectionTypes[i];
            sectionTypeSelect.appendChild(updateOption);
        }
    }

    defineMaterialNameOptions() {
        const materialNameSelect = this.shadowRoot.querySelector(".material-name");
        for (let i = materialNameSelect.length - 1; i >= 0; i--) {
            materialNameSelect.options[i] = null;
        }
        if (this.props.materials.length > 0) {
            for (let i = 0; i < this.props.materials.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = this.props.materials[i].name.replace(/['"]+/g, "");
                updateOption.innerHTML = this.props.materials[i].name.replace(/['"]+/g, "");;
                materialNameSelect.appendChild(updateOption);
            }
        }
    }

    defineSectionNameOptions() {
        const sectionNameSelect = this.shadowRoot.querySelector(".section-name");
        for (let i = sectionNameSelect.length - 1; i >= 0; i--) {
            sectionNameSelect.options[i] = null;
        }
        const sectionType = this.shadowRoot.querySelector(".section-type");
        switch (sectionType.value) {
            case "truss":
                if (this.props.trussSections.length > 0) {
                    for (let i = 0; i < this.props.trussSections.length; i++) {
                        let updateOption = document.createElement("option");
                        updateOption.value = this.props.trussSections[i].name.replace(/['"]+/g, "");
                        updateOption.innerHTML = this.props.trussSections[i].name.replace(/['"]+/g, "");;
                        sectionNameSelect.appendChild(updateOption);
                    }
                }
                break;
            case "beam":
                if (this.props.beamSections.length > 0) {
                    for (let i = 0; i < this.props.beamSections.length; i++) {
                        let updateOption = document.createElement("option");
                        updateOption.value = this.props.beamSections[i].name.replace(/['"]+/g, "");
                        updateOption.innerHTML = this.props.beamSections[i].name.replace(/['"]+/g, "");;
                        sectionNameSelect.appendChild(updateOption);
                    }
                }
                break;
        }
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

    updateProperties() {
        const selectedPropertiesNameField = this.shadowRoot.querySelector(".properties-name");
        if (selectedPropertiesNameField.value == "") {
            if (selectedPropertiesNameField.classList.contains("highlighted") === false) {
                selectedPropertiesNameField.classList.add("highlighted");
            }
        }

        const materialNameField = this.shadowRoot.querySelector(".material-name");
        if (materialNameField.value == "") {
            if (materialNameField.classList.contains("highlighted") === false) {
                materialNameField.classList.add("highlighted");
            }
        }

        const sectionNameField = this.shadowRoot.querySelector(".section-name");
        if (sectionNameField.value == "") {
            if (sectionNameField.classList.contains("highlighted") === false) {
                sectionNameField.classList.add("highlighted");
            }
        }

        if (selectedPropertiesNameField.value == "" || materialNameField.value == "" || sectionNameField.value == "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const sectionTypeField = this.shadowRoot.querySelector(".section-type");

        const propertiesDataInProps = this.props.properties.find(properties => 
            properties.materialName == `"${materialNameField.value}"` && 
            properties.sectionType == `"${sectionTypeField.value}"` &&
            properties.sectionName == `"${sectionNameField.value}"`);
        if (propertiesDataInProps != null) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The properties with the same data does already exist!";
                return;
            } else {
                return;
            }
        }

        const oldPropertiesValues = this.props.properties.find(properties => properties.name == `"${selectedPropertiesNameField.value}"`);
        const message = {"update_properties": {
            "actionId": this.props.actionId,
            "name": selectedPropertiesNameField.value, 
            "old_properties_values": { 
                "material_name":  oldPropertiesValues.materialName, 
                "section_name": oldPropertiesValues.sectionName,
            },
            "new_properties_values": { 
                "material_name":  materialNameField.value, 
                "section_name": sectionNameField.value,
            }
        }};
        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
        this.shadowRoot.querySelector(".material-name-filter").value = null;
        this.shadowRoot.querySelector(".section-name-filter").value = null;
    }

    cancelPropertiesUpdate() {
        this.definePropertiesNameOptions();
        this.shadowRoot.querySelector(".material-name-filter").value = null;
        this.shadowRoot.querySelector(".section-name-filter").value = null;
        const selectedPropertiesNameField = this.shadowRoot.querySelector(".properties-name");
        this.dropHighlight(selectedPropertiesNameField);
        const materialNameField = this.shadowRoot.querySelector(".material-name");
        this.dropHighlight(materialNameField);
        const sectionName = this.shadowRoot.querySelector(".section-name");
        this.dropHighlight(sectionName);
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }
}

export default FeaPropertiesUpdatePropertiesMenu;
