import { communicatorWithProperties } from "../../../../../js/js_modules/communicator_with_properties.js";

export function addMaterialToProperties(action_id, name, young_modulus, poisson_ratio, is_action_id_should_be_increased) 
{
    const materialData = { "action_id": action_id, "name": name, "young_modulus": young_modulus,
        "poisson_ratio": poisson_ratio, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.addMaterialToProperties = materialData;
    } catch (error) {
        throw error;
    }
}

export function updateMaterialInProperties(action_id, name, young_modulus, poisson_ratio, is_action_id_should_be_increased) 
{
    const materialData = { "action_id": action_id, "name": name, "young_modulus": young_modulus,
        "poisson_ratio": poisson_ratio, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.updateMaterialInProperties = materialData;
    } catch (error) {
        throw error;
    }
}

export function deleteMaterialFromProperties(action_id, name, is_action_id_should_be_increased) 
{
    const materialData = { "action_id": action_id, "name": name, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.deleteMaterialFromProperties = materialData;
    } catch (error) {
        throw error;
    }
}

export function restoreMaterialInProperties(action_id, name, is_action_id_should_be_increased) 
{
    const materialData = { "action_id": action_id, "name": name, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.restoreMaterialInProperties = materialData;
    } catch (error) {
        throw error;
    }
}

export function addTrussSectionToProperties(action_id, name, area, area2, is_action_id_should_be_increased) 
{
    const trussSectionData = { "action_id": action_id, "name": name, "area": area,
        "area2": area2, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.addTrussSectionToProperties = trussSectionData;
    } catch (error) {
        throw error;
    }
}

export function updateTrussSectionInProperties(action_id, name, area, area2, is_action_id_should_be_increased) 
{
    const trussSectionData = { "action_id": action_id, "name": name, "area": area,
        "area2": area2, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.updateTrussSectionInProperties = trussSectionData;
    } catch (error) {
        throw error;
    }
}

export function deleteTrussSectionFromProperties(action_id, name, is_action_id_should_be_increased) 
{
    const trussSectionData = { "action_id": action_id, "name": name, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.deleteTrussSectionFromProperties = trussSectionData;
    } catch (error) {
        throw error;
    }
}

export function restoreTrussSectionInProperties(action_id, name, is_action_id_should_be_increased) 
{
    const trussSectionData = { "action_id": action_id, "name": name, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.restoreTrussSectionInProperties = trussSectionData;
    } catch (error) {
        throw error;
    }
}

export function addBeamSectionToProperties(action_id, name, area, i11, i22, i12, it, is_action_id_should_be_increased) 
{
    const beamSectionData = { "action_id": action_id, "name": name, "area": area,
        "i11": i11, "i22": i22, "i12": i12, "it": it, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.addBeamSectionToProperties = beamSectionData;
    } catch (error) {
        throw error;
    }
}

export function updateBeamSectionInProperties(action_id, name, area, i11, i22, i12, it, is_action_id_should_be_increased) 
{
    const beamSectionData = { "action_id": action_id, "name": name, "area": area,
        "i11": i11, "i22": i22, "i12": i12, "it": it, "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.updateBeamSectionInProperties = beamSectionData;
    } catch (error) {
        throw error;
    }
}

export function deleteBeamSectionFromProperties(action_id, name, is_action_id_should_be_increased) 
{
    const beamSectionData = { "action_id": action_id, "name": name, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.deleteBeamSectionFromProperties = beamSectionData;
    } catch (error) {
        throw error;
    }
}

export function restoreBeamSectionInProperties(action_id, name, is_action_id_should_be_increased) 
{
    const beamSectionData = { "action_id": action_id, "name": name, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.restoreBeamSectionInProperties = beamSectionData;
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
        communicatorWithProperties.addPropertiesToProperties = propertiesData;
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
        communicatorWithProperties.updatePropertiesInProperties = propertiesData;
    } catch (error) {
        throw error;
    }
}

export function deletePropertiesFromProperties(action_id, name, is_action_id_should_be_increased) 
{
    const propertiesData = { "action_id": action_id, "name": name, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.deletePropertiesFromProperties = propertiesData;
    } catch (error) {
        throw error;
    }
}

export function restorePropertiesInProperties(action_id, name, is_action_id_should_be_increased) 
{
    const propertiesData = { "action_id": action_id, "name": name, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.restorePropertiesInProperties = propertiesData;
    } catch (error) {
        throw error;
    }
}

export function addAssignedPropertiesToProperties(action_id, name, line_numbers, is_action_id_should_be_increased) 
{
    const assignedPropertiesData = { "action_id": action_id, "name": name, "line_numbers": line_numbers,
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.addAssignedPropertiesToProperties = assignedPropertiesData;
    } catch (error) {
        throw error;
    }
}

export function updateAssignedPropertiesInProperties(action_id, name, line_numbers, is_action_id_should_be_increased) 
{
    const assignedPropertiesData = { "action_id": action_id, "name": name, "line_numbers": line_numbers, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.updateAssignedPropertiesInProperties = assignedPropertiesData;
    } catch (error) {
        throw error;
    }
}

export function deleteAssignedPropertiesFromProperties(action_id, name, is_action_id_should_be_increased) 
{
    const assignedPropertiesData = { "action_id": action_id, "name": name, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.deleteAssignedPropertiesFromProperties = assignedPropertiesData;
    } catch (error) {
        throw error;
    }
}

export function restoreAssignedPropertiesInProperties(action_id, name, is_action_id_should_be_increased) 
{
    const assignedPropertiesData = { "action_id": action_id, "name": name, 
        "is_action_id_should_be_increased": is_action_id_should_be_increased };
    try {
        communicatorWithProperties.restoreAssignedPropertiesInProperties = assignedPropertiesData;
    } catch (error) {
        throw error;
    }
}

export function deleteLineNumbersFromProperties(action_id, lineNumbers) 
{
    const lineNumbersData = { "action_id": action_id, "lineNumbers": lineNumbers };
    try {
        communicatorWithProperties.deleteLineNumbersFromProperties = lineNumbersData;
    } catch (error) {
        throw error;
    }
}

export function clearPropertiesModuleByActionId(action_id) {
    communicatorWithProperties.clearPropertiesModuleByActionId = action_id;
}

export function extractMaterials(handler) 
{
    communicatorWithProperties.extractMaterials = handler;
}

export function extractTrussSections(handler) 
{
    communicatorWithProperties.extractTrussSections = handler;
}

export function extractBeamSections(handler) 
{
    communicatorWithProperties.extractBeamSections = handler;
}

export function extractProperties(handler) 
{
    communicatorWithProperties.extractProperties = handler;
}

export function extractAssignedProperties(handler) 
{
    communicatorWithProperties.extractAssignedProperties = handler;
}
