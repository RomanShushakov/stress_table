class FeaSectionMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "section-truss-menu-button",
                "section-beam-menu-button",
            ],

            menuNames: {
                "section-truss-menu-button": "section-truss-menu",
                "section-beam-menu-button": "section-beam-menu",
            },

            captions: {
                "section-truss-menu-button": "Truss",
                "section-beam-menu-button": "Beam",
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

                .section-menu-buttons-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    border-bottom: 0.1rem solid #4a5060;
                    align-items: center;
                }

                .section-menu-buttons-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 4rem;
                }

                .section-truss-menu-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 2.5rem;
                    margin-right: 0rem;
                }

                .section-truss-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 2.3rem;
                    height: 1.7rem;
                    border-bottom: 0.15rem solid #3b4453;
                }

                .section-truss-menu-button-icon {
                    margin: 0rem;
                    padding: 0rem;
                    width: 2.3rem;
                    height: 1.7rem;
                    color: #D9D9D9;
                }

                .section-truss-menu-button:hover .section-truss-menu-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .section-truss-menu-button-icon {
                    color: #8bbee4;
                }

                .active .section-truss-menu-button-icon {
                    color: #72C5FF;
                }

                .active:hover .section-truss-menu-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .section-truss-menu-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }

                .section-beam-menu-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

                .section-beam-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 2.3rem;
                    height: 1.7rem;
                    border-bottom: 0.15rem solid #3b4453;
                }

                .section-beam-menu-button-icon {
                    margin: 0rem;
                    padding: 0rem;
                    width: 2.3rem;
                    height: 1.7rem;
                    color: #D9D9D9;
                }

                .section-beam-menu-button:hover .section-beam-menu-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .section-beam-menu-button-icon {
                    color: #8bbee4;
                }

                .active .section-beam-menu-button-icon {
                    color: #72C5FF;
                }

                .active:hover .section-beam-menu-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .section-beam-menu-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }
            </style>

            <div class=wrapper>
                <div class="section-menu-buttons-content">

                    <p class="section-menu-buttons-caption">Section</p>

                    <button class="section-truss-menu-button">
                        <div class="section-truss-menu-button-icon-content">
                            <svg class="section-truss-menu-button-icon" width="50" height="37" viewBox="0 0 50 37" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Truss section</title>
                                <g stroke="currentColor">
                                    <path d="M16.0991 1L4.09842 25.9567H26.0306M16.0991 1L26.0306 25.9567M16.0991 
                                        1H37.2036M26.0306 25.9567L37.2036 1M26.0306 25.9567H46.7214L37.2036 1"
                                    />
                                    <path d="M45.9016 26.955L48.0312 31.4472H43.7721L45.9016 26.955Z"/>
                                    <ellipse cx="44.2623" cy="32.9446" rx="0.819672" ry="0.99827" stroke-width="0.5"/>
                                    <ellipse cx="4.50823" cy="32.4455" rx="1.22951" ry="1.4974" stroke-width="0.5"/>
                                    <ellipse cx="47.5409" cy="32.9446" rx="0.819672" ry="0.99827" stroke-width="0.5"/>
                                    <line x1="41.8033" y1="34.6912" x2="50.0001" y2="34.6912" stroke-width="0.5"/>
                                    <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                        -0.772853 42.6727 34.9412)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                        -0.772853 44.3121 34.9412)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584
                                        -0.772853 45.9514 34.9412)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                        -0.772853 47.5907 34.9412)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                        -0.772853 49.2301 34.9412)" stroke-width="0.5"
                                    />
                                    <line x1="0.819702" y1="34.6912" x2="8.19675" y2="34.6912" stroke-width="0.5"/>
                                    <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                        -0.772853 0.869415 34.9412)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                        -0.772853 2.50873 34.9412)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                        -0.772853 4.14804 34.9412)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                        -0.772853 5.78745 34.9412)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                        -0.772853 7.42676 34.9412)" stroke-width="0.5"
                                    />
                                    <line x1="4.34842" y1="25.9567" x2="4.34842" y2="30.9481" stroke-width="0.5"/>
                                    <path d="M26.0527 36.1162C26.1504 36.2138 26.3087 36.2138 26.4063 36.1162L27.9973 
                                        34.5252C28.0949 34.4276 28.0949 34.2693 27.9973 34.1717C27.8997 34.074 27.7414 
                                        34.074 27.6437 34.1717L26.2295 35.5859L24.8153 34.1717C24.7177 34.074 24.5594 
                                        34.074 24.4618 34.1717C24.3641 34.2693 24.3641 34.4276 24.4618 34.5252L26.0527 
                                        36.1162ZM25.9795 25.9567V35.9394H26.4795V25.9567H25.9795Z"
                                    />
                                </g>
                            </svg>
                        </div>
                    </button>

                    <button class="section-beam-menu-button">
                        <div class="section-beam-menu-button-icon-content">
                            <svg class="section-beam-menu-button-icon" width="53" height="36" viewBox="0 0 53 36" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Beam section</title>
                                <g stroke="currentColor">
                                    <path d="M2.09842 23.7126H46.7979M2.09842 7.10491H46.7979M2.09842 
                                        26.3349V4.52045H46.7979V26.3349H25.0991H2.09842Z"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 0.0606537)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 24.5352)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 22.787)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 21.0388)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 19.2906)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 17.5424)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 15.7943)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 14.0461)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 12.2979)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 10.5497)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 8.80155)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 7.05337)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 5.30519)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 3.55701)" stroke-width="0.5"
                                    />
                                    <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                        -0.625897 -0.779906 52.0984 1.80883)" stroke-width="0.5"
                                    />
                                    <path d="M2.05915 35.2374C2.15679 35.3351 2.31508 35.3351 2.41271 
                                        35.2374L4.0037 33.6464C4.10133 33.5488 4.10133 33.3905 4.0037 
                                        33.2929C3.90607 33.1953 3.74778 33.1953 3.65014 33.2929L2.23593 
                                        34.7071L0.821718 33.2929C0.724087 33.1953 0.565796 33.1953 0.468164 
                                        33.2929C0.370533 33.3905 0.370533 33.5488 0.468164 33.6464L2.05915 
                                        35.2374ZM1.98593 26.3349L1.98593 35.0607H2.48593L2.48593 26.3349H1.98593Z"
                                    />
                                    <line x1="47.5901" y1="1.89818" x2="47.5901" y2="28.995"/>
                                </g>
                            </svg>

                        </div>
                    </button>

                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".section-truss-menu-button").addEventListener("click", 
            () => this.activate("section-truss-menu-button"));
        
        this.shadowRoot.querySelector(".section-beam-menu-button").addEventListener("click", 
            () => this.activate("section-beam-menu-button"));
    }

    set activateButton(buttonName) {
        this.activate(buttonName);
    }

    connectedCallback() {
        this.activate("section-truss-menu-button");
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
            this.shadowRoot.querySelector(".section-menu-buttons-caption").innerHTML = caption;
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

export default FeaSectionMenuButtons;
