class HidingContentButton extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {};

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: flex;
                }

                .active {
                    background: #a9a9a9;
                }

                .hiding-content-button {
                    width: ${this.getAttribute("button-width")};
                    font-family: inherit;
                    font-size: ${this.getAttribute("button-font-size")};
                    line-height: 1.15;
                    margin-bottom: 0.25rem;
                    border-radius: 5px;
                    border: 2px solid #737373;
                }

                .hiding-content-button:hover {
                    background: #d2d2d2;
                }

                .content {
                    margin: 0rem;
                    background: #adadad;
                    border: 2px solid #737373;
                    border-radius: 5px;
                    padding: 0.5rem;
                    align-items: center;
                    display: flex;
                    flex-direction: ${this.getAttribute("content-direction")};
                    left: ${this.getAttribute("content-left")};
                    position: ${this.getAttribute("content-position")};
                }

                .hidden {
                    display: none;
                }
            </style>
            <div class="wrapper">
                <button class="hiding-content-button"></button>
                <div class="content hidden">
                    <slot></slot>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".hiding-content-button").addEventListener("click", (event) => this.toggle(event));
    }

    connectedCallback() {
        this.shadowRoot.querySelector(".hiding-content-button").innerHTML = this.getAttribute("name");
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

    toggle(event) {
        const content = this.shadowRoot.querySelector(".content");
        const button = this.shadowRoot.querySelector(".hiding-content-button");
        if (content.classList.contains("hidden") === false) {
            content.classList.add("hidden");
            button.classList.remove("active");
        } else {
            content.classList.remove("hidden");
            button.classList.add("active");
            this.hideSiblings(event);
        }
    }

    hideSiblings(event) {
        this.dispatchEvent(new CustomEvent("hide siblings", {
            bubbles: true,
            composed: true,
            detail: {
                from: `${this.getAttribute("full-name")}`,
            },
        }));

    }

}

export default HidingContentButton;