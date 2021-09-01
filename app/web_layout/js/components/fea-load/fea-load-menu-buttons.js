class FeaLoadMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "load-add-load-menu-button",
                "load-update-load-menu-button",
                "load-delete-load-menu-button",
            ],

            menuNames: {
                "load-add-load-menu-button": "load-add-load-menu",
                "load-update-load-menu-button": "load-update-load-menu",
                "load-delete-load-menu-button": "load-delete-load-menu",
            },

            captions: {
                "load-add-load-menu-button": "Add",
                "load-update-load-menu-button": "Update",
                "load-delete-load-menu-button": "Delete",
            },

            loadType: null,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: flex;
                }
                .wrapper {
                    display: flex;
                    flex-direction: column;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .load-menu-buttons-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    border-bottom: 0.1rem solid #4a5060;
                    align-items: center;
                }

                .load-menu-buttons-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 4rem;
                }

                .load-add-load-menu-button {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 2.5rem;
                    margin-right: 0rem;
                }

                .load-update-load-menu-button {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

                .load-delete-load-menu-button {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

            </style>
            <div class=wrapper>
                <div class="load-menu-buttons-content">
                    <p class="load-menu-buttons-caption">Add</p>
                    <add-button class="load-add-load-menu-button" name="load"></add-button>
                    <update-button class="load-update-load-menu-button" name="load"></update-button>
                    <delete-button class="load-delete-load-menu-button" name="load"></delete-button>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".load-add-load-menu-button").addEventListener("click", 
            () => this.activateInner("load-add-load-menu-button"));

        this.shadowRoot.querySelector(".load-update-load-menu-button").addEventListener("click", 
            () => this.activateInner("load-update-load-menu-button"));

        this.shadowRoot.querySelector(".load-delete-load-menu-button").addEventListener("click", 
            () => this.activateInner("load-delete-load-menu-button"));
    }

    connectedCallback() {
    }

    disconnectedCallback() {
    }

    static get observedAttributes() {
        return ["load-type"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (this.state.loadType === null) {
            this.state.loadType = this.getAttribute("load-type");
            this.activateOuter("load-add-load-menu-button");
        } else {
            this.activateOuter("load-add-load-menu-button");
            this.state.loadType = this.getAttribute("load-type");
        }
    }

    adoptedCallback() {
    }

    activateInner(buttonName) {
        for (let i = 0; i < this.state.buttonNames.length; i ++) {
            if (this.state.buttonNames[i] !== buttonName && 
                this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`)
                    .classList.contains("active") === true) {
                this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`).classList.remove("active");
                this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`).setAttribute("is-active", false);
                const menuName = this.state.menuNames[this.state.buttonNames[i]];
                this.dispatchEvent(new CustomEvent("deactivate-menu", {
                    bubbles: true,
                    composed: true,
                    detail: {
                        "menuName": `${menuName}-${this.state.loadType}`,
                    }
                }));
            }
        } 
        const currentButton = this.shadowRoot.querySelector(`.${buttonName}`);
        if (currentButton.classList.contains("active") === false) {
            currentButton.classList.add("active");
            currentButton.setAttribute("is-active", true);
            const caption = this.state.captions[buttonName];
            this.shadowRoot.querySelector(".load-menu-buttons-caption").innerHTML = caption;
            const menuName = this.state.menuNames[buttonName];
            this.dispatchEvent(new CustomEvent("activate-menu", {
                bubbles: true,
                composed: true,
                detail: {
                    "menuName": `${menuName}-${this.getAttribute("load-type")}`,
                }
            }));
        }
    }

    activateOuter(buttonName) {
        for (let i = 0; i < this.state.buttonNames.length; i ++) {
            if (this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`)
                    .classList.contains("active") === true) {
                this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`).classList.remove("active");
                this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`).setAttribute("is-active", false);
                const menuName = this.state.menuNames[this.state.buttonNames[i]];
                this.dispatchEvent(new CustomEvent("deactivate-menu", {
                    bubbles: true,
                    composed: true,
                    detail: {
                        "menuName": `${menuName}-${this.state.loadType}`,
                    }
                }));
            }
        } 
        const currentButton = this.shadowRoot.querySelector(`.${buttonName}`);
        if (currentButton.classList.contains("active") === false) {
            currentButton.classList.add("active");
            currentButton.setAttribute("is-active", true);
            const caption = this.state.captions[buttonName];
            this.shadowRoot.querySelector(".load-menu-buttons-caption").innerHTML = caption;
            const menuName = this.state.menuNames[buttonName];
            this.dispatchEvent(new CustomEvent("activate-menu", {
                bubbles: true,
                composed: true,
                detail: {
                    "menuName": `${menuName}-${this.getAttribute("load-type")}`,
                }
            }));
        }
    }
}

export default FeaLoadMenuButtons;
