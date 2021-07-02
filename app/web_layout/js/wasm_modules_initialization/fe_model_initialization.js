import init, { FEModel } from "../../wasm/fe_model/fe_model.js";


export async function initializeFEModel() {
    await init();
    const fe_model = FEModel.create();
    return fe_model;    
}
