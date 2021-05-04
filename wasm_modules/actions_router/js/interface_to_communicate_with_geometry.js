import { communicatorWithGeometry } from "/app/web_layout/js/js_modules/communicator_with_geometry.js";


export function addPointToGeometry(actionId, number, x, y, z) 
{
    const pointData = [actionId, number, x, y, z];
    communicatorWithGeometry.addPointToGeometry = pointData;
}
