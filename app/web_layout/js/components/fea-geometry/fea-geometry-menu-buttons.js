class FeaGeometryMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "geometry-point-menu-button",
                "geometry-line-menu-button",
            ],

            menuNames: {
                "geometry-point-menu-button": "geometry-point-menu",
                "geometry-line-menu-button": "geometry-line-menu",
            },

            captions: {
                "geometry-point-menu-button": "Point",
                "geometry-line-menu-button": "Line",
            }
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
                    width: 12rem;
                }

                .geometry-menu-buttons-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    border-bottom: 0.1rem solid #4a5060;
                    align-items: center;
                }

                .geometry-menu-buttons-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 4rem;
                }

                .geometry-point-menu-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 2.5rem;
                    margin-right: 0rem;
                }

                .geometry-point-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    border-bottom: 0.15rem solid #3b4453;
                }

                .geometry-point-menu-button-icon {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .geometry-point-menu-button:hover .geometry-point-menu-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .geometry-point-menu-button-icon {
                    color: #8bbee4;
                }

                .active .geometry-point-menu-button-icon {
                    color: #72C5FF;
                }

                .active:hover .geometry-point-menu-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .geometry-point-menu-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }

                .geometry-line-menu-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

                .geometry-line-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    border-bottom: 0.15rem solid #3b4453;
                }

                .geometry-line-menu-button-icon {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .geometry-line-menu-button:hover .geometry-line-menu-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .geometry-line-menu-button-icon {
                    color: #8bbee4;
                }

                .active .geometry-line-menu-button-icon {
                    color: #72C5FF;
                }

                .active:hover .geometry-line-menu-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .geometry-line-menu-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }
            </style>

            <div class=wrapper>
                <div class="geometry-menu-buttons-content">

                    <p class="geometry-menu-buttons-caption">Point</p>

                    <button class="geometry-point-menu-button">
                        <div class="geometry-point-menu-button-icon-content">
                            <svg class="geometry-point-menu-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Point</title>
                                <g stroke="currentColor">
                                    <rect x="15" y="15" width="5" height="5"/>
                                </g>
                            </svg>
                        </div>
                    </button>

                    <button class="geometry-line-menu-button">
                        <div class="geometry-line-menu-button-icon-content">
                            <svg class="geometry-line-menu-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Line</title>
                                <g stroke="currentColor">
                                    <rect x="28" y="2" width="5" height="5"/>
                                    <rect x="3" y="27" width="5" height="5"/>
                                    <line x1="8.64645" y1="26.6464" x2="27.6464" y2="7.64645"/>
                                </g>
                            </svg>
                        </div>
                    </button>

                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".geometry-point-menu-button").addEventListener("click", 
            () => this.activate("geometry-point-menu-button"));
        
        this.shadowRoot.querySelector(".geometry-line-menu-button").addEventListener("click", 
            () => this.activate("geometry-line-menu-button"));
    }

    set activateButton(buttonName) {
        this.activate(buttonName);
    }

    connectedCallback() {
        this.activate("geometry-point-menu-button");
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

    activate(buttonName) {
        for (let i = 0; i < this.state.buttonNames.length; i ++) {
            if (this.state.buttonNames[i] !== buttonName && 
                this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`)
                    .classList.contains("active") === true) {
                this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`).classList.remove("active");
                const menuName = this.state.menuNames[this.state.buttonNames[i]];
                this.dispatchEvent(new CustomEvent("deactivate-menu", {
                    bubbles: true,
                    composed: true,
                    detail: {
                        "menuName": menuName,
                    }
                }));
            }
        } 
        const currentButton = this.shadowRoot.querySelector(`.${buttonName}`);
        if (currentButton.classList.contains("active") === false) {
            currentButton.classList.add("active");
            const caption = this.state.captions[buttonName];
            this.shadowRoot.querySelector(".geometry-menu-buttons-caption").innerHTML = caption;
            const menuName = this.state.menuNames[buttonName];
            this.dispatchEvent(new CustomEvent("activate-menu", {
                bubbles: true,
                composed: true,
                detail: {
                    "menuName": menuName,
                }
            }));
        }
    }
}

export default FeaGeometryMenuButtons;
