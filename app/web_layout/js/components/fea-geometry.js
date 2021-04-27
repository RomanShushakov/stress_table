class FeaGeometry extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {};

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                }

                .hidden {
                    display: none;
                }
            </style>
            <div class="wrapper">

                <button class="geometry">Geometry</button>

                <div class="geometry-container">
                    <button class="point">Point</button>
                    <button class="line">Line</button>
                </div>

                <div class="point-container">

                    <div class="actions-over-point">
                        <button class="select-add-action-over-point">Add</button>
                        <button class="select-update-action-over-point">Update</button>
                        <button class="select-delete-action-over-point">Delete</button>
                    </div>

                    <div class="add-action-over-point">

                        <ul class="add-action-over-point-fields">
                            <li>
                                <p class="added-point-number">#1 New</p>
                            </li>
                            <li>
                                <p class="add-action-over-point-fields-description">X coordinate:</p>
                                <input class="add-x-coord"/>
                            </li>
                            <li>
                                <p class="add-action-over-point-fields-description">Y coordinate:</p>
                                <input class="add-y-coord"/>
                            </li>
                            <li>
                                <p class="add-action-over-point-fields-description">Z coordinate:</p>
                                <input class="add-z-coord"/>
                            </li>
                        </ul>

                        <button class="point-add-action-apply">Apply</button>

                    </div>

                    <div class="update-action-over-point">

                        <ul class="update-action-over-point-fields">
                            <li>
                                <select class="updated-point-number">
                                    <option>#1</option>
                                </select>
                            </li>
                            <li>
                                <p class="update-action-over-point-fields-description">X coordinate:</p>
                                <input class="update-x-coord"/>
                            </li>
                            <li>
                                <p class="update-action-over-point-fields-description">Y coordinate:</p>
                                <input class="update-y-coord"/>
                            </li>
                            <li>
                                <p class="update-action-over-point-fields-description">Z coordinate:</p>
                                <input class="update-z-coord"/>
                            </li>
                        </ul>
                        
                        <button class="point-update-action-apply">Apply</button>

                    </div>

                    <div class="delete-action-over-point">

                        <ul class="delete-action-over-point-fields">
                            <li>
                                <select class="deleted-point-number">
                                    <option>#1</option>
                                </select>
                            </li>
                        </ul>
                        
                        <button class="point-delete-action-apply">Apply</button>

                    </div>
                </div>

                <div class="line-container">

                    <div class="actions-over-line">
                        <button class="select-add-action-over-line">Add</button>
                        <button class="select-update-action-over-line">Update</button>
                        <button class="select-delete-action-over-line">Delete</button>
                    </div>

                    <div class="add-action-over-line">

                        <ul class="add-action-over-line-fields">
                            <li>
                                <p class="added-line-number">#1 New</p>
                            </li>
                            <li>
                                <p class="add-action-over-line-fields-description">Select start point:</p>
                                <select class="selected-point-number">
                                    <option>#1</option>
                                </select>
                            </li>
                            <li>
                                <p class="add-action-over-line-fields-description">Select end point:</p>
                                <select class="selected-point-number">
                                    <option>#2</option>
                                </select>
                            </li>
                        </ul>

                        <button class="line-add-action-apply">Apply</button>

                    </div>

                    <div class="update-action-over-line">

                        <ul class="update-action-over-line-fields">
                            <li>
                                <select class="updated-line-number">
                                    <option>#1</option>
                                </select>
                            </li>
                            <li>
                                <p class="update-action-over-line-fields-description">Change line start point:</p>
                                <select class="selected-point-number">
                                    <option>#2</option>
                                </select>
                            </li>
                        </ul>
                        
                        <button class="line-update-action-apply">Apply</button>

                    </div>

                    <div class="delete-action-over-line">

                        <ul class="delete-action-over-line-fields">
                            <li>
                                <select class="deleted-line-number">
                                    <option>#1</option>
                                </select>
                            </li>
                        </ul>
                        
                        <button class="line-delete-action-apply">Apply</button>

                    </div>
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

export default FeaGeometry;