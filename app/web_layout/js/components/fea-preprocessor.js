// class FeaPreprocessor extends HTMLElement {
//     constructor() {
//         super();

//         this.props = {};

//         this.state = {};

//         this.attachShadow({ mode: "open" });

//         this.shadowRoot.innerHTML = `
//             <style>
//                 :host {
//                     display: block;
//                 }
//             </style>
//             <div>
//                 <p>Hello from fea-preprocessor</p>
//                 <slot></slot>
//             </div>
//         `;
//     }

//     connectedCallback() {
//     }

//     disconnectedCallback() {
//     }
    
//     static get observedAttributes() {
//         return [];
//     }
    
//     attributeChangedCallback(name, oldValue, newValue) {
//     }
    
//     adoptedCallback() {
//     }

// }

// export default FeaPreprocessor;


import FeaPreprocessorMenu from "./fea-preprocessor-menu.js";
import FeaGeometry from "./fea-geometry.js";
import FeaProperties from "./fea-properties.js";
import FeaNode from "./fea-node.js";
import FeaElement from "./fea-element.js";
import FeaDisplacement from "./fea-displacement.js";
import FeaLoad from "./fea-load.js";
import FeaPreprocessorCanvas from "./fea-preprocessor-canvas.js";

customElements.define("fea-preprocessor-menu", FeaPreprocessorMenu);
customElements.define("fea-geometry", FeaGeometry);
customElements.define("fea-properties", FeaProperties);
customElements.define("fea-node", FeaNode);
customElements.define("fea-element", FeaElement);
customElements.define("fea-displacement", FeaDisplacement);
customElements.define("fea-load", FeaLoad);
customElements.define("fea-preprocessor-canvas", FeaPreprocessorCanvas);



import AbstractView from "./abstract-view.js";

export default class extends AbstractView {
    constructor(params) {
        super(params);
        this.setTitle("Settings");
    }

    getHtml() {
        return `
            <fea-preprocessor-menu>
                <fea-geometry></fea-geometry>
                <fea-properties></fea-properties>
                <fea-node></fea-node>
                <fea-element></fea-element>
                <fea-displacement></fea-displacement>
                <fea-load></fea-load>
            </fea-preprocessor-menu>
            <fea-preprocessor-canvas></fea-preprocessor-canvas>
        `;
    }
}
