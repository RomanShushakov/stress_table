class FeaPropertiesAssignPropertiesMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,                 // u32;
            isFEModelLoaded: false,         // load status of wasm module "fe_model";
            lines: new Map(),               // map: { number: u32, start_point_number: u32, end_point_number: u32 }, ...};
            properties: [],                 // array of: [{ name: String, material_name: String, cross_section_name: String,
                                            //              cross_section_type: String }];
            assignedPropertiesToLines: [],  // array of: [{ name: String, 
                                            //              related_lines_data: 
                                            //                  array of [ { line_number: u32, local_axis_1_direction: [f64; 3] or null }, ...],
                                            //              related_line_elements_numbers: [u32 ...] }];
        };

        this.state = {
            selectedLines: new Set(),
            assignToLines: new Set(),
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

                .selected-lines-field-content {
                    display: flex;
                    flex-direction: column;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                }

                .selected-lines-caption-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    align-items: center;
                }

                .selected-lines-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 7rem;
                }

                .clear-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin-left: 0.8rem;
                    width: 3.5rem;
                    height: 1.5rem;
                    font-size: 70%;
                }

                .clear-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .selected-lines {
                    margin-top: 0.5rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding-left: 0.3rem;
                    width: 11rem;
                    background-color: #3b4453;
                    border: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                }

                .selected-lines:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .selected-lines:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .selected-lines-field-buttons {
                    margin-top: 0.3rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                }

                .add-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 4.5rem;
                    height: 1.5rem;
                    font-size: 70%;
                }

                .add-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .remove-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 6.5rem;
                    height: 1.5rem;
                    font-size: 70%;
                }

                .remove-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .assign-to-lines-field-content {
                    display: flex;
                    flex-direction: column;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                }

                .assign-to-lines-caption-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    align-items: center;
                }

                .assign-to-lines-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 7rem;
                }

                .assign-to-lines-field-buttons {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                }

                .preview-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin-left: 0.8rem;
                    width: 3.5rem;
                    height: 1.5rem;
                    font-size: 70%;
                }

                .preview-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .assign-to-lines {
                    margin-top: 0.5rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding-left: 0.3rem;
                    width: 11rem;
                    height: 5rem;
                    resize: none;
                    background-color: #3b4453;
                    border: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                    font-size: 90%;
                }

                /* width */
                ::-webkit-scrollbar {
                    width: 0.5rem;
                }

                /* Track */
                ::-webkit-scrollbar-track {
                    background: #2f3641;
                }

                /* Handle */
                ::-webkit-scrollbar-thumb {
                    background: #80848b;
                }

                /* Handle on hover */
                ::-webkit-scrollbar-thumb:hover {
                    background: #555555;
                }

                .assign-to-lines:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .assign-to-lines:focus {
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

                <div class="selected-lines-field-content">
                    <div class="selected-lines-caption-content">
                        <p class="selected-lines-caption">Selected lines:</p>
                        <div class="selected-lines-field-buttons">
                            <button class="clear-button">Clear</button>
                        </div>
                    </div>
                    <input class="selected-lines" type="text" placeholder="ex 1, 2, ..., etc."/>
                </div>

                <div class="selected-lines-field-buttons">
                    <button class="add-button">Add to list</button>
                    <button class="remove-button">Remove from list</button>
                </div>

                <div class="assign-to-lines-field-content">
                    <div class="assign-to-lines-caption-content">
                        <p class="assign-to-lines-caption">Assign to lines:</p>
                        <div class="assign-to-lines-field-buttons">
                            <button class="preview-button">Preview</button>
                        </div>
                    </div>
                    <textarea class="assign-to-lines" type="text" placeholder="ex 1, 2, ..., etc."/></textarea>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.assignProperties());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelPropertiesAssign());

        this.shadowRoot.querySelector(".properties-name").addEventListener("change",
            (event) => this.updateSelectedPropertiesData(event.target.value));

        this.shadowRoot.querySelector(".properties-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".properties-name-filter").value,
                this.shadowRoot.querySelector(".properties-name"));
        });

        this.shadowRoot.querySelector(".selected-lines").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".selected-lines");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".assign-to-lines").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".assign-to-lines");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".add-button").addEventListener("click", () => {
            this.addToAssignToLines();
        });

        this.shadowRoot.querySelector(".remove-button").addEventListener("click", () => {
            this.removeFromAssignToLines();
        });

        this.shadowRoot.querySelector(".clear-button").addEventListener("click", () => {
            this.state.selectedLines.clear();
            this.updateSelectedLinesField();
        });

        this.shadowRoot.querySelector(".preview-button").addEventListener("click", () => this.previewSelectedLines());
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set isFEModelLoaded(value) {
        this.props.isFEModelLoaded = value;
    }

    set lines(value) {
        this.props.lines = value;
    }

    set properties(value) {
        this.props.properties = value;
    }

    set assignedPropertiesToLines(value) {
        this.props.assignedPropertiesToLines = value;
    }

    set addLineToClient(line) {
        this.props.lines.set(line.number, { "start_point_number": line.start_point_number, "end_point_number": line.end_point_number });
    }

    set updateLineInClient(_line) {
    }

    set deleteLineFromClient(line) {
        this.props.lines.delete(line.number);
    }

    set selectLineInClientForDataAssign(lineNumber) {
        this.addToSelectedLines(lineNumber);
        this.updateSelectedLinesField();
    }

    set addPropertiesToClient(properties) {
        this.props.properties.push(properties);
        this.props.properties.sort((a, b) => a.name - b.name);
        this.definePropertiesNameOptions();
    }

    set updatePropertiesInClient(_properties) {
    }

    set deletePropertiesFromClient(properties) {
        let propertiesIndexInProps = this.props.properties
            .findIndex(existedProperties => existedProperties.name == properties.name);
        this.props.properties.splice(propertiesIndexInProps, 1);
        this.props.properties.sort((a, b) => a.name - b.name);
        this.definePropertiesNameOptions();
    }

    set addAssignedPropertiesToLinesToClient(assignedPropertiesToLines) {
        this.props.assignedPropertiesToLines.push(assignedPropertiesToLines);
        this.props.assignedPropertiesToLines.sort((a, b) => a.name - b.name);
        this.definePropertiesNameOptions();
    }

    set updateAssignedPropertiesToLinesInClient(assignedPropertiesToLines) {
        let assignedPropertiesToLinesInProps = this.props.assignedPropertiesToLines
            .find(existedAssignedPropertiesToLines => 
                existedAssignedPropertiesToLines.name == assignedPropertiesToLines.name);
        assignedPropertiesToLinesInProps.related_lines_data = assignedPropertiesToLines.related_lines_data;
        this.definePropertiesNameOptions();
    }

    set deleteAssignedPropertiesToLinesFromClient(assignedPropertiesToLines) {
        let assignedPropertiesToLinesIndexInProps = this.props.assignedPropertiesToLines
            .findIndex(existedAssignedPropertiesToLines => 
                existedAssignedPropertiesToLines.name == assignedPropertiesToLines.name);
        this.props.assignedPropertiesToLines.splice(assignedPropertiesToLinesIndexInProps, 1);
        this.props.assignedPropertiesToLines.sort((a, b) => a.name - b.name);
        this.definePropertiesNameOptions();
    }

    set rendererError(error) {
        const assignToLinesField = this.shadowRoot.querySelector(".assign-to-lines");
        if (assignToLinesField.classList.contains("highlighted") === false) {
            assignToLinesField.classList.add("highlighted");
        }
        if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = error;
        }
    }

    set feModelError(error) {
        const assignToLinesField = this.shadowRoot.querySelector(".assign-to-lines");
        if (assignToLinesField.classList.contains("highlighted") === false) {
            assignToLinesField.classList.add("highlighted");
        }
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
        document.querySelector("fea-app").dispatchEvent(new CustomEvent("enableLinesSelectionMode"));
        const frame = () => {
            this.getFEModelLoadStatus();
            if (this.props.isFEModelLoaded === true) {
                clearInterval(id);
                this.getLines();
                this.getProperties();
                this.getAssignedPropertiesToLines();
                this.definePropertiesNameOptions();
            }
        }
        const id = setInterval(frame, 10);
    }

    disconnectedCallback() {
        document.querySelector("fea-app").dispatchEvent(new CustomEvent("disableLinesSelectionMode"));
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

    getLines() {
        this.dispatchEvent(new CustomEvent("getLines", {
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

    getAssignedProperties() {
        this.dispatchEvent(new CustomEvent("getAssignedProperties", {
            bubbles: true,
            composed: true,
        }));
    }

    getAssignedPropertiesToLines() {
        this.dispatchEvent(new CustomEvent("getAssignedPropertiesToLines", {
            bubbles: true,
            composed: true,
        }));
    }

    addToSelectedLines(lineNumber) {
        const selectedLinesField = this.shadowRoot.querySelector(".selected-lines");
        let selectedLines = selectedLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        selectedLines = selectedLines.map((item) => parseInt(item));
        this.state.selectedLines = new Set(selectedLines);
        this.state.selectedLines.add(lineNumber);
    }

    addToAssignToLines() {
        const selectedLinesField = this.shadowRoot.querySelector(".selected-lines");
        let selectedLines = selectedLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < selectedLines.length; i++) {
            if (this.isNumeric(selectedLines[i]) === false || this.isInt(selectedLines[i]) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only integer numbers could be used as selected lines values!";
                }
                return;
            }
            if (this.props.lines.has(parseInt(selectedLines[i])) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed lines numbers could be used as selected lines values!";
                }
                return;
            }

        }
        selectedLines = selectedLines.map((item) => parseInt(item));
        const union = new Set([...selectedLines, ...this.state.assignToLines]);
        this.state.assignToLines = union;
        this.state.selectedLines.clear();
        this.updateSelectedLinesField();
        this.updateAssignToLinesField();
    }

    removeFromAssignToLines() {
        const selectedLinesField = this.shadowRoot.querySelector(".selected-lines");
        let selectedLines = selectedLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < selectedLines.length; i++) {
            if (this.isNumeric(selectedLines[i]) === false || this.isInt(selectedLines[i]) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only integer numbers could be used as selected lines values!";
                }
                return;
            }
            if (this.props.lines.has(parseInt(selectedLines[i])) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed lines numbers could be used as selected lines values!";
                }
                return;
            }
        }
        selectedLines = selectedLines.map((item) => parseInt(item));
        const selectedLinesSet = new Set(selectedLines);
        let difference = new Set([...this.state.assignToLines].filter((lineNumber) => !selectedLinesSet.has(lineNumber)));
        this.state.assignToLines = difference;
        this.state.selectedLines.clear();
        this.updateSelectedLinesField();
        this.updateAssignToLinesField();
    }

    updateSelectedLinesField() {
        let selectedLinesFieldValue = "";
        for (let item of this.state.selectedLines) {
            selectedLinesFieldValue += `${item}, `
        }
        this.shadowRoot.querySelector(".selected-lines").value = selectedLinesFieldValue;
    }

    updateAssignToLinesField() {
        let assignToLinesFieldValue = "";
        for (let item of this.state.assignToLines) {
            assignToLinesFieldValue += `${item}, `
        }
        this.shadowRoot.querySelector(".assign-to-lines").value = assignToLinesFieldValue;
    }

    filter(keywordField, selectField) {
        for (let i = 0; i < selectField.length; i++) {
            let txt = selectField.options[i].value;
            if (txt.substring(0, keywordField.length).toLowerCase() !== keywordField.toLowerCase() && keywordField.trim() !== "") {
                selectField.options[i].style.display = "none";
            } else {
                if (txt !== "") {
                    selectField.options[i].style.display = "list-item";
                } else {
                    selectField.options[i].style.display = "none";
                }
                
            }
        }
    }

    definePropertiesNameOptions() {
        const propertiesAssignNameSelect = this.shadowRoot.querySelector(".properties-name");
        for (let i = propertiesAssignNameSelect.length - 1; i >= 0; i--) {
            propertiesAssignNameSelect.options[i] = null;
        }
        for (let i = 0; i < this.props.properties.length; i++) {
            let assignOption = document.createElement("option");
            assignOption.value = this.props.properties[i].name.replace(/['"]+/g, "");
            assignOption.innerHTML = this.props.properties[i].name.replace(/['"]+/g, "");
            propertiesAssignNameSelect.appendChild(assignOption);
        }
        this.updateSelectedPropertiesData(propertiesAssignNameSelect.value);
    }

    updateSelectedPropertiesData(selectedPropertiesName) {
        const selectedAssignedPropertiesToLinesInProps = this.props.assignedPropertiesToLines
            .find(existedAssignedPropertiesToLines => existedAssignedPropertiesToLines.name == `"${selectedPropertiesName}"`);
        let assignToLinesFieldValue = "";
        if (selectedAssignedPropertiesToLinesInProps !== undefined) {
            let assignedToLines = new Array();
            selectedAssignedPropertiesToLinesInProps.related_lines_data.forEach(
                relatedLineData => assignedToLines.push(parseInt(relatedLineData.line_number)));
            assignedToLines.sort();
            this.state.assignToLines = new Set(assignedToLines);
            for (let i = 0; i < assignedToLines.length; i++) {
                assignToLinesFieldValue += `${assignedToLines[i]}, `
            }
        } else {
            this.state.assignToLines = new Set();
        }
        this.shadowRoot.querySelector(".assign-to-lines").value = assignToLinesFieldValue;
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    previewSelectedLines() {
        const assignToLinesField = this.shadowRoot.querySelector(".assign-to-lines");
        const assignToLines = assignToLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < assignToLines.length; i++) {
            if (this.isNumeric(assignToLines[i]) === false || this.isInt(assignToLines[i]) === false) {
                if (assignToLinesField.classList.contains("highlighted") === false) {
                    assignToLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only integer numbers could be used as assign to lines values!";
                }
                return;
            }
            assignToLines[i] = Number.parseInt(assignToLines[i]);
        }
        if (assignToLines.length > 0) {
            this.dispatchEvent(new CustomEvent("previewSelectedLineNumbers", {
                bubbles: true,
                composed: true,
                detail: { "line_numbers": assignToLines },
            }));
        }
    }

    assignProperties() {
        const equals = (a, b) => a.length === b.length && a.every((v, i) => v === b[i]);
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
        const assignToLinesField = this.shadowRoot.querySelector(".assign-to-lines");
        const assignToLines = assignToLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < assignToLines.length; i++) {
            if (this.isNumeric(assignToLines[i]) === false || this.isInt(assignToLines[i]) === false) {
                if (assignToLinesField.classList.contains("highlighted") === false) {
                    assignToLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only integer numbers could be used as assign to lines values!";
                }
                return;
            }
            assignToLines[i] = parseInt(assignToLines[i]);
        }
        const propertiesData = this.props.properties.find(properties => properties.name == `"${selectedPropertiesNameField.value}"`);
        const selectedAssignedPropertiesToLinesInProps = this.props.assignedPropertiesToLines
            .find(existedAssignedPropertiesToLines => existedAssignedPropertiesToLines.name == `"${selectedPropertiesNameField.value}"`);
        if (selectedAssignedPropertiesToLinesInProps !== undefined) {
            if (assignToLines.length === 0) {
                this.getActionId();
                const message = { 
                    "delete_assigned_properties_to_lines": { 
                        "actionId": this.props.actionId,
                        "name": propertiesData.name.replace(/['"]+/g, ""),
                    } 
                };
                this.dispatchEvent(new CustomEvent("clientMessage", {
                    bubbles: true,
                    composed: true,
                    detail: {
                        message: message,
                    },
                }));
            } else {
                let selectedAssignedPropertiesLineNumbers = Array.from(
                    Object.keys(selectedAssignedPropertiesToLinesInProps.related_lines_data));
                selectedAssignedPropertiesLineNumbers = selectedAssignedPropertiesLineNumbers.map((item) => parseInt(item));

                this.getActionId();
                const message = {
                    "update_assigned_properties_to_lines": {
                        "actionId": this.props.actionId,
                        "name": propertiesData.name.replace(/['"]+/g, ""),
                        "old_assigned_properties_to_lines_values": {
                            "line_numbers": selectedAssignedPropertiesLineNumbers,
                        },
                        "new_assigned_properties_to_lines_values": {
                            "line_numbers": assignToLines,
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
            }
        } else {
            if (assignToLines.length > 0) {
                this.getActionId();
                const message = { 
                    "add_assigned_properties_to_lines": { 
                        "actionId": this.props.actionId,
                        "name": propertiesData.name.replace(/['"]+/g, ""),
                        "line_numbers": assignToLines
                    } 
                };
                this.dispatchEvent(new CustomEvent("clientMessage", {
                    bubbles: true,
                    composed: true,
                    detail: {
                        message: message,
                    },
                }));
            } else {
                if (assignToLinesField.classList.contains("highlighted") === false) {
                    assignToLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "Note: The highlighted fields should be filled!";
                    return;
                } else {
                    return;
                }
            }
        }
        this.shadowRoot.querySelector(".properties-name-filter").value = null;
        // if (this.props.properties.length > 0) {
        //     this.definePropertiesNameOptions();
        // }
        this.state.selectedLines.clear();
        this.updateSelectedLinesField();
    }

    cancelPropertiesAssign() {
        this.shadowRoot.querySelector(".properties-name-filter").value = null;
        if (this.props.properties.length > 0) {
            this.definePropertiesNameOptions();
        }
        const selectedPropertiesNameForAssignField = this.shadowRoot.querySelector(".properties-name");
        this.dropHighlight(selectedPropertiesNameForAssignField);
        const assignToLinesField = this.shadowRoot.querySelector(".assign-to-lines");
        this.dropHighlight(assignToLinesField);
        this.state.selectedLines.clear();
        this.updateSelectedLinesField();
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

    isInt(str) {
        const n = parseFloat(str);
        return n % 1 === 0;
    }
}

export default FeaPropertiesAssignPropertiesMenu;
