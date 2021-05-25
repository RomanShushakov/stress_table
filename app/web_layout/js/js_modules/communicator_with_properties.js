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
            this.state.properties.delete_material(materialData.actionId, materialData.name, materialData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }

    set undoDeleteMaterialFromProperties(materialData) {
        try {
            this.state.properties.undo_delete_material(materialData.actionId, materialData.name, materialData.isActionIdShouldBeIncreased);
        } catch (error) {
            throw error;
        }
    }
}

export const communicatorWithProperties = new CommunicatorWithProperties();
