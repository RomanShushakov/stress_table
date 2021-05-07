import { initializeGeometry } from "../wasm_modules_initialization/geometry_initialization.js";

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


    set deletePointFromGeometry(pointData) {
        try {
            this.state.geometry.delete_point(pointData.actionId, pointData.number);
        } catch (error) {
            throw error;
        }
    }

    set addLineToGeometry(lineData) {
        try {
            this.state.geometry.add_line(lineData.actionId, lineData.number, lineData.startPointNumber, lineData.endPointNumber);
        } catch (error) {
            throw error;
        }
    }

    set updateLineInGeometry(lineData) {
        try {
            this.state.geometry.update_line(lineData.actionId, lineData.number, lineData.startPointNumber, lineData.endPointNumber);
        } catch (error) {
            throw error;
        }
    }

    set addWholeGeometryToPreprocessor(_empty) {
        this.state.geometry.add_whole_geometry_to_preprocessor();
    }

    set deleteLineFromGeometry(lineData) {
        try {
            this.state.geometry.delete_line(lineData.actionId, lineData.number);
        } catch (error) {
            throw error;
        }
    }
}

export const communicatorWithGeometry = new CommunicatorWithGeometry();
