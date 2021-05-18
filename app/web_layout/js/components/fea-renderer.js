import { initializeRenderer } from "../wasm_modules_initialization/renderer_initialization.js";

import { PointObjectType, LineObjectType } from "../../wasm/renderer/renderer.js";


class FeaRenderer extends HTMLElement {
    constructor() {
        super();

        this.props = {
            canvasWidth: null,
            canvasHeight: null,
        };

        this.state = {
            canvasText: null,
            canvasGL: null,
            renderer: null,
            animationId: null,
            renderLoop: null,
            isPaused: true,
            isRotate: false,
            isPan: false,
            isShiftPressed: false,
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
                    border-left: 0.1rem solid #5c687a;
                    border-right: 0.1rem solid #5c687a;
                    border-bottom: 0.1rem solid #5c687a;
                }
                
                .renderer-container {
                    position: relative;
                    border-bottom: 0.1rem solid #5c687a;
                }
                
                .renderer-canvas-text {
                    background-color: transparent;
                    position: absolute;
                    left: 0px;
                    top: 0px;
                    z-index: 10;
                }
                
                .renderer-canvas-gl {
                    vertical-align: top;
                }

                .object-info-field {
                    margin-top: 0.2rem;
                    margin-bottom: 0.2rem;
                    margin-left: 0.2rem;
                    color: #6c6c6d;
                }
            </style>
            <div class="wrapper">
                <div class="renderer-container">
                    <canvas class="renderer-canvas-text"></canvas>
                    <canvas class="renderer-canvas-gl"></canvas>
                </div>
                <div class="object-info">
                    <p class="object-info-field">Object:</p>
                </div>
            </div>
        `;

        window.addEventListener("keydown", (event) => this.onKeyDown(event));
        window.addEventListener("keyup", () => this.onKeyUp());
        this.shadowRoot.querySelector(".renderer-canvas-text").addEventListener("mousemove", (event) => this.onMouseMove(event));
        this.shadowRoot.querySelector(".renderer-canvas-text").addEventListener("mouseleave", () => this.onMouseLeave());
        this.shadowRoot.querySelector(".renderer-canvas-text").addEventListener("mousedown", () => this.onMouseDown());
        this.shadowRoot.querySelector(".renderer-canvas-text").addEventListener("mouseup", () => this.onMouseUp());
        this.shadowRoot.querySelector(".renderer-canvas-text").addEventListener("wheel", (event) => this.onMouseWheel(event));
        this.shadowRoot.querySelector(".renderer-canvas-text").addEventListener("click", () => this.onMouseClick());
    }

    set addPointToRenderer(point) {
        this.state.renderer.add_point_object(point.number, point.x, point.y, point.z, PointObjectType.Point);
        if (this.state.isPaused === true)
        {
            this.state.renderer.tick();
        }
    }

    set updatePointInRenderer(point) {
        this.state.renderer.update_point_object(point.number, point.x, point.y, point.z, PointObjectType.Point);
        if (this.state.isPaused === true)
        {
            this.state.renderer.tick();
        }
    }

    set deletePointFromRenderer(point) {
        this.state.renderer.delete_point_object(point.number, PointObjectType.Point);
        if (this.state.isPaused === true)
        {
            this.state.renderer.tick();
        }
    }

    set addLineToRenderer(line) {
        this.state.renderer.add_normalized_line_object(line.number, line.startPointNumber, line.endPointNumber, LineObjectType.Line);
        if (this.state.isPaused === true)
        {
            this.state.renderer.tick();
        }
    }

    set updateLineInRenderer(line) {
        this.state.renderer.update_normalized_line_object(line.number, line.startPointNumber, line.endPointNumber, LineObjectType.Line);
        if (this.state.isPaused === true)
        {
            this.state.renderer.tick();
        }
    }

    set deleteLineFromRenderer(line) {
        this.state.renderer.delete_normalized_line_object(line.number, LineObjectType.Line);
        if (this.state.isPaused === true)
        {
            this.state.renderer.tick();
        }
    }

    set objectInfo(objectInfo) {
        this.shadowRoot.querySelector(".object-info-field").innerHTML = `Object: ${objectInfo}`;
    }

    set canvasSize(size) {
        this.props.canvasWidth = size.width;
        this.props.canvasHeight = size.height;
        this.updateCanvasSize();
    }

    async connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.state.canvasText = this.shadowRoot.querySelector(".renderer-canvas-text");
        this.state.canvasGL = this.shadowRoot.querySelector(".renderer-canvas-gl");
        this.props.canvasWidth = window.innerWidth;
        this.props.canvasHeight = window.innerHeight;
        this.state.canvasText.width = this.props.canvasWidth;
        this.state.canvasText.height = this.props.canvasHeight;
        this.state.canvasGL.width = this.props.canvasWidth;
        this.state.canvasGL.height = this.props.canvasHeight;
        this.state.renderer = await initializeRenderer(this.state.canvasText, this.state.canvasGL);
        this.state.renderLoop = () => {
            this.state.renderer.tick();
            this.state.animationId = requestAnimationFrame(this.state.renderLoop);
        };
        this.updateCanvasSize();
    }

    static get observedAttributes() {
    }
    
    attributeChangedCallback(name, oldValue, newValue) {
    }


    play() {
        this.state.renderLoop();
    }


    pause() {
        cancelAnimationFrame(this.state.animationId);
        this.state.animationId = null;
    }


    updateCanvasSize() {
        if (this.state.renderer !== null) {
            this.state.renderer.update_canvas_size(this.props.canvasWidth, this.props.canvasHeight);
            this.state.renderer.tick();
        }
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
            const dTheta = event.movementX * 2.0 * Math.PI / this.props.canvasWidth;
            this.state.renderer.increment_angle_theta(dTheta);
            const dPhi = event.movementY * 2.0 * Math.PI / this.props.canvasHeight;
            this.state.renderer.increment_angle_phi(dPhi);
        }
        if (this.state.isPan === true) {
            const dx = event.movementX / this.props.canvasWidth;
            this.state.renderer.increment_dx(dx);
            const dy =  -event.movementY / this.props.canvasHeight;
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
        const dScale = this.state.renderer.extract_d_scale() + event.deltaY / this.props.canvasHeight;
        if (1.0 + dScale > 50.0) {
            this.state.renderer.change_d_scale(48.95);
        } else if (1.0 + dScale < 0.0) {
            this.state.renderer.change_d_scale(-0.95);
        } else {
            this.state.renderer.change_d_scale(dScale);
        }
    }


    dropSelection() {
        this.shadowRoot.querySelector(".object-info-field").innerHTML = "Object:";
    }


    onMouseClick() {
        this.state.renderer.select_object(() => this.dropSelection());
    }
}

export default FeaRenderer;
