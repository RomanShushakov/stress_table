class FeaPropertiesBeamSectionOrientationMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,                         // u32;
            isFEModelLoaded: false,                 // load status of wasm module "fe_model";
            lines: new Map(),                       // map: { number: u32, start_point_number: u32, end_point_number: u32 }, ...};
            properties: [],                         // array of: [{ name: String, material_name: String, cross_section_name: String,
                                                    //              cross_section_type: String }];
            assignedPropertiesToLines: [],          // array of: [{ name: String, 
                                                    //              related_lines_data: 
                                                    //                  array of [ { line_number: u32, local_axis_1_direction: [f64; 3] or null }, ...],
                                                    //              related_nodes_numbers: [u32 ...] }];
            beamSectionsLocalAxis1Directions: [],   // array of: [[f64; 3]...];    
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

                .local-axis-1-direction-input-field-content {
                    display: flex;
                    flex-direction: column;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                }

                .local-axis-1-direction-input-caption-content {
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

                .local-axis-1-direction-input-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 12rem;
                }

                .local-axis-1-direction-input {
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

                .local-axis-1-direction-input:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .local-axis-1-direction-input:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .local-axis-1-direction-input-buttons {
                    margin-top: 0.3rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                }

                .add-inputted-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 5rem;
                    height: 1.5rem;
                    font-size: 70%;
                }

                .add-inputted-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .remove-inputted-button {
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

                .remove-inputted-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .local-axis-1-direction-field-content {
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

                .local-axis-1-direction-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 4rem;
                }

                .local-axis-1-direction-select-filter-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .local-axis-1-direction-filter-label {
                    position: relative;
                }
                  
                .local-axis-1-direction-filter-label:before {
                    content: "";
                    position: absolute;
                    left: 0rem;
                    top: 0rem;
                    bottom: 0rem;
                    width: 0.8rem;
                    background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
                }

                .local-axis-1-direction-filter {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding-left: 1.3rem;
                    width: 5.5rem;
                    background-color: #3b4453;
                    border: #4a5060;
                    border-bottom: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                }

                .local-axis-1-direction-filter::placeholder {
                    font-size: 85%;
                }

                .local-axis-1-direction-filter:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .local-axis-1-direction-filter:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .local-axis-1-direction {
                    width: 7rem;
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

                .local-axis-1-direction option {
                    background-color: #484f60;
                }

                .local-axis-1-direction:hover {
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

                .local-axis-1-direction-input-info {
                    display: flex;
                    margin: 0rem;
                    padding: 0rem;
                }

                .local-axis-1-direction-input-info-message {
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

                <div class="local-axis-1-direction-input-field-content">
                    <div class="local-axis-1-direction-input-caption-content">
                        <p class="local-axis-1-direction-input-caption">Input local axis 1 direction:</p>
                    </div>
                    <input class="local-axis-1-direction-input" type="text" placeholder="ex 1.0, 0.0, 0.0"/>
                </div>

                <div class="local-axis-1-direction-input-buttons">
                    <button class="add-inputted-button">Add inputted</button>
                    <button class="remove-inputted-button">Remove inputted</button>
                </div>

                <div class="local-axis-1-direction-input-info">
                    <p class="local-axis-1-direction-input-info-message"></p>
                </div>

                <div class="local-axis-1-direction-field-content">
                    <p class="local-axis-1-direction-caption">Local axis 1 direction</p>
                    <div class="local-axis-1-direction-select-filter-content">
                        <label class="local-axis-1-direction-filter-label">
                            <input class="local-axis-1-direction-filter" type="text" placeholder="Filter..."/>
                        </label>
                        <select class="local-axis-1-direction"></select>
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

        this.shadowRoot.querySelector(".add-inputted-button").addEventListener("click", () => 
            this.addBeamSectionLocalAxis1Direction());

        this.shadowRoot.querySelector(".remove-inputted-button").addEventListener("click", () => 
            this.removeBeamSectionLocalAxis1Direction());

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => 
            this.updateBeamSectionOrientationData());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => 
            this.cancelBeamSectionOrientationDataUpdate());

        this.shadowRoot.querySelector(".local-axis-1-direction").addEventListener("change", (event) => 
            this.updateSelectedBeamSectionOrientationData(event.target.value)
        );

        this.shadowRoot.querySelector(".local-axis-1-direction-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".local-axis-1-direction-filter").value,
                this.shadowRoot.querySelector(".local-axis-1-direction"));
        });

        this.shadowRoot.querySelector(".local-axis-1-direction-input").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".local-axis-1-direction-input");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML = "";
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
            this.updateSelectedLinesField();
        });

        this.shadowRoot.querySelector(".preview-button").addEventListener("click", () => 
            this.previewBeamSectionOrientationOnSelectedLines());

        this.shadowRoot.querySelector(".assign-to-lines").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".assign-to-lines");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });
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

    set beamSectionsLocalAxis1Directions(value) {
        this.props.beamSectionsLocalAxis1Directions = value;
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
    }

    set updatePropertiesInClient(_properties) {
    }

    set deletePropertiesFromClient(properties) {
        let propertiesIndexInProps = this.props.properties
            .findIndex(existedProperties => existedProperties.name == properties.name);
        this.props.properties.splice(propertiesIndexInProps, 1);
        this.props.properties.sort((a, b) => a.name - b.name);
    }

    set addAssignedPropertiesToLinesToClient(assignedPropertiesToLines) {
        this.props.assignedPropertiesToLines.push(assignedPropertiesToLines);
        this.props.assignedPropertiesToLines.sort((a, b) => a.name - b.name);
        this.defineLocalAxis1DirectionOptions();
    }

    set updateAssignedPropertiesToLinesInClient(assignedPropertiesToLines) {
        let assignedPropertiesToLinesInProps = this.props.assignedPropertiesToLines
            .find(existedAssignedPropertiesToLines => 
                existedAssignedPropertiesToLines.name == assignedPropertiesToLines.name);
        assignedPropertiesToLinesInProps.related_lines_data = assignedPropertiesToLines.related_lines_data;
        this.defineLocalAxis1DirectionOptions();
    }

    set deleteAssignedPropertiesToLinesFromClient(assignedPropertiesToLines) {
        let assignedPropertiesToLinesIndexInProps = this.props.assignedPropertiesToLines
            .findIndex(existedAssignedPropertiesToLines => 
                existedAssignedPropertiesToLines.name == assignedPropertiesToLines.name);
        this.props.assignedPropertiesToLines.splice(assignedPropertiesToLinesIndexInProps, 1);
        this.props.assignedPropertiesToLines.sort((a, b) => a.name - b.name);
        this.defineLocalAxis1DirectionOptions();
    }

    set addBeamSectionLocalAxis1DirectionToClient(beamSectionLocalAxis1DirectionData) {
        this.props.beamSectionsLocalAxis1Directions.push(beamSectionLocalAxis1DirectionData);
        this.props.beamSectionsLocalAxis1Directions.sort((a, b) => a - b);
        this.defineLocalAxis1DirectionOptions();
    }

    set removeBeamSectionLocalAxis1DirectionFromClient(beamSectionLocalAxis1DirectionData) {
        const equals = (a, b) => a.length === b.length && a.every((v, i) => v === b[i]);
        let beamSectionLocalAxis1DirectionIndexInProps = this.props.beamSectionsLocalAxis1Directions
            .findIndex(existedbeamSectionLocalAxis1Direction => equals(existedbeamSectionLocalAxis1Direction,
                beamSectionLocalAxis1DirectionData));
        this.props.beamSectionsLocalAxis1Directions.splice(beamSectionLocalAxis1DirectionIndexInProps, 1);
        this.props.beamSectionsLocalAxis1Directions.sort((a, b) => a - b);
        this.defineLocalAxis1DirectionOptions();
    }

    set updateBeamSectionOrientationDataInClient(_beamSectionOrientationData) {
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

    set feModelError(errorData) {
        if (errorData.message.update_beam_section_orientation_data !== undefined) {
            const assignToLinesField = this.shadowRoot.querySelector(".assign-to-lines");
            if (assignToLinesField.classList.contains("highlighted") === false) {
                assignToLinesField.classList.add("highlighted");
            }
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = errorData.error;
            }
        } else {
            const localAxis1DirectionInputField = this.shadowRoot.querySelector(".local-axis-1-direction-input");
            if (localAxis1DirectionInputField.classList.contains("highlighted") === false) {
                localAxis1DirectionInputField.classList.add("highlighted");
            }
            if (this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML = errorData.error;
            }
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
                this.getBeamSectionsLocalAxis1Directions();
                this.defineLocalAxis1DirectionOptions();
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

    getAssignedPropertiesToLines() {
        this.dispatchEvent(new CustomEvent("getAssignedPropertiesToLines", {
            bubbles: true,
            composed: true,
        }));
    }

    getBeamSectionsLocalAxis1Directions() {
        this.dispatchEvent(new CustomEvent("getBeamSectionsLocalAxis1Directions", {
            bubbles: true,
            composed: true,
        }));
    }

    addBeamSectionLocalAxis1Direction() {
        const localAxis1DirectionInputField = this.shadowRoot.querySelector(".local-axis-1-direction-input");
        let localAxis1Direction = localAxis1DirectionInputField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < localAxis1Direction.length; i++) {
            if (this.isNumeric(localAxis1Direction[i]) === false) {
                if (localAxis1DirectionInputField.classList.contains("highlighted") === false) {
                    localAxis1DirectionInputField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML = 
                        "Note: Only numbers could be used as local axis 1 direction values!";
                }
                return;
            }
        }
        localAxis1Direction = localAxis1Direction.map((item) => parseFloat(item));
        this.getActionId();
        const message = { 
            "add_beam_section_local_axis_1_direction": { 
                "actionId": this.props.actionId,
                "local_axis_1_direction": localAxis1Direction,
            } 
        };
        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
        localAxis1DirectionInputField.value = null;
    }

    removeBeamSectionLocalAxis1Direction() {
        const localAxis1DirectionInputField = this.shadowRoot.querySelector(".local-axis-1-direction-input");
        let localAxis1Direction = localAxis1DirectionInputField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < localAxis1Direction.length; i++) {
            if (this.isNumeric(localAxis1Direction[i]) === false) {
                if (localAxis1DirectionInputField.classList.contains("highlighted") === false) {
                    localAxis1DirectionInputField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML = 
                        "Note: Only numbers could be used as local axis 1 direction values!";
                }
                return;
            }
        }
        localAxis1Direction = localAxis1Direction.map((item) => parseFloat(item));
        this.getActionId();
        const message = { 
            "remove_beam_section_local_axis_1_direction": { 
                "actionId": this.props.actionId,
                "local_axis_1_direction": localAxis1Direction,
            } 
        };
        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
        localAxis1DirectionInputField.value = null;
    }

    addToSelectedLines(lineNumber) {
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
            if (this.isNumeric(selectedLines[i]) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only numbers could be used as selected lines values!";
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

    defineLocalAxis1DirectionOptions() {
        const localAxis1DirectionSelect = this.shadowRoot.querySelector(".local-axis-1-direction");
        for (let i = localAxis1DirectionSelect.length - 1; i >= 0; i--) {
            localAxis1DirectionSelect.options[i] = null;
        }
        for (let i = 0; i < this.props.beamSectionsLocalAxis1Directions.length; i++) {
            let localAxis1DirectionOption = document.createElement("option");
            localAxis1DirectionOption.value = this.props.beamSectionsLocalAxis1Directions[i];
            localAxis1DirectionOption.innerHTML = this.props.beamSectionsLocalAxis1Directions[i];
            localAxis1DirectionSelect.appendChild(localAxis1DirectionOption);
        }
        if (localAxis1DirectionSelect.value !== "") {
            this.updateSelectedBeamSectionOrientationData(localAxis1DirectionSelect.value);
        } else {
            this.state.assignToLines.clear();
            this.shadowRoot.querySelector(".assign-to-lines").value = "";
        }
    }

    updateSelectedBeamSectionOrientationData(selectedLocalAxis1Direction) {
        const localAxis1Direction = selectedLocalAxis1Direction
            .split(",")
            .map((item) => parseFloat(item))
            .filter((item) => item !== "");
        const equals = (a, b) => a.length === b.length && a.every((v, i) => v === b[i]);
        let assignedToLines = new Array();
        for (let i = 0; i < this.props.assignedPropertiesToLines.length; i++) {
            let linesNumbers = new Array();
            this.props.assignedPropertiesToLines[i].related_lines_data.forEach(
                relatedLineData => {
                        if (relatedLineData.local_axis_1_direction !== null) {
                            if (equals(relatedLineData.local_axis_1_direction, localAxis1Direction)) {
                                linesNumbers.push(parseInt(relatedLineData.line_number));
                            }
                        }
                    });
            for (let j = 0; j < linesNumbers.length; j++) {
                assignedToLines.push(linesNumbers[j]);
            }
        }
        assignedToLines.sort();
        this.state.assignToLines = new Set(assignedToLines);
        let assignToLinesFieldValue = "";
        for (let k = 0; k < assignedToLines.length; k++) {
            assignToLinesFieldValue += `${assignedToLines[k]}, `
        }
        this.shadowRoot.querySelector(".assign-to-lines").value = assignToLinesFieldValue;
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        const highlightedElement = this.shadowRoot.querySelector(".local-axis-1-direction");
        this.dropHighlight(highlightedElement);
    }

    previewBeamSectionOrientationOnSelectedLines() {
        const localAxis1DirectionSelect = this.shadowRoot.querySelector(".local-axis-1-direction");
        if (localAxis1DirectionSelect.value == "") {
            return;
        }
        const assignToLinesField = this.shadowRoot.querySelector(".assign-to-lines");
        const assignToLines = assignToLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < assignToLines.length; i++) {
            if (this.isNumeric(assignToLines[i]) === false) {
                if (assignToLinesField.classList.contains("highlighted") === false) {
                    assignToLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only numbers could be used as assign to lines values!";
                }
                return;
            }
            if (this.props.lines.has(parseInt(assignToLines[i])) === false) {
                if (assignToLinesField.classList.contains("highlighted") === false) {
                    assignToLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed lines numbers could be used as assign to lines values!";
                }
                return;
            }
            assignToLines[i] = Number.parseInt(assignToLines[i]);
        }
        if (assignToLines.length > 0) {
            const localAxis1Direction = localAxis1DirectionSelect.value
                .split(",")
                .map((item) => item.replace(/\s/g,'', ""))
                .filter((item) => item !== "")
                .map((item) => parseInt(item));
            this.dispatchEvent(new CustomEvent("previewBeamSectionOrientation", {
                bubbles: true,
                composed: true,
                detail: { 
                    "local_axis_1_direction": localAxis1Direction,
                    "line_numbers": assignToLines, 
                },
            }));
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

    updateBeamSectionOrientationData() {
        const selectedLocalAxis1DirectionField = this.shadowRoot.querySelector(".local-axis-1-direction");
        if (selectedLocalAxis1DirectionField.value === "") {
            if (selectedLocalAxis1DirectionField.classList.contains("highlighted") === false) {
                selectedLocalAxis1DirectionField.classList.add("highlighted");
            }
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const selectedLocalAxis1Direction = selectedLocalAxis1DirectionField.value
            .split(",")
            .map((item) => parseFloat(item))
            .filter((item) => item !== "");

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
            
            if (this.props.lines.has(parseInt(assignToLines[i])) === false) {
                if (assignToLinesField.classList.contains("highlighted") === false) {
                    assignToLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed lines numbers could be used as assign to lines values!";
                }
                return;
            }
        }

        const equals = (a, b) => a.length === b.length && a.every((v, i) => v === b[i]);

        let oldLineNumbers = new Array();
        for (let i = 0; i < this.props.assignedPropertiesToLines.length; i++) {
            let linesNumbers = new Array();
            this.props.assignedPropertiesToLines[i].related_lines_data.forEach(
                relatedLineData => {
                        if (relatedLineData.local_axis_1_direction !== null) {
                            if (equals(relatedLineData.local_axis_1_direction, selectedLocalAxis1Direction)) {
                                linesNumbers.push(parseInt(relatedLineData.line_number));
                            }
                        }
                    });
            for (let j = 0; j < linesNumbers.length; j++) {
                oldLineNumbers.push(linesNumbers[j]);
            }
        }

        const lineNumbers = assignToLines.map((item) => parseInt(item));

        this.getActionId();
        const message = { 
            "update_beam_section_orientation_data": { 
                "actionId": this.props.actionId,
                "local_axis_1_direction": selectedLocalAxis1Direction,
                "old_beam_section_orientation_values": {
                    "line_numbers": oldLineNumbers,
                },
                "new_beam_section_orientation_values": {
                    "line_numbers": lineNumbers,
                },
            } 
        };
        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
        this.shadowRoot.querySelector(".local-axis-1-direction-filter").value = null;
    }

    cancelBeamSectionOrientationDataUpdate() {
        this.shadowRoot.querySelector(".local-axis-1-direction-filter").value = null;
        if (this.props.beamSectionsLocalAxis1Directions.length > 0) {
            this.defineLocalAxis1DirectionOptions();
        }
        const selectedLocalAxis1DirectionForAssignField = this.shadowRoot.querySelector(".local-axis-1-direction");
        this.dropHighlight(selectedLocalAxis1DirectionForAssignField);
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

export default FeaPropertiesBeamSectionOrientationMenu;
