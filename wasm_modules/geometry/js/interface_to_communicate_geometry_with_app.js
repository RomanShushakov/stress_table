import { communicatorWithApp } from "/app/web_layout/js/js_modules/communicator_with_app.js";


export function addPointToApp(number, x, y, z, isPreprocessorRequest) 
{
    const pointData = [number, x, y, z, isPreprocessorRequest];
    communicatorWithApp.addPointToApp = pointData;
}

export function updatePointToApp(number, x, y, z) 
{
    const pointData = [number, x, y, z];
    communicatorWithApp.updatePointToApp = pointData;
}
