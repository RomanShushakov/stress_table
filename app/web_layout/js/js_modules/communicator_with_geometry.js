import { initializeGeometry } from "../wasm_js_interface_modules/geometry_initialization.js";

class CommunicatorWithGeometry {
    constructor() {
        this.state = {
            geometry: null,
        };

        this.initGeometry();
    }

    async initGeometry() {
        this.state.geometry = await initializeGeometry();
    }

    set addPointToGeometry(data) {
        try {
            this.state.geometry.add_point(data[0], data[1], data[2], data[3], data[4]);
        } catch (error) {
            throw error;
        }
    }

    set updatePointToGeometry(data) {
        try {
            this.state.geometry.update_point(data[0], data[1], data[2], data[3], data[4]);
        } catch (error) {
            throw error;
        }
    }

    set addWholeGeometryToPreprocessor(_empty) {
        this.state.geometry.add_whole_geometry_to_preprocessor();
    }
}

export const communicatorWithGeometry = new CommunicatorWithGeometry();
