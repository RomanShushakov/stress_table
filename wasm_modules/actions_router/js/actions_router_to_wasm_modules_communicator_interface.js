import { ActionsRouterToWasmModulesCommunicator } from "/app/web_layout/js/js_modules/actions_router_to_wasm_modules_communicator.js";

const communicator = new ActionsRouterToWasmModulesCommunicator();


export function addPoint() 
{
    console.log("Yeah!!!");
    communicator.addPoint = "_greeting";
}
