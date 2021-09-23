class FeaContoursDisplacementMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            isRendererLoaded: false,     // load status of wasm module "renderer";
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

                .magnitude-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin: 0rem;
                }

                .magnitude-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .magnitude {
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

                .magnitude::-webkit-outer-spin-button,
                .magnitude::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .magnitude[type=number] {
                    -moz-appearance: textfield;
                }

                .magnitude:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .magnitude:focus {
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
                <div class="magnitude-field-content">
                    <p class="magnitude-caption">Magnitude</p>
                    <input class="magnitude" type="number" step="1"/>
                </div>
                
                <div class="apply-cancel-buttons">
                    <button class="apply-button">Apply</button>
                </div>

                <div class="analysis-info">
                    <p class="analysis-info-message"></p>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.plotDisplacement());

        this.shadowRoot.querySelector(".magnitude").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".magnitude");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });
    }

    set isRendererLoaded(value) {
        this.props.isRendererLoaded = value;
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
            this.getRendererLoadStatus();
            if (this.props.isRendererLoaded === true) {
                this.defineNewMagnitude();
                clearInterval(id);
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

    getRendererLoadStatus() {
        this.dispatchEvent(new CustomEvent("getRendererLoadStatus", {
            bubbles: true,
            composed: true,
        }));
    }

    defineNewMagnitude() {
        this.shadowRoot.querySelector(".magnitude").value = 1.0;
    }


    plotDisplacement() {
        const newMagnitudeField = this.shadowRoot.querySelector(".magnitude");
        if (newMagnitudeField.value === "") {
            if (newMagnitudeField.classList.contains("highlighted") === false) {
                newMagnitudeField.classList.add("highlighted");
            }
        }

        if (newMagnitudeField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(newMagnitudeField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values!";
                return;
            } else {
                return;
            }
        }

        const message = { "magnitude": newMagnitudeField.value };

        this.dispatchEvent(new CustomEvent("plotDisplacement", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
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

export default FeaContoursDisplacementMenu;
