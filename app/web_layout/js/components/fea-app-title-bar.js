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

                .save-button-icon {
                    width: 1.5rem;
                }

                .load-button-icon {
                    width: 1.5rem;
                }

                .undo-button-icon {
                    width: 1.5rem;
                }

                .redo-button-icon {
                    width: 1.5rem;
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
                                <svg class="save-button-icon" viewBox="0 0 100.25 100.25">
                                    <title>Save</title>
                                    <path d="M83.061,27.94l-11-11c-0.281-0.281-0.663-0.439-1.061-0.439H18c-0.828,0-1.5,0.671-1.5,1.5v64c0,0.829,
                                        0.672,1.5,1.5,1.5h64
                                        c0.828,0,1.5-0.671,1.5-1.5v-53C83.5,28.602,83.342,28.221,83.061,27.94z M34.5,19.5h31v21h-31V19.5z 
                                        M71.5,80.5h-43v-26h43V80.5z
                                        M80.5,80.5h-6V53c0-0.829-0.672-1.5-1.5-1.5H27c-0.828,0-1.5,0.671-1.5,1.5v27.5h-6v-61h12V42c0,
                                        0.829,0.672,1.5,1.5,1.5h34
                                        c0.828,0,1.5-0.671,1.5-1.5V19.5h1.879L80.5,29.621V80.5z"/>
                                </svg>
                            </button>
                        </li>
                        <li>
                            <button class="load-button">
                                <svg class="load-button-icon" viewBox="0 0 32 32">
                                    <title>Load</title>
                                    <defs></defs>
                                    <g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd" sketch:type="MSPage">
                                        <g sketch:type="MSArtboardGroup" fill="#000000">
                                            <path d="M16,16 L12.75,12.75 L12,13.5 L16.5,18 L21,13.5 L20.25,12.75 L17,16 L17,5 L16,5 
                                                L16,16 L16,16 Z M18,11 L23.4000244,11 L27.7750244,18 L21,18 L21,20.0020869 
                                                C21,21.1017394 20.1057373,22 19.0026083,22 L13.9973917,22 
                                                C12.8958578,22 12,21.1055038 12,20.0020869 L12,18 L5.22497559,18 L5.22497559,18 
                                                L9.59997559,11 L15,11 L15,10 L9,10 L4,18 L4,18.5 L4,27 L29,27 L29,18.5 L29,18 
                                                L24,10 L18,10 L18,11 L18,11 L18,11 Z M22,19 L28,19 L28,26 L5,26 L5,19 L11,19 L11,20.5 
                                                C11,21.8807119 12.1152735,23 13.4960703,23 L19.5039297,23 C20.8824713,23 22,21.8903379
                                                22,20.5 L22,19 L22,19 L22,19 Z" id="inbox-download" sketch:type="MSShapeGroup">
                                            </path>
                                        </g>
                                    </g>
                                </svg>
                            </button>
                        </li>
                        <li>
                            <button class="undo-button">
                                <svg class="undo-button-icon" viewBox="0 0 21 8">
                                    <title>Undo</title>
                                    <g stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
                                        <g transform="translate(-238.000000, -1534.000000)">
                                            <g transform="translate(100.000000, 1428.000000)">
                                                <g transform="translate(136.000000, 98.000000)">
                                                    <g>
                                                        <polygon id="Path" points="0 0 24 0 24 24 0 24"></polygon>
                                                        <path d="M12.5,8 C9.85,8 7.45,8.99 5.6,10.6 L3.71,8.71 C3.08,8.08 2,8.52 2,9.41 
                                                            L2,15 C2,15.55 2.45,16 3,16 L8.59,16 C9.48,16 9.93,14.92 9.3,14.29 
                                                            L7.39,12.38 C8.78,11.22 10.55,10.5 12.51,10.5 C15.67,10.5 18.4,12.34 19.7,15 
                                                            C19.97,15.56 20.61,15.84 21.2,15.64 C21.91,15.41 22.27,14.6 21.95,13.92 
                                                            C20.23,10.42 16.65,8 12.5,8 Z" fill="#1D1D1D">
                                                        </path>
                                                    </g>
                                                </g>
                                            </g>
                                        </g>
                                    </g>
                                </svg>
                            </button>
                        </li>
                        <li>
                            <button class="redo-button">
                                <svg class="redo-button-icon" viewBox="0 0 21 8">
                                    <title>Redo</title>
                                    <g stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
                                        <g transform="translate(-713.000000, -1534.000000)">
                                            <g transform="translate(100.000000, 1428.000000)">
                                                <g transform="translate(612.000000, 98.000000)">
                                                    <g>
                                                        <polygon id="Path" points="0 0 24 0 24 24 0 24"></polygon>
                                                        <path d="M18.4,10.6 C16.55,8.99 14.15,8 11.5,8 C7.34,8 3.76,10.42 2.06,13.93 
                                                            C1.74,14.6 2.1,15.4 2.81,15.64 C3.4,15.84 4.04,15.56 4.31,15 C5.61,12.34 8.34,10.5 
                                                            11.5,10.5 C13.45,10.5 15.23,11.22 16.62,12.38 L14.71,14.29 C14.08,14.92 14.52,16 
                                                            15.41,16 L21,16 C21.55,16 22,15.55 22,15 L22,9.41 C22,8.52 20.92,8.07 20.29,8.7 
                                                            L18.4,10.6 Z" fill="#1D1D1D">
                                                        </path>
                                                    </g>
                                                </g>
                                            </g>
                                        </g>
                                    </g>
                                </svg>
                            </button>
                        </li>
                        <li>
                            Show/Hide geometry
                        </li>
                        <li>
                            Show/Hide elements
                        </li>
                        <li>
                            View
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
            const message = JSON.stringify({"undo": { "actionId": this.props.actionId } });
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
        const message = JSON.stringify({"redo": { "actionId": this.props.actionId } });
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