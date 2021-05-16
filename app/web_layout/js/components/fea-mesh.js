class FeaMesh extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: {
                node: "Node",
            },
            buttonFullNames: {
                node: "node",
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

                .mesh-button {
                    margin: 0rem;
                    padding-top: 0.7rem;
                    padding-bottom: 0.7rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .mesh-button:hover {
                    background: #2d303b;
                }

                .mesh-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .mesh-button-icon {
                    color: #2E3440;
                    width: 3rem;
                    height: 3rem;
                }

                .mesh-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                    width: 3rem;
                    font-size: 85%;
                }

            </style>
            <div class=wrapper>
                <button class="mesh-button">
                    <div class="mesh-button-icon-content">
                        <svg class="mesh-button-icon" width="102" height="102" viewBox="0 0 102 102" fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Mesh</title>
                            <rect x="63.5" y="88.5" width="12.5" height="12.5" rx="0.01" stroke="#D9D9D9"/>
                            <rect x="63.5" y="76" width="12.5" height="12.5" stroke="#D9D9D9"/>
                            <rect x="1" y="88.5" width="12.5" height="12.5" stroke="#D9D9D9"/>
                            <rect x="13.5" y="88.5" width="12.5" height="12.5" stroke="#D9D9D9"/>
                            <rect x="26" y="88.5" width="12.5" height="12.5" stroke="#D9D9D9"/>
                            <rect x="38.5" y="88.5" width="12.5" height="12.5" stroke="#D9D9D9"/>
                            <rect x="51" y="88.5" width="12.5" height="12.5" stroke="#D9D9D9"/>
                            <rect x="1" y="76" width="12.5" height="12.5" stroke="#D9D9D9"/>
                            <rect x="13.5" y="76" width="12.5" height="12.5" stroke="#D9D9D9"/>
                            <rect x="26" y="76" width="12.5" height="12.5" stroke="#D9D9D9"/>
                            <rect x="38.5" y="76" width="12.5" height="12.5" stroke="#D9D9D9"/>
                            <rect x="51" y="76" width="12.5" height="12.5" stroke="#D9D9D9"/>
                            <rect x="13.5" y="38.5" width="12.5" height="12.5" transform="rotate(-90 13.5 38.5)" stroke="#D9D9D9"/>
                            <rect x="1" y="38.5" width="12.5" height="12.5" rx="0.01" transform="rotate(-90 1 38.5)" stroke="#D9D9D9"/>
                            <rect x="13.5" y="76" width="12.5" height="12.5" transform="rotate(-90 13.5 76)" stroke="#D9D9D9"/>
                            <rect x="13.5" y="63.5022" width="12.5" height="12.5" transform="rotate(-90.0103 13.5 63.5022)" stroke="#D9D9D9"/>
                            <rect x="13.5" y="51" width="12.5" height="12.5" transform="rotate(-90 13.5 51)" stroke="#D9D9D9"/>
                            <rect x="1" y="76" width="12.5" height="12.5" transform="rotate(-90 1 76)" stroke="#D9D9D9"/>
                            <rect x="1" y="63.5" width="12.5" height="12.5" transform="rotate(-90 1 63.5)" stroke="#D9D9D9"/>
                            <rect x="1" y="51" width="12.5" height="12.5" transform="rotate(-90 1 51)" stroke="#D9D9D9"/>
                            <path d="M76 88.5041C76 88.5015 76.0011 88.4989 76.0029 88.4971L84.3163 80.1837C84.3226 80.1774 84.3333 
                                80.1819 84.3333 80.1908V92.6625C84.3333 92.6652 84.3323 92.6677 84.3304 92.6696L76.0171 
                                100.983C76.0108 100.989 76 100.985 76 100.976V88.5041Z" stroke="#D9D9D9"
                            />
                            <path d="M92.6667 59.3375C92.6667 59.3348 92.6677 59.3323 92.6696 59.3304L100.983 
                                51.0171C100.989 51.0108 101 51.0152 101 51.0241V63.4959C101 63.4985 100.999 63.5011 
                                100.997 63.5029L92.6838 71.8163C92.6775 71.8226 92.6667 71.8181 92.6667 71.8092V59.3375Z" stroke="#D9D9D9"
                            />
                            <path d="M92.6667 71.8333L101 63.5V76L92.6667 84.3333V71.8333Z" stroke="#D9D9D9"/>
                            <path d="M76 76L84.3333 67.6667V80.1667L76 88.5V76Z" stroke="#D9D9D9"/>
                            <path d="M26 38.5L34.3333 30.1667V42.6667L26 51V38.5Z" stroke="#D9D9D9"/>
                            <path d="M42.6667 9.33748C42.6667 9.33482 42.6677 9.33228 42.6696 9.33041L50.9829
                                1.01707C50.9892 1.01077 51 1.01523 51 1.02414V13.4959C51 13.4985 50.9989 13.5011 
                                50.9971 13.5029L42.6837 21.8163C42.6774 21.8226 42.6667 21.8181 42.6667 21.8092V9.33748Z" stroke="#D9D9D9"
                            />
                            <path d="M42.6667 21.8333L51 13.5V26L42.6667 34.3333V21.8333Z" stroke="#D9D9D9"/>
                            <path d="M26 26L34.3333 17.6667V30.1667L26 38.5V26Z" stroke="#D9D9D9"/>
                            <path d="M26 63.5L34.3333 55.1667V67.6667L26 76V63.5Z" stroke="#D9D9D9"/>
                            <path d="M42.6667 34.3333L51 26V38.5L42.6667 46.8333V34.3333Z" stroke="#D9D9D9"/>
                            <path d="M42.6667 46.8333L51 38.5V51L42.6667 59.3333V46.8333Z" stroke="#D9D9D9"/>
                            <path d="M26 51L34.3333 42.6667V55.1667L26 63.5V51Z" stroke="#D9D9D9"/>
                            <path d="M63.5 76L71.8333 67.6667H84.3333L76 76H63.5Z" stroke="#D9D9D9"/>
                            <path d="M80.1908 59.3333C80.1819 59.3333 80.1775 59.3226 80.1838 59.3163L88.4971 51.0029C88.499 
                                51.0011 88.5015 51 88.5042 51H100.976C100.985 51 100.989 51.0108 100.983 51.0171L92.6696 
                                59.3304C92.6677 59.3323 92.6652 59.3333 92.6625 59.3333H80.1908Z" stroke="#D9D9D9"
                            />
                            <path d="M1.02414 26C1.01523 26 1.01077 25.9892 1.01707 25.9829L9.33041 17.6696C9.33228 
                                17.6677 9.33482 17.6667 9.33748 17.6667H21.8092C21.8181 17.6667 21.8226 17.6774 
                                21.8163 17.6837L13.5029 25.9971C13.5011 25.9989 13.4985 26 13.4959 26H1.02414Z" stroke="#D9D9D9"
                            />
                            <path d="M17.6667 9.33333L26 1H38.5L30.1667 9.33333H17.6667Z" stroke="#D9D9D9"/>
                            <path d="M13.5 26L21.8333 17.6667H34.3333L26 26H13.5Z" stroke="#D9D9D9"/>
                            <path d="M30.1908 9.33333C30.1819 9.33333 30.1774 9.32256 30.1837 9.31626L38.4971 
                                1.00293C38.4989 1.00105 38.5015 1 38.5041 1H50.9758C50.9848 1 50.9892 1.01077 
                                50.9829 1.01707L42.6696 9.3304C42.6677 9.33228 42.6652 9.33333 42.6625 9.33333H30.1908Z" stroke="#D9D9D9"
                            />
                            <path d="M26 76L34.3333 67.6667H46.8333L38.5 76H26Z" stroke="#D9D9D9"/>
                            <path d="M42.6667 59.3333L51 51H63.5L55.1667 59.3333H42.6667Z" stroke="#D9D9D9"/>
                            <path d="M38.5 76L46.8333 67.6667H59.3333L51 76H38.5Z" stroke="#D9D9D9"/>
                            <path d="M55.1667 59.3333L63.5 51H76L67.6667 59.3333H55.1667Z" stroke="#D9D9D9"/>
                            <path d="M51 76L59.3333 67.6667H71.8333L63.5 76H51Z" stroke="#D9D9D9"/>
                            <path d="M67.6667 59.3333L76 51H88.5L80.1667 59.3333H67.6667Z" stroke="#D9D9D9"/>
                            <path d="M84.3333 67.6667L92.6666 59.3333V71.8333L84.3333 80.1667V67.6667Z" stroke="#72C5FF"/>
                            <path d="M84.3333 80.1667L92.6666 71.8333V84.3333L84.3333 92.6667V80.1667Z" stroke="#72C5FF"/>
                            <path d="M34.3333 17.6667L42.6667 9.33334V21.8333L34.3333 30.1667V17.6667Z" stroke="#72C5FF"/>
                            <path d="M34.3333 30.1667L42.6667 21.8333V34.3333L34.3333 42.6667V30.1667Z" stroke="#72C5FF"/>
                            <path d="M34.3333 42.6667L42.6667 34.3333V46.8333L34.3333 55.1667V42.6667Z" stroke="#72C5FF"/>
                            <path d="M34.3333 55.1667L42.6667 46.8333V59.3333L34.3333 67.6667V55.1667Z" stroke="#72C5FF"/>
                            <path d="M71.8333 67.6667L80.1667 59.3333H92.6667L84.3333 67.6667H71.8333Z" stroke="#72C5FF"/>
                            <path d="M9.33334 17.6667L17.6667 9.33334H30.1667L21.8333 17.6667H9.33334Z" stroke="#72C5FF"/>
                            <path d="M21.8333 17.6667L30.1667 9.33334H42.6667L34.3333 17.6667H21.8333Z" stroke="#72C5FF"/>
                            <path d="M34.3333 67.6667L42.6667 59.3333H55.1667L46.8333 67.6667H34.3333Z" stroke="#72C5FF"/>
                            <path d="M46.8333 67.6667L55.1667 59.3333H67.6667L59.3333 67.6667H46.8333Z" stroke="#72C5FF"/>
                            <path d="M59.3333 67.6667L67.6667 59.3333H80.1667L71.8333 67.6667H59.3333Z" stroke="#72C5FF"/>
                        </svg>
                        <p class="mesh-button-icon-caption">Mesh</p>
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

export default FeaMesh;