import { initializeFEModel } from "../wasm_modules_initialization/fe_model_initialization.js";

class CommunicatorWithFEModel {
    constructor() {
        this.state = {
            feModel: null,     // wasm module "fe_model";
        };

        this.initFEModel();
    }

    async initFEModel() {
        this.state.feModel = await initializeFEModel();
        window.dispatchEvent(new CustomEvent("feModelLoaded", {
            bubbles: true,
            composed: true,
        }));
    }

    set addPointToGeometry(pointData) {
        try {
            this.state.feModel.add_point(pointData.action_id, pointData.number, pointData.x, pointData.y, pointData.z,
                pointData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updatePointInGeometry(pointData) {
        try {
            this.state.feModel.update_point(pointData.action_id, pointData.number, pointData.x, pointData.y, pointData.z,
                pointData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }


    set deletePointFromGeometry(pointData) {
        try {
            this.state.feModel.delete_point(pointData.action_id,
                pointData.number, pointData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set restorePointInGeometry(pointData) {
        try {
            this.state.feModel.restore_point(pointData.action_id,
                pointData.number, pointData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set addLineToGeometry(lineData) {
        try {
            this.state.feModel.add_line(lineData.action_id, lineData.number, lineData.start_point_number, lineData.end_point_number,
                lineData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updateLineInGeometry(lineData) {
        try {
            this.state.feModel.update_line(lineData.action_id, lineData.number, lineData.start_point_number, lineData.end_point_number,
                lineData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set deleteLineFromGeometry(lineData) {
        try {
            this.state.feModel.delete_line(lineData.action_id,
                lineData.number, lineData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set restoreLineInGeometry(lineData) {
        try {
            this.state.feModel.restore_line(lineData.action_id,
                lineData.number, lineData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set showPointInfo(pointInfoData) {
        try {
            this.state.feModel.show_point_info(pointInfoData.number, pointInfoData.handler);
        } catch (error) {
            throw error;
        }
    }

    set showLineInfo(lineInfoData) {
        try {
            this.state.feModel.show_line_info(lineInfoData.number, lineInfoData.handler);
        } catch (error) {
            throw error;
        }
    }

    set clearGeometryModuleByActionId(action_id) {
        this.state.feModel.clear_geometry_module_by_action_id(action_id);
    }

    set extractPoints(handler) {
        this.state.feModel.extract_points(handler);
    }

    set extractLines(handler) {
        this.state.feModel.extract_lines(handler);
    }

    set addMaterialToProperties(materialData) {
        try {
            this.state.feModel.add_material(materialData.action_id, materialData.name, 
                materialData.young_modulus, materialData.poisson_ratio, materialData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updateMaterialInProperties(materialData) {
        try {
            this.state.feModel.update_material(materialData.action_id, materialData.name, 
                materialData.young_modulus, materialData.poisson_ratio, materialData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set deleteMaterialFromProperties(materialData) {
        try {
            this.state.feModel.delete_material(materialData.action_id, materialData.name, 
                materialData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set restoreMaterialInProperties(materialData) {
        try {
            this.state.feModel.restore_material(materialData.action_id, materialData.name, 
                materialData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set addTrussSectionToProperties(trussSectionData) {
        try {
            this.state.feModel.add_truss_section(trussSectionData.action_id, trussSectionData.name, 
                trussSectionData.area, trussSectionData.area2, trussSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updateTrussSectionInProperties(trussSectionData) {
        try {
            this.state.feModel.update_truss_section(trussSectionData.action_id, trussSectionData.name, 
                trussSectionData.area, trussSectionData.area2, trussSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set deleteTrussSectionFromProperties(trussSectionData) {
        try {
            this.state.feModel.delete_truss_section(trussSectionData.action_id,
                trussSectionData.name, trussSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set restoreTrussSectionInProperties(trussSectionData) {
        try {
            this.state.feModel.restore_truss_section(trussSectionData.action_id, trussSectionData.name, 
                trussSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set addBeamSectionToProperties(beamSectionData) {
        try {
            this.state.feModel.add_beam_section(beamSectionData.action_id, beamSectionData.name, 
                beamSectionData.area, beamSectionData.i11, beamSectionData.i22, beamSectionData.i12,
                beamSectionData.it, beamSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updateBeamSectionInProperties(beamSectionData) {
        try {
            this.state.feModel.update_beam_section(beamSectionData.action_id, beamSectionData.name, 
                beamSectionData.area, beamSectionData.i11, beamSectionData.i22, beamSectionData.i12,
                beamSectionData.it, beamSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set deleteBeamSectionFromProperties(beamSectionData) {
        try {
            this.state.feModel.delete_beam_section(beamSectionData.action_id,
                beamSectionData.name, beamSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set restoreBeamSectionInProperties(beamSectionData) {
        try {
            this.state.feModel.restore_beam_section(beamSectionData.action_id, beamSectionData.name, 
                beamSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set addPropertiesToProperties(propertiesData) {
        try {
            this.state.feModel.add_properties(propertiesData.action_id, propertiesData.name, 
                propertiesData.material_name, propertiesData.cross_section_name,
                propertiesData.cross_section_type, propertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updatePropertiesInProperties(propertiesData) {
        try {
            this.state.feModel.update_properties(propertiesData.action_id, propertiesData.name, 
                propertiesData.material_name, propertiesData.cross_section_name,
                propertiesData.cross_section_type, propertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set deletePropertiesFromProperties(propertiesData) {
        try {
            this.state.feModel.delete_properties(propertiesData.action_id,
                propertiesData.name, propertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set restorePropertiesInProperties(propertiesData) {
        try {
            this.state.feModel.restore_properties(propertiesData.action_id, propertiesData.name, 
                propertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set addAssignedPropertiesToProperties(assignedPropertiesData) {
        try {
            this.state.feModel.add_assigned_properties(assignedPropertiesData.action_id, assignedPropertiesData.name, 
                assignedPropertiesData.line_numbers, assignedPropertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updateAssignedPropertiesInProperties(assignedPropertiesData) {
        try {
            this.state.feModel.update_assigned_properties(assignedPropertiesData.action_id, assignedPropertiesData.name, 
                assignedPropertiesData.line_numbers, assignedPropertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set deleteAssignedPropertiesFromProperties(assignedPropertiesData) {
        try {
            this.state.feModel.delete_assigned_properties(assignedPropertiesData.action_id,
                assignedPropertiesData.name, assignedPropertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set restoreAssignedPropertiesInProperties(assignedPropertiesData) {
        try {
            this.state.feModel.restore_assigned_properties(assignedPropertiesData.action_id, assignedPropertiesData.name, 
                assignedPropertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set addBeamSectionLocalAxis1DirectionToProperties(beamSectionLocalAxis1DirectionData) {
        try {
            this.state.feModel.add_beam_section_local_axis_1_direction(beamSectionLocalAxis1DirectionData.action_id,
                beamSectionLocalAxis1DirectionData.local_axis_1_direction, 
                beamSectionLocalAxis1DirectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set removeBeamSectionLocalAxis1DirectionFromProperties(beamSectionLocalAxis1DirectionData) {
        try {
            this.state.feModel.remove_beam_section_local_axis_1_direction(beamSectionLocalAxis1DirectionData.action_id,
                beamSectionLocalAxis1DirectionData.local_axis_1_direction, 
                beamSectionLocalAxis1DirectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set restoreBeamSectionLocalAxis1DirectionInProperties(beamSectionLocalAxis1DirectionData) {
        try {
            this.state.feModel.restore_beam_section_local_axis_1_direction(beamSectionLocalAxis1DirectionData.action_id,
                beamSectionLocalAxis1DirectionData.local_axis_1_direction, 
                beamSectionLocalAxis1DirectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set clearPropertiesModuleByActionId(action_id) {
        this.state.feModel.clear_properties_module_by_action_id(action_id);
    }

    set extractMaterials(handler) {
        this.state.feModel.extract_materials(handler);
    }

    set extractTrussSections(handler) {
        this.state.feModel.extract_truss_sections(handler);
    }

    set extractBeamSections(handler) {
        this.state.feModel.extract_beam_sections(handler);
    }

    set extractProperties(handler) {
        this.state.feModel.extract_properties(handler);
    }

    set extractAssignedProperties(handler) {
        this.state.feModel.extract_assigned_properties(handler);
    }

    set extractBeamSectionsOrientations(handler) {
        this.state.feModel.extract_beam_sections_orientations(handler);
    }
}

export const communicatorWithFEModel = new CommunicatorWithFEModel();
