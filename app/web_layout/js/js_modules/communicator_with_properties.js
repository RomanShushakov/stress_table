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
    }

    set addMaterialToProperties(materialData) {
        try {
            this.state.properties.add_material(materialData.actionId, materialData.name, 
                materialData.youngModulus, materialData.poissonRatio, materialData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set updateMaterialInProperties(materialData) {
        try {
            this.state.properties.update_material(materialData.actionId, materialData.name, 
                materialData.youngModulus, materialData.poissonRatio, materialData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set deleteMaterialFromProperties(materialData) {
        try {
            this.state.properties.delete_material(materialData.actionId, materialData.name, 
                materialData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set restoreMaterialInProperties(materialData) {
        try {
            this.state.properties.restore_material(materialData.actionId, materialData.name, 
                materialData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set addTrussSectionToProperties(trussSectionData) {
        try {
            this.state.properties.add_truss_section(trussSectionData.actionId, trussSectionData.name, 
                trussSectionData.area, trussSectionData.area2, trussSectionData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set updateTrussSectionInProperties(trussSectionData) {
        try {
            this.state.properties.update_truss_section(trussSectionData.actionId, trussSectionData.name, 
                trussSectionData.area, trussSectionData.area2, trussSectionData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set deleteTrussSectionFromProperties(trussSectionData) {
        try {
            this.state.properties.delete_truss_section(trussSectionData.actionId,
                trussSectionData.name, trussSectionData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set restoreTrussSectionInProperties(trussSectionData) {
        try {
            this.state.properties.restore_truss_section(trussSectionData.actionId, trussSectionData.name, 
                trussSectionData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set clearPropertiesModuleByActionId(actionId) {
        this.state.properties.clear_properties_module_by_action_id(actionId);
    }
}

export const communicatorWithProperties = new CommunicatorWithProperties();
