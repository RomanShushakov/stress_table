import init_post, { Renderer } from "../../wasm/renderer/renderer_post.js";


export async function initializeRenderer(canvasTextElement, canvasGLElement) {
    await init_post();
    const renderer = Renderer.create(canvasTextElement, canvasGLElement);
    return renderer;    
}
