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
    } catch (error) {
        throw error;
    }
}

export function restorePointInGeometry(action_id, number, is_action_id_should_be_increased) 
{
    const pointData = { "action_id": action_id, "number": number, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.restorePointInGeometry = pointData;
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
    } catch (error) {
        throw error;
    }
}

export function restoreLineInGeometry(action_id, number, is_action_id_should_be_increased) 
{
    const lineData = { "action_id": action_id, "number": number, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.restoreLineInGeometry = lineData;
    } catch (error) {
        throw error;
    }
}

export function showPointInfo(number, handler) 
{
    const pointInfoData = {"number": number, "handler": handler};
    try {
        communicatorWithFEModel.showPointInfo = pointInfoData;
    } catch (error) {
        throw error;
    }
}

export function showLineInfo(number, handler) 
{
    const lineInfoData = { "number": number, "handler": handler };
    try {
        communicatorWithFEModel.showLineInfo = lineInfoData;
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

export function addMaterialToProperties(action_id, name, young_modulus, poisson_ratio, is_action_id_should_be_increased) 
{
    const materialData = { "action_id": action_id, "name": name, "young_modulus": young_modulus,
        "poisson_ratio": poisson_ratio, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.addMaterialToProperties = materialData;
    } catch (error) {
        throw error;
    }
}

export function updateMaterialInProperties(action_id, name, young_modulus, poisson_ratio, is_action_id_should_be_increased) 
{
    const materialData = { "action_id": action_id, "name": name, "young_modulus": young_modulus,
        "poisson_ratio": poisson_ratio, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.updateMaterialInProperties = materialData;
    } catch (error) {
        throw error;
    }
}

export function deleteMaterialFromProperties(action_id, name, is_action_id_should_be_increased) 
{
    const materialData = { "action_id": action_id, "name": name, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.deleteMaterialFromProperties = materialData;
    } catch (error) {
        throw error;
    }
}

export function restoreMaterialInProperties(action_id, name, is_action_id_should_be_increased) 
{
    const materialData = { "action_id": action_id, "name": name, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.restoreMaterialInProperties = materialData;
    } catch (error) {
        throw error;
    }
}

export function addTrussSectionToProperties(action_id, name, area, area2, is_action_id_should_be_increased) 
{
    const trussSectionData = { "action_id": action_id, "name": name, "area": area,
        "area2": area2, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.addTrussSectionToProperties = trussSectionData;
    } catch (error) {
        throw error;
    }
}

export function updateTrussSectionInProperties(action_id, name, area, area2, is_action_id_should_be_increased) 
{
    const trussSectionData = { "action_id": action_id, "name": name, "area": area,
        "area2": area2, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.updateTrussSectionInProperties = trussSectionData;
    } catch (error) {
        throw error;
    }
}

export function deleteTrussSectionFromProperties(action_id, name, is_action_id_should_be_increased) 
{
    const trussSectionData = { "action_id": action_id, "name": name, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.deleteTrussSectionFromProperties = trussSectionData;
    } catch (error) {
        throw error;
    }
}

export function restoreTrussSectionInProperties(action_id, name, is_action_id_should_be_increased) 
{
    const trussSectionData = { "action_id": action_id, "name": name, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.restoreTrussSectionInProperties = trussSectionData;
    } catch (error) {
        throw error;
    }
}

export function addBeamSectionToProperties(action_id, name, area, i11, i22, i12, it, is_action_id_should_be_increased) 
{
    const beamSectionData = { "action_id": action_id, "name": name, "area": area,
        "i11": i11, "i22": i22, "i12": i12, "it": it, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.addBeamSectionToProperties = beamSectionData;
    } catch (error) {
        throw error;
    }
}

export function updateBeamSectionInProperties(action_id, name, area, i11, i22, i12, it, is_action_id_should_be_increased) 
{
    const beamSectionData = { "action_id": action_id, "name": name, "area": area,
        "i11": i11, "i22": i22, "i12": i12, "it": it, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.updateBeamSectionInProperties = beamSectionData;
    } catch (error) {
        throw error;
    }
}

export function deleteBeamSectionFromProperties(action_id, name, is_action_id_should_be_increased) 
{
    const beamSectionData = { "action_id": action_id, "name": name, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.deleteBeamSectionFromProperties = beamSectionData;
    } catch (error) {
        throw error;
    }
}

export function restoreBeamSectionInProperties(action_id, name, is_action_id_should_be_increased) 
{
    const beamSectionData = { "action_id": action_id, "name": name, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.restoreBeamSectionInProperties = beamSectionData;
    } catch (error) {
        throw error;
    }
}

export function addPropertiesToProperties(action_id, name, material_name, cross_section_name, cross_section_type, 
    is_action_id_should_be_increased) 
{
    const propertiesData = { "action_id": action_id, "name": name, "material_name": material_name,
        "cross_section_name": cross_section_name, "cross_section_type": cross_section_type,
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.addPropertiesToProperties = propertiesData;
    } catch (error) {
        throw error;
    }
}

export function updatePropertiesInProperties(action_id, name, material_name, cross_section_name, cross_section_type,
    is_action_id_should_be_increased) 
{
    const propertiesData = { "action_id": action_id, "name": name, "material_name": material_name,
        "cross_section_name": cross_section_name, "cross_section_type": cross_section_type, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.updatePropertiesInProperties = propertiesData;
    } catch (error) {
        throw error;
    }
}

export function deletePropertiesFromProperties(action_id, name, is_action_id_should_be_increased) 
{
    const propertiesData = { "action_id": action_id, "name": name, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.deletePropertiesFromProperties = propertiesData;
    } catch (error) {
        throw error;
    }
}

export function restorePropertiesInProperties(action_id, name, is_action_id_should_be_increased) 
{
    const propertiesData = { "action_id": action_id, "name": name, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.restorePropertiesInProperties = propertiesData;
    } catch (error) {
        throw error;
    }
}

export function addAssignedPropertiesToProperties(action_id, name, line_numbers, is_action_id_should_be_increased) 
{
    const assignedPropertiesData = { "action_id": action_id, "name": name, "line_numbers": line_numbers,
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.addAssignedPropertiesToProperties = assignedPropertiesData;
    } catch (error) {
        throw error;
    }
}

export function updateAssignedPropertiesInProperties(action_id, name, line_numbers, is_action_id_should_be_increased) 
{
    const assignedPropertiesData = { "action_id": action_id, "name": name, "line_numbers": line_numbers, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.updateAssignedPropertiesInProperties = assignedPropertiesData;
    } catch (error) {
        throw error;
    }
}

export function deleteAssignedPropertiesFromProperties(action_id, name, is_action_id_should_be_increased) 
{
    const assignedPropertiesData = { "action_id": action_id, "name": name, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.deleteAssignedPropertiesFromProperties = assignedPropertiesData;
    } catch (error) {
        throw error;
    }
}

export function restoreAssignedPropertiesInProperties(action_id, name, is_action_id_should_be_increased) 
{
    const assignedPropertiesData = { "action_id": action_id, "name": name, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.restoreAssignedPropertiesInProperties = assignedPropertiesData;
    } catch (error) {
        throw error;
    }
}

export function addBeamSectionLocalAxis1DirectionToProperties(action_id, local_axis_1_direction,
    is_action_id_should_be_increased) {
    const beamSectionLocalAxis1DirectionData = { "action_id": action_id, 
        "local_axis_1_direction": local_axis_1_direction, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.addBeamSectionLocalAxis1DirectionToProperties = beamSectionLocalAxis1DirectionData;
    } catch (error) {
        throw error;
    }
}

export function removeBeamSectionLocalAxis1DirectionFromProperties(action_id, local_axis_1_direction,
    is_action_id_should_be_increased) {
    const beamSectionLocalAxis1DirectionData = { "action_id": action_id, 
        "local_axis_1_direction": local_axis_1_direction, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.removeBeamSectionLocalAxis1DirectionFromProperties = beamSectionLocalAxis1DirectionData;
    } catch (error) {
        throw error;
    }
}

export function restoreBeamSectionLocalAxis1DirectionInProperties(action_id, local_axis_1_direction,
    is_action_id_should_be_increased) {
    const beamSectionLocalAxis1DirectionData = { "action_id": action_id, 
        "local_axis_1_direction": local_axis_1_direction, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithFEModel.restoreBeamSectionLocalAxis1DirectionInProperties = beamSectionLocalAxis1DirectionData;
    } catch (error) {
        throw error;
    }
}

export function clearPropertiesModuleByActionId(action_id) {
    communicatorWithFEModel.clearPropertiesModuleByActionId = action_id;
}

export function extractMaterials(handler) 
{
    communicatorWithFEModel.extractMaterials = handler;
}

export function extractTrussSections(handler) 
{
    communicatorWithFEModel.extractTrussSections = handler;
}

export function extractBeamSections(handler) 
{
    communicatorWithFEModel.extractBeamSections = handler;
}

export function extractProperties(handler) 
{
    communicatorWithFEModel.extractProperties = handler;
}

export function extractAssignedProperties(handler) 
{
    communicatorWithFEModel.extractAssignedProperties = handler;
}

export function extractBeamSectionsOrientations(handler) 
{
    communicatorWithFEModel.extractBeamSectionsOrientations = handler;
}
