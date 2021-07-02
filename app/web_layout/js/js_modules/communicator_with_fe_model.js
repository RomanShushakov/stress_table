import { initializeFEModel } from "../wasm_modules_initialization/fe_model_initialization.js";

class CommunicatorWithFEModel {
    constructor() {
        this.state = {
            feModel: null,     // wasm module "fe_model";
            objectInfo: null,   // String;
            lineNumbers: [],    // array of: [u32...];
        };

        this.initFEModel();
    }

    async initFEModel() {
        this.state.feModel = await initializeFEModel();
        window.dispatchEvent(new CustomEvent("feModelLoaded", {
            bubbles: true,
            composed: true,
        }));
    }

    set addPointToGeometry(pointData) {
        try {
            this.state.feModel.add_point(pointData.action_id, pointData.number, pointData.x, pointData.y, pointData.z,
                pointData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updatePointInGeometry(pointData) {
        try {
            this.state.feModel.update_point(pointData.action_id, pointData.number, pointData.x, pointData.y, pointData.z,
                pointData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }


    set deletePointFromGeometry(pointData) {
        try {
            const deletedLineNumbers = this.state.feModel.delete_point(pointData.action_id,
                pointData.number, pointData.is_action_id_should_be_increased);
            this.state.lineNumbers = deletedLineNumbers;
        } catch (error) {
            throw error;
        }
    }

    set restorePointInGeometry(pointData) {
        try {
            const restoredLineNumbers = this.state.feModel.restore_point(pointData.action_id,
                pointData.number, pointData.is_action_id_should_be_increased);
            this.state.lineNumbers = restoredLineNumbers;
        } catch (error) {
            throw error;
        }
    }

    set addLineToGeometry(lineData) {
        try {
            this.state.feModel.add_line(lineData.action_id, lineData.number, lineData.start_point_number, lineData.end_point_number,
                lineData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updateLineInGeometry(lineData) {
        try {
            this.state.feModel.update_line(lineData.action_id, lineData.number, lineData.start_point_number, lineData.end_point_number,
                lineData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set deleteLineFromGeometry(lineData) {
        try {
            const deletedLineNumbers = this.state.feModel.delete_line(lineData.action_id,
                lineData.number, lineData.is_action_id_should_be_increased);
            this.state.lineNumbers = deletedLineNumbers;
        } catch (error) {
            throw error;
        }
    }

    set restoreLineInGeometry(lineData) {
        try {
            const restoredLineNumbers = this.state.feModel.restore_line(lineData.action_id,
                lineData.number, lineData.is_action_id_should_be_increased);
            this.state.lineNumbers = restoredLineNumbers;
        } catch (error) {
            throw error;
        }
    }

    set showPointInfo(number) {
        try {
            const pointInfo = this.state.feModel.show_point_info(number);
            this.state.objectInfo = pointInfo;
        } catch (error) {
            throw error;
        }
    }

    set showLineInfo(number) {
        try {
            const lineInfo = this.state.feModel.show_line_info(number);
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

    get lineNumbers() {
        const lineNumbers = this.state.lineNumbers;
        this.state.lineNumbers = [];
        return lineNumbers;
    }

    set clearGeometryModuleByActionId(action_id) {
        this.state.feModel.clear_geometry_module_by_action_id(action_id);
    }

    set extractPoints(handler) {
        this.state.feModel.extract_points(handler);
    }

    set extractLines(handler) {
        this.state.feModel.extract_lines(handler);
    }
}

export const communicatorWithFEModel = new CommunicatorWithFEModel();
