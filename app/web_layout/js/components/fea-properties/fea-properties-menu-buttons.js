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
                "properties-beam-section-orientation-menu-button",
            ],

            menuNames: {
                "properties-add-properties-menu-button": "properties-add-properties-menu",
                "properties-update-properties-menu-button": "properties-update-properties-menu",
                "properties-delete-properties-menu-button": "properties-delete-properties-menu",
                "properties-assign-properties-menu-button": "properties-assign-properties-menu",
                "properties-beam-section-orientation-menu-button": "properties-beam-section-orientation-menu",
            },

            captions: {
                "properties-add-properties-menu-button": "Add",
                "properties-update-properties-menu-button": "Update",
                "properties-delete-properties-menu-button": "Delete",
                "properties-assign-properties-menu-button": "Assign",
                "properties-beam-section-orientation-menu-button": "Beam section orientation",
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

                .properties-menu-buttons-caption {
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0.3rem;
                    padding-left: 0rem;
                    padding-right: 0rem;
                    color: #D9D9D9;
                    border-bottom: 0.1rem solid #4a5060;
                    font-size: 85%;
                    width: 12rem;
                }

                .properties-menu-buttons-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    border-bottom: 0.1rem solid #4a5060;
                    align-items: center;
                }

                .properties-add-properties-menu-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin: 0rem;
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

                .properties-beam-section-orientation-menu-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

                .properties-beam-section-orientation-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    border-bottom: 0.15rem solid #3b4453;
                }

                .properties-beam-section-orientation-menu-button-icon {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .properties-beam-section-orientation-menu-button:hover .properties-beam-section-orientation-menu-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .properties-beam-section-orientation-menu-button-icon {
                    color: #8bbee4;
                }

                .active .properties-beam-section-orientation-menu-button-icon {
                    color: #72C5FF;
                }

                .active:hover .properties-beam-section-orientation-menu-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .properties-beam-section-orientation-menu-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }
            </style>

            <div class=wrapper>

                <div class="properties-menu-buttons-content">

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

                    <button class="properties-beam-section-orientation-menu-button">
                        <div class="properties-beam-section-orientation-menu-button-icon-content">

                            <svg class="properties-beam-section-orientation-menu-button-icon" width="36" height="35" 
                                viewBox="0 0 36 35" fill="none" xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Beam section orientation</title>
                                <path d="M26.9635 27.5397V28.8922H28.0573V29.3708H26.9635V32.8913C26.9635
                                    33.1842 27.0156 33.4023 27.1198 33.5456C27.2272 33.6888 27.4046 33.7604 
                                    27.652 33.7604C27.7496 33.7604 27.9075 33.7441 28.1256 33.7116L28.15 
                                    34.1901C27.997 34.2454 27.7887 34.2731 27.525 34.2731C27.1246 34.2731 
                                    26.8333 34.1575 26.651 33.9264C26.4687 33.692 26.3776 33.3486 26.3776 
                                    32.8961V29.3708H25.4059V28.8922H26.3776V27.5397H26.9635Z" fill="#D9D9D9"
                                />
                                <path d="M25.8459 15.9975L25.8654 16.9008C26.064 16.5753 26.3114 16.3279 
                                    26.6076 16.1586C26.9038 15.9861 27.231 15.8998 27.589 15.8998C28.1554 
                                    15.8998 28.577 16.0593 28.8537 16.3784C29.1304 16.6974 29.2704 17.1759 
                                    29.2736 17.8139V21.2807H28.6926V17.809C28.6893 17.337 28.5884 16.9855 
                                    28.3898 16.7543C28.1945 16.5232 27.8804 16.4077 27.4474 16.4077C27.0861 
                                    16.4077 26.7655 16.5216 26.4855 16.7495C26.2088 16.9741 26.0038 17.2784 
                                    25.8703 17.6625V21.2807H25.2892V15.9975H25.8459ZM33.4142 
                                    21.2807H32.8283V14.8745L30.8947 15.5922V15.0453L33.3166 14.1469H33.4142V21.2807Z" 
                                    fill="#D9D9D9"
                                />
                                <path d="M16.0459 3.7168L16.0654 4.62012C16.264 4.2946 16.5114 4.0472 
                                    16.8076 3.87793C17.1038 3.7054 17.431 3.61914 17.7891 3.61914C18.3555 
                                    3.61914 18.777 3.77865 19.0537 4.09766C19.3304 4.41667 19.4704 4.89518 
                                    19.4736 5.5332V9H18.8926V5.52832C18.8893 5.05632 18.7884 4.70475 18.5898 
                                    4.47363C18.3945 4.24251 18.0804 4.12695 17.6474 4.12695C17.2861 4.12695 
                                    16.9655 4.24089 16.6855 4.46875C16.4088 4.69336 16.2038 4.99772 16.0703 
                                    5.38184V9H15.4892V3.7168H16.0459ZM25.3476 9H20.8945V8.55078L23.331 
                                    5.78711C23.7347 5.32161 24.0146 4.9375 24.1709 4.63477C24.3271 4.33203 
                                    24.4053 4.03743 24.4053 3.75098C24.4053 3.30176 24.2734 2.94694 24.0098 
                                    2.68652C23.7461 2.42285 23.3831 2.29102 22.9209 2.29102C22.4424 2.29102 
                                    22.055 2.44401 21.7588 2.75C21.4658 3.05599 21.3193 3.45312 21.3193 
                                    3.94141H20.7383C20.7383 3.54102 20.8278 3.17643 21.0068 2.84766C21.1891 
                                    2.51888 21.4463 2.26172 21.7783 2.07617C22.1136 1.88737 22.4945 1.79297 
                                    22.9209 1.79297C23.5719 1.79297 24.0797 1.96387 24.4443 2.30566C24.8122 
                                    2.64746 24.9961 3.11296 24.9961 3.70215C24.9961 4.03418 24.8919 4.39714 
                                    24.6836 4.79102C24.4785 5.1849 24.1155 5.67643 23.5947 6.26562L21.6123 
                                    8.50684H25.3476V9Z" fill="#D9D9D9"
                                />
                                <g fill="currentColor">
                                    <path d="M23.4 29.3259C23.6936 29.3259 23.9316 29.1172 23.9316 
                                        28.8597V24.663C23.9316 24.4055 23.6936 24.1967 23.4 24.1967C23.1064 24.1967 
                                        22.8684 24.4055 22.8684 24.663V28.3934H18.6158C18.3223 28.3934 18.0843 
                                        28.6021 18.0843 28.8597C18.0843 29.1172 18.3223 29.3259 18.6158 
                                        29.3259L23.4 29.3259ZM0.62412 9.54024L23.0241 29.1894L23.7759 
                                        28.5299L1.37588 8.88081L0.62412 9.54024Z"
                                    />
                                    <path d="M23.7536 19.3886C23.9488 19.1934 23.9488 18.8768 23.7536 18.6815L20.5716 
                                        15.4996C20.3763 15.3043 20.0597 15.3043 19.8645 15.4996C19.6692 15.6948 
                                        19.6692 16.0114 19.8645 16.2067L22.6929 19.0351L19.8645 21.8635C19.6692 
                                        22.0588 19.6692 22.3754 19.8645 22.5706C20.0597 22.7659 20.3763 22.7659 
                                        20.5716 22.5706L23.7536 19.3886ZM12.2 19.5351L23.4 19.5351V18.5351L12.2 
                                        18.5351V19.5351Z"
                                    />
                                    <path d="M12.5536 8.85698C12.3583 8.66172 12.0417 8.66172 11.8465 8.85698L8.66448 
                                        12.039C8.46922 12.2342 8.46922 12.5508 8.66448 12.7461C8.85974 12.9413 
                                        9.17632 12.9413 9.37159 12.7461L12.2 9.91764L15.0284 12.7461C15.2237 
                                        12.9413 15.5403 12.9413 15.7355 12.7461C15.9308 12.5508 15.9308 12.2342 
                                        15.7355 12.039L12.5536 8.85698ZM12.7 19.0351V9.21053H11.7V19.0351H12.7Z"
                                    />
                                </g>
                            </svg>
                        </div>
                    </button>

                </div>

                <p class="properties-menu-buttons-caption">Add</p>

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

        this.shadowRoot.querySelector(".properties-beam-section-orientation-menu-button").addEventListener("click", 
            () => this.activate("properties-beam-section-orientation-menu-button"));
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
