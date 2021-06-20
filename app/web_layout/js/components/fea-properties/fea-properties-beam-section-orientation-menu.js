class FeaPropertiesBeamSectionOrientationMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,                 // u32;
            assignedProperties: [],         // array of: [{ name: String, lineNumbers: [u32...] }];
            beamSectionsOrientations: [],   // array of: [{ propertiesName: String, localAxis1Direction: [f64; 3],
                                            //              lineNumbers: [u32...] }];
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.deleteProperties());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelPropertiesAssign());

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

        this.shadowRoot.querySelector(".add-button").addEventListener("click", () => {
            this.addToAssignToLines();
        });

        this.shadowRoot.querySelector(".remove-button").addEventListener("click", () => {
            this.removeFromAssignToLines();
        });

        this.shadowRoot.querySelector(".clear-button").addEventListener("click", () => {
            this.state.selectedLines.clear();
            this.updateSelecedLinesField();
        });
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set addLineToClient(line) {
        this.props.lines.set(line.number, { "startPointNumber": line.startPointNumber, "endPointNumber": line.endPointNumber });
    }

    set updateLineInClient(_line) {
    }

    set deleteLineFromClient(line) {
        this.props.lines.delete(line.number);
    }

    set selectLineInClientForDataAssign(lineNumber) {
        this.addToSelectedLines(lineNumber);
        this.updateSelecedLinesField();
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        document.querySelector("fea-app").dispatchEvent(new CustomEvent("enableLinesMultipleSelectionMode"));
    }

    disconnectedCallback() {
        document.querySelector("fea-app").dispatchEvent(new CustomEvent("disableLinesMultipleSelectionMode"));
    }

    static get observedAttributes() {
        return [];
    }

    attributeChangedCallback(name, oldValue, newValue) {
    }

    adoptedCallback() {
    }

    addToSelectedLines(lineNumber) {
        const selectedLinesField = this.shadowRoot.querySelector(".selected-lines");
        let selectedLines = selectedLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < selectedLines.length; i++) {
            if (this.isNumeric(selectedLines[i]) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only numbers could be used as selected lines values!";;
                }
                return;
            }
            if (this.props.lines.has(parseInt(selectedLines[i])) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed lines numbers could be used as selected lines values!";;
                }
                return;
            }

        }
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
            if (this.isNumeric(selectedLines[i]) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only numbers could be used as selected lines values!";;
                }
                return;
            }
            if (this.props.lines.has(parseInt(selectedLines[i])) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed lines numbers could be used as selected lines values!";;
                }
                return;
            }

        }
        selectedLines = selectedLines.map((item) => parseInt(item));
        const union = new Set([...selectedLines, ...this.state.assignToLines]);
        this.state.assignToLines = union;
        this.state.selectedLines = new Set(selectedLines);
        this.updateSelecedLinesField();
        this.updateAssignToLinesField();
    }

    removeFromAssignToLines() {
        const selectedLinesField = this.shadowRoot.querySelector(".selected-lines");
        let selectedLines = selectedLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < selectedLines.length; i++) {
            if (this.isNumeric(selectedLines[i]) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only numbers could be used as selected lines values!";;
                }
                return;
            }
            if (this.props.lines.has(parseInt(selectedLines[i])) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed lines numbers could be used as selected lines values!";;
                }
                return;
            }
        }
        selectedLines = selectedLines.map((item) => parseInt(item));
        const selectedLinesSet = new Set(selectedLines);
        let difference = new Set([...this.state.assignToLines].filter((lineNumber) => !selectedLinesSet.has(lineNumber)));
        this.state.assignToLines = difference;
        this.state.selectedLines = selectedLinesSet;
        this.updateSelecedLinesField();
        this.updateAssignToLinesField();
    }

    updateSelecedLinesField() {
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
                if (txt !== "") {
                    selectField.options[i].style.display = "list-item";
                } else {
                    selectField.options[i].style.display = "none";
                }
                
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
        const assignToLinesField = this.shadowRoot.querySelector(".assign-to-lines");
        const assignToLines = assignToLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < assignToLinesField.length; i++) {
            if (this.isNumeric(assignToLines[i]) === false) {
                if (assignToLinesField.classList.contains("highlighted") === false) {
                    assignToLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only numbers could be used as assign to lines values!";;
                }
                return;
            }
            if (this.props.lines.has(parseInt(assignToLines[i])) === false) {
                if (assignToLinesField.classList.contains("highlighted") === false) {
                    assignToLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed lines numbers could be used as assign to lines values!";;
                }
                return;
            }
        }
        const assignPropertiesData = this.props.properties.find(properties => properties.name == `"${selectedPropertiesNameField.value}"`);
        const message = { 
            "assign_properties": { 
                "actionId": this.props.actionId,
                "name": assignPropertiesData.name.replace(/['"]+/g, ""),
                "usedIn": assignToLines
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
    }

    cancelPropertiesAssign() {
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

    isNumeric(str) {
        if (typeof str != "string") {
            return false;
        }
        return !isNaN(str) && !isNaN(parseFloat(str));
    }
}

export default FeaPropertiesBeamSectionOrientationMenu;
