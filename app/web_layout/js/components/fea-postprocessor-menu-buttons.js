class FeaPostprocessorMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "contours-menu-button",
                "symbols-menu-button",
            ],

            menuNames: {
                "contours-menu-button": "material-menu",
                "symbols-menu-button": "geometry-menu",
            }
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: flex;
                }

                .wrapper {
                    background-color: #2e3440;
                    display: flex;
                    flex-direction: column;
                    position: relative;
                }

                .contours-menu-button {
                    margin: 0rem;
                    padding-top: 0.7rem;
                    padding-bottom: 0.7rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .contours-menu-button:hover {
                    background: #2d303b;
                }

                .contours-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .contours-menu-button-icon {
                    color: #2E3440;
                    width: 3.5rem;
                    height: 3.5rem;
                }

                .contours-menu-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                    width: 3.5rem;
                    font-size: 85%;
                }

                .active .contours-menu-button-icon {
                    color: #3b4453;
                }

                .active:hover .contours-menu-button-icon {
                    color: #242932;
                }

                .symbols-menu-button {
                    margin: 0rem;
                    padding-top: 0.7rem;
                    padding-bottom: 0.7rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .symbols-menu-button:hover {
                    background: #2d303b;
                }

                .symbols-menu-button:hover .symbols-menu-button-icon {
                    color: #2d303b;
                }

                .active .symbols-menu-button-icon {
                    color: #3b4453;
                }

                .active:hover .symbols-menu-button-icon {
                    color: #242932;
                }

                .symbols-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .symbols-menu-button-icon {
                    color: #2E3440;
                    width: 3.5rem;
                    height: 3.5rem;
                }

                .symbols-menu-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                    width: 3.5rem;
                    font-size: 85%;
                }

                .active:hover {
                    background: #242932;
                }

                .active {
                    background: #3b4453;
                }

                .back-to-model-button {
                    background: #0996d7;
                    border: 0.2rem solid #2E3440;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 4.2rem;
                    height: 3.5rem;
                }

                .back-to-model-button:hover {
                    border: 0.2rem solid #4a5060;
                }

            </style>
            <div class="wrapper">

                <button class="contours-menu-button">
                    <div class="contours-menu-button-icon-content">

                        <svg class=contours-menu-button-icon width="102" height="87" viewBox="0 0 102 87" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Contours</title>
                            <path d="M34.3333 1H1V85.7457H101V57.4971H34.3333V1Z" stroke="#D9D9D9"/>
                            <path d="M47 57.5C39.8571 65.3506 40.8571 66 35.5 66C30.1428 66 27.4642 65.5638 25.6785 
                                63.8192C23.8928 62.0746 23.0001 58.1493 23 56.8409C22.9999 55.5324 23.8928 50.2987 
                                25.6785 48.1179C27.4643 45.9372 27.3214 43.4361 30 43C32.1429 42.6511 34.0536 41.8546
                                34.5 42" stroke="#622629"
                            />
                            <path d="M67.5 57.5C59 69.5 59.5 70 52 73.5C44.5 77 34 75.8864 27.5 75C16.5 73.5 17.5 72 14.5 
                                67C11.3916 61.8195 13 58.5 14.5 52C16 45.5 17.5 42 21 38C24.5 34 28.2351 31.4487 29.5 
                                30.5C31.5 29 33.5 29.1667 34 29" stroke="#B1AB19"
                            />
                            <path d="M14 1C9.2 6.6 6.66667 11.6667 6 13.5L3 21L1 26.5" stroke="#7475E6"/>
                            <path d="M101 72L94.5 79L83 85.5" stroke="#7475E6"/>
                            <path d="M96.5 57.5L89 69L84 74L77.5 78L59.5 85.5" stroke="#72C5FF"/>
                            <path d="M1 75.5L6 82L12.5 85.5" stroke="#72C5FF"/>
                            <path d="M28.5 1L19.5 12.5L13 23L7 34L2.5 45.5L1 52" stroke="#72C5FF"/>
                            <path d="M34 14L22 25L14 35L7.5 50L5 62.5L6 69.5L11.5 77L20 82L31.5 83H45.5L55 81L63 
                                76.5L70.5 70.5L76.5 63.5L80 57.5" stroke="#234D2D"
                            />
                        </svg>
                        <p class="contours-menu-button-icon-caption">Contours</p>
                    </div>
                </button>

                <button class="symbols-menu-button">
                    <div class="symbols-menu-button-icon-content">

                        <svg class=symbols-menu-button-icon width="102" height="87" viewBox="0 0 102 87" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Symbols</title>
                            <path d="M23.3837 8.61996L11.6202 20.8927" stroke="#72C5FF"/>
                            <path d="M11.4915 8.74594L23.5123 20.7668" stroke="#72C5FF"/>
                            <path d="M84.3837 65.62L72.6202 77.8927" stroke="#72C5FF"/>
                            <path d="M72.4915 65.7459L84.5123 77.7668" stroke="#72C5FF"/>
                            <path d="M55.3837 65.62L43.6202 77.8927" stroke="#72C5FF"/>
                            <path d="M43.4915 65.7459L55.5123 77.7668" stroke="#72C5FF"/>
                            <path d="M23.3837 65.62L11.6202 77.8927" stroke="#72C5FF"/>
                            <path d="M11.4915 65.7459L23.5123 77.7668" stroke="#72C5FF"/>
                            <path d="M23.3837 36.62L11.6202 48.8927" stroke="#72C5FF"/>
                            <path d="M11.4915 36.7459L23.5123 48.7668" stroke="#72C5FF"/>
                            <path d="M34.3333 1H1V85.7457H101V57.4971H34.3333V1Z" stroke="#D9D9D9"/>
                        </svg>
                        <p class="symbols-menu-button-icon-caption">Symbols</p>
                    </div>
                </button>

                <button class="back-to-model-button">Back to model</button>

            </div>
        `;

        this.shadowRoot.querySelector(".contours-menu-button").addEventListener("click", () => this.toggle("contours-menu-button"));

        this.shadowRoot.querySelector(".symbols-menu-button").addEventListener("click", () => this.toggle("symbols-menu-button"));

        this.shadowRoot.querySelector(".back-to-model-button").addEventListener("click", () => this.activatePreprocessorMenu());
    }

    set toggleButton(buttonName) {
        this.toggle(buttonName);
    }

    connectedCallback() {
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

    activatePreprocessorMenu() {
        this.dispatchEvent(new CustomEvent("activatePreprocessorMenu", {
            bubbles: true,
            composed: true,
        }));
    }

    toggle(buttonName) {
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
        if (currentButton.classList.contains("active") === true) {
            currentButton.classList.remove("active");
            const menuName = this.state.menuNames[buttonName];
            this.dispatchEvent(new CustomEvent("deactivate-menu", {
                bubbles: true,
                composed: true,
                detail: {
                    "menuName": menuName,
                }
            }));
        } else {
            currentButton.classList.add("active");
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

export default FeaPostprocessorMenuButtons;
