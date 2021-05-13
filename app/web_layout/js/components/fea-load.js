class FeaLoad extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: {
                load: "Load",
            },
            buttonFullNames: {
                load: "load",
            }
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: flex;
                }

                .wrapper {
                    margin-top: 1rem;
                    background-color: #2e3440;
                    display: flex;
                }

                .load-button {
                    margin: 0rem;
                    padding: 0rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .load-button:hover {
                    background: #2d303b;
                }

                .load-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .load-button-icon {
                    color: #2E3440;
                    width: 3.5rem;
                    height: 3.5rem;
                }

                .load-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                }

            </style>
            <div class=wrapper>
                <button class="load-button">
                    <div class="load-button-icon-content">
                        <svg class="load-button-icon" width="102" height="102" viewBox="0 0 102 102" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <title>Load</title>
                            <path d="M85.5 56.7797L78.1388 45.339H92.8612L85.5 56.7797Z" fill="#72C5FF" stroke="#72C5FF"/>
                            <path d="M86 24.5763L86 44.9152" stroke="#72C5FF"/>
                            <path d="M101 101V73.0339" stroke="#72C5FF" stroke-linecap="round" stroke-dasharray="4 4"/>
                            <path d="M34.5 57.7797C76 60.322 92.5 64.983 101 73.0339" stroke="#72C5FF" stroke-dasharray="4 4"/>
                            <path d="M34.5 85.7458C76 88.2881 92.5 92.9491 101 101" stroke="#72C5FF" stroke-dasharray="4 4"/>
                            <path d="M34.3333 1H1V85.7457H101V57.4971H34.3333V1Z" stroke="#D9D9D9"/>
                        </svg>
                        <p class="load-button-icon-caption">Load</p>
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

export default FeaLoad;