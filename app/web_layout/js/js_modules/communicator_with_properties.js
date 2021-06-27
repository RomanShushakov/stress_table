import { initializeProperties } from "../wasm_modules_initialization/properties_initialization.js";

class CommunicatorWithProperties {
    constructor() {
        this.state = {
            properties: null,       // wasm module "properties";
        };

        this.initProperties();
    }

    async initProperties() {
        this.state.properties = await initializeProperties();
        window.dispatchEvent(new CustomEvent("propertiesLoaded", {
            bubbles: true,
            composed: true,
        }));
    }

    set addMaterialToProperties(materialData) {
        try {
            this.state.properties.add_material(materialData.action_id, materialData.name, 
                materialData.young_modulus, materialData.poisson_ratio, materialData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updateMaterialInProperties(materialData) {
        try {
            this.state.properties.update_material(materialData.action_id, materialData.name, 
                materialData.young_modulus, materialData.poisson_ratio, materialData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set deleteMaterialFromProperties(materialData) {
        try {
            this.state.properties.delete_material(materialData.action_id, materialData.name, 
                materialData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set restoreMaterialInProperties(materialData) {
        try {
            this.state.properties.restore_material(materialData.action_id, materialData.name, 
                materialData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set addTrussSectionToProperties(trussSectionData) {
        try {
            this.state.properties.add_truss_section(trussSectionData.action_id, trussSectionData.name, 
                trussSectionData.area, trussSectionData.area2, trussSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updateTrussSectionInProperties(trussSectionData) {
        try {
            this.state.properties.update_truss_section(trussSectionData.action_id, trussSectionData.name, 
                trussSectionData.area, trussSectionData.area2, trussSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set deleteTrussSectionFromProperties(trussSectionData) {
        try {
            this.state.properties.delete_truss_section(trussSectionData.action_id,
                trussSectionData.name, trussSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set restoreTrussSectionInProperties(trussSectionData) {
        try {
            this.state.properties.restore_truss_section(trussSectionData.action_id, trussSectionData.name, 
                trussSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set addBeamSectionToProperties(beamSectionData) {
        try {
            this.state.properties.add_beam_section(beamSectionData.action_id, beamSectionData.name, 
                beamSectionData.area, beamSectionData.i11, beamSectionData.i22, beamSectionData.i12,
                beamSectionData.it, beamSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updateBeamSectionInProperties(beamSectionData) {
        try {
            this.state.properties.update_beam_section(beamSectionData.action_id, beamSectionData.name, 
                beamSectionData.area, beamSectionData.i11, beamSectionData.i22, beamSectionData.i12,
                beamSectionData.it, beamSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set deleteBeamSectionFromProperties(beamSectionData) {
        try {
            this.state.properties.delete_beam_section(beamSectionData.action_id,
                beamSectionData.name, beamSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set restoreBeamSectionInProperties(beamSectionData) {
        try {
            this.state.properties.restore_beam_section(beamSectionData.action_id, beamSectionData.name, 
                beamSectionData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set addPropertiesToProperties(propertiesData) {
        try {
            this.state.properties.add_properties(propertiesData.action_id, propertiesData.name, 
                propertiesData.material_name, propertiesData.cross_section_name,
                propertiesData.cross_section_type, propertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updatePropertiesInProperties(propertiesData) {
        try {
            this.state.properties.update_properties(propertiesData.action_id, propertiesData.name, 
                propertiesData.material_name, propertiesData.cross_section_name,
                propertiesData.cross_section_type, propertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set deletePropertiesFromProperties(propertiesData) {
        try {
            this.state.properties.delete_properties(propertiesData.action_id,
                propertiesData.name, propertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set restorePropertiesInProperties(propertiesData) {
        try {
            this.state.properties.restore_properties(propertiesData.action_id, propertiesData.name, 
                propertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set addAssignedPropertiesToProperties(assignedPropertiesData) {
        try {
            this.state.properties.add_assigned_properties(assignedPropertiesData.action_id, assignedPropertiesData.name, 
                assignedPropertiesData.line_numbers, assignedPropertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set updateAssignedPropertiesInProperties(assignedPropertiesData) {
        try {
            this.state.properties.update_assigned_properties(assignedPropertiesData.action_id, assignedPropertiesData.name, 
                assignedPropertiesData.line_numbers, assignedPropertiesData.is_action_id_should_be_increased);
        } catch (error) {
            throw error;
        }
    }

    set deleteLineNumbersFromProperties(lineNumbersData) {
        try {
            this.state.properties.delete_line_numbers(lineNumbersData.action_id,
                lineNumbersData.lineNumbers);
        } catch (error) {
            throw error;
        }
    }

    set clearPropertiesModuleByActionId(action_id) {
        this.state.properties.clear_properties_module_by_action_id(action_id);
    }

    set extractMaterials(handler) {
        this.state.properties.extract_materials(handler);
    }

    set extractTrussSections(handler) {
        this.state.properties.extract_truss_sections(handler);
    }

    set extractBeamSections(handler) {
        this.state.properties.extract_beam_sections(handler);
    }

    set extractProperties(handler) {
        this.state.properties.extract_properties(handler);
    }

    set extractAssignedProperties(handler) {
        this.state.properties.extract_assigned_properties(handler);
    }
}

export const communicatorWithProperties = new CommunicatorWithProperties();
