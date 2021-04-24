import init, { Renderer } from "../../wasm/renderer/renderer.js";

export async function initializeRenderer(canvasElement, canvasWidth, canvasHeight) {
    await init();

    const renderer = Renderer.create(canvasElement, canvasWidth, canvasHeight);

    return renderer;    
}
