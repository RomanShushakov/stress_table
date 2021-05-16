class FeaGeometryAddPointMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
            points: [ ],
        };

        this.state = {};

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: flex;
                }

                .wrapper {
                    display: flex;
                    flex-direction: column;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                    align-items: center;
                }

                .point-number-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin: 0rem;
                }

                .point-number-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .point-number {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    width: 5rem;
                    background-color: #3b4453;
                    border: #4a5060;
                    border-bottom: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                }

                .point-number::-webkit-outer-spin-button,
                .point-number::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .point-number[type=number] {
                    -moz-appearance: textfield;
                }

                .point-number:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-number:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-x-coord-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .point-x-coord-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .point-x-coord {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    width: 5rem;
                    background-color: #3b4453;
                    border: #4a5060;
                    border-bottom: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                }

                .point-x-coord[type=number]::-webkit-outer-spin-button,
                .point-x-coord[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .point-x-coord[type=number] {
                    -moz-appearance: textfield;
                }

                .point-x-coord:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-x-coord:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-y-coord-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .point-y-coord-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .point-y-coord {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    width: 5rem;
                    background-color: #3b4453;
                    border: #4a5060;
                    border-bottom: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                }

                .point-y-coord[type=number]::-webkit-outer-spin-button,
                .point-y-coord[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .point-y-coord[type=number] {
                    -moz-appearance: textfield;
                }

                .point-y-coord:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-y-coord:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-z-coord-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                }

                .point-z-coord-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .point-z-coord {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    width: 5rem;
                    background-color: #3b4453;
                    border: #4a5060;
                    border-bottom: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                }

                .point-z-coord[type=number]::-webkit-outer-spin-button,
                .point-z-coord[type=number]::-webkit-inner-spin-button {
                    -webkit-appearance: none;
                    margin: 0;
                }

                .point-z-coord[type=number] {
                    -moz-appearance: textfield;
                }

                .point-z-coord:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .point-z-coord:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .apply-cancel-buttons {
                    margin: 0rem;
                    padding: 0rem;
                }

                .apply-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 4rem;
                    height: 1.7rem;
                }

                .apply-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .cancel-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 4rem;
                    height: 1.7rem;
                }

                .cancel-button:hover {
                    border: 0.2rem solid #4a5060;
                }
            </style>

            <div class=wrapper>
                <div class="point-number-field-content">
                    <p class="point-number-caption">Point number</p>
                    <input class="point-number" type="number" step="1" min="1"/>
                </div>

                <div class="point-x-coord-field-content">
                    <p class="point-x-coord-caption">X-coordinate</p>
                    <input class="point-x-coord" type="number"/>
                </div>

                <div class="point-y-coord-field-content">
                    <p class="point-y-coord-caption">Y-coordinate</p>
                    <input class="point-y-coord" type="number"/>
                </div>

                <div class="point-z-coord-field-content">
                    <p class="point-z-coord-caption">Z-coordinate</p>
                    <input class="point-z-coord" type="number"/>
                </div>
                
                <div class="analysis-info">
                    <p class="analysis-info-message"></p>
                </div>

                <div class="apply-cancel-buttons">
                    <button class="apply-button">Apply</button>
                    <button class="cancel-button">Cancel</button>
                </div>
            </div>
        `;
    }

    connectedCallback() {
    }

    disconnectedCallback() {
    }

    static get observedAttributes() {
        return [];
    }

    attributeChangedCallback(name, oldValue, newValue) {
    }

    adoptedCallback() {
    }
}

export default FeaGeometryAddPointMenu;