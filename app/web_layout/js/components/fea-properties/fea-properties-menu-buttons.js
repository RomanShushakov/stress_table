class FeaPropertiesMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "properties-add-properties-menu-button",
                "properties-update-properties-menu-button",
                "properties-delete-properties-menu-button",
                "properties-assign-properties-menu-button",
                "properties-beam-section-orientation-menu-button",
            ],

            menuNames: {
                "properties-add-properties-menu-button": "properties-add-properties-menu",
                "properties-update-properties-menu-button": "properties-update-properties-menu",
                "properties-delete-properties-menu-button": "properties-delete-properties-menu",
                "properties-assign-properties-menu-button": "properties-assign-properties-menu",
                "properties-beam-section-orientation-menu-button": "properties-beam-section-orientation-menu",
            },

            captions: {
                "properties-add-properties-menu-button": "Add",
                "properties-update-properties-menu-button": "Update",
                "properties-delete-properties-menu-button": "Delete",
                "properties-assign-properties-menu-button": "Assign",
                "properties-beam-section-orientation-menu-button": "Beam section orientation",
            }
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

                .properties-menu-buttons-caption {
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0.3rem;
                    padding-left: 0rem;
                    padding-right: 0rem;
                    color: #D9D9D9;
                    border-bottom: 0.1rem solid #4a5060;
                    font-size: 85%;
                    width: 12rem;
                }

                .properties-menu-buttons-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    border-bottom: 0.1rem solid #4a5060;
                    align-items: center;
                }

                .properties-add-properties-menu-button {
                    padding: 0rem;
                    margin: 0rem;
                }

                .properties-update-properties-menu-button {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

                .properties-delete-properties-menu-button {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

                .properties-assign-properties-menu-button {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }

                .properties-beam-section-orientation-menu-button {
                    padding: 0rem;
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0.5rem;
                    margin-right: 0rem;
                }
            </style>

            <div class=wrapper>
                <div class="properties-menu-buttons-content">
                    <add-button class="properties-add-properties-menu-button" name="properties"></add-button>
                    <update-button class="properties-update-properties-menu-button" name="properties"></update-button>
                    <delete-button class="properties-delete-properties-menu-button" name="properties"></delete-button>
                    <assign-button class="properties-assign-properties-menu-button" name="properties"></assign-button>
                    <beam-section-orientation-button class="properties-beam-section-orientation-menu-button">
                    </beam-section-orientation-button>
                </div>
                <p class="properties-menu-buttons-caption">Add</p>

            </div>
        `;

        this.shadowRoot.querySelector(".properties-add-properties-menu-button").addEventListener("click", 
            () => this.activate("properties-add-properties-menu-button"));

        this.shadowRoot.querySelector(".properties-update-properties-menu-button").addEventListener("click", 
            () => this.activate("properties-update-properties-menu-button"));

        this.shadowRoot.querySelector(".properties-delete-properties-menu-button").addEventListener("click", 
            () => this.activate("properties-delete-properties-menu-button"));

        this.shadowRoot.querySelector(".properties-assign-properties-menu-button").addEventListener("click", 
            () => this.activate("properties-assign-properties-menu-button"));

        this.shadowRoot.querySelector(".properties-beam-section-orientation-menu-button").addEventListener("click", 
            () => this.activate("properties-beam-section-orientation-menu-button"));
    }

    connectedCallback() {
        this.activate("properties-add-properties-menu-button");
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

    activate(buttonName) {
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
                        "menuName": menuName,
                    }
                }));
            }
        } 
        const currentButton = this.shadowRoot.querySelector(`.${buttonName}`);
        if (currentButton.classList.contains("active") === false) {
            currentButton.classList.add("active");
            currentButton.setAttribute("is-active", true);
            const caption = this.state.captions[buttonName];
            this.shadowRoot.querySelector(".properties-menu-buttons-caption").innerHTML = caption;
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

export default FeaPropertiesMenuButtons;
