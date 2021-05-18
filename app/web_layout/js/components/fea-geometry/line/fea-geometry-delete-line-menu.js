class FeaGeometryDeleteLineMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
            lines: [],
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

                .line-number-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin: 0rem;
                    align-items: center;
                }

                .line-number-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .line-number-select-filter-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .line-number-filter-label {
                    position: relative;
                }
                  
                .line-number-filter-label:before {
                    content: "";
                    position: absolute;
                    left: 0rem;
                    top: 0rem;
                    bottom: 0rem;
                    width: 0.8rem;
                    background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
                }

                .line-number-filter {
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

                .line-number-filter::placeholder {
                    font-size: 85%;
                }

                .line-number-filter::-webkit-outer-spin-button,
                .line-number-filter::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .line-number-filter[type=number] {
                    -moz-appearance: textfield;
                }

                .line-number-filter:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .line-number-filter:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .line-number {
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

                .line-number option {
                    background-color: #484f60;
                }

                .line-number:hover {
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
                <div class="line-number-field-content">
                    <p class="line-number-caption">Line number</p>
                    <div class="line-number-select-filter-content">
                        <label class="line-number-filter-label">
                            <input class="line-number-filter" type="number" placeholder="Filter..."/>
                        </label>
                        <select class="line-number"></select>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.deleteLine());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelLineDelete());

        this.shadowRoot.querySelector(".line-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".line-number-filter").value,
                this.shadowRoot.querySelector(".line-number"));
        });

        this.shadowRoot.querySelector(".line-number").addEventListener("change", 
            () => this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "");
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set addLineToClient(line) {
        this.props.lines.push(line);
        this.props.lines.sort((a, b) => a.number - b.number);
        this.defineLineNumberOptions();
    }

    set updateLineInClient(_line) {
    }

    set deleteLineFromClient(line) {
        let lineIndexInProps = this.props.lines.findIndex(existedLine => existedLine.number == line.number);
        this.props.lines.splice(lineIndexInProps, 1);
        this.props.lines.sort((a, b) => a.number - b.number);
        this.defineLineNumberOptions();
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.defineLineNumberOptions();
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

    defineLineNumberOptions() {
        const lineDeleteNumberSelect = this.shadowRoot.querySelector(".line-number");
        for (let i = lineDeleteNumberSelect.length - 1; i >= 0; i--) {
            lineDeleteNumberSelect.options[i] = null;
        }
        if (this.props.lines.length > 0) {
            for (let i = 0; i < this.props.lines.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = this.props.lines[i].number;
                updateOption.innerHTML = this.props.lines[i].number;
                lineDeleteNumberSelect.appendChild(updateOption);
            }
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

    deleteLine() {
        const selectedLineNumberField = this.shadowRoot.querySelector(".line-number");
        if (selectedLineNumberField.value == "") {
            if (selectedLineNumberField.classList.contains("highlighted") === false) {
                selectedLineNumberField.classList.add("highlighted");
            }
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }
        const message = {"delete_line": {
            "actionId": this.props.actionId,
            "number": selectedLineNumberField.value, 
        }};
        this.dispatchEvent(new CustomEvent("client message", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
        this.shadowRoot.querySelector(".line-number-filter").value = null;
    }

    cancelLineDelete() {
        this.defineLineNumberOptions();
        this.shadowRoot.querySelector(".line-number-filter").value = null;
        const selectedLineNumberField = this.shadowRoot.querySelector(".line-number");
        this.dropHighlight(selectedLineNumberField);
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }
}

export default FeaGeometryDeleteLineMenu;
