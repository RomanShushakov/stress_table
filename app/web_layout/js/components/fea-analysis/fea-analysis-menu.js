class FeaAnalysisMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            isFEModelLoaded: false,     // load status of wasm module "fe_model";
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
                    padding: 1rem;
                }

                .analysis-menu-caption {
                    margin: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0.3rem;
                    padding-left: 0rem;
                    padding-right: 0rem;
                    color: #D9D9D9;
                    border-bottom: 0.1rem solid #4a5060;
                    font-size: 85%;
                }

                .analysis-buttons {
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    align-self: center;
                }

                .check-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 5rem;
                    height: 1.7rem;
                }

                .check-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .analyze-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 5rem;
                    height: 1.7rem;
                }

                .analyze-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .analysis-info {
                    display: flex;
                    margin: 0rem;
                    padding: 0rem;
                    flex-direction: column;
                    align-items: center;
                }

                .analysis-info-message {
                    margin-top: 1rem;
                    margin-bottom: 0.5rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 80%;
                    width: 12rem;
                }

                .hide-message-button {
                    background: #3d5d78;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 6rem;
                    height: 1.7rem;
                    font-size: 70%;
                }

                .hide-message-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .show-result-button {
                    background: #3d5d78;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 6rem;
                    height: 1.7rem;
                    font-size: 70%;
                }

                .show-result-button:hover {
                    border: 0.2rem solid #4a5060;
                }

            </style>
            <div class=wrapper>
                <p class="analysis-menu-caption">Analysis</p>

                <div class="analysis-buttons">
                    <button class="check-button">Check</button>
                    <button class="analyze-button">Analyze</button>
                </div>

                <div class="analysis-info">
                    <p class="analysis-info-message"></p>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".check-button").addEventListener("click", () => this.checkModel());

        this.shadowRoot.querySelector(".analyze-button").addEventListener("click", () => this.analyzeModel());
    }

    set isFEModelLoaded(value) {
        this.props.isFEModelLoaded = value;
    }

    set feModelError(error) {
        if (this.shadowRoot.querySelector(".hide-message-button") != undefined) {
            this.shadowRoot.querySelector(".hide-message-button").remove();
        }
        if (this.shadowRoot.querySelector(".show-result-button") != undefined) {
            this.shadowRoot.querySelector(".show-result-button").remove();
        }

        if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = error;
        } else {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = error;
        }
        const hideMessageButton = document.createElement("button");
        hideMessageButton.className = "hide-message-button";
        hideMessageButton.innerHTML = "Hide message";
        hideMessageButton.addEventListener("click", () => {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
            this.shadowRoot.querySelector(".hide-message-button").remove();
        });
        this.shadowRoot.querySelector(".analysis-info").append(hideMessageButton);
    }

    set feModelCheckSuccess(message) {
        if (this.shadowRoot.querySelector(".hide-message-button") != undefined) {
            this.shadowRoot.querySelector(".hide-message-button").remove();
        }
        if (this.shadowRoot.querySelector(".show-result-button") != undefined) {
            this.shadowRoot.querySelector(".show-result-button").remove();
        }

        if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = message;
        } else {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = message;
        }
        const hideMessageButton = document.createElement("button");
        hideMessageButton.className = "hide-message-button";
        hideMessageButton.innerHTML = "Hide message";
        hideMessageButton.addEventListener("click", () => {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
            this.shadowRoot.querySelector(".hide-message-button").remove();
        });
        this.shadowRoot.querySelector(".analysis-info").append(hideMessageButton);
    }

    set feModelAnalysisSuccess(message) {
        if (this.shadowRoot.querySelector(".hide-message-button") != undefined) {
            this.shadowRoot.querySelector(".hide-message-button").remove();
        }
        if (this.shadowRoot.querySelector(".show-result-button") != undefined) {
            this.shadowRoot.querySelector(".show-result-button").remove();
        }

        if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = message;
        } else {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = message;
        }
        const showResultButton = document.createElement("button");
        showResultButton.className = "show-result-button";
        showResultButton.innerHTML = "Show result";
        showResultButton.addEventListener("click", () => {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
            this.shadowRoot.querySelector(".show-result-button").remove();
            
            this.dispatchEvent(new CustomEvent("activatePosprocessorMenu", {
                bubbles: true,
                composed: true,
            }));
        });
        this.shadowRoot.querySelector(".analysis-info").append(showResultButton);
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

    getFEModelLoadStatus() {
        this.dispatchEvent(new CustomEvent("getFEModelLoadStatus", {
            bubbles: true,
            composed: true,
        }));
    }

    checkModel() {
        this.dispatchEvent(new CustomEvent("checkModel", {
            bubbles: true,
            composed: true,
        }));
    }

    analyzeModel() {
        this.dispatchEvent(new CustomEvent("analyzeModel", {
            bubbles: true,
            composed: true,
        }));
    }
}

export default FeaAnalysisMenu;
