class FeaElement extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: {
                element: "Element",
            },
            buttonFullNames: {
                element: "element",
            }
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: flex;
                }

                .wrapper {
                    background-color: #eee;
                    display: flex;
                    align-items: center;
                    box-sizing: content-box;
                    flex-direction: column;
                    border-right: 1px solid #9a9a9a;
                    border-left: 1px solid #9a9a9a;
                }
            </style>
            <div class=wrapper>
                <hiding-content-button 
                    class=element
                    name=${this.state.buttonNames.element}
                    full-name=${this.state.buttonFullNames.element}
                    content-position=relative
                    content-direction=row
                    button-width=12rem
                    button-font-size=100%
                >
                </hiding-content-button>
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

export default FeaElement;