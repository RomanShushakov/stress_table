class FeaPropertiesUpdatePropertiesMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,             // u32;
            isFEModelLoaded: false,     // load status of wasm module "fe_model";
            materials: [],              // array of: [{ name: String, young_modulus: f64, poisson_ratio: f64 }, ...];
            trussSections: [],          // array of: [{ name: String, area: f64, area2: f64 or null }];
            beamSections: [],           // array of: [{ name: String, area: f64, i11: f64, i22: f64, i12: f64, it: f64 }];
            properties: [],             // array of: [{ name: String, material_name: String, cross_section_name: String,
                                        //              cross_section_type: String }];
        };

        this.state = {
            crossSectionTypes: ["truss", "beam"],
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

                .cross-section-type-field-content {
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

                .cross-section-type-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .cross-section-type-select-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .cross-section-type {
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

                .cross-section-type option {
                    background-color: #484f60;
                }

                .cross-section-type:hover {
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

                .cross-section-name-field-content {
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

                .cross-section-name-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .cross-section-name-select-filter-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .cross-section-name-filter-label {
                    position: relative;
                }
                  
                .cross-section-name-filter-label:before {
                    content: "";
                    position: absolute;
                    left: 0rem;
                    top: 0rem;
                    bottom: 0rem;
                    width: 0.8rem;
                    background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
                }

                .cross-section-name-filter {
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

                .cross-section-name-filter::placeholder {
                    font-size: 85%;
                }

                .cross-section-name-filter:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .cross-section-name-filter:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .cross-section-name {
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

                .cross-section-name option {
                    background-color: #484f60;
                }

                .cross-section-name:hover {
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

                <div class="cross-section-type-field-content">
                    <p class="cross-section-type-caption">Section type</p>
                    <div class="cross-section-type-select-content">
                        <select class="cross-section-type"></select>
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

                <div class="cross-section-name-field-content">
                    <p class="cross-section-name-caption">Section name</p>
                    <div class="cross-section-name-select-filter-content">
                        <label class="cross-section-name-filter-label">
                            <input class="cross-section-name-filter" type="text" placeholder="Filter..."/>
                        </label>
                        <select class="cross-section-name"></select>
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

        this.shadowRoot.querySelector(".cross-section-type").addEventListener("change", () => this.defineSectionNameOptions());

        this.shadowRoot.querySelector(".material-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".material-name-filter").value,
                this.shadowRoot.querySelector(".material-name"));
        });

        this.shadowRoot.querySelector(".material-name").addEventListener("change",
            () => this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "");

        this.shadowRoot.querySelector(".cross-section-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".cross-section-name-filter").value,
                this.shadowRoot.querySelector(".cross-section-name"));
        });

        this.shadowRoot.querySelector(".cross-section-name").addEventListener("change",
            () => this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "");
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set isFEModelLoaded(value) {
        this.props.isFEModelLoaded = value;
    }

    set materials(value) {
        this.props.materials = value;
    }

    set trussSections(value) {
        this.props.trussSections = value;
    }

    set beamSections(value) {
        this.props.beamSections = value;
    }

    set properties(value) {
        this.props.properties = value;
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

    set addPropertiesToClient(properties) {
        this.props.properties.push(properties);
        this.props.properties.sort((a, b) => a.name - b.name);
        this.definePropertiesNameOptions();
    }

    set updatePropertiesInClient(properties) {
        let propertiesInProps = this.props.properties
            .find(existedProperties => existedProperties.name == properties.name);
        propertiesInProps.material_name = properties.material_name;
        propertiesInProps.cross_section_name = properties.cross_section_name;
        propertiesInProps.cross_section_type = properties.cross_section_type;
        this.definePropertiesNameOptions();
    }

    set deletePropertiesFromClient(properties) {
        let propertiesIndexInProps = this.props.properties
            .findIndex(existedProperties => existedProperties.name == properties.name);
        this.props.properties.splice(propertiesIndexInProps, 1);
        this.props.properties.sort((a, b) => a.name - b.name);
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
        const frame = () => {
            this.getFEModelLoadStatus();
            if (this.props.isFEModelLoaded === true) {
                clearInterval(id);
                this.getMaterials();
                this.getTrussSections();
                this.getBeamSections();
                this.getProperties();
                this.definePropertiesNameOptions();
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

    getMaterials() {
        this.dispatchEvent(new CustomEvent("getMaterials", {
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

    getBeamSections() {
        this.dispatchEvent(new CustomEvent("getBeamSections", {
            bubbles: true,
            composed: true,
        }));
    }

    getProperties() {
        this.dispatchEvent(new CustomEvent("getProperties", {
            bubbles: true,
            composed: true,
        }));
    }

    updateSelectedPropertiesData(selectedPropertiesName) {
        const selectedPropertiesInProps = this.props.properties
            .find(existedProperties => existedProperties.name == `"${selectedPropertiesName}"`);
        const selectedPropertiesSectionType = selectedPropertiesInProps.cross_section_type;
        const selectedPropertiesMaterialName = selectedPropertiesInProps.material_name;
        const selectedPropertiesSectionName = selectedPropertiesInProps.cross_section_name;
        const crossSectionTypeSelect = this.shadowRoot.querySelector(".cross-section-type");
        const crossSectionTypeOptions = crossSectionTypeSelect.options;
        for (let option, i = 0; option = crossSectionTypeOptions[i]; i++) {
            if (option.value == selectedPropertiesSectionType.replace(/['"]+/g, "")) {
                crossSectionTypeSelect.selectedIndex = i;
                break;
            }
        }
        const materialNameSelect = this.shadowRoot.querySelector(".material-name");
        const materialNameOptions = materialNameSelect.options;
        for (let option, i = 0; option = materialNameOptions[i]; i++) {
            if (option.value == selectedPropertiesMaterialName.replace(/['"]+/g, "")) {
                materialNameSelect.selectedIndex = i;
                break;
            }
        }
        this.defineSectionNameOptions();
        const sectionNameSelect = this.shadowRoot.querySelector(".cross-section-name");
        const sectionNameOptions = sectionNameSelect.options;
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
        if (this.props.properties.length > 0) {
            for (let i = 0; i < this.props.properties.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = this.props.properties[i].name.replace(/['"]+/g, "");
                updateOption.innerHTML = this.props.properties[i].name.replace(/['"]+/g, "");
                propertiesUpdateNameSelect.appendChild(updateOption);
            }
            const selectedSectionType = this.props.properties[0].cross_section_type;
            const selectedMaterialName = this.props.properties[0].material_name;
            const selectedSectionName = this.props.properties[0].cross_section_name;
            const crossSectionTypeSelect = this.shadowRoot.querySelector(".cross-section-type");
            const crossSectionTypeSelectOptions = crossSectionTypeSelect.options;
            for (let option, i = 0; option = crossSectionTypeSelectOptions[i]; i++) {
                if (option.value == selectedSectionType.replace(/['"]+/g, "")) {
                    crossSectionTypeSelect.selectedIndex = i;
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
            this.defineSectionNameOptions();
            const sectionNameSelect = this.shadowRoot.querySelector(".cross-section-name");
            const sectionNameSelectOptions = sectionNameSelect.options;
            for (let option, i = 0; option = sectionNameSelectOptions[i]; i++) {
                if (option.value == selectedSectionName.replace(/['"]+/g, "")) {
                    sectionNameSelect.selectedIndex = i;
                    break;
                }
            }
        } else {
            this.clearSectionNameOptions();
        }
    }

    defineSectionTypeOptions() {
        const crossSectionTypeSelect = this.shadowRoot.querySelector(".cross-section-type");
        for (let i = crossSectionTypeSelect.length - 1; i >= 0; i--) {
            crossSectionTypeSelect.options[i] = null;
        }
        for (let i = 0; i < this.state.crossSectionTypes.length; i++) {
            let updateOption = document.createElement("option");
            updateOption.value = this.state.crossSectionTypes[i];
            updateOption.innerHTML = this.state.crossSectionTypes[i];
            crossSectionTypeSelect.appendChild(updateOption);
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
                updateOption.innerHTML = this.props.materials[i].name.replace(/['"]+/g, "");
                materialNameSelect.appendChild(updateOption);
            }
        }
    }

    clearSectionNameOptions() {
        const sectionNameSelect = this.shadowRoot.querySelector(".cross-section-name");
        for (let i = sectionNameSelect.length - 1; i >= 0; i--) {
            sectionNameSelect.options[i] = null;
        }
    }

    defineSectionNameOptions() {
        const sectionNameSelect = this.shadowRoot.querySelector(".cross-section-name");
        for (let i = sectionNameSelect.length - 1; i >= 0; i--) {
            sectionNameSelect.options[i] = null;
        }
        const crossSectionType = this.shadowRoot.querySelector(".cross-section-type");
        switch (crossSectionType.value) {
            case "truss":
                if (this.props.trussSections.length > 0) {
                    for (let i = 0; i < this.props.trussSections.length; i++) {
                        let updateOption = document.createElement("option");
                        updateOption.value = this.props.trussSections[i].name.replace(/['"]+/g, "");
                        updateOption.innerHTML = this.props.trussSections[i].name.replace(/['"]+/g, "");
                        sectionNameSelect.appendChild(updateOption);
                    }
                }
                break;
            case "beam":
                if (this.props.beamSections.length > 0) {
                    for (let i = 0; i < this.props.beamSections.length; i++) {
                        let updateOption = document.createElement("option");
                        updateOption.value = this.props.beamSections[i].name.replace(/['"]+/g, "");
                        updateOption.innerHTML = this.props.beamSections[i].name.replace(/['"]+/g, "");
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

        const crossSectionNameField = this.shadowRoot.querySelector(".cross-section-name");
        if (crossSectionNameField.value == "") {
            if (crossSectionNameField.classList.contains("highlighted") === false) {
                crossSectionNameField.classList.add("highlighted");
            }
        }

        if (selectedPropertiesNameField.value == "" || materialNameField.value == "" || crossSectionNameField.value == "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML =
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const crossSectionTypeField = this.shadowRoot.querySelector(".cross-section-type");

        const propertiesDataInProps = this.props.properties.find(properties =>
            properties.material_name == `"${materialNameField.value}"` &&
            properties.cross_section_type == `"${crossSectionTypeField.value}"` &&
            properties.cross_section_name == `"${crossSectionNameField.value}"`);
        if (propertiesDataInProps != null) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML =
                    "Note: The properties with the same data does already exist!";
                return;
            } else {
                return;
            }
        }

        this.getActionId();

        const oldPropertiesValues = this.props.properties.find(properties => properties.name == `"${selectedPropertiesNameField.value}"`);
        const message = {
            "update_properties": {
                "actionId": this.props.actionId,
                "name": selectedPropertiesNameField.value,
                "old_properties_values": {
                    "material_name": oldPropertiesValues.material_name.replace(/['"]+/g, ""),
                    "cross_section_name": oldPropertiesValues.cross_section_name.replace(/['"]+/g, ""),
                    "cross_section_type": oldPropertiesValues.cross_section_type.replace(/['"]+/g, ""),
                },
                "new_properties_values": {
                    "material_name": materialNameField.value,
                    "cross_section_name": crossSectionNameField.value,
                    "cross_section_type": crossSectionTypeField.value,
                }
            }
        };
        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
        this.shadowRoot.querySelector(".properties-name-filter").value = null;
        this.shadowRoot.querySelector(".material-name-filter").value = null;
        this.shadowRoot.querySelector(".cross-section-name-filter").value = null;
    }

    cancelPropertiesUpdate() {
        this.definePropertiesNameOptions();
        this.shadowRoot.querySelector(".properties-name-filter").value = null;
        this.shadowRoot.querySelector(".material-name-filter").value = null;
        this.shadowRoot.querySelector(".cross-section-name-filter").value = null;
        const selectedPropertiesNameField = this.shadowRoot.querySelector(".properties-name");
        this.dropHighlight(selectedPropertiesNameField);
        const materialNameField = this.shadowRoot.querySelector(".material-name");
        this.dropHighlight(materialNameField);
        const sectionName = this.shadowRoot.querySelector(".cross-section-name");
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
