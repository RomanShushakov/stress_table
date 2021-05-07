import init, { ActionsRouter } from "../../wasm/actions_router/actions_router.js";


export async function initializeActionsRouter() {
    await init();
    const actionsRouter = ActionsRouter.create();
    return actionsRouter;    
}
