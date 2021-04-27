// class FeaPostprocessor extends HTMLElement {
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
//                 <p>Hello from fea-postprocessor</p>
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

// export default FeaPostprocessor;



import FeaPostprocessorMenu from "./fea-postprocessor-menu.js";
import FeaPlotDisplacements from "./fea-plot-displacements.js";
import FeaPlotStresses from "./fea-plot-stresses.js";
import FeaPlotStrains from "./fea-plot-strains.js";
import FeaPlotForces from "./fea-plot-forces.js";
import FeaPostprocessorCanvas from "./fea-postprocessor-canvas.js";


customElements.define("fea-postprocessor-menu", FeaPostprocessorMenu);
customElements.define("fea-plot-displacements", FeaPlotDisplacements);
customElements.define("fea-plot-stresses", FeaPlotStresses);
customElements.define("fea-plot-strains", FeaPlotStrains);
customElements.define("fea-plot-forces", FeaPlotForces);
customElements.define("fea-postprocessor-canvas", FeaPostprocessorCanvas);



import AbstractView from "./abstract-view.js";

export default class extends AbstractView {
    constructor(params) {
        super(params);
        this.setTitle("Settings");
    }

    getHtml() {
        return `
            <fea-postprocessor-menu>
                <fea-plot-displacements></fea-plot-displacements>
                <fea-plot-stresses></fea-plot-stresses>
                <fea-plot-strains></fea-plot-strains>
                <fea-plot-forces></fea-plot-forces>
            </fea-postprocessor-menu>
            <fea-postprocessor-canvas></fea-postprocessor-canvas>
      
        `;
    }
}