class FeaSymbolsReactionsMenu extends HTMLElement {
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

                .reactions-type-radio-buttons {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: row;
                }

                .reactions-type-force-radio-button-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                }

                .reactions-type-content-force-checkboxes {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0.25rem;
                    display: flex;
                    flex-direction: column;
                }

                .resultant-force-reaction-type-checkbox-caption {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                }

                .reactions-type-moment-radio-button-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                }

                .reactions-type-content-moment-checkboxes {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0.25rem;
                    display: flex;
                    flex-direction: column;
                }

                .resultant-moment-reaction-type-checkbox-caption {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                }

                input[type="radio"] {
                    -webkit-appearance: none;
                    -moz-appearance: none;
                    width: 1rem;
                    height: 1rem;
                    border: 1px solid #6c6c6d;
                    border-radius: 50%;
                    outline: none;
                    
                }
                  
                input[type="radio"]:hover {
                    box-shadow: 0 0 0.25rem 0 #acb1ab inset;
                }
                
                input[type="radio"]:before {
                    content: "";
                    display: block;
                    width: 60%;
                    height: 60%;
                    margin: 20% auto;    
                    border-radius: 50%;    
                }
                
                input[type="radio"]:checked:before {
                    background: #0996d7;
                }

                input[type="checkbox"] {
                    -webkit-appearance: none;
                    -moz-appearance: none;
                    width: 1rem;
                    height: 1rem;
                    border: 1px solid #6c6c6d;
                    outline: none;
                    
                }
                  
                input[type="checkbox"]:hover {
                    box-shadow: 0 0 0.25rem 0 #acb1ab inset;
                }
                
                input[type="checkbox"]:before {
                    content: "";
                    display: block;
                    width: 70%;
                    height: 70%;
                    margin: 15% auto;       
                }
                
                input[type="checkbox"]:checked:before {
                    background: #0996d7;
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

            <div class="wrapper">
                <div class="reactions-type-radio-buttons">
                    <div class="reactions-type-force-radio-button-content">

                        <input type="radio" id="force-reaction-type-radio" name="reactions-type" value="force">
                        <label for="force-reaction-type-radio">Force</label>

                        <div class="reactions-type-content-force-checkboxes">
                            <div>
                                <input type="checkbox" id="fx-force-reaction-type-checkbox" name="force-reactions-type" value="fx">
                                <label for="fx-force-reaction-type-checkbox">Fx</label>
                            </div>

                            <div>
                                <input type="checkbox" id="fy-force-reaction-type-checkbox" name="force-reactions-type" value="fy">
                                <label for="fy-force-reaction-type-checkbox">Fy</label>
                            </div>

                            <div>
                                <input type="checkbox" id="fz-force-reaction-type-checkbox" name="force-reactions-type" value="fz">
                                <label for="fz-force-reaction-type-checkbox">Fz</label>
                            </div>

                            <p class="resultant-force-reaction-type-checkbox-caption">or</p>

                            <div>
                                <input type="checkbox" id="resultant-force-reaction-type-checkbox" name="force-reactions-type" 
                                    value="resultant"
                                >
                                <label for="resultant-force-reaction-type-checkbox">Resultant</label>
                            </div>
                        </div>
                    </div>

                    <div class="reactions-type-moment-radio-button-content">

                        <input type="radio" id="moment-reaction-type-radio" name="reactions-type" value="moment">
                        <label for="moment-reaction-type-radio">Moment</label>


                        <div class="reactions-type-content-moment-checkboxes">
                            <div>
                                <input type="checkbox" id="mx-moment-reaction-type-checkbox" name="moment-reactions-type" value="mx">
                                <label for="mx-moment-reaction-type-checkbox">Mx</label>
                            </div>

                            <div>
                                <input type="checkbox" id="my-moment-reaction-type-checkbox" name="moment-reactions-type" value="my">
                                <label for="my-moment-reaction-type-checkbox">My</label>
                            </div>

                            <div>
                                <input type="checkbox" id="mz-moment-reaction-type-checkbox" name="moment-reactions-type" value="mz">
                                <label for="mz-moment-reaction-type-checkbox">Mz</label>
                            </div>

                            <p class="resultant-moment-reaction-type-checkbox-caption">or</p>

                            <div>
                                <input type="checkbox" id="resultant-moment-reaction-type-checkbox" name="moment-reactions-type" 
                                    value="resultant"
                                >
                                <label for="resultant-moment-reaction-type-checkbox">Resultant</label>
                            </div>
                        </div>

                    </div>
                </div>

                <div class="apply-cancel-buttons">
                    <button class="apply-button">Apply</button>
                </div>

                <div class="analysis-info">
                    <p class="analysis-info-message"></p>
                </div>
            </div>
        `;

        this.shadowRoot.getElementById("force-reaction-type-radio").addEventListener("change", 
            (event) => {
                this.changeReactionTypeCheckboxesActivity(event.target.id);
                event.stopPropagation();
            });

        this.shadowRoot.getElementById("moment-reaction-type-radio").addEventListener("change", 
            (event) => {
                this.changeReactionTypeCheckboxesActivity(event.target.id);
                event.stopPropagation();
            });

        this.shadowRoot.getElementById("resultant-force-reaction-type-checkbox").addEventListener("change", 
            (event) => {
                this.changeForceOrMomentReactionTypeCheckboxesActivity(event.target.id);
                event.stopPropagation();
            });

        this.shadowRoot.getElementById("fx-force-reaction-type-checkbox").addEventListener("change", 
            (event) => {
                this.changeForceOrMomentReactionTypeCheckboxesActivity(event.target.id);
                event.stopPropagation();
            });

        this.shadowRoot.getElementById("fy-force-reaction-type-checkbox").addEventListener("change", 
            (event) => {
                this.changeForceOrMomentReactionTypeCheckboxesActivity(event.target.id);
                event.stopPropagation();
            });

        this.shadowRoot.getElementById("fz-force-reaction-type-checkbox").addEventListener("change", 
            (event) => {
                this.changeForceOrMomentReactionTypeCheckboxesActivity(event.target.id);
                event.stopPropagation();
            });

        this.shadowRoot.getElementById("mx-moment-reaction-type-checkbox").addEventListener("change", 
            (event) => {
                this.changeForceOrMomentReactionTypeCheckboxesActivity(event.target.id);
                event.stopPropagation();
            });

        this.shadowRoot.getElementById("my-moment-reaction-type-checkbox").addEventListener("change", 
            (event) => {
                this.changeForceOrMomentReactionTypeCheckboxesActivity(event.target.id);
                event.stopPropagation();
            });

        this.shadowRoot.getElementById("mz-moment-reaction-type-checkbox").addEventListener("change", 
            (event) => {
                this.changeForceOrMomentReactionTypeCheckboxesActivity(event.target.id);
                event.stopPropagation();
            });

        this.shadowRoot.getElementById("resultant-moment-reaction-type-checkbox").addEventListener("change", 
            (event) => {
                this.changeForceOrMomentReactionTypeCheckboxesActivity(event.target.id);
                event.stopPropagation();
            });

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.plotDisplacement());
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
                this.shadowRoot.getElementById("force-reaction-type-radio").checked = true;
                this.changeReactionTypeCheckboxesActivity("force-reaction-type-radio");
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

    changeReactionTypeCheckboxesActivity(id) {
        const fxForceReactionCheckBox = this.shadowRoot.getElementById("fx-force-reaction-type-checkbox");
        const fyForceReactionCheckBox = this.shadowRoot.getElementById("fy-force-reaction-type-checkbox");
        const fzForceReactionCheckBox = this.shadowRoot.getElementById("fz-force-reaction-type-checkbox");
        const resultantForceReactionCheckBox = this.shadowRoot.getElementById("resultant-force-reaction-type-checkbox");

        const mxMomentReactionCheckBox = this.shadowRoot.getElementById("mx-moment-reaction-type-checkbox");
        const myMomentReactionCheckBox = this.shadowRoot.getElementById("my-moment-reaction-type-checkbox");
        const mzMomentReactionCheckBox = this.shadowRoot.getElementById("mz-moment-reaction-type-checkbox");
        const resultantMomentReactionCheckBox = this.shadowRoot.getElementById("resultant-moment-reaction-type-checkbox");

        switch (id) {
            case "force-reaction-type-radio":
                fxForceReactionCheckBox.disabled = false;
                fyForceReactionCheckBox.disabled = false;
                fzForceReactionCheckBox.disabled = false;
                resultantForceReactionCheckBox.disabled = false;

                mxMomentReactionCheckBox.disabled = true;
                myMomentReactionCheckBox.disabled = true;
                mzMomentReactionCheckBox.disabled = true;
                resultantMomentReactionCheckBox.disabled = true;
                mxMomentReactionCheckBox.checked = false;
                myMomentReactionCheckBox.checked = false;
                mzMomentReactionCheckBox.checked = false;
                resultantMomentReactionCheckBox.checked = false;
                break;
            case "moment-reaction-type-radio":
                mxMomentReactionCheckBox.disabled = false;
                myMomentReactionCheckBox.disabled = false;
                mzMomentReactionCheckBox.disabled = false;
                resultantMomentReactionCheckBox.disabled = false;

                fxForceReactionCheckBox.disabled = true;
                fyForceReactionCheckBox.disabled = true;
                fzForceReactionCheckBox.disabled = true;
                resultantForceReactionCheckBox.disabled = true;
                fxForceReactionCheckBox.checked = false;
                fyForceReactionCheckBox.checked = false;
                fzForceReactionCheckBox.checked = false;
                resultantForceReactionCheckBox.checked = false;
        }
    }

    changeForceOrMomentReactionTypeCheckboxesActivity(id) {
        const fxForceReactionCheckBox = this.shadowRoot.getElementById("fx-force-reaction-type-checkbox");
        const fyForceReactionCheckBox = this.shadowRoot.getElementById("fy-force-reaction-type-checkbox");
        const fzForceReactionCheckBox = this.shadowRoot.getElementById("fz-force-reaction-type-checkbox");
        const resultantForceReactionCheckBox = this.shadowRoot.getElementById("resultant-force-reaction-type-checkbox");

        const mxMomentReactionCheckBox = this.shadowRoot.getElementById("mx-moment-reaction-type-checkbox");
        const myMomentReactionCheckBox = this.shadowRoot.getElementById("my-moment-reaction-type-checkbox");
        const mzMomentReactionCheckBox = this.shadowRoot.getElementById("mz-moment-reaction-type-checkbox");
        const resultantMomentReactionCheckBox = this.shadowRoot.getElementById("resultant-moment-reaction-type-checkbox");

        switch (id) {
            case "fx-force-reaction-type-checkbox":
                if (fxForceReactionCheckBox.checked === true) {
                    resultantForceReactionCheckBox.checked = false;
                }
                break;
            case "fy-force-reaction-type-checkbox":
                if (fyForceReactionCheckBox.checked === true) {
                    resultantForceReactionCheckBox.checked = false;
                }
                break;
            case "fz-force-reaction-type-checkbox":
                if (fzForceReactionCheckBox.checked === true) {
                    resultantForceReactionCheckBox.checked = false;
                }
                break;
            case "resultant-force-reaction-type-checkbox":
                if (resultantForceReactionCheckBox.checked === true) {
                    fxForceReactionCheckBox.checked = false;
                    fyForceReactionCheckBox.checked = false;
                    fzForceReactionCheckBox.checked = false;
                }
                break;
            case "mx-moment-reaction-type-checkbox":
                if (mxMomentReactionCheckBox.checked === true) {
                    resultantMomentReactionCheckBox.checked = false;
                }
                break;
            case "my-moment-reaction-type-checkbox":
                if (myMomentReactionCheckBox.checked === true) {
                    resultantMomentReactionCheckBox.checked = false;
                }
                break;
            case "mz-moment-reaction-type-checkbox":
                if (mzMomentReactionCheckBox.checked === true) {
                    resultantMomentReactionCheckBox.checked = false;
                }
                break;
            case "resultant-moment-reaction-type-checkbox":
                if (resultantMomentReactionCheckBox.checked === true) {
                    mxMomentReactionCheckBox.checked = false;
                    myMomentReactionCheckBox.checked = false;
                    mzMomentReactionCheckBox.checked = false;
                }
        }
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

export default FeaSymbolsReactionsMenu;
