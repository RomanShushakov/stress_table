class FeaAnalysis extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: {
                element: "Analysis",
            },
            buttonFullNames: {
                element: "analysis",
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

                .analysis-button {
                    margin: 0rem;
                    padding-top: 0.7rem;
                    padding-bottom: 0.7rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .analysis-button:hover {
                    background: #2d303b;
                }

                .analysis-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .analysis-button-icon {
                    color: #2E3440;
                    width: 3rem;
                    height: 3rem;
                }

                .analysis-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                    width: 3rem;
                    font-size: 85%;
                }

            </style>
            <div class=wrapper>
                <button class="analysis-button">
                    <div class="analysis-button-icon-content">
                        <svg class="analysis-button-icon" width="102" height="102" viewBox="0 0 102 102" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <title>Analysis</title>
                            <rect x="12.8577" y="1" width="79.0514" height="60.5263" rx="0.01" stroke="#D9D9D9"/>
                            <rect x="17.6008" y="6.26318" width="69.5652" height="50" rx="0.01" stroke="#72C5FF"/>
                            <rect x="1" y="82.579" width="82.2134" height="18.4211" rx="0.01" stroke="#D9D9D9"/>
                            <path d="M18.7836 64.161C18.7855 64.159 18.7881 64.1579 18.7908 64.1579H100.976C100.985 
                                64.1579 100.99 64.1685 100.984 64.1748L83.2164 82.5759C83.2145 82.5778 83.2119 82.579 
                                83.2092 82.579H1.02356C1.01474 82.579 1.01023 82.5684 1.01636 82.562L18.7836 64.161Z" stroke="#D9D9D9"
                            />
                            <path d="M83.2134 82.583C83.2134 82.5804 83.2144 82.5779 83.2162 82.576L100.983 
                                64.1757C100.989 64.1692 101 64.1737 101 64.1827V82.5749C101 82.5775 100.999 
                                82.58 100.997 82.5819L83.2306 100.982C83.2244 100.989 83.2134 100.984 83.2134 
                                100.975V82.583Z" stroke="#D9D9D9"
                            />
                            <rect x="16.0198" y="82.579" width="3.95257" height="18.4211" rx="0.01" stroke="#72C5FF"/>
                            <path d="M83.2134 82.579H80.8519C80.8464 82.579 80.8419 82.5835 80.8419 
                                82.589V100.99C80.8419 100.996 80.8464 101 80.8519 101H83.2134M83.2134 
                                82.579V101M83.2134 82.579L85.5675 79.9667C85.5737 79.9599 85.585 79.9643
                                85.585 79.9734V98.3646C85.585 98.3671 85.5841 98.3694 85.5824 98.3713L83.2134 101"
                                stroke="#72C5FF"
                            />
                            <rect x="32.6205" y="61.5263" width="5.5336" height="11.4035" rx="0.01" fill="#2E3440" stroke="#D9D9D9"/>
                            <rect x="66.6126" y="61.5263" width="5.5336" height="11.4035" rx="0.01" fill="#2E3440" stroke="#D9D9D9"/>
                            <path d="M38.1541 61.5363C38.1541 61.5308 38.1586 61.5263 38.1641 61.5263H40.5157C40.5212 
                                61.5263 40.5257 61.5308 40.5257 61.5363V70.7325C40.5257 70.7352 40.5245 
                                70.7379 40.5225 70.7398L38.1709 72.9143C38.1645 72.9202 38.1541 72.9157 
                                38.1541 72.9069V61.5363Z" fill="#2E3440" stroke="#D9D9D9"
                            />
                            <path d="M72.1462 61.5363C72.1462 61.5308 72.1507 61.5263 72.1562 
                                61.5263H74.5078C74.5133 61.5263 74.5178 61.5308 74.5178 61.5363V70.7325C74.5178 
                                70.7352 74.5166 70.7379 74.5146 70.7398L72.163 72.9143C72.1566 72.9202 72.1462 
                                72.9157 72.1462 72.9069V61.5363Z" fill="#2E3440" stroke="#D9D9D9"
                            />
                        </svg>
                        <p class="analysis-button-icon-caption">Analysis</p>
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

export default FeaAnalysis;