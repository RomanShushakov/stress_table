import { initializeGeometry } from "../wasm_js_interface_modules/geometry_initialization.js";

class CommunicatorWithGeometry
{
    constructor() 
    {
        this.state = {
            geometry: null,
        };

        this.initGeometry();
    }

    async initGeometry() {
        this.state.geometry = await initializeGeometry();
    }

    set addPointToGeometry(data) {
        this.state.geometry.add_point(data[0], data[1], data[2], data[3], data[4]);
    }
}

export const communicatorWithGeometry = new CommunicatorWithGeometry();
