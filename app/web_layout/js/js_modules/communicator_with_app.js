class CommunicatorWithApp
{
    constructor() 
    {
        this.props = {};

        this.state = {};
    }


    set addPointToApp(pointData) {
        document.querySelector("fea-app").dispatchEvent(new CustomEvent("add point", {
            bubbles: true,
            composed: true,
            detail: {
                pointData: pointData,
            },
        }));
    }

    set updatePointToApp(pointData) {
        document.querySelector("fea-app").dispatchEvent(new CustomEvent("update point", {
            bubbles: true,
            composed: true,
            detail: {
                pointData: pointData,
            },
        }));
    }
}

export const communicatorWithApp = new CommunicatorWithApp();
