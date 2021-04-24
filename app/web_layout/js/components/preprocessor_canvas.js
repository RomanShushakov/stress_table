import { initializeRenderer } from "../wasm_js_interface_modules/renderer_initialization.js";

const coefficient = 0.8;


class PreprocessorCanvas extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            canvasGL: null,
            renderer: null,
            canvasWidth: null,
            canvasHeight: null,
            rotate: false,
            pan: false,
            shiftPressed: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                }
            </style>
            <canvas class="preprocessor_canvas_gl"></canvas>
        `;

        window.addEventListener("resize", () => this.updateCanvasSize());
        window.addEventListener("keydown", (event) => this.onKeyDown(event));
        window.addEventListener("keyup", () => this.onKeyUp());
        this.shadowRoot.querySelector(".preprocessor_canvas_gl").addEventListener("mousemove", (event) => this.onMouseMove(event));
        this.shadowRoot.querySelector(".preprocessor_canvas_gl").addEventListener("mouseleave", () => this.onMouseLeave());
        this.shadowRoot.querySelector(".preprocessor_canvas_gl").addEventListener("mousedown", () => this.onMouseDown());
        this.shadowRoot.querySelector(".preprocessor_canvas_gl").addEventListener("mouseup", () => this.onMouseUp());
        this.shadowRoot.querySelector(".preprocessor_canvas_gl").addEventListener("mousewheel", (event) => this.onMouseWheel(event));
    }


    async connectedCallback() {
        this.state.canvasGL = this.shadowRoot.querySelector(".preprocessor_canvas_gl");
        this.state.canvasWidth = window.innerWidth * coefficient;
        this.state.canvasHeight = window.innerHeight * coefficient;
        this.state.renderer = await initializeRenderer(
            this.state.canvasGL, this.state.canvasWidth, this.state.canvasHeight);
        let animationId = null;

        const renderLoop = () => {
            this.state.renderer.tick();
            animationId = requestAnimationFrame(renderLoop);
        };
        renderLoop();
    }


    updateCanvasSize() {
        this.state.canvasWidth = window.innerWidth * coefficient;
        this.state.canvasHeight = window.innerHeight * coefficient;
        this.state.renderer.update_canvas_size(this.state.canvasWidth, this.state.canvasHeight);
    }


    onKeyDown(event) {
        if (event.shiftKey === true) {
            this.state.shiftPressed = true;
        }
    }


    onKeyUp() {
        this.state.shiftPressed = false;
    }


    onMouseMove(event) {
        const mouseX = event.clientX;
        const mouseY = event.clientY;
        const boundingRect = this.state.canvasGL.getBoundingClientRect();
        const x = mouseX - boundingRect.left;
        const y = boundingRect.bottom - mouseY;
        this.state.renderer.change_cursor_coordinates(x, y);
        if (this.state.rotate === true) {
            const dTheta = event.movementX * 2.0 * Math.PI / this.state.canvasWidth;
            this.state.renderer.increment_angle_theta(dTheta);
            const dPhi = event.movementY * 2.0 * Math.PI / this.state.canvasHeight;
            this.state.renderer.increment_angle_phi(dPhi);
        }
        if (this.state.pan === true) {
            const dx = event.movementX / this.state.canvasWidth;
            this.state.renderer.increment_dx(dx);
            const dy =  -event.movementY / this.state.canvasHeight;
            this.state.renderer.increment_dy(dy);
        }
    }


    onMouseLeave() {
        this.state.rotate = false;
        this.state.pan = false;
    }


    onMouseDown() {
        if (this.state.shiftPressed === true) {
            this.state.pan = true;
        } else {
            this.state.rotate = true;
        }
    }


    onMouseUp() {
        this.state.rotate = false;
        this.state.pan = false;
    }


    onMouseWheel(event) {
        let dScale = this.state.renderer.extract_d_scale() + event.deltaY / this.state.canvasHeight;
        if (1.0 + dScale > 50.0) {
            this.state.renderer.change_d_scale(48.95);
        } else if (1.0 + dScale < 0.0) {
            this.state.renderer.change_d_scale(-0.95);
        } else {
            this.state.renderer.change_d_scale(dScale);
        }
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