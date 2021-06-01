import { communicatorWithProperties } from "/app/web_layout/js/js_modules/communicator_with_properties.js";
// import { communicatorWithProperties } from "../../../../../js/js_modules/communicator_with_properties.js";

export function addMaterialToProperties(actionId, name, youngModulus, poissonRatio, isActionIdShouldBeIncreased) 
{
    const materialData = { "actionId": actionId, "name": name, "youngModulus": youngModulus,
        "poissonRatio": poissonRatio, "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithProperties.addMaterialToProperties = materialData;
    } catch (error) {
        throw error;
    }
}

export function updateMaterialInProperties(actionId, name, youngModulus, poissonRatio, isActionIdShouldBeIncreased) 
{
    const materialData = { "actionId": actionId, "name": name, "youngModulus": youngModulus,
        "poissonRatio": poissonRatio, "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithProperties.updateMaterialInProperties = materialData;
    } catch (error) {
        throw error;
    }
}

export function deleteMaterialFromProperties(actionId, name, isActionIdShouldBeIncreased) 
{
    const materialData = { "actionId": actionId, "name": name, "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithProperties.deleteMaterialFromProperties = materialData;
    } catch (error) {
        throw error;
    }
}

export function undoDeleteMaterialFromProperties(actionId, name, isActionIdShouldBeIncreased) 
{
    const materialData = { "actionId": actionId, "name": name, "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithProperties.undoDeleteMaterialFromProperties = materialData;
    } catch (error) {
        throw error;
    }
}

export function addTrussSectionToProperties(actionId, name, area, area2, isActionIdShouldBeIncreased) 
{
    const trussSectionData = { "actionId": actionId, "name": name, "area": area,
        "area2": area2, "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithProperties.addTrussSectionToProperties = trussSectionData;
    } catch (error) {
        throw error;
    }
}

export function updateTrussSectionInProperties(actionId, name, area, area2, isActionIdShouldBeIncreased) 
{
    const trussSectionData = { "actionId": actionId, "name": name, "area": area,
        "area2": area2, "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithProperties.updateTrussSectionInProperties = trussSectionData;
    } catch (error) {
        throw error;
    }
}

export function deleteTrussSectionFromProperties(actionId, name, isActionIdShouldBeIncreased) 
{
    const trussSectionData = { "actionId": actionId, "name": name, "isActionIdShouldBeIncreased": isActionIdShouldBeIncreased };
    try {
        communicatorWithProperties.deleteTrussSectionFromProperties = trussSectionData;
    } catch (error) {
        throw error;
    }
}
