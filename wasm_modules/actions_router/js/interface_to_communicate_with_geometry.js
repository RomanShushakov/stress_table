import { communicatorWithGeometry } from "/app/web_layout/js/js_modules/communicator_with_geometry.js";


export function addPointToGeometry(actionId, number, x, y, z) 
{
    const pointData = [actionId, number, x, y, z];
    try {
        communicatorWithGeometry.addPointToGeometry = pointData;
    } catch (error) {
        throw error;
    }
}


export function updatePointToGeometry(actionId, number, x, y, z) 
{
    const pointData = [actionId, number, x, y, z];
    try {
        communicatorWithGeometry.updatePointToGeometry = pointData;
    } catch (error) {
        throw error;
    }
}


export function addWholeGeometryToPreprocessor() 
{
    communicatorWithGeometry.addWholeGeometryToPreprocessor = "_empty";
}
