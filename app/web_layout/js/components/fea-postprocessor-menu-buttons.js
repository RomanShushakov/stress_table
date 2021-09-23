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
                "contours-menu-button": "contours-menu",
                "symbols-menu-button": "symbols-menu",
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

                        <svg class=contours-menu-button-icon width="102" height="86" viewBox="0 0 102 86" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Contours</title>
                            <path d="M1 1H14L9.5 7L6 13.5L3.5 19.5L1 26V1Z" fill="#7475E6" stroke="#E3E1E1" stroke-width="0.5"/>
                            <path d="M1 51.5V26L6 13.5L9.5 7L14 1H28.5L24 6.5L15.5 18.5L7.5 33L3 44L1 51.5Z" 
                                fill="url(#paint0_radial)" stroke="#E3E1E1" stroke-width="0.5"
                            />
                            <path d="M34.5 1H28.5L24 6.5L15.5 18.5L7.5 33L3 44L1 51V75.5L6 82L12 85.5H59.5L77.5 
                                78L84.5 73.5L89 69L96.5 57.5H80L76.5 63.5L70.5 70.5L63 76.5L55 81L45 83H31L19.5 
                                82L11.5 77L6 69.5L5 62.5L7.5 50L14 35L22 25L34.5 14V1Z" fill="url(#paint1_radial)" 
                                stroke="#E3E1E1" stroke-width="0.5"
                            />
                            <path d="M1 85.5V75.5L6 82L12 85.5H1Z" fill="#72C5FF" stroke="#E3E1E1" stroke-width="0.5"/>
                            <path d="M89 69L96.5 57.5H101V72L94.5 79L83 85.5H59.5L77.5 78L84.5 73.5L89 69Z" 
                                fill="url(#paint2_radial)" stroke="#E3E1E1" stroke-width="0.5"
                            />
                            <path d="M94.5 79L83 85.5H101V72L94.5 79Z" fill="#7475E6" stroke="#E3E1E1" stroke-width="0.5"/>
                            <path d="M22 25L34.5 14V29L30.5 30L28 31.5L23 36L18.5 41L16 46.5L13.5 56.5L12.5 
                                61.5L14.5 67L18 72.5L20.5 74L26 75L34 76H41L48 75L52 73.5L56 71.5L59.5 
                                68.5L67.5 57.5H80L76.5 63.5L70.5 70.5L63 76.5L55 81L45 83H31L19.5 82L11.5 
                                77L6 69.5L5 62.5L7.5 50L14 35L22 25Z" fill="url(#paint3_radial)" stroke="#E3E1E1" 
                                stroke-width="0.5"
                            />
                            <path d="M30.5 30L34.5 29V42L32.5 42.5L29 43.5L27.5 45L25.5 48.5L24 52L23 
                                55.5V57.5L23.5 59.5L25 63L26.5 64.5L29 65.5L32.5 66H37L39.5 65.5L41 
                                64.5L47 57.5H67.5L59.5 68.5L56 71.5L52 73.5L48 75L41 76H34L20.5 74L18 72.5L14.5
                                67L12.5 61.5L16 46.5L18.5 41L23 36L28 31.5L30.5 30Z" fill="url(#paint4_radial)" 
                                stroke="#E3E1E1" stroke-width="0.5"
                            />
                            <path d="M29 43.5L34.5 42V57.5H47L41 64.5L39.5 65.5L37 66H32.5L29 65.5L26.5 64.5L25 63L23.5 
                                59.5L23 57.5V55.5L24 52L25.5 48.5L27.5 45L29 43.5Z" fill="#622629" stroke="#E3E1E1" 
                                stroke-width="0.5"
                            />
                            <defs>
                                <radialGradient id="paint0_radial" cx="0" cy="0" r="1" gradientUnits="userSpaceOnUse" 
                                    gradientTransform="translate(51.5 41.5) rotate(-169.563) scale(57.959 59.909)"
                                >
                                    <stop offset="0.645833" stop-color="#72C5FF"/>
                                    <stop offset="1" stop-color="#7475E6"/>
                                </radialGradient>
                                <radialGradient id="paint1_radial" cx="0" cy="0" r="1" gradientUnits="userSpaceOnUse" 
                                    gradientTransform="translate(43.5 50) rotate(146.622) scale(50.8945 42.4774)"
                                >
                                    <stop offset="0.796875" stop-color="#234D2D"/>
                                    <stop offset="1" stop-color="#72C5FF"/>
                                </radialGradient>
                                <radialGradient id="paint2_radial" cx="0" cy="0" r="1" gradientUnits="userSpaceOnUse" 
                                    gradientTransform="translate(52.5 40) rotate(45.7073) scale(57.28 84.8972)"
                                >
                                    <stop offset="0.75" stop-color="#72C5FF"/>
                                    <stop offset="1" stop-color="#7475E6"/>
                                </radialGradient>
                                <radialGradient id="paint3_radial" cx="0" cy="0" r="1" gradientUnits="userSpaceOnUse" 
                                    gradientTransform="translate(42.5 48.5) rotate(134.502) scale(40.6602 38.5002)"
                                >
                                    <stop stop-color="#B1AB19"/>
                                    <stop offset="1" stop-color="#234D2D"/>
                                </radialGradient>
                                <radialGradient id="paint4_radial" cx="0" cy="0" r="1" gradientUnits="userSpaceOnUse" 
                                    gradientTransform="translate(46 47.5) rotate(130.955) scale(35.0892 25.4568)"
                                >
                                    <stop offset="0.526042" stop-color="#622629"/>
                                    <stop offset="1" stop-color="#B1AB19"/>
                                </radialGradient>
                            </defs>
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
        for (let i = 0; i < this.state.buttonNames.length; i ++) {
            if (this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`)
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
