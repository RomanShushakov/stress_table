import { initializeRenderer } from "../wasm_js_interface_modules/renderer_initialization.js";

// Our app container, app-wide state is managed here

class PreprocessorCanvas extends HTMLElement {
    constructor() {
        super();

        // any external prop can be defined here
        this.props = {};

        // our application level state is defined here, with initial values
        this.state = {
            // amount: 2, // the step amount to increment/decrement
            // total: 0,  // the running total
        };

        // give this component a shadowDOM
        this.attachShadow({ mode: "open" });

        // add shadowDOM and slot in the lightDOM
        // this.shadowRoot.innerHTML = feAnalysisTemplate;

        this.shadowRoot.innerHTML = `
            <style>
            :host {
                display: block;
            }
            </style>
            <p>Hello from preprocessor canvas</p>
            <canvas class="preprocessor_canvas"></canvas>
        `;


        // // add our event listeners for listening to state change requests
        // this.addEventListener('x-increment', (event) => this.onIncrement(event));
        // this.addEventListener('x-decrement', (event) => this.onDecrement(event));
        // this.addEventListener('x-update-amount', (event) => this.onUpdateAmount(event));
    }


    async connectedCallback() {
        let render = await initializeRenderer(this.shadowRoot.querySelector(".preprocessor_canvas"));
        let animationId = null;

        const renderLoop = () => {
            render.tick();
            animationId = requestAnimationFrame(renderLoop);
        };
        renderLoop();
    }

    // connectedCallback() {
    //     // update the shadowDOM with the intitial props/state
    //     this.updateChildren();
    // }

    // onDecrement(event) {
    //     // decrement our total by the current amount
    //     this.state.total = this.state.total - this.state.amount;

    //     // update the shadowDOM with the current props/state
    //     this.updateChildren();
    // }

    // onIncrement(event) {
    //     // increment our total by the current amount
    //     this.state.total = this.state.total + this.state.amount;

    //     // update the shadowDOM with the current props/state
    //     this.updateChildren();
    // }

    // onUpdateAmount(event) {
    //     // update our state to the desired amount
    //     this.state.amount = event.detail.amount;

    //     // update the shadowDOM with the current props/state
    //     this.updateChildren();
    // }

    // updateChildren() {
    //     // set the props of our child components (one-way data binding)
    //     this.querySelector('x-controls').amount = this.state.amount;
    //     this.querySelector('x-counter').total = this.state.total;
    // }
}

export default PreprocessorCanvas;