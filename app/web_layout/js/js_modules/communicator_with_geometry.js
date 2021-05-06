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

    set addPointToGeometry(pointData) {
        try {
            this.state.geometry.add_point(pointData.actionId, pointData.number, pointData.x, pointData.y, pointData.z);
        } catch (error) {
            throw error;
        }
    }

    set updatePointInGeometry(pointData) {
        try {
            this.state.geometry.update_point(pointData.actionId, pointData.number, pointData.x, pointData.y, pointData.z);
        } catch (error) {
            throw error;
        }
    }

    set addWholeGeometryToPreprocessor(_empty) {
        this.state.geometry.add_whole_geometry_to_preprocessor();
    }


    set deletePointFromGeometry(pointData) {
        try {
            this.state.geometry.delete_point(pointData.actionId, pointData.number, pointData.x, pointData.y, pointData.z);
        } catch (error) {
            throw error;
        }
    }
}

export const communicatorWithGeometry = new CommunicatorWithGeometry();
