class DeleteButton extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {};

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                }

                .delete-button {
                    background: #3b4453;
                    border: #3b4453;
                    padding: 0rem;
                    margin: 0rem;
                }

                .delete-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    border-bottom: 0.15rem solid #3b4453;
                }

                .delete-button-icon {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .delete-button:hover .delete-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .delete-button-icon {
                    color: #8bbee4;
                }

                .active .delete-button-icon {
                    color: #72C5FF;
                }

                .active:hover .delete-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .delete-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }
            </style>

            <button class="delete-button">
                <div class="delete-button-icon-content">
                    <svg class="delete-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <title>Delete ${this.getAttribute("name")}</title>
                        <g stroke="currentColor">
                            <path d="M7.00184 9.01157C7.00087 9.0055 7.00557 9 7.01172 
                                9H27.9885C27.9946 9 27.9993 9.00535 27.9985 9.01135L25.0012 
                                30.9914C25.0005 30.9963 24.9963 31 24.9913 31H10.5085C10.5036 
                                31 10.4994 30.9964 10.4987 30.9916L7.00184 9.01157Z"
                            />
                            <rect x="14.5" y="4.5" width="6" height="3" rx="0.5"/>
                            <line x1="5" y1="8" x2="30" y2="8" stroke-width="2"/>
                            <line x1="11.4942" y1="12.924" x2="13.4942" y2="25.924"/>
                            <line x1="23.4942" y1="13.076" x2="21.4942" y2="26.076"/>
                            <line x1="17.5" y1="13" x2="17.5" y2="26"/>
                        </g>
                    </svg>
                </div>
            </button>
        `;
    }

    connectedCallback() {
    }

    disconnectedCallback() {
    }

    static get observedAttributes() {
        return ["is-active"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        this.changeButtonActivity();
    }

    adoptedCallback() {
    }

    changeButtonActivity() {
        const deleteButton = this.shadowRoot.querySelector(".delete-button");
        if (this.getAttribute("is-active") === "true") {
            if (deleteButton.classList.contains("active") === false) {
                deleteButton.classList.add("active");
            }
        }
        if (this.getAttribute("is-active") === "false") {
            if (deleteButton.classList.contains("active") === true) {
                deleteButton.classList.remove("active");
            }
        }
    }
}

export default DeleteButton;
