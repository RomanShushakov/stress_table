class FeaBoundaryCondition extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: {
                displacement: "Boundary condition",
            },
            buttonFullNames: {
                displacement: "boundary condition",
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

                .boundary-condition-button {
                    margin: 0rem;
                    padding-top: 0.325rem;
                    padding-bottom: 0.325rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .boundary-condition-button:hover {
                    background: #2d303b;
                }

                .boundary-condition-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .boundary-condition-button-icon {
                    color: #2E3440;
                    width: 3rem;
                    height: 3rem;
                }

                .boundary-condition-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                    width: 3rem;
                    font-size: 85%;
                }

            </style>
            <div class=wrapper>
                <button class="boundary-condition-button">
                    <div class="boundary-condition-button-icon-content">
                        <svg class="boundary-condition-button-icon" width="100" height="101" viewBox="0 0 100 101" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Boundary condition</title>
                            <path d="M35.7143 1H7.14285V93.0761H92.8571V62.3841H35.7143V1Z" stroke="#D9D9D9"/>
                            <line x1="100" y1="94.0761" y2="94.0761" stroke="#72C5FF" stroke-width="2"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 2.38095 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 7.14285 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 11.9048 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 16.6667 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 21.4286 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 26.1905 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 30.9524 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 69.0476 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 73.8095 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 78.5714 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 83.3333 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 88.0952 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 92.8571 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 35.7143 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 40.4762 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 45.2381 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 50 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 54.7619 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 59.5238 101)" stroke="#72C5FF"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 64.2857 101)" stroke="#72C5FF"/>
                        </svg>
                        <p class="boundary-condition-button-icon-caption">Boundary condition</p>
                    </div>
                </button>
            </div>
        `;
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

export default FeaBoundaryCondition;