class FeaAppTitleBar extends HTMLElement {
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
            </style>
            <div>
                <p>Hello from fea-app-title-bar</p>
                <nav>
                    <ul>
                        <li>
                            Save
                        </li>
                        <li>
                            Load
                        </li>
                        <li>
                            Undo
                        </li>
                        <li>
                            Redo
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

}

export default FeaAppTitleBar;