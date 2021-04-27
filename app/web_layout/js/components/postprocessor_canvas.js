import { initializeRenderer } from "../wasm_js_interface_modules/renderer_post_initialization.js";

const coefficient = 0.8;


class PostprocessorCanvas extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            canvasText: null,
            canvasGL: null,
            renderer: null,
            animationId: null,
            renderLoop: null,
            isPaused: true,
            canvasWidth: null,
            canvasHeight: null,
            isRotate: false,
            isPan: false,
            isShiftPressed: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                }

                .postprocessor_canvas {
                    display: flex;
                    flex-direction: column;
                    border: 1px solid #000000;
                    margin-top: 0.5rem;
                }
                
                .postprocessor_canvas_container {
                    position: relative;
                }
                
                .postprocessor_canvas_text {
                    background-color: transparent;
                    position: absolute;
                    left: 0px;
                    top: 0px;
                    z-index: 10;
                }
                
                
                .postprocessor_canvas_gl {
                    vertical-align: top;
                }
            </style>
            <div class="postprocessor_canvas_container">
                <canvas class="postprocessor_canvas_text"></canvas>
                <canvas class="postprocessor_canvas_gl"></canvas>
                <button class="post_add_point">Add point</button>
                <button class="post_add_node">Add node</button>
            </div>
        `;

        window.addEventListener("resize", () => this.updateCanvasSize());
        window.addEventListener("keydown", (event) => this.onKeyDown(event));
        window.addEventListener("keyup", () => this.onKeyUp());
        this.shadowRoot.querySelector(".postprocessor_canvas_text").addEventListener("mousemove", (event) => this.onMouseMove(event));
        this.shadowRoot.querySelector(".postprocessor_canvas_text").addEventListener("mouseleave", () => this.onMouseLeave());
        this.shadowRoot.querySelector(".postprocessor_canvas_text").addEventListener("mousedown", () => this.onMouseDown());
        this.shadowRoot.querySelector(".postprocessor_canvas_text").addEventListener("mouseup", () => this.onMouseUp());
        this.shadowRoot.querySelector(".postprocessor_canvas_text").addEventListener("mousewheel", (event) => this.onMouseWheel(event));
        this.shadowRoot.querySelector(".postprocessor_canvas_text").addEventListener("click", () => this.onMouseClick());

        this.shadowRoot.querySelector(".post_add_point").addEventListener("click", () => this.addPoint());
        this.shadowRoot.querySelector(".post_add_node").addEventListener("click", () => this.addNode());
    }


    async connectedCallback() {
        this.state.canvasText = this.shadowRoot.querySelector(".postprocessor_canvas_text");
        this.state.canvasGL = this.shadowRoot.querySelector(".postprocessor_canvas_gl");
        this.state.canvasWidth = window.innerWidth * coefficient;
        this.state.canvasHeight = window.innerHeight * coefficient;
        this.state.canvasText.width = this.state.canvasWidth;
        this.state.canvasText.height = this.state.canvasHeight;
        this.state.canvasGL.width = this.state.canvasWidth;
        this.state.canvasGL.height = this.state.canvasHeight;
        this.state.renderer = await initializeRenderer(this.state.canvasText, this.state.canvasGL);
        this.state.renderLoop = () => {
            this.state.renderer.tick();
            this.state.animationId = requestAnimationFrame(this.state.renderLoop);
        };
    }


    play() {
        this.state.renderLoop();
    }


    pause() {
        cancelAnimationFrame(this.state.animationId);
        this.state.animationId = null;
    }


    updateCanvasSize() {
        this.state.canvasWidth = window.innerWidth * coefficient;
        this.state.canvasHeight = window.innerHeight * coefficient;
        this.state.renderer.update_canvas_size(this.state.canvasWidth, this.state.canvasHeight);
    }


    onKeyDown(event) {
        if (event.shiftKey === true) {
            this.state.isShiftPressed = true;
        }
    }


    onKeyUp() {
        this.state.isShiftPressed = false;
    }


    onMouseMove(event) {
        if (this.state.isPaused === true) {
            this.play();
            this.state.isPaused = false;
        }
        const mouseX = event.clientX;
        const mouseY = event.clientY;
        const boundingRect = this.state.canvasGL.getBoundingClientRect();
        const x = mouseX - boundingRect.left;
        const y = boundingRect.bottom - mouseY;
        this.state.renderer.change_cursor_coordinates(x, y);
        if (this.state.isRotate === true) {
            const dTheta = event.movementX * 2.0 * Math.PI / this.state.canvasWidth;
            this.state.renderer.increment_angle_theta(dTheta);
            const dPhi = event.movementY * 2.0 * Math.PI / this.state.canvasHeight;
            this.state.renderer.increment_angle_phi(dPhi);
        }
        if (this.state.isPan === true) {
            const dx = event.movementX / this.state.canvasWidth;
            this.state.renderer.increment_dx(dx);
            const dy =  -event.movementY / this.state.canvasHeight;
            this.state.renderer.increment_dy(dy);
        }
    }


    onMouseLeave() {
        if (this.state.isPaused === false) {
            this.pause();
            this.state.isPaused = true;
        }
        this.state.isRotate = false;
        this.state.isPan = false;
    }


    onMouseDown() {
        if (this.state.isShiftPressed === true) {
            this.state.isPan = true;
        } else {
            this.state.isRotate = true;
        }
    }


    onMouseUp() {
        this.state.isRotate = false;
        this.state.isPan = false;
    }


    onMouseWheel(event) {
        const dScale = this.state.renderer.extract_d_scale() + event.deltaY / this.state.canvasHeight;
        if (1.0 + dScale > 50.0) {
            this.state.renderer.change_d_scale(48.95);
        } else if (1.0 + dScale < 0.0) {
            this.state.renderer.change_d_scale(-0.95);
        } else {
            this.state.renderer.change_d_scale(dScale);
        }
    }


    onMouseClick() {
        const selectedObject = this.state.renderer.select_object();
        console.log(selectedObject);
    }


    addPoint() {
        this.state.renderer.add_point(1, 1, 2, 3);
        if (this.state.isPaused === true)
        {
            this.state.renderer.tick();
        }
    }


    addNode() {
        this.state.renderer.add_node(1, 1, 2, 3);
        if (this.state.isPaused === true)
        {
            this.state.renderer.tick();
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

export default PostprocessorCanvas;