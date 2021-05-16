class HidingContentButton extends HTMLElement {
    constructor() {
        super();

        this.props = {
            canToggle: false,
        };

        this.state = {};

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: flex;
                }

                .wrapper {
                    background: ${this.getAttribute("content-background")};
                }

                .hiding-content-button {
                    width: ${this.getAttribute("button-width")};
                    font-size: ${this.getAttribute("button-font-size")};
                    background: ${this.getAttribute("button-default-background")};
                    border: ${this.getAttribute("button-default-background")};
                    margin-right: ${this.getAttribute("button-margin-right")};
                }

                .hiding-content-button:hover {
                    background: ${this.getAttribute("button-hover-background")};
                }

                .content {
                    margin: 0rem;
                    background: ${this.getAttribute("content-background")};
                    padding: ${this.getAttribute("content-padding")};
                    flex-direction: ${this.getAttribute("content-direction")};
                    left: ${this.getAttribute("content-left")};
                    top: ${this.getAttribute("content-top")};
                    position: ${this.getAttribute("content-position")};
                }

                .active {
                    background: ${this.getAttribute("button-active-background")};
                }

                .hidden {
                    display: none;
                }
            </style>
            <div class="wrapper">
                <button class="hiding-content-button">
                    <slot name="icon-content"></slot>
                </button>
                <div class="content hidden">
                    <slot name="content"></slot>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".hiding-content-button").addEventListener("click", (event) => this.activate(event));
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

    set deactivate(_bool) {
        const content = this.shadowRoot.querySelector(".content");
        const button = this.shadowRoot.querySelector(".hiding-content-button");
        if (content.classList.contains("hidden") === false) {
            content.classList.add("hidden");
            button.classList.remove("active");
        }
    }

    set disable(b) {
        const content = this.shadowRoot.querySelector(".content");
        const button = this.shadowRoot.querySelector(".hiding-content-button");
        if (b === true) {
            button.disabled = true;
            if (content.classList.contains("hidden") === false) {
                content.classList.add("hidden");
                button.classList.remove("active");
                this.updateContentHeight();
            }
        } else {
            button.disabled = false;
        }
    }

    set close(_empty) {
        const content = this.shadowRoot.querySelector(".content");
        const button = this.shadowRoot.querySelector(".hiding-content-button");
        if (content.classList.contains("hidden") === false) {
            content.classList.add("hidden");
            button.classList.remove("active");
            this.hideSiblings();
            this.updateContentHeight();
        }
    }

    activate() {
        const content = this.shadowRoot.querySelector(".content");
        const button = this.shadowRoot.querySelector(".hiding-content-button");
        if (content.classList.contains("hidden") === true) {
            content.classList.remove("hidden");
            button.classList.add("active");
            this.hideSiblings();
            this.menuOpen();
            this.activateMenu();
        }
        this.updateContentHeight();
    }

    toggle() {
        const content = this.shadowRoot.querySelector(".content");
        const button = this.shadowRoot.querySelector(".hiding-content-button");
        if (content.classList.contains("hidden") === false) {
            content.classList.add("hidden");
            button.classList.remove("active");
        } else {
            content.classList.remove("hidden");
            button.classList.add("active");
            this.hideSiblings();
            this.menuOpen();
        }
        this.updateContentHeight();
    }

    activateMenu() {
        this.dispatchEvent(new CustomEvent("activate menu", {
            bubbles: true,
            composed: true,
            detail: {
                from: `${this.getAttribute("full-name")}`,
            },
        }));
    }

    menuOpen() {
        this.dispatchEvent(new CustomEvent("menu open", {
            bubbles: true,
            composed: true,
            detail: {
                from: `${this.getAttribute("full-name")}`,
            },
        }));
    }

    hideSiblings() {
        this.dispatchEvent(new CustomEvent("hide siblings", {
            bubbles: true,
            composed: true,
            detail: {
                from: `${this.getAttribute("full-name")}`,
            },
        }));
    }

    updateContentHeight() {
        const additionalHeight = this.findAdditionalHeight();
        this.dispatchEvent(new CustomEvent("update content height", {
            bubbles: true,
            composed: true,
            detail: {
                from: this.getAttribute("full-name"),
                height: additionalHeight,
            },
        }));
    }

    findAdditionalHeight() {
        if (this.shadowRoot.querySelector(".content").classList.contains("hidden") === true) {
            return 0;
        } else {
            return this.offsetHeight;
        }
    }
}

export default HidingContentButton;