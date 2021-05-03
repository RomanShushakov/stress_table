import { initializeGeometry } from "../wasm_js_interface_modules/geometry_initialization.js";

export class ActionsRouterToWasmModulesCommunicator
{
    constructor() 
    {
        this.state = {
            geometry: null,
        };

        this.initGeometry();
    }

    async initGeometry() {
        this.state.geometry = await initializeGeometry();
    }

    set addPoint(_data) {
        this.state.geometry.add_point();
        document.querySelector("fea-app").dispatchEvent(new CustomEvent("add point", {
            bubbles: true,
            composed: true,
            detail: {
                message: "Hooraah!!!",
            },
        }));
    }
}