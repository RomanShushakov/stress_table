class FeaPreprocessorMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "geometry-menu-button",
                "properties-menu-button",
                "mesh-menu-button",
                "load-menu-button",
                "boundary-condition-menu-button",
                "analysis-menu-button",
            ],

            menuNames: {
                "geometry-menu-button": "geometry-menu",
                "properties-menu-button": "properties-menu",
                "mesh-menu-button": "mesh-menu",
                "load-menu-button": "load-menu",
                "boundary-condition-menu-button": "boundary-condition-menu",
                "analysis-menu-button": "analysis-menu",
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

                .geometry-menu-button {
                    margin: 0rem;
                    padding-top: 0.7rem;
                    padding-bottom: 0.7rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .geometry-menu-button:hover {
                    background: #2d303b;
                }

                .geometry-menu-button:hover .geometry-menu-button-icon {
                    color: #2d303b;
                }

                .active .geometry-menu-button-icon {
                    color: #3b4453;
                }

                .active:hover .geometry-menu-button-icon {
                    color: #242932;
                }

                .geometry-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .geometry-menu-button-icon {
                    color: #2E3440;
                    width: 3.5rem;
                    height: 3.5rem;
                }

                .geometry-menu-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                    width: 3.5rem;
                    font-size: 85%;
                }

                .properties-menu-button {
                    margin: 0rem;
                    padding-top: 0.7rem;
                    padding-bottom: 0.7rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .properties-menu-button:hover {
                    background: #2d303b;
                }

                .properties-menu-button:hover .properties-menu-button-icon {
                    color: #2d303b;
                }

                .properties-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .properties-menu-button-icon {
                    color: #2E3440;
                    width: 3.5rem;
                    height: 3.5rem;
                }

                .properties-menu-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                    width: 3.5rem;
                    font-size: 85%;
                }

                .active .properties-menu-button-icon {
                    color: #3b4453;
                }

                .active:hover .properties-menu-button-icon {
                    color: #242932;
                }

                .mesh-menu-button {
                    margin: 0rem;
                    padding-top: 0.7rem;
                    padding-bottom: 0.7rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .mesh-menu-button:hover {
                    background: #2d303b;
                }

                .mesh-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .mesh-menu-button-icon {
                    color: #2E3440;
                    width: 3.5rem;
                    height: 3.5rem;
                }

                .mesh-menu-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                    width: 3.5rem;
                    font-size: 85%;
                }

                .active .mesh-menu-button-icon {
                    color: #3b4453;
                }

                .active:hover .mesh-menu-button-icon {
                    color: #242932;
                }

                .load-menu-button {
                    margin: 0rem;
                    padding-top: 0.7rem;
                    padding-bottom: 0.7rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .load-menu-button:hover {
                    background: #2d303b;
                }

                .load-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .load-menu-button-icon {
                    color: #2E3440;
                    width: 3.5rem;
                    height: 3.5rem;
                }

                .load-menu-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                    width: 3.5rem;
                    font-size: 85%;
                }

                .active .load-menu-button-icon {
                    color: #3b4453;
                }

                .active:hover .load-menu-button-icon {
                    color: #242932;
                }

                .boundary-condition-menu-button {
                    margin: 0rem;
                    padding-top: 0.325rem;
                    padding-bottom: 0.325rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .boundary-condition-menu-button:hover {
                    background: #2d303b;
                }

                .boundary-condition-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .boundary-condition-menu-button-icon {
                    color: #2E3440;
                    width: 3.5rem;
                    height: 3.5rem;
                }

                .boundary-condition-menu-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                    width: 3.5rem;
                    font-size: 85%;
                }

                .active .boundary-condition-menu-button-icon {
                    color: #3b4453;
                }

                .active:hover .boundary-condition-menu-button-icon {
                    color: #242932;
                }

                .analysis-menu-button {
                    margin: 0rem;
                    padding-top: 0.7rem;
                    padding-bottom: 0.7rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .analysis-menu-button:hover {
                    background: #2d303b;
                }

                .analysis-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .analysis-menu-button-icon {
                    color: #2E3440;
                    width: 3.5rem;
                    height: 3.5rem;
                }

                .analysis-menu-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                    width: 3.5rem;
                    font-size: 85%;
                }

                .active .analysis-menu-button-icon {
                    color: #3b4453;
                }

                .active:hover .analysis-menu-button-icon {
                    color: #242932;
                }

                .active:hover {
                    background: #242932;
                }

                .active {
                    background: #3b4453;
                }

                .hidden {
                    display: none;
                }
            </style>
            <div class="wrapper">
                <button class="geometry-menu-button">
                    <div class="geometry-menu-button-icon-content">
                        <svg class=geometry-menu-button-icon width="100" height="100" viewBox="0 0 100 100" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Geometry</title>
                            <g fill="currentColor">
                                <path d="M6.77588 74.5L44.4645 1.0947L82.1532 74.5H6.77588Z" stroke="#D9D9D9"/>
                                <rect x="52.5" y="29.5" width="40" height="42" stroke="#72C5FF"/>
                                <circle cx="77.5" cy="65.5" r="22" stroke="#D9D9D9"/>
                                <path d="M33.8771 87.5L21.5761 66L33.8771 44.5H58.4554L69.4385 66L58.4554 
                                    87.5H33.8771Z" stroke="#72C5FF"
                                />
                            </g>
                        </svg>
                        <p class="geometry-menu-button-icon-caption">Geometry</p>
                    </div>
                </button>

                <button class="properties-menu-button">
                    <div class="properties-menu-button-icon-content">
                        <svg class="properties-menu-button-icon" width="101" height="101" viewBox="0 0 101 101" fill="none" 
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
                        <p class="properties-menu-button-icon-caption">Properties</p>
                    </div>
                </button>

                <button class="mesh-menu-button">
                    <div class="mesh-menu-button-icon-content">
                        <svg class="mesh-menu-button-icon" width="102" height="102" viewBox="0 0 102 102" fill="none"
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
                        <p class="mesh-menu-button-icon-caption">Mesh</p>
                    </div>
                </button>

                <button class="load-menu-button">
                    <div class="load-menu-button-icon-content">
                        <svg class="load-menu-button-icon" width="102" height="102" viewBox="0 0 102 102" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Load</title>
                            <path d="M85.5 56.7797L78.1388 45.339H92.8612L85.5 56.7797Z" fill="#72C5FF" stroke="#72C5FF"/>
                            <path d="M86 24.5763L86 44.9152" stroke="#72C5FF"/>
                            <path d="M101 101V73.0339" stroke="#72C5FF" stroke-linecap="round" stroke-dasharray="4 4"/>
                            <path d="M34.5 57.7797C76 60.322 92.5 64.983 101 73.0339" stroke="#72C5FF" stroke-dasharray="4 4"/>
                            <path d="M34.5 85.7458C76 88.2881 92.5 92.9491 101 101" stroke="#72C5FF" stroke-dasharray="4 4"/>
                            <path d="M34.3333 1H1V85.7457H101V57.4971H34.3333V1Z" stroke="#D9D9D9"/>
                        </svg>
                        <p class="load-menu-button-icon-caption">Load</p>
                    </div>
                </button>

                <button class="boundary-condition-menu-button">
                    <div class="boundary-condition-menu-button-icon-content">
                        <svg class="boundary-condition-menu-button-icon" width="100" height="101" viewBox="0 0 100 101" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Boundary condition</title>
                            <path d="M35.7143 1H7.14285V93.0761H92.8571V62.3841H35.7143V1Z" stroke="#D9D9D9"/>
                            <line x1="100" y1="94.0761" y2="94.0761" stroke="#72C5FF" stroke-width="2"/>
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 2.38095 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 7.14285 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 11.9048 101)"
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 16.6667 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 21.4286 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 26.1905 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 30.9524 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 69.0476 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 73.8095 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 78.5714 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 83.3333 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 88.0952 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 92.8571 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 35.7143 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 40.4762 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 45.2381 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 50 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 54.7619 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 59.5238 101)" 
                                stroke="#72C5FF"
                            />
                            <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 64.2857 101)" 
                                stroke="#72C5FF"
                            />
                        </svg>
                        <p class="boundary-condition-menu-button-icon-caption">Boundary condition</p>
                    </div>
                </button>

                <button class="analysis-menu-button">
                    <div class="analysis-menu-button-icon-content">
                        <svg class="analysis-menu-button-icon" width="102" height="102" viewBox="0 0 102 102" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
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
                        <p class="analysis-menu-button-icon-caption">Analysis</p>
                    </div>
                </button>

            </div>
        `;

        this.shadowRoot.querySelector(".geometry-menu-button").addEventListener("click", () => this.toggle("geometry-menu-button"));

        this.shadowRoot.querySelector(".properties-menu-button").addEventListener("click", () => this.toggle("properties-menu-button"));

        this.shadowRoot.querySelector(".mesh-menu-button").addEventListener("click", () => this.toggle("mesh-menu-button"));

        this.shadowRoot.querySelector(".load-menu-button").addEventListener("click", () => this.toggle("load-menu-button"));

        this.shadowRoot.querySelector(".boundary-condition-menu-button").addEventListener("click", 
            () => this.toggle("boundary-condition-menu-button"));

        this.shadowRoot.querySelector(".analysis-menu-button").addEventListener("click", () => this.toggle("analysis-menu-button"));
    }

    set toggleButton(buttonName) {
        this.toggle(buttonName);
    }


    connectedCallback() {
        this.toggle("geometry-menu-button");
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

    activatePostprocessor() {
        this.dispatchEvent(new CustomEvent("activate-postprocessor", {
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

export default FeaPreprocessorMenuButtons;
