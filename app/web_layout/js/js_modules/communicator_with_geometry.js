import { initializeGeometry } from "../wasm_modules_initialization/geometry_initialization.js";

class CommunicatorWithGeometry {
    constructor() {
        this.state = {
            geometry: null,         // wasm module "actions_router";
            objectInfo: null,       // String;
            deletedLineNumbers: [], // array of: [u32...];
        };

        this.initGeometry();
    }

    async initGeometry() {
        this.state.geometry = await initializeGeometry();
        window.dispatchEvent(new CustomEvent("geometryLoaded", {
            bubbles: true,
            composed: true,
        }));
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
            const deletedLineNumbers = this.state.geometry.delete_point(pointData.actionId,
                pointData.number, pointData.isActionIdShouldBeIncreased);
            this.deletedLineNumbers = deletedLineNumbers;
        } catch (error) {
            throw error;
        }
    }

    set restorePointInGeometry(pointData) {
        try {
            this.state.geometry.restore_point(pointData.actionId, pointData.number, pointData.isActionIdShouldBeIncreased);
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

    set restoreLineInGeometry(lineData) {
        try {
            this.state.geometry.restore_line(lineData.actionId, lineData.number, lineData.isActionIdShouldBeIncreased);
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

    set clearGeometryModuleByActionId(actionId) {
        this.state.geometry.clear_geometry_module_by_action_id(actionId);
    }

    set extractPoints(handler) {
        this.state.geometry.extract_points(handler);
    }

    set extractLines(handler) {
        this.state.geometry.extract_lines(handler);
    }
}

export const communicatorWithGeometry = new CommunicatorWithGeometry();
