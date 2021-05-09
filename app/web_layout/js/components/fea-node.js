class FeaNode extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: {
                node: "Node",
            },
            buttonFullNames: {
                node: "node",
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
                    class=node
                    name=${this.state.buttonNames.node}
                    full-name=${this.state.buttonFullNames.node}
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

export default FeaNode;