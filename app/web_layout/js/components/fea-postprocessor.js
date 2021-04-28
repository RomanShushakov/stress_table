class FeaPostprocessor extends HTMLElement {
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
                <p>Hello from fea-postprocessor</p>
                <fea-postprocessor-menu>
                    <fea-plot-displacements></fea-plot-displacements>
                    <fea-plot-stresses></fea-plot-stresses>
                    <fea-plot-strains></fea-plot-strains>
                    <fea-plot-forces></fea-plot-forces>
                </fea-postprocessor-menu>
                <fea-postprocessor-canvas></fea-postprocessor-canvas>
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

export default FeaPostprocessor;
