import init, { Geometry } from "../../wasm/geometry/geometry.js";


export async function initializeGeometry() {
    await init();
    const geometry = Geometry.create();
    return geometry;    
}
