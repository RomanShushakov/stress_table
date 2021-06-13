class FeaPropertiesMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "properties-add-properties-menu-button",
                "properties-update-properties-menu-button",
                "properties-delete-properties-menu-button",
                "properties-assign-properties-menu-button",
            ],

            menuNames: {
                "properties-add-properties-menu-button": "properties-add-properties-menu",
                "properties-update-properties-menu-button": "properties-update-properties-menu",
                "properties-delete-properties-menu-button": "properties-delete-properties-menu",
                "properties-assign-properties-menu-button": "properties-assign-properties-menu",
            },

            captions: {
                "properties-add-properties-menu-button": "Add",
                "properties-update-properties-menu-button": "Update",
                "properties-delete-properties-menu-button": "Delete",
                "properties-assign-properties-menu-button": "Assign",
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

                .properties-menu-buttons-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    border-bottom: 0.1rem solid #4a5060;
                    align-items: center;
                }

                .properties-menu-buttons-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 4rem;
                }

                .properties-add-properties-menu-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

                .properties-add-properties-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    border-bottom: 0.15rem solid #3b4453;
                }

                .properties-add-properties-menu-button-icon {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .properties-add-properties-menu-button:hover .properties-add-properties-menu-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .properties-add-properties-menu-button-icon {
                    color: #8bbee4;
                }

                .active .properties-add-properties-menu-button-icon {
                    color: #72C5FF;
                }

                .active:hover .properties-add-properties-menu-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .properties-add-properties-menu-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }

                .properties-update-properties-menu-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

                .properties-update-properties-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    border-bottom: 0.15rem solid #3b4453;
                }

                .properties-update-properties-menu-button-icon {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .properties-update-properties-menu-button:hover .properties-update-properties-menu-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .properties-update-properties-menu-button-icon {
                    color: #8bbee4;
                }

                .active .properties-update-properties-menu-button-icon {
                    color: #72C5FF;
                }

                .active:hover .properties-update-properties-menu-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .properties-update-properties-menu-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }

                .properties-delete-properties-menu-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

                .properties-delete-properties-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    border-bottom: 0.15rem solid #3b4453;
                }

                .properties-delete-properties-menu-button-icon {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .properties-delete-properties-menu-button:hover .properties-delete-properties-menu-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .properties-delete-properties-menu-button-icon {
                    color: #8bbee4;
                }

                .active .properties-delete-properties-menu-button-icon {
                    color: #72C5FF;
                }

                .active:hover .properties-delete-properties-menu-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .properties-delete-properties-menu-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }

                .properties-assign-properties-menu-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

                .properties-assign-properties-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    border-bottom: 0.15rem solid #3b4453;
                }

                .properties-assign-properties-menu-button-icon {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .properties-assign-properties-menu-button:hover .properties-assign-properties-menu-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .properties-assign-properties-menu-button-icon {
                    color: #8bbee4;
                }

                .active .properties-assign-properties-menu-button-icon {
                    color: #72C5FF;
                }

                .active:hover .properties-assign-properties-menu-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .properties-assign-properties-menu-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }
            </style>

            <div class=wrapper>
                <div class="properties-menu-buttons-content">

                    <p class="properties-menu-buttons-caption">Add</p>

                    <button class="properties-add-properties-menu-button">
                        <div class="properties-add-properties-menu-button-icon-content">
                            <svg class="properties-add-properties-menu-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >  
                                <title>Add properties</title>
                                <g stroke="currentColor">
                                    <line x1="7" y1="17.5" x2="28" y2="17.5" stroke-width="2"/>
                                    <line x1="17.5" y1="7" x2="17.5" y2="28" stroke-width="2"/>
                                </g>
                            </svg>
                        </div>
                    </button>

                    <button class="properties-update-properties-menu-button">
                        <div class="properties-update-properties-menu-button-icon-content">
                            <svg class="properties-update-properties-menu-button-icon" width="36" height="35" viewBox="0 0 36 35" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Update properties</title>
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

                    <button class="properties-delete-properties-menu-button">
                        <div class="properties-delete-properties-menu-button-icon-content">
                            <svg class="properties-delete-properties-menu-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Delete properties</title>
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

                    <button class="properties-assign-properties-menu-button">
                        <div class="properties-assign-properties-menu-button-icon-content">

                            <svg class="properties-assign-properties-menu-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Assign properties</title>
                                <g stroke="currentColor">
                                    <path d="M7.63328 10.5C9.14299 9.01664 11.0665 8.00646 13.1605 7.5972C15.2546 7.18794 
                                        17.4251 7.39799 19.3976 8.20078C21.3702 9.00356 23.0561 10.363 24.2423 12.1073C25.4285 
                                        13.8515 26.0616 15.9022 26.0616 18C26.0616 20.0978 25.4285 22.1485 24.2423 23.8927C23.0561 
                                        25.637 21.3702 26.9964 19.3976 27.7992C17.4251 28.602 15.2546 28.8121 13.1605 
                                        28.4028C11.0665 27.9935 9.14299 26.9834 7.63328 25.5" 
                                        stroke-linecap="round"
                                    />
                                    <path d="M17.6274 7.5972C19.7215 7.18794 21.892 7.39798 23.8646 8.20077C25.8371 9.00356 
                                        27.523 10.363 28.7092 12.1073C29.8954 13.8515 30.5285 15.9022 30.5285 18C30.5285 
                                        20.0978 29.8954 22.1485 28.7092 23.8927C27.5231 25.637 25.8371 26.9964 23.8646 
                                        27.7992C21.892 28.602 19.7215 28.8121 17.6274 28.4028"
                                    />
                                    <path d="M10.7996 16.6342C10.2473 16.6342 9.79962 17.082 9.79962 17.6342C9.79962 
                                        18.1865 10.2473 18.6342 10.7996 18.6342V16.6342ZM20.0683 18.3414C20.4589 
                                        17.9508 20.4589 17.3177 20.0683 16.9271L13.7044 10.5632C13.3138 10.1727 
                                        12.6807 10.1727 12.2902 10.5632C11.8996 10.9537 11.8996 11.5869 12.2902 
                                        11.9774L17.947 17.6342L12.2902 23.2911C11.8996 23.6816 11.8996 24.3148 
                                        12.2902 24.7053C12.6807 25.0958 13.3138 25.0958 13.7044 24.7053L20.0683 
                                        18.3414ZM10.7996 18.6342L19.3612 18.6342V16.6342L10.7996 16.6342V18.6342Z"
                                    />
                                    <line x1="7.19391" y1="17.9028" x2="5.47147" y2="17.9028" 
                                        stroke-width="2" stroke-linecap="round"
                                    />
                                </g>
                            </svg>
                        </div>
                    </button>

                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".properties-add-properties-menu-button").addEventListener("click", 
            () => this.activate("properties-add-properties-menu-button"));

        this.shadowRoot.querySelector(".properties-update-properties-menu-button").addEventListener("click", 
            () => this.activate("properties-update-properties-menu-button"));

        this.shadowRoot.querySelector(".properties-delete-properties-menu-button").addEventListener("click", 
            () => this.activate("properties-delete-properties-menu-button"));

        this.shadowRoot.querySelector(".properties-assign-properties-menu-button").addEventListener("click", 
            () => this.activate("properties-assign-properties-menu-button"));
    }

    connectedCallback() {
        this.activate("properties-add-properties-menu-button");
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
            this.shadowRoot.querySelector(".properties-menu-buttons-caption").innerHTML = caption;
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

export default FeaPropertiesMenuButtons;
