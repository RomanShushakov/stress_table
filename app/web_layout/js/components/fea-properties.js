class FeaProperties extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: {
                properties: "Properties",
            },
            buttonFullNames: {
                properties: "properties",
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
                }

                .properties-button {
                    margin: 0rem;
                    padding-top: 0.7rem;
                    padding-bottom: 0.7rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .properties-button:hover {
                    background: #2d303b;
                }

                .properties-button:hover .properties-button-icon {
                    color: #2d303b;
                }

                .properties-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .properties-button-icon {
                    color: #2E3440;
                    width: 3rem;
                    height: 3rem;
                }

                .properties-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                    width: 3rem;
                    font-size: 85%;
                }

            </style>
            <div class=wrapper>
                <button class="properties-button">
                    <div class="properties-button-icon-content">
                        <svg class="properties-button-icon" width="101" height="101" viewBox="0 0 101 101" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Properties</title>
                            <g fill="currentColor">
                                <path d="M1 1H52.5625L76 35.375V91.625H1V1Z" stroke="#D9D9D9"/>
                                <ellipse cx="66.7609" cy="68.1875" rx="34.2391" ry="32.8125"/>
                                <path d="M8.6087 9.33334V83.2917H37.6472M49.3563 9.33334V34.3333H66.2174" stroke="#72C5FF"/>
                                <path d="M63.0756 39.5417L62.2567 49.3519L56.5241 51.3139L49.1537 45.0354L43.4211 
                                    50.5291L49.5631 57.9849L47.5158 63.0862L36.8696 63.871V71.7192L47.1063 72.504L49.5631 
                                    77.6053L42.6021 85.0611L48.3347 90.5548L56.1147 84.2763L61.4378 86.6307L62.2567 
                                    96.8333H70.4461L71.265 87.0231L77.4071 84.6687L85.5965 91.3396L91.3291 
                                    85.8459L83.9586 77.9977L86.4154 72.8964L96.6522 72.1116V64.6558L86.4154 
                                    63.871L84.3681 58.3773L91.3291 50.5291L85.5965 45.0354L77.4071 51.7063L72.084 
                                    49.7443L71.265 39.5417H63.0756Z" stroke="#D9D9D9"
                                />
                                <ellipse cx="66.7609" cy="68.1875" rx="10.3261" ry="9.89583" stroke="#D9D9D9"/>
                            </g>
                        </svg>
                        <p class="properties-button-icon-caption">Properties</p>
                    </div>
                </button>
            </div>
        `;
    }

    set close(_empty) {
        this.shadowRoot.querySelector(".properties").close = "_empty";
        // this.refreshGeometryFields();
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

}

export default FeaProperties;