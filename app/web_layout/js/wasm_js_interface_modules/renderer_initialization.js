import init, { Renderer } from "../../wasm/renderer/renderer.js";


export async function initializeRenderer(canvasElement) {
    await init();
    const renderer = Renderer.create(canvasElement);
    return renderer;    
}
