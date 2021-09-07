class FeaLoadMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            menuNames: {
                "load-add-load-menu-concentrated-load": "fea-load-add-concentrated-load-menu",
                "load-update-load-menu-concentrated-load": "fea-load-update-concentrated-load-menu",
                "load-delete-load-menu-concentrated-load": "fea-load-delete-concentrated-load-menu",
                "load-add-load-menu-distributed-load": "fea-load-add-distributed-load-menu",
                "load-update-load-menu-distributed-load": "fea-load-update-distributed-load-menu",
                "load-delete-load-menu-distributed-load": "fea-load-delete-distributed-load-menu",
            },

            loadTypes: ["Concentrated load", "Distributed load"],
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
                    padding: 1rem;
                }

                .load-type-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0;
                    margin-left: 0;
                    margin-right: 0;
                }

                .load-menu-caption {
                    margin: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0.3rem;
                    padding-left: 0rem;
                    padding-right: 0rem;
                    color: #D9D9D9;
                    border-bottom: 0.1rem solid #4a5060;
                    font-size: 85%;
                }

                .load-type-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 3rem;
                }

                .load-type-select-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .load-type {
                    width: 8rem;
                    margin-top: 0.5rem;
                    background-color: #3b4453;
                    border: #4a5060;
                    border-bottom: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                    -webkit-appearance: none;
                    -moz-appearance: none;
                    background: url('data:image/svg+xml,<svg width="4" height="4" viewBox="0 0 4 4" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M1 1L2 2L3 1" stroke="rgb(112, 112, 114)" stroke-width="0.5"/></svg>') right / contain no-repeat;
                }

                .load-type option {
                    background-color: #484f60;
                }

                .load-type:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }



            </style>

            <div class=wrapper>
                <p class="load-menu-caption">Load</p>

                <div class="load-type-field-content">
                    <p class="load-type-caption">Type</p>
                    <div class="load-type-select-content">
                        <select class="load-type"></select>
                    </div>
                </div>
                <fea-load-menu-buttons></fea-load-menu-buttons>
                <slot></slot>
            </div>
        `;

        this.addEventListener("activate-menu", (event) => this.activateMenu(event));

        this.addEventListener("deactivate-menu", (event) => this.deactivateMenu(event));

        this.shadowRoot.querySelector(".load-type").addEventListener("change", () => this.defineLoadTypeForLoadMenuButtons());
    }

    set selectConcentratedLoadInClient(concentratedLoadpointNumber) {
        const loadTypeSelect = this.shadowRoot.querySelector(".load-type");
                const loadTypeOptions = loadTypeSelect.options;
                for (let option, i = 0; option = loadTypeOptions[i]; i++) {
                    if (option.value == "Concentrated load") {
                        loadTypeSelect.selectedIndex = i;
                        break;
                    }
                }
        this.defineLoadTypeForLoadMenuButtons();
        this.shadowRoot.querySelector("fea-load-menu-buttons").activateButton = "load-update-load-menu-button";
        this.querySelector("fea-load-update-concentrated-load-menu").selectConcentratedLoadInClient = 
            concentratedLoadpointNumber;
    }

    connectedCallback() {
        this.defineLoadType();
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

    defineLoadType() {
        const loadTypeSelect = this.shadowRoot.querySelector(".load-type");
        for (let i = loadTypeSelect.length - 1; i >= 0; i--) {
            loadTypeSelect.options[i] = null;
        }
        for (let i = 0; i < this.state.loadTypes.length; i++) {
            let updateOption = document.createElement("option");
            updateOption.value = this.state.loadTypes[i];
            updateOption.innerHTML = this.state.loadTypes[i];
            loadTypeSelect.appendChild(updateOption);
        }
        this.defineLoadTypeForLoadMenuButtons();
    }

    defineLoadTypeForLoadMenuButtons() {
        const loadTypeSelect = this.shadowRoot.querySelector(".load-type");
        switch (loadTypeSelect.value) {
            case "Concentrated load":
                this.shadowRoot.querySelector("fea-load-menu-buttons").setAttribute("load-type", "concentrated-load");
                break;
            case "Distributed load":
                this.shadowRoot.querySelector("fea-load-menu-buttons").setAttribute("load-type", "distributed-load");
                break;
        }
    }

    activateMenu(event) {
        const menuName = event.detail.menuName;
        const menu = document.createElement(this.state.menuNames[menuName]);
        this.append(menu);
        event.stopPropagation();
    }

    deactivateMenu(event) {
        const menuName = event.detail.menuName;
        this.querySelector(this.state.menuNames[menuName]).remove();
        event.stopPropagation();
    }
}

export default FeaLoadMenu;
