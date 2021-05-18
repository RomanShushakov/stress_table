class FeaGeometryPointMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "geometry-add-point-menu-button",
                "geometry-update-point-menu-button",
                "geometry-delete-point-menu-button",
            ],

            menuNames: {
                "geometry-add-point-menu-button": "geometry-add-point-menu",
                "geometry-update-point-menu-button": "geometry-update-point-menu",
                "geometry-delete-point-menu-button": "geometry-delete-point-menu",
            },

            captions: {
                "geometry-add-point-menu-button": "Add",
                "geometry-update-point-menu-button": "Update",
                "geometry-delete-point-menu-button": "Delete",
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
                }

                .geometry-point-menu-buttons-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    border-bottom: 0.1rem solid #4a5060;
                    align-items: center;
                }

                .geometry-point-menu-buttons-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 4rem;
                }

                .geometry-add-point-menu-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 2.5rem;
                    margin-right: 0rem;
                }

                .geometry-add-point-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    border-bottom: 0.15rem solid #3b4453;
                }

                .geometry-add-point-menu-button-icon {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .geometry-add-point-menu-button:hover .geometry-add-point-menu-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .geometry-add-point-menu-button-icon {
                    color: #8bbee4;
                }

                .active .geometry-add-point-menu-button-icon {
                    color: #72C5FF;
                }

                .active:hover .geometry-add-point-menu-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .geometry-add-point-menu-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }

                .geometry-update-point-menu-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

                .geometry-update-point-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    border-bottom: 0.15rem solid #3b4453;
                }

                .geometry-update-point-menu-button-icon {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .geometry-update-point-menu-button:hover .geometry-update-point-menu-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .geometry-update-point-menu-button-icon {
                    color: #8bbee4;
                }

                .active .geometry-update-point-menu-button-icon {
                    color: #72C5FF;
                }

                .active:hover .geometry-update-point-menu-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .geometry-update-point-menu-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }

                .geometry-delete-point-menu-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

                .geometry-delete-point-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    border-bottom: 0.15rem solid #3b4453;
                }

                .geometry-delete-point-menu-button-icon {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .geometry-delete-point-menu-button:hover .geometry-delete-point-menu-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .geometry-delete-point-menu-button-icon {
                    color: #8bbee4;
                }

                .active .geometry-delete-point-menu-button-icon {
                    color: #72C5FF;
                }

                .active:hover .geometry-delete-point-menu-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .geometry-delete-point-menu-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }
            </style>

            <div class=wrapper>
                <div class="geometry-point-menu-buttons-content">

                    <p class="geometry-point-menu-buttons-caption">Add</p>

                    <button class="geometry-add-point-menu-button">
                        <div class="geometry-add-point-menu-button-icon-content">
                            <svg class="geometry-add-point-menu-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >  
                                <title>Add point</title>
                                <g stroke="currentColor">
                                    <line x1="7" y1="17.5" x2="28" y2="17.5" stroke-width="2"/>
                                    <line x1="17.5" y1="7" x2="17.5" y2="28" stroke-width="2"/>
                                </g>
                            </svg>
                        </div>
                    </button>

                    <button class="geometry-update-point-menu-button">
                        <div class="geometry-update-point-menu-button-icon-content">
                            <svg class="geometry-update-point-menu-button-icon" width="36" height="35" viewBox="0 0 36 35" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Update point</title>
                                <g stroke="currentColor">
                                    <path d="M17.2055 8L16.9178 11.4247L14.9041 12.1096L12.3151 9.91781L10.3014 
                                        11.8356L12.4589 14.4384L11.7397 16.2192L8 16.4932V19.2329L11.5959 
                                        19.5068L12.4589 21.2877L10.0137 23.8904L12.0274 25.8082L14.7603 
                                        23.6164L16.6301 24.4384L16.9178 28H19.7945L20.0822 24.5753L22.2397 
                                        23.7534L25.1164 26.0822L27.1301 24.1644L24.5411 21.4247L25.4041 
                                        19.6438L29 19.3699V16.7671L25.4041 16.4932L24.6849 14.5753L27.1301 
                                        11.8356L25.1164 9.91781L22.2397 12.2466L20.3699 11.5616L20.0822 
                                        8H17.2055Z"
                                    />
                                    <circle cx="18.5" cy="17.5" r="3.5"/>
                                    <path d="M34 18.5C34 20.4042 33.6249 22.2897 32.8963 24.0489C32.1676 
                                        25.8081 31.0995 27.4066 29.753 28.753C28.4066 30.0995 26.8081 
                                        31.1676 25.0489 31.8963C23.2897 32.6249 21.4042 33 19.5 33"
                                    />
                                    <path d="M32.9962 20L34 17.3919L35.0038 20H32.9962Z"/>
                                    <path d="M2.5 16.5C2.5 14.5958 2.87505 12.7103 3.60375 10.9511C4.33244 
                                        9.19187 5.4005 7.5934 6.74695 6.24695C8.0934 4.9005 9.69187 
                                        3.83244 11.4511 3.10375C13.2103 2.37505 15.0958 2 17 2"
                                    />
                                    <path d="M3.50384 15L2.5 17.6081L1.49616 15H3.50384Z"/>
                                </g>
                            </svg>
                        </div>
                    </button>

                    <button class="geometry-delete-point-menu-button">
                        <div class="geometry-delete-point-menu-button-icon-content">
                            <svg class="geometry-delete-point-menu-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Delete point</title>
                                <g stroke="currentColor">
                                    <path d="M7.00184 9.01157C7.00087 9.0055 7.00557 9 7.01172 
                                        9H27.9885C27.9946 9 27.9993 9.00535 27.9985 9.01135L25.0012 
                                        30.9914C25.0005 30.9963 24.9963 31 24.9913 31H10.5085C10.5036 
                                        31 10.4994 30.9964 10.4987 30.9916L7.00184 9.01157Z"
                                    />
                                    <rect x="14.5" y="4.5" width="6" height="3" rx="0.5"/>
                                    <line x1="5" y1="8" x2="30" y2="8" stroke-width="2"/>
                                    <line x1="11.4942" y1="12.924" x2="13.4942" y2="25.924"/>
                                    <line x1="23.4942" y1="13.076" x2="21.4942" y2="26.076"/>
                                    <line x1="17.5" y1="13" x2="17.5" y2="26"/>
                                </g>
                            </svg>
                        </div>
                    </button>

                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".geometry-add-point-menu-button").addEventListener("click", 
            () => this.activate("geometry-add-point-menu-button"));

        this.shadowRoot.querySelector(".geometry-update-point-menu-button").addEventListener("click", 
            () => this.activate("geometry-update-point-menu-button"));

        this.shadowRoot.querySelector(".geometry-delete-point-menu-button").addEventListener("click", 
            () => this.activate("geometry-delete-point-menu-button"));
    }

    set activateButton(buttonName) {
        this.activate(buttonName);
    }

    connectedCallback() {
        this.activate("geometry-add-point-menu-button");
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
            this.shadowRoot.querySelector(".geometry-point-menu-buttons-caption").innerHTML = caption;
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

export default FeaGeometryPointMenuButtons;
