import { communicatorWithFEModel } from "../../../../../js/js_modules/communicator_with_fe_model.js";

export function addPointToGeometry(action_id, number, x, y, z, is_action_id_should_be_increased) 
{
    const pointData = { "action_id": action_id, "number": number, "x": x, "y": y, "z": z, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.addPointToGeometry = pointData;
    } catch (error) {
        throw error;
    }
}

export function updatePointInGeometry(action_id, number, x, y, z, is_action_id_should_be_increased) 
{
    const pointData = { "action_id": action_id, "number": number, "x": x, "y": y, "z": z,
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.updatePointInGeometry = pointData;
    } catch (error) {
        throw error;
    }
}

export function deletePointFromGeometry(action_id, number, is_action_id_should_be_increased) 
{
    const pointData = { "action_id": action_id, "number": number, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.deletePointFromGeometry = pointData;
        const deletedLineNumbers = communicatorWithFEModel.lineNumbers;
        return deletedLineNumbers;
    } catch (error) {
        throw error;
    }
}

export function restorePointInGeometry(action_id, number, is_action_id_should_be_increased) 
{
    const pointData = { "action_id": action_id, "number": number, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.restorePointInGeometry = pointData;
        const restoredLineNumbers = communicatorWithFEModel.lineNumbers;
        return restoredLineNumbers;
    } catch (error) {
        throw error;
    }
}

export function addLineToGeometry(action_id, number, start_point_number, end_point_number, is_action_id_should_be_increased) 
{
    const lineData = { 
        "action_id": action_id, "number": number,
        "start_point_number": start_point_number, "end_point_number": end_point_number,
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.addLineToGeometry = lineData;
    } catch (error) {
        throw error;
    }
}

export function updateLineInGeometry(action_id, number, start_point_number, end_point_number, is_action_id_should_be_increased) 
{
    const lineData = { 
        "action_id": action_id, "number": number,
        "start_point_number": start_point_number, "end_point_number": end_point_number,
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.updateLineInGeometry = lineData;
    } catch (error) {
        throw error;
    }
}

export function deleteLineFromGeometry(action_id, number, is_action_id_should_be_increased) 
{
    const lineData = { "action_id": action_id, "number": number, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.deleteLineFromGeometry = lineData;
        const deletedLineNumbers = communicatorWithFEModel.lineNumbers;
        return deletedLineNumbers;
    } catch (error) {
        throw error;
    }
}

export function restoreLineInGeometry(action_id, number, is_action_id_should_be_increased) 
{
    const lineData = { "action_id": action_id, "number": number, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.restoreLineInGeometry = lineData;
        const restoredLineNumbers = communicatorWithFEModel.lineNumbers;
        return restoredLineNumbers;
    } catch (error) {
        throw error;
    }
}

export function showPointInfo(number) 
{
    try {
        communicatorWithFEModel.showPointInfo = number;
        const pointInfo = communicatorWithFEModel.objectInfo;
        return pointInfo;
    } catch (error) {
        throw error;
    }
}

export function showLineInfoFromGeometry(number) 
{
    try {
        communicatorWithFEModel.showLineInfo = number;
        const lineInfo = communicatorWithFEModel.objectInfo;
        return lineInfo;
    } catch (error) {
        throw error;
    }
}

export function clearGeometryModuleByActionId(action_id) {
    communicatorWithFEModel.clearGeometryModuleByActionId = action_id;
}

export function extractPoints(handler) 
{
    communicatorWithFEModel.extractPoints = handler;
}

export function extractLines(handler) 
{
    communicatorWithFEModel.extractLines = handler;
}
