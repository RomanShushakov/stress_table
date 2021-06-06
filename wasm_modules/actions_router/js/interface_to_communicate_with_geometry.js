// import { communicatorWithGeometry } from "/app/web_layout/js/js_modules/communicator_with_geometry.js";
import { communicatorWithGeometry } from "../../../../../js/js_modules/communicator_with_geometry.js";

export function addPointToGeometry(actionId, number, x, y, z, isActionIdShouldBeIncreased) 
{
    const pointData = { "actionId": actionId, "number": number, "x": x, "y": y, "z": z, 
        "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithGeometry.addPointToGeometry = pointData;
    } catch (error) {
        throw error;
    }
}

export function updatePointInGeometry(actionId, number, x, y, z, isActionIdShouldBeIncreased) 
{
    const pointData = { "actionId": actionId, "number": number, "x": x, "y": y, "z": z,
        "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithGeometry.updatePointInGeometry = pointData;
    } catch (error) {
        throw error;
    }
}

export function deletePointFromGeometry(actionId, number, isActionIdShouldBeIncreased) 
{
    const pointData = { "actionId": actionId, "number": number, "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithGeometry.deletePointFromGeometry = pointData;
    } catch (error) {
        throw error;
    }
}

export function restorePointInGeometry(actionId, number, isActionIdShouldBeIncreased) 
{
    const pointData = { "actionId": actionId, "number": number, "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithGeometry.restorePointInGeometry = pointData;
    } catch (error) {
        throw error;
    }
}

export function addLineToGeometry(actionId, number, startPointNumber, endPointNumber, isActionIdShouldBeIncreased) 
{
    const lineData = { 
        "actionId": actionId, "number": number,
        "startPointNumber": startPointNumber, "endPointNumber": endPointNumber,
        "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithGeometry.addLineToGeometry = lineData;
    } catch (error) {
        throw error;
    }
}

export function updateLineInGeometry(actionId, number, startPointNumber, endPointNumber, isActionIdShouldBeIncreased) 
{
    const lineData = { 
        "actionId": actionId, "number": number,
        "startPointNumber": startPointNumber, "endPointNumber": endPointNumber,
        "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithGeometry.updateLineInGeometry = lineData;
    } catch (error) {
        throw error;
    }
}

export function deleteLineFromGeometry(actionId, number, isActionIdShouldBeIncreased) 
{
    const lineData = { "actionId": actionId, "number": number, "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithGeometry.deleteLineFromGeometry = lineData;
    } catch (error) {
        throw error;
    }
}

export function restoreLineInGeometry(actionId, number, isActionIdShouldBeIncreased) 
{
    const lineData = { "actionId": actionId, "number": number, "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
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

export function extractGeometry(isActionIdShouldBeIncreased) 
{
    communicatorWithGeometry.extractGeometry = isActionIdShouldBeIncreased;
}

export function clearGeometryModuleByActionId(actionId) {
    communicatorWithGeometry.clearGeometryModuleByActionId = actionId;
}
