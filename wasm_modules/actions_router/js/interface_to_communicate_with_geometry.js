import { communicatorWithGeometry } from "../../../../../js/js_modules/communicator_with_geometry.js";

export function addPointToGeometry(action_id, number, x, y, z, is_action_id_should_be_increased) 
{
    const pointData = { "action_id": action_id, "number": number, "x": x, "y": y, "z": z, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithGeometry.addPointToGeometry = pointData;
    } catch (error) {
        throw error;
    }
}

export function updatePointInGeometry(action_id, number, x, y, z, is_action_id_should_be_increased) 
{
    const pointData = { "action_id": action_id, "number": number, "x": x, "y": y, "z": z,
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithGeometry.updatePointInGeometry = pointData;
    } catch (error) {
        throw error;
    }
}

export function deletePointFromGeometry(action_id, number, is_action_id_should_be_increased) 
{
    const pointData = { "action_id": action_id, "number": number, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithGeometry.deletePointFromGeometry = pointData;
        const deletedLineNumbers = communicatorWithGeometry.deletedLineNumbers;
        return deletedLineNumbers;
    } catch (error) {
        throw error;
    }
}

export function restorePointInGeometry(action_id, number, is_action_id_should_be_increased) 
{
    const pointData = { "action_id": action_id, "number": number, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithGeometry.restorePointInGeometry = pointData;
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
        communicatorWithGeometry.addLineToGeometry = lineData;
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
        communicatorWithGeometry.updateLineInGeometry = lineData;
    } catch (error) {
        throw error;
    }
}

export function deleteLineFromGeometry(action_id, number, is_action_id_should_be_increased) 
{
    const lineData = { "action_id": action_id, "number": number, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithGeometry.deleteLineFromGeometry = lineData;
    } catch (error) {
        throw error;
    }
}

export function restoreLineInGeometry(action_id, number, is_action_id_should_be_increased) 
{
    const lineData = { "action_id": action_id, "number": number, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithGeometry.restoreLineInGeometry = lineData;
    } catch (error) {
        throw error;
    }
}

export function showPointInfo(number) 
{
    try {
        communicatorWithGeometry.showPointInfo = number;
        const pointInfo = communicatorWithGeometry.objectInfo;
        return pointInfo;
    } catch (error) {
        throw error;
    }
}

export function showLineInfoFromGeometry(number) 
{
    try {
        communicatorWithGeometry.showLineInfo = number;
        const lineInfo = communicatorWithGeometry.objectInfo;
        return lineInfo;
    } catch (error) {
        throw error;
    }
}

export function clearGeometryModuleByActionId(action_id) {
    communicatorWithGeometry.clearGeometryModuleByActionId = action_id;
}

export function extractPoints(handler) 
{
    communicatorWithGeometry.extractPoints = handler;
}

export function extractLines(handler) 
{
    communicatorWithGeometry.extractLines = handler;
}
