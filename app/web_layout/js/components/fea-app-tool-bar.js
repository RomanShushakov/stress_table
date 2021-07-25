class FeaAppToolBar extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
        };

        this.state = {};

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                }

                .wrapper {
                    padding-top: 0.5rem;
                    background: #1b1f25;
                    border-left: 0.1rem solid #5c687a;
                    border-right: 0.1rem solid #5c687a;
                    border-bottom: 0.1rem solid #5c687a;
                }

                .save-button {
                    background: #1b1f25;
                    border: #1b1f25;
                }

                .save-button :hover {
                    background: #20232a;
                    border: #20232a;
                }

                .save-button-icon {
                    width: 1.7rem;
                    height: 1.7rem;
                    padding: 0.25rem;
                }

                .load-button {
                    background: #1b1f25;
                    border: #1b1f25;
                }

                .load-button :hover {
                    background: #20232a;
                    border: #20232a;
                }

                .load-button-icon {
                    width: 1.7rem;
                    height: 1.7rem;
                    padding: 0.25rem;
                }

                .undo-button {
                    background: #1b1f25;
                    border: #1b1f25;
                }

                .undo-button :hover {
                    background: #20232a;
                    border: #20232a;
                }

                .undo-button-icon {
                    width: 1.7rem;
                    height: 1.7rem;
                    padding: 0.25rem;
                }

                .redo-button {
                    background: #1b1f25;
                    border: #1b1f25;
                }

                .redo-button :hover {
                    background: #20232a;
                    border: #20232a;
                }

                .redo-button-icon {
                    width: 1.7rem;
                    height: 1.7rem;
                    padding: 0.25rem;
                }

                .show-hide-geometry-button {
                    background: #1b1f25;
                    border: #1b1f25;
                }

                .show-hide-geometry-button :hover {
                    background: #20232a;
                    border: #20232a;
                }

                .show-hide-geometry-button-icon {
                    width: 1.7rem;
                    height: 1.7rem;
                    padding: 0.25rem;
                }

                .show-hide-mesh-button {
                    background: #1b1f25;
                    border: #1b1f25;
                }

                .show-hide-mesh-button :hover {
                    background: #20232a;
                    border: #20232a;
                }

                .show-hide-mesh-button-icon {
                    width: 1.7rem;
                    height: 1.7rem;
                    padding: 0.25rem;
                }

                .x-y-view-button {
                    background: #1b1f25;
                    border: #1b1f25;
                }

                .x-y-view-button :hover {
                    background: #20232a;
                    border: #20232a;
                }

                .z-y-view-button {
                    background: #1b1f25;
                    border: #1b1f25;
                }

                .z-y-view-button :hover {
                    background: #20232a;
                    border: #20232a;
                }

                .x-z-view-button {
                    background: #1b1f25;
                    border: #1b1f25;
                }

                .x-z-view-button :hover {
                    background: #20232a;
                    border: #20232a;
                }

                .view-button-icon {
                    width: 1.7rem;
                    height: 1.7rem;
                    padding: 0.25rem;
                }

                .isometric-view-button {
                    background: #1b1f25;
                    border: #1b1f25;
                }

                .isometric-view-button :hover {
                    background: #20232a;
                    border: #20232a;
                }

                .isometric-view-button-icon {
                    width: 2.1rem;
                    height: 2.1rem;
                }

                .tool-bars {
                    display: flex;
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }
            </style>
            <div class="wrapper">
                <nav>
                    <ul class="tool-bars">
                        <li>
                            <button class="save-button">
                                <svg class="save-button-icon" width="22" height="25" viewBox="0 0 22 25" fill="none" 
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <title>Save</title>
                                    <path d="M1 4.01V23.99C1 23.9955 1.00448 24 1.01 24H20.99C20.9955 
                                        24 21 23.9955 21 23.99V8.50414C21 8.50149 20.9989 8.49895 20.9971 
                                        8.49707L19 6.5L16.5029 4.00293C16.5011 4.00105 16.4985 4 16.4959 
                                        4H1.01C1.00448 4 1 4.00448 1 4.01Z" stroke="#D9D9D9"
                                    />
                                    <rect x="6" y="14" width="10" height="10" stroke="#D9D9D9"/>
                                    <rect x="6" y="4" width="10" height="5" stroke="#D9D9D9"/>
                                    <rect x="8" y="16" width="2" height="5" stroke="#D9D9D9"/>
                                    <path d="M17 1C15 1.5 11 2 11 7.49999" stroke="#72C5FF"/>
                                    <line x1="8.35355" y1="4.64645" x2="11.3536" y2="7.64645" stroke="#72C5FF"/>
                                    <line x1="10.6464" y1="7.64645" x2="13.6464" y2="4.64645" stroke="#72C5FF"/>
                                </svg>
                            </button>
                        </li>
                        <li>
                            <button class="load-button">
                                <svg class="load-button-icon" width="26" height="21" viewBox="0 0 26 21" fill="none" 
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <title>Load</title>
                                    <path d="M1 5.01V19.9759C1 19.9848 1.01077 19.9892 1.01707 19.9829L9.99707
                                        11.0029C9.99895 11.0011 10.0015 11 10.0041 11H15.99C15.9955 11 16 10.9955 
                                        16 10.99V5.01C16 5.00448 15.9955 5 15.99 5H9.01C9.00448 5 9 4.99552 
                                        9 4.99V3.51C9 3.50448 8.99552 3.5 8.99 3.5H4.51C4.50448 3.5 4.5 3.50448 
                                        4.5 3.51V4.99C4.5 4.99552 4.49552 5 4.49 5H1.01C1.00448 5 1 5.00448 1 5.01Z" 
                                        stroke="#D9D9D9"
                                    />
                                    <path d="M9.98299 11.0029L1.01704 19.9829C1.01075 19.9892 1.01521 20 1.02412 
                                        20H16.0099C16.0126 20 16.0151 19.9989 16.017 19.9971L24.983 11.0171C24.9893
                                        11.0108 24.9848 11 24.9759 11H9.99007C9.98742 11 9.98487 11.0011 9.98299 11.0029Z" 
                                        stroke="#D9D9D9"
                                    />
                                    <path d="M14 1.00001C16 0.5 19 -0.500012 22.5 3.99998" stroke="#72C5FF"/>
                                    <path d="M23 1.5L22.6464 4.64645" stroke="#72C5FF"/>
                                    <line x1="22.8419" y1="4.47434" x2="19.8419" y2="3.47434" stroke="#72C5FF"/>
                                </svg>
                            </button>
                        </li>
                        <li>
                            <button class="undo-button">
                                <svg class="undo-button-icon" width="30" height="22" viewBox="0 0 30 22" fill="none" 
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <title>Undo</title>
                                    <path d="M12 19 0 11.5 12 4V9h3.8335C20.87061 9 24 11.62891 24 15.86035a5.59289 
                                        5.59289 0 0 1-.062.71924l-.1792 1.52148-.7544-1.09423C21.291 14.52148 
                                        18.41943 14 16.31249 14H12ZM2 11.5l9 5.624V13h5.31249a9.0867 9.0867 
                                        0 0 1 6.67433 2.42432C22.78125 12.01807 20.14014 10 15.8335 10H11V5.876Z" style="fill:#D9D9D9;"
                                    />
                                </svg>
                            </button>
                        </li>
                        <li>
                            <button class="redo-button">
                                <svg class="redo-button-icon" width="31" height="22" viewBox="0 0 31 22" fill="none" 
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <title>Redo</title>
                                    <path d="M12 14H7.6875C5.58057 14 2.709 14.52148.99561 17.00684l-.7544 1.09423L.062 
                                        16.57959A5.59289 5.59289 0 0 1 0 15.86035C0 11.62891 3.12939 9 8.1665 9H12V4l12 
                                        7.5L12 19Zm1-8.124V10H8.1665c-4.30664 0-6.94775 2.01807-7.15332 5.42432A9.08669 
                                        9.08669 0 0 1 7.6875 13H13v4.124L22 11.5Z" style="fill:#D9D9D9;"
                                    />
                                </svg>
                            </button>
                        </li>
                        <li>
                            <button class="show-hide-geometry-button">
                                <svg class="show-hide-geometry-button-icon" width="26" height="26" viewBox="0 0 26 26" fill="none" 
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <title>Show/Hide geometry</title>
                                    <path d="M16 10.0041L16.0007 24.9759C16.0007 24.9848 16.0115 24.9892 
                                        16.0178 24.9829L24.9971 16.0029C24.9989 16.0011 25 15.9985 25 15.9959L24.9992 
                                        1.02414C24.9992 1.01523 24.9885 1.01077 24.9822 1.01707L16.0029 9.99707C16.0011 
                                        9.99895 16 10.0015 16 10.0041Z" fill="#7475E6" stroke="#d9d9d9"
                                    />
                                    <path d="M1 10.01V24.99C1 24.9955 1.00448 25 1.01 25H15.99C15.9955 25 16 24.9955 16 
                                        24.99L16 10.01C16 10.0045 15.9955 10 15.99 10H1.01C1.00448 10 1 10.0045 1 10.01Z" 
                                        fill="#7475E6" stroke="#d9d9d9"
                                    />
                                    <path d="M9.98299 1.00293L1.01704 9.98293C1.01075 9.98924 1.01521 10 1.02412 10H16.0099C16.0126 
                                        10 16.0151 9.99895 16.017 9.99707L24.983 1.01707C24.9893 1.01076 24.9848 1 24.9759 
                                        1H9.99007C9.98742 1 9.98487 1.00106 9.98299 1.00293Z" fill="#7475E6" stroke="#d9d9d9"
                                    />
                                </svg>
                            </button>
                        </li>
                        <li>
                            <button class="show-hide-mesh-button">
                                <svg class="show-hide-mesh-button-icon" width="26" height="26" viewBox="0 0 26 26" fill="none" 
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <title>Show/Hide mesh</title>
                                    <rect x="11" y="20" width="5" height="5" rx="0.01" stroke="#D9D9D9"/>
                                    <rect x="1" y="20" width="5" height="5" stroke="#D9D9D9"/>
                                    <rect x="6" y="20" width="5" height="5" stroke="#D9D9D9"/>
                                    <rect x="11" y="10" width="5" height="5" stroke="#D9D9D9"/>
                                    <rect x="1" y="10" width="5" height="5" rx="0.01" stroke="#D9D9D9"/>
                                    <rect x="6" y="10" width="5" height="5" stroke="#D9D9D9"/>
                                    <rect x="11" y="15" width="5" height="5" stroke="#D9D9D9"/>
                                    <rect x="1" y="15" width="5" height="5" stroke="#D9D9D9"/>
                                    <rect x="6" y="15" width="5" height="5" stroke="#D9D9D9"/>
                                    <path d="M11 10L14 7H19L16 10H11Z" stroke="#D9D9D9"/>
                                    <path d="M1.02414 10C1.01523 10 1.01077 9.98923 1.01707 9.98293L3.99707 
                                        7.00293C3.99895 7.00105 4.00149 7 4.00414 7H8.97586C8.98477 7 8.98923 
                                        7.01077 8.98293 7.01707L6.00293 9.99707C6.00105 9.99895 5.99851
                                        10 5.99586 10H1.02414Z" stroke="#D9D9D9"
                                    />
                                    <path d="M6 10L9 7H14L11 10H6Z" stroke="#D9D9D9"/>
                                    <path d="M17.0241 4C17.0152 4 17.0108 3.98923 17.0171 3.98293L19.9971 
                                        1.00293C19.9989 1.00105 20.0015 1 20.0041 1H24.9759C24.9848 
                                        1 24.9892 1.01077 24.9829 1.01707L22.0029 3.99707C22.0011 3.99895 
                                        21.9985 4 21.9959 4H17.0241Z" stroke="#D9D9D9"
                                    />
                                    <path d="M7 4L10 1H15L12 4H7Z" stroke="#D9D9D9"/>
                                    <path d="M12 4L15 1H20L17 4H12Z" stroke="#D9D9D9"/>
                                    <path d="M14 7L17 4H22L19 7H14Z" stroke="#D9D9D9"/>
                                    <path d="M4 7L7 4H12L9 7H4Z" stroke="#D9D9D9"/>
                                    <path d="M9 7L12 4H17L14 7H9Z" stroke="#D9D9D9"/>
                                    <path d="M16 20.0041C16 20.0015 16.0011 19.9989 16.0029 
                                        19.9971L18.9829 17.0171C18.9892 17.0108 19 17.0152 19 
                                        17.0241V21.9959C19 21.9985 18.9989 22.0011 18.9971 22.0029L16.0171 
                                        24.9829C16.0108 24.9892 16 24.9848 16 24.9759V20.0041Z" stroke="#D9D9D9"
                                    />
                                    <path d="M22 14L25 11V16L22 19V14Z" stroke="#D9D9D9"/>
                                    <path d="M19 17L22 14V19L19 22V17Z" stroke="#D9D9D9"/>
                                    <path d="M16 10L19 7V12L16 15V10Z" stroke="#D9D9D9"/>
                                    <path d="M22 4.00414C22 4.00149 22.0011 3.99895 22.0029 3.99707L24.9829 
                                        1.01707C24.9892 1.01077 25 1.01523 25 1.02414V5.99586C25 5.99851 
                                        24.9989 6.00105 24.9971 6.00293L22.0171 8.98293C22.0108 8.98923
                                        22 8.98477 22 8.97586V4.00414Z" stroke="#D9D9D9"
                                    />
                                    <path d="M19 7L22 4V9L19 12V7Z" stroke="#D9D9D9"/>
                                    <path d="M16 15L19 12V17L16 20V15Z" stroke="#D9D9D9"/>
                                    <path d="M22 9L25 6V11L22 14V9Z" stroke="#D9D9D9"/>
                                    <path d="M19 12L22 9V14L19 17V12Z" stroke="#D9D9D9"/>
                                </svg>
                            </button>
                        </li>
                        <li>
                            <button class="x-y-view-button">
                                <svg class="view-button-icon" width="67" height="64" viewBox="0 0 67 64" fill="none" 
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <title>Plane-XY</title>
                                    <line x1="12" y1="54.5" x2="42" y2="54.5" stroke="#622629" stroke-width="5"/>
                                    <path d="M9.49999 10L16 27L2.99999 27L9.49999 10Z" fill="#234D2D" stroke="#234D2D" stroke-width="3"/>
                                    <path d="M59 54.5L42 61V48L59 54.5Z" fill="#622629" stroke="#622629" stroke-width="3"/>
                                    <line x1="9.85602" y1="56.7927" x2="9.85602" y2="26.7927" stroke="#234D2D" stroke-width="5"/>
                                    <path d="M60.2891 36.2305L63.6973 30.7812H65.9043L61.4121 37.832L66.0117 
                                        45H63.7852L60.2891 39.4531L56.7734 45H54.5566L59.166 37.832L54.6641 
                                        30.7812H56.8613L60.2891 36.2305Z" fill="#D9D9D9"
                                    />
                                    <path d="M26.9863 10.9199L30.6973 3.78125H32.8262L27.9238 
                                        12.6973V18H26.0488V12.6973L21.1465 3.78125H23.2949L26.9863 10.9199Z" fill="#D9D9D9"
                                    />
                                </svg>
                            </button>
                        </li>
                        <li>
                            <button class="z-y-view-button">
                            <svg class="view-button-icon" width="66" height="64" viewBox="0 0 66 64" fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Plane-ZY</title>
                                <line x1="12" y1="54.5" x2="42" y2="54.5" stroke="#7475E6" stroke-width="5"/>
                                <path d="M9.49999 10L16 27L2.99999 27L9.49999 10Z" fill="#234D2D" stroke="#234D2D" stroke-width="3"/>
                                <path d="M59 54.5L42 61V48L59 54.5Z" fill="#7475E6" stroke="#7475E6" stroke-width="3"/>
                                <line x1="9.85602" y1="56.7927" x2="9.85602" y2="26.7927" stroke="#234D2D" stroke-width="5"/>
                                <path d="M57.0566 43.4668H65.1914V45H54.8398V43.5938L62.6328 
                                    32.3242H54.9668V30.7812H64.8789V32.1582L57.0566 43.4668Z" fill="#D9D9D9"
                                />
                                <path d="M26.9863 10.9199L30.6973 3.78125H32.8262L27.9238 
                                    12.6973V18H26.0488V12.6973L21.1465 3.78125H23.2949L26.9863 10.9199Z" fill="#D9D9D9"
                                >
                            </svg>
                            </button>
                        </li>
                        <li>
                            <button class="x-z-view-button">
                                <svg class="view-button-icon" width="67" height="64" viewBox="0 0 67 64" fill="none" 
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <title>Plane-XZ</title>
                                    <line x1="12" y1="54.5" x2="42" y2="54.5" stroke="#622629" stroke-width="5"/>
                                    <path d="M9.49999 10L16 27L2.99999 27L9.49999 10Z" fill="#7475E6" stroke="#7475E6" stroke-width="3"/>
                                    <path d="M59 54.5L42 61V48L59 54.5Z" fill="#622629" stroke="#622629" stroke-width="3"/>
                                    <line x1="9.85602" y1="56.7927" x2="9.85602" y2="26.7927" stroke="#7475E6" stroke-width="5"/>
                                    <path d="M60.2891 36.2305L63.6973 30.7812H65.9043L61.4121 37.832L66.0117 
                                        45H63.7852L60.2891 39.4531L56.7734 45H54.5566L59.166 37.832L54.6641 
                                        30.7812H56.8613L60.2891 36.2305Z" fill="#D9D9D9"
                                    />
                                    <path d="M24.0566 16.4668H32.1914V18H21.8398V16.5938L29.6328 
                                        5.32422H21.9668V3.78125H31.8789V5.1582L24.0566 16.4668Z" fill="#D9D9D9"
                                    />
                                </svg>
                            </button>
                        </li>
                        <li>
                            <button class="isometric-view-button">
                                <svg class="isometric-view-button-icon" width="101" height="83" viewBox="0 0 101 83" fill="none" 
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <title>Isometric</title>
                                    <line x1="51.0761" y1="52.6001" x2="81.7653" y2="67.2347" stroke="#622629" stroke-width="5"/>
                                    <path d="M49.5 8L56 25L43 25L49.5 8Z" fill="#234D2D" stroke="#234D2D" stroke-width="3"/>
                                    <path d="M92.5968 72.3999L74.4544 70.9497L80.05 59.2156L92.5968 72.3999Z" fill="#622629" 
                                        stroke="#622629" stroke-width="3"
                                    />
                                    <line y1="-2.5" x2="34" y2="-2.5" transform="matrix(-0.902623 0.430431 0.430431 
                                        0.902623 50.3946 54.8566)" stroke="#7475E6" stroke-width="5"
                                    />
                                    <path d="M7.7978 72.3999L25.9402 70.9497L20.3446 59.2156L7.7978 
                                        72.3999Z" fill="#7475E6" stroke="#7475E6" stroke-width="3
                                    "/>
                                    <line x1="49.856" y1="53.7927" x2="49.856" y2="19.7927" stroke="#234D2D" stroke-width="5"/>
                                    <path d="M90.5469 44.4766L94.6367 37.9375H97.2852L91.8945 46.3984L97.4141 
                                        55H94.7422L90.5469 48.3438L86.3281 55H83.668L89.1992 46.3984L83.7969 
                                        37.9375H86.4336L90.5469 44.4766Z" fill="#D9D9D9"
                                    />
                                    <path d="M6.66797 54.1602H16.4297V56H4.00781V54.3125L13.3594 
                                        40.7891H4.16016V38.9375H16.0547V40.5898L6.66797 54.1602Z" fill="#D9D9D9"
                                    />
                                    <path d="M69.1836 13.5039L73.6367 4.9375H76.1914L70.3086 
                                        15.6367V22H68.0586V15.6367L62.1758 4.9375H64.7539L69.1836 13.5039Z" fill="#D9D9D9"
                                    />
                                </svg>
                            </button>
                        </li>
                    </ul>
                </nav>
            </div>
        `;

        this.shadowRoot.querySelector(".undo-button").addEventListener("click", () => this.undo());
        this.shadowRoot.querySelector(".redo-button").addEventListener("click", () => this.redo());
        this.shadowRoot.querySelector(".x-y-view-button").addEventListener("click", () => this.changeView("planeXY"));
        this.shadowRoot.querySelector(".z-y-view-button").addEventListener("click", () => this.changeView("planeZY"));
        this.shadowRoot.querySelector(".x-z-view-button").addEventListener("click", () => this.changeView("planeXZ"));
        this.shadowRoot.querySelector(".isometric-view-button").addEventListener("click", () => this.changeView("isometric"));
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set feModelError(error) {
        throw error;
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
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

    undo() {
        this.dispatchEvent(new CustomEvent("getActionIdForToolBar", {
            bubbles: true,
            composed: true,
        }));
        if (this.props.actionId > 1) {
            this.dispatchEvent(new CustomEvent("decreaseActionId", {
                bubbles: true,
                composed: true,
            }));
            this.dispatchEvent(new CustomEvent("getActionIdForToolBar", {
                bubbles: true,
                composed: true,
            }));
            const message = {"undo": { "actionId": this.props.actionId } };
            this.dispatchEvent(new CustomEvent("clientMessage", {
                bubbles: true,
                composed: true,
                detail: {
                    message: message,
                },
            }));
        }
    }

    redo() {
        this.dispatchEvent(new CustomEvent("getActionIdForToolBar", {
            bubbles: true,
            composed: true,
        }));
        const message = {"redo": { "actionId": this.props.actionId } };
        this.dispatchEvent(new CustomEvent("clientMessage", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));   
    }

    changeView(viewName) {
        this.dispatchEvent(new CustomEvent("changeView", {
            bubbles: true,
            composed: true,
            detail: { "selectedView": viewName },
        }));   
    }
}

export default FeaAppToolBar;
