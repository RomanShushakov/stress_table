import { initializeGeometry } from "../wasm_modules_initialization/geometry_initialization.js";

class CommunicatorWithGeometry {
    constructor() {
        this.state = {
            geometry: null,         // wasm module "actions_router";
            objectInfo: null,       // String;
        };

        this.initGeometry();
    }

    async initGeometry() {
        this.state.geometry = await initializeGeometry();
    }

    set addPointToGeometry(pointData) {
        try {
            this.state.geometry.add_point(pointData.actionId, pointData.number, pointData.x, pointData.y, pointData.z,
                pointData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set updatePointInGeometry(pointData) {
        try {
            this.state.geometry.update_point(pointData.actionId, pointData.number, pointData.x, pointData.y, pointData.z,
                pointData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }


    set deletePointFromGeometry(pointData) {
        try {
            this.state.geometry.delete_point(pointData.actionId, pointData.number, pointData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set undoDeletePointFromGeometry(pointData) {
        try {
            this.state.geometry.undo_delete_point(pointData.actionId, pointData.number, pointData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set addLineToGeometry(lineData) {
        try {
            this.state.geometry.add_line(lineData.actionId, lineData.number, lineData.startPointNumber, lineData.endPointNumber,
                lineData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set updateLineInGeometry(lineData) {
        try {
            this.state.geometry.update_line(lineData.actionId, lineData.number, lineData.startPointNumber, lineData.endPointNumber,
                lineData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set deleteLineFromGeometry(lineData) {
        try {
            this.state.geometry.delete_line(lineData.actionId, lineData.number, lineData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set undoDeleteLineFromGeometry(lineData) {
        try {
            this.state.geometry.undo_delete_line(lineData.actionId, lineData.number, lineData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set showPointInfo(number) {
        try {
            const pointInfo = this.state.geometry.show_point_info(number);
            this.state.objectInfo = pointInfo;
        } catch (error) {
            throw error;
        }
    }

    set showLineInfo(number) {
        try {
            const lineInfo = this.state.geometry.show_line_info(number);
            this.state.objectInfo = lineInfo;
        } catch (error) {
            throw error;
        }
    }

    get objectInfo() {
        const objectInfo = this.state.objectInfo;
        this.state.objectInfo = null;
        return objectInfo;
    }

    set addWholeGeometryToPreprocessor(isActionIdShouldBeIncreased) {
        this.state.geometry.add_whole_geometry_to_preprocessor(isActionIdShouldBeIncreased);
    }
}

export const communicatorWithGeometry = new CommunicatorWithGeometry();
