class FeaSymbolsMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            menuNames: {
                "reactions": "fea-symbols-reactions-menu",
                "element-forces": "fea-load-add-distributed-line-load-menu",
            },

            contourResultsTypes: ["Reactions", "Element forces"],
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

                .symbol-results-type-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0;
                    margin-left: 0;
                    margin-right: 0;
                    align-items: baseline;
                }

                .symbol-plot-caption {
                    margin: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0.3rem;
                    padding-left: 0rem;
                    padding-right: 0rem;
                    color: #D9D9D9;
                    border-bottom: 0.1rem solid #4a5060;
                    font-size: 85%;
                }

                .symbol-results-type-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 4rem;
                }

                .symbol-results-type-select-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .symbol-results-type {
                    width: 7rem;
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

                .symbol-results-type option {
                    background-color: #484f60;
                }

                .symbol-results-type:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }
            </style>

            <div class=wrapper>
                <p class="symbol-plot-caption">Symbol plot</p>

                <div class="symbol-results-type-field-content">
                    <p class="symbol-results-type-caption">Result type</p>
                    <div class="symbol-results-type-select-content">
                        <select class="symbol-results-type"></select>
                    </div>
                </div>
                <slot></slot>
            </div>
        `;

        this.shadowRoot.querySelector(".symbol-results-type").addEventListener("change", () => this.defineSymbolResultTypeMenu());
    }

    connectedCallback() {
        this.defineSymbolResultType();
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

    defineSymbolResultType() {
        const contourResultTypeSelect = this.shadowRoot.querySelector(".symbol-results-type");
        for (let i = contourResultTypeSelect.length - 1; i >= 0; i--) {
            contourResultTypeSelect.options[i] = null;
        }
        for (let i = 0; i < this.state.contourResultsTypes.length; i++) {
            let updateOption = document.createElement("option");
            updateOption.value = this.state.contourResultsTypes[i];
            updateOption.innerHTML = this.state.contourResultsTypes[i];
            contourResultTypeSelect.appendChild(updateOption);
        }
        this.defineSymbolResultTypeMenu();
    }

    defineSymbolResultTypeMenu() {
        const loadTypeSelect = this.shadowRoot.querySelector(".symbol-results-type");
        switch (loadTypeSelect.value) {
            case "Reactions":
                this.deactivateMenu("element-forces");
                this.activateMenu("reactions");
                break;
            case "Element forces":
                this.deactivateMenu("reactions");
                this.activateMenu("element-forces");
                break;
        }
    }

    activateMenu(menuName) {
        const menu = document.createElement(this.state.menuNames[menuName]);
        this.append(menu);
    }

    deactivateMenu(menuName) {
        if (this.querySelector(this.state.menuNames[menuName]) != null) {
            this.querySelector(this.state.menuNames[menuName]).remove();
        }
    }
}

export default FeaSymbolsMenu;
