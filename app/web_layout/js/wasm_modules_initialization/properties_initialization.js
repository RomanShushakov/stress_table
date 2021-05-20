import init, { Properties } from "../../wasm/properties/properties.js";


export async function initializeProperties() {
    await init();
    const properties = Properties.create();
    return properties;    
}
