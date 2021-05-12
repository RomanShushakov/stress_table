class FeaAppTitleBar extends HTMLElement {
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
                    border-bottom: 0.1rem solid #5c687a;
                }

                .save-button {
                    background: #1b1f25;
                    border: #1b1f25;
                }

                .save-button :hover {
                    background: #212933;
                    border: #212933;
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
                    background: #212933;
                    border: #212933;
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
                    background: #212933;
                    border: #212933;
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
                    background: #212933;
                    border: #212933;
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
                    background: #212933;
                    border: #212933;
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
                    background: #212933;
                    border: #212933;
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
                    background: #212933;
                    border: #212933;
                }

                .z-y-view-button {
                    background: #1b1f25;
                    border: #1b1f25;
                }

                .z-y-view-button :hover {
                    background: #212933;
                    border: #212933;
                }

                .x-z-view-button {
                    background: #1b1f25;
                    border: #1b1f25;
                }

                .x-z-view-button :hover {
                    background: #212933;
                    border: #212933;
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
                    background: #212933;
                    border: #212933;
                }

                .isometric-view-button-icon {
                    width: 2.1rem;
                    height: 2.1rem;
                }

                .title-bars {
                    display: flex;
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }
            </style>
            <div class="wrapper">
                <nav>
                    <ul class="title-bars">
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
                                    <path d="M15.8807 10.0041L15.8815 24.9757C15.8815 24.9846 15.8923 24.9891 15.8986 
                                        24.9828L24.8062 16.0029C24.8081 16.0011 24.8091 15.9985 24.8091 15.9959L24.8084 
                                        1.02428C24.8084 1.01535 24.7976 1.0109 24.7913 1.01724L15.8836 9.99708C15.8818 
                                        9.99895 15.8807 10.0015 15.8807 10.0041Z" stroke="#d9d9d9"
                                    />
                                    <path d="M1 10.01V24.99C1 24.9955 1.00448 25 1.01 25H15.8707C15.8762 25 15.8807 
                                        24.9955 15.8807 24.99L15.8807 10.01C15.8807 10.0045 15.8762 10 15.8707 
                                        10H1.01C1.00448 10 1 10.0045 1 10.01Z" stroke="#d9d9d9"
                                    />
                                    <path d="M9.91153 1.00296L1.01688 9.98296C1.01062 9.98928 1.01509 10 1.02398 
                                        10H15.8905C15.8932 10 15.8957 9.99893 15.8976 9.99704L24.7923 1.01704C24.7985
                                        1.01072 24.794 1 24.7852 1H9.91863C9.91597 1 9.91341 1.00107 9.91153 1.00296Z" 
                                        stroke="#d9d9d9"
                                    />
                                    <line x1="1" y1="20.5" x2="15.8807" y2="20.5" stroke="#d9d9d9"/>
                                    <path d="M3.97614 6.5H18.8569" stroke="#d9d9d9"/>
                                    <line x1="6.95229" y1="3.5" x2="21.833" y2="3.5" stroke="#d9d9d9"/>
                                    <line x1="1" y1="15.5" x2="15.8807" y2="15.5" stroke="#d9d9d9"/>
                                    <line y1="-0.5" x2="12.9484" y2="-0.5" transform="matrix(0.704278 -0.709924 
                                        0.704278 0.709924 5.96024 10.4853)" stroke="#d9d9d9"
                                    />
                                    <line y1="-0.5" x2="12.9484" y2="-0.5" transform="matrix(0.704278 
                                        -0.709924 0.704278 0.709924 10.9205 10.4853)" stroke="#d9d9d9"
                                    />
                                    <line y1="-0.5" x2="12.9484" y2="-0.5" transform="matrix(0.704278 -0.709924 
                                        0.704278 0.709924 15.8807 21.1924)" stroke="#d9d9d9"
                                    />
                                    <line y1="-0.5" x2="12.9484" y2="-0.5" transform="matrix(0.704278 
                                        -0.709924 0.704278 0.709924 15.8807 16.1924)" stroke="#d9d9d9"
                                    />
                                    <line x1="5.46819" y1="10" x2="5.46819" y2="25" stroke="#d9d9d9"/>
                                    <line x1="10.4284" y1="10" x2="10.4284" y2="25" stroke="#d9d9d9"/>
                                    <line x1="22.333" y1="3" x2="22.333" y2="18.5" stroke="#d9d9d9"/>
                                    <line x1="19.3569" y1="6" x2="19.3569" y2="21.5" stroke="#d9d9d9"/>
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
    }

    set actionId(value) {
        this.props.actionId = value;
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
        if (this.props.actionId > 1) {
            this.dispatchEvent(new CustomEvent("decrease action id", {
                bubbles: true,
                composed: true,
            }));
            const message = {"undo": { "actionId": this.props.actionId } };
            this.dispatchEvent(new CustomEvent("client message", {
                bubbles: true,
                composed: true,
                detail: {
                    message: message,
                },
            }));
        }
    }

    redo() {
        const message = {"redo": { "actionId": this.props.actionId } };
        this.dispatchEvent(new CustomEvent("client message", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));   
    }
}

export default FeaAppTitleBar;