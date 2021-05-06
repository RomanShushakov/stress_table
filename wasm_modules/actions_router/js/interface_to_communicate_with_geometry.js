import { communicatorWithGeometry } from "/app/web_layout/js/js_modules/communicator_with_geometry.js";


export function addPointToGeometry(actionId, number, x, y, z) 
{
    const pointData = { "actionId": actionId, "number": number, "x": x, "y": y, "z": z };
    try {
        communicatorWithGeometry.addPointToGeometry = pointData;
    } catch (error) {
        throw error;
    }
}


export function updatePointInGeometry(actionId, number, x, y, z) 
{
    const pointData = { "actionId": actionId, "number": number, "x": x, "y": y, "z": z };
    try {
        communicatorWithGeometry.updatePointInGeometry = pointData;
    } catch (error) {
        throw error;
    }
}


export function addWholeGeometryToPreprocessor() 
{
    communicatorWithGeometry.addWholeGeometryToPreprocessor = "_empty";
}


export function deletePointFromGeometry(actionId, number, x, y, z) 
{
    const pointData = { "actionId": actionId, "number": number, "x": x, "y": y, "z": z };
    try {
        communicatorWithGeometry.deletePointFromGeometry = pointData;
    } catch (error) {
        throw error;
    }
}