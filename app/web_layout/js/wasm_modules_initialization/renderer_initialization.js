import init, { Renderer } from "../../wasm/renderer/renderer.js";


export async function initializeRenderer(canvasTextElement, canvasGLElement) {
    await init();
    const renderer = Renderer.create(canvasTextElement, canvasGLElement);
    return renderer;    
}
