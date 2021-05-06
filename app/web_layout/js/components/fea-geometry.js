class FeaGeometry extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
            points: [ ],
            lines: [ ]
        };

        this.state = {
            buttonNames: {
                geometry: "Geometry", point: "Point", line: "Line",
                pointAdd: "Add", pointUpdate: "Update", pointDelete: "Delete",
                lineAdd: "Add", lineUpdate: "Update", lineDelete: "Delete",
            },
            buttonFullNames: {
                geometry: "geometry", point: "point", line: "line",
                pointAdd: "pointAdd", pointUpdate: "pointUpdate", pointDelete: "pointDelete",
                lineAdd: "lineAdd", lineUpdate: "lineUpdate", lineDelete: "lineDelete",
            }
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: flex;
                }

                .wrapper {
                    background-color: #eee;
                    display: flex;
                    align-items: center;
                    box-sizing: content-box;
                    flex-direction: column;
                    border-right: 1px solid #9a9a9a;
                    border-left: 1px solid #9a9a9a;
                }


                .add-action-over-point-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .add-action-over-point-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                }

                .add-point-number {
                    width: 9.65rem;
                    margin-bottom: 0.5rem;
                }

                .add-x-coord {
                    width: 9.65rem;
                    margin-bottom: 0.5rem;
                }

                .add-y-coord {
                    width: 9.65rem;
                    margin-bottom: 0.5rem;
                }

                .add-z-coord {
                    width: 9.65rem;
                    margin-bottom: 0.5rem;
                }

                .analysis-info {
                    margin: 0rem;
                    display: flex;
                }

                .point-add-message {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.9rem;
                    color: #ff0000;
                    max-width: 9rem;
                }

                .add-action-over-point-apply-cancel-buttons-container {
                    margin: 0rem;
                }

                .point-add-action-apply {
                    width: 5rem;
                    padding: 0rem;
                }

                .point-add-action-cancel {
                    width: 5rem;
                    padding: 0rem;
                }

                .update-action-over-point-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .update-action-over-point-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                }

                .point-number-update {
                    display: flex;
                    flex-direction: row;
                }

                .select-and-search-number {
                    display: flex;
                    flex-direction: column;
                }

                .search-point-number-for-update {
                    width: 5rem;
                    height: 1rem;
                    margin-bottom: 0.5rem;
                }

                .selected-point-number-for-update-info {
                    margin-top: 0.05rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.8rem;
                }

                .updated-point-number {
                    margin-bottom: 0.62rem;
                    margin-right: 0.5rem;
                    min-width: 3rem;
                    max-width: 5rem;
                }

                .update-x-coord {
                    width: 9.65rem;
                    margin-bottom: 0.5rem;
                }

                .update-y-coord {
                    width: 9.65rem;
                    margin-bottom: 0.5rem;
                }

                .update-z-coord {
                    width: 9.65rem;
                    margin-bottom: 0.5rem;
                }

                .analysis-info {
                    margin: 0rem;
                    display: flex;
                }

                .point-update-message {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.9rem;
                    color: #ff0000;
                    max-width: 9rem;
                }

                .update-action-over-point-apply-cancel-buttons-container {
                    margin: 0rem;
                }

                .point-update-action-apply {
                    width: 5rem;
                    padding: 0rem;
                }

                .point-update-action-cancel {
                    width: 5rem;
                    padding: 0rem;
                }

                .delete-action-over-point-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .delete-action-over-point-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                    width: 10.15rem;
                }

                .point-number-delete {
                    display: flex;
                    flex-direction: row;
                }

                .search-point-number-for-delete {
                    width: 5rem;
                    height: 1rem;
                    margin-bottom: 0.5rem;
                }

                .selected-point-number-for-delete-info {
                    margin-top: 0.05rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.8rem;
                }

                .deleted-point-number {
                    margin-bottom: 0.62rem;
                    margin-right: 0.5rem;
                    min-width: 3rem;
                    max-width: 5rem;
                }

                .analysis-info {
                    margin: 0rem;
                    display: flex;
                }

                .point-delete-message {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.9rem;
                    color: #ff0000;
                    max-width: 9rem;
                }

                .delete-action-over-point-apply-cancel-buttons-container {
                    margin: 0rem;
                }

                .point-delete-action-apply {
                    width: 5rem;
                    padding: 0rem;
                }

                .point-delete-action-cancel {
                    width: 5rem;
                    padding: 0rem;
                }

                .add-action-over-line-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .add-action-over-line-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                }

                .line-number-add-start-point {
                    display: flex;
                    flex-direction: row;
                }

                .selected-start-point-number-for-line-addition-info {
                    margin-top: 0.05rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.8rem;
                }

                .search-start-point-number-for-line-addition {
                    width: 5rem;
                    height: 1rem;
                    margin-bottom: 0.5rem;
                }

                .line-number-add-end-point {
                    display: flex;
                    flex-direction: row;
                }

                .selected-end-point-number-for-line-addition-info {
                    margin-top: 0.05rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.8rem;
                }

                .search-end-point-number-for-line-addition {
                    width: 5rem;
                    height: 1rem;
                    margin-bottom: 0.5rem;
                }

                .add-line-number {
                    width: 9.65rem;
                    margin-bottom: 0.5rem;
                }

                .selected-start-point-number-for-line-addition {
                    margin-bottom: 0.62rem;
                    margin-right: 0.5rem;
                    min-width: 3rem;
                    max-width: 5rem;
                }

                .selected-end-point-number-for-line-addition {
                    margin-bottom: 0.62rem;
                    margin-right: 0.5rem;
                    min-width: 3rem;
                    max-width: 5rem;
                }

                .analysis-info {
                    margin: 0rem;
                    display: flex;
                }

                .line-add-message {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.9rem;
                    color: #ff0000;
                    max-width: 9rem;
                }

                .add-action-over-line-apply-cancel-buttons-container {
                    margin: 0rem;
                }

                .line-add-action-apply {
                    width: 5rem;
                    padding: 0rem;
                }

                .line-add-action-cancel {
                    width: 5rem;
                    padding: 0rem;
                }

                .update-action-over-line-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .update-action-over-line-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                    width: 10.15rem;
                }

                .line-number-update {
                    display: flex;
                    flex-direction: row;
                }

                .updated-line-number {
                    margin-bottom: 0.62rem;
                    margin-right: 0.5rem;
                    min-width: 3rem;
                    max-width: 5rem;
                }

                .search-line-number-for-update {
                    width: 5rem;
                    height: 1rem;
                    margin-bottom: 0.5rem;
                }

                .selected-line-number-for-update-info {
                    margin-top: 0.05rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.8rem;
                }

                .line-number-update-start-point {
                    display: flex;
                    flex-direction: row;
                }

                .search-start-point-number-for-line-update {
                    width: 5rem;
                    height: 1rem;
                    margin-bottom: 0.5rem;
                }

                .selected-start-point-number-for-line-update-info {
                    margin-top: 0.05rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.8rem;
                }

                .selected-start-point-number-for-line-update {
                    margin-bottom: 0.62rem;
                    margin-right: 0.5rem;
                    min-width: 3rem;
                    max-width: 5rem;
                }

                .line-number-update-end-point {
                    display: flex;
                    flex-direction: row;
                }

                .selected-end-point-number-for-line-update {
                    margin-bottom: 0.62rem;
                    margin-right: 0.5rem;
                    min-width: 3rem;
                    max-width: 5rem;
                }

                .selected-end-point-number-for-line-update-info {
                    margin-top: 0.05rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.8rem;
                }

                .search-end-point-number-for-line-update {
                    width: 5rem;
                    height: 1rem;
                    margin-bottom: 0.5rem;
                }

                .analysis-info {
                    margin: 0rem;
                    display: flex;
                }

                .line-update-message {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.9rem;
                    color: #ff0000;
                    max-width: 9rem;
                }

                .update-action-over-line-apply-cancel-buttons-container {
                    margin: 0rem;
                }

                .line-update-action-apply {
                    width: 5rem;
                    padding: 0rem;
                }

                .line-update-action-cancel {
                    width: 5rem;
                    padding: 0rem;
                }

                .delete-action-over-line-fields {
                    list-style-type: none;
                    padding: 0rem;
                    margin: 0rem;
                }

                .delete-action-over-line-fields-description {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.85rem;
                    width: 10.15rem;
                }


                .line-number-delete {
                    display: flex;
                    flex-direction: row;
                }

                .deleted-line-number {
                    margin-bottom: 0.62rem;
                    margin-right: 0.5rem;
                    min-width: 3rem;
                    max-width: 5rem;
                }

                .selected-line-number-for-delete-info {
                    margin-top: 0.05rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.8rem;
                }

                .search-line-number-for-delete {
                    width: 5rem;
                    height: 1rem;
                    margin-bottom: 0.5rem;
                }

                .analysis-info {
                    margin: 0rem;
                    display: flex;
                }

                .line-delete-message {
                    margin-top: 0rem;
                    margin-bottom: 0.05rem;
                    font-size: 0.9rem;
                    color: #ff0000;
                    max-width: 9rem;
                }

                .delete-action-over-line-apply-cancel-buttons-container {
                    margin: 0rem;
                }

                .line-delete-action-apply {
                    width: 5rem;
                    padding: 0rem;
                }

                .line-delete-action-cancel {
                    width: 5rem;
                    padding: 0rem;
                }

                .highlighted {
                    border: 2px solid #ff0000;
                }
            </style>

            <div class="wrapper">
                <hiding-content-button 
                    class=geometry
                    name=${this.state.buttonNames.geometry}
                    full-name=${this.state.buttonFullNames.geometry}
                    content-position=relative
                    content-direction=row
                    button-width=12rem
                    button-font-size=100%
                >
                    <hiding-content-button 
                        class=point
                        name=${this.state.buttonNames.point} 
                        full-name=${this.state.buttonFullNames.point}
                        content-position=absolute
                        content-direction=row
                        content-left=0rem
                        button-width=5.85rem
                        button-font-size=100%
                        button-margin-right=0.3rem
                    >
                        <hiding-content-button 
                            class=point-add
                            name=${this.state.buttonNames.pointAdd}
                            full-name=${this.state.buttonFullNames.pointAdd}
                            content-position=absolute
                            content-direction=column
                            content-left=0rem
                            button-width=3rem
                            button-font-size=85%
                            button-margin-right=0.25rem
                            content-background=#adadad
                            content-border="2px solid #737373"
                            content-padding=0.5rem
                        >
                            <ul class="add-action-over-point-fields">
                                <li>
                                    <p class="add-action-over-point-fields-description">New point number:</p>
                                    <input class="add-point-number" type="number" step="1"/>
                                </li>
                                <li>
                                    <p class="add-action-over-point-fields-description">X coordinate:</p>
                                    <input class="add-x-coord" type="number"/>
                                </li>
                                <li>
                                    <p class="add-action-over-point-fields-description">Y coordinate:</p>
                                    <input class="add-y-coord" type="number"/>
                                </li>
                                <li>
                                    <p class="add-action-over-point-fields-description">Z coordinate:</p>
                                    <input class="add-z-coord" type="number"/>
                                </li>
                            </ul>
                            <div class="analysis-info">
                                <p class="point-add-message"></p>
                            </div>
                            <div class="add-action-over-point-apply-cancel-buttons-container">
                                <button class="point-add-action-apply">Apply</button>
                                <button class="point-add-action-cancel">Cancel</button>
                            </div> 
                        </hiding-content-button>
                        <hiding-content-button 
                            class=point-update
                            name=${this.state.buttonNames.pointUpdate} 
                            full-name=${this.state.buttonFullNames.pointUpdate}
                            content-position=absolute
                            content-direction=column
                            content-left=0rem
                            button-width=4.5rem
                            button-font-size=85%
                            button-margin-right=0.25rem
                            content-background=#adadad
                            content-border="2px solid #737373"
                            content-padding=0.5rem
                        >
                            <ul class="update-action-over-point-fields">
                                <li>
                                    <p class="update-action-over-point-fields-description">Select point number:</p>
                                    <div class="point-number-update">
                                        <select class="updated-point-number" size="3"></select>
                                        <div class="select-and-search-number">  
                                            <input class="search-point-number-for-update" type="number" placeholder="Filter..."/>
                                            <p class="selected-point-number-for-update-info">You select: </p>
                                        </div>    
                                    </div>                    
                                </li>
                                <li>
                                    <p class="update-action-over-point-fields-description">X coordinate:</p>
                                    <input class="update-x-coord" type="number"/>
                                </li>
                                <li>
                                    <p class="update-action-over-point-fields-description">Y coordinate:</p>
                                    <input class="update-y-coord" type="number"/>
                                </li>
                                <li>
                                    <p class="update-action-over-point-fields-description">Z coordinate:</p>
                                    <input class="update-z-coord" type="number"/>
                                </li>
                            </ul>
                            <div class="analysis-info">
                                <p class="point-update-message"></p>
                            </div>
                            <div class="update-action-over-point-apply-cancel-buttons-container">
                                <button class="point-update-action-apply">Apply</button>
                                <button class="point-update-action-cancel">Cancel</button>
                            </div> 
                        </hiding-content-button>
                        <hiding-content-button
                            class=point-delete
                            name=${this.state.buttonNames.pointDelete} 
                            full-name=${this.state.buttonFullNames.pointDelete}
                            content-position=absolute 
                            content-direction=column
                            content-left=0rem
                            button-width=4rem
                            button-font-size=85%
                            content-background=#adadad
                            content-border="2px solid #737373"
                            content-padding=0.5rem
                        >
                            <ul class="delete-action-over-point-fields">
                                <li>
                                    <p class="delete-action-over-point-fields-description">Select point number:</p>
                                    <div class="point-number-delete">
                                        <select class="deleted-point-number" size="3"></select>
                                        <div class="select-and-search-number">  
                                            <input class="search-point-number-for-delete" type="number" placeholder="Filter..."/>
                                            <p class="selected-point-number-for-delete-info">You select: </p>
                                        </div>
                                    </div>
                                </li>
                            </ul>
                            <div class="analysis-info">
                                <p class="point-delete-message"></p>
                            </div>
                            <div class="delete-action-over-point-apply-cancel-buttons-container">
                                <button class="point-delete-action-apply">Apply</button>
                                <button class="point-delete-action-cancel">Cancel</button>
                            </div>                       
                        </hiding-content-button>
                    </hiding-content-button>
                    <hiding-content-button
                        class=line
                        name=${this.state.buttonNames.line}
                        full-name=${this.state.buttonFullNames.line}
                        content-position=absolute
                        content-direction=row
                        content-left=0rem
                        button-width=5.85rem
                        button-font-size=100%
                    >
                        <hiding-content-button 
                            class=line-add
                            name=${this.state.buttonNames.lineAdd}
                            full-name=${this.state.buttonFullNames.lineAdd}
                            content-position=absolute
                            content-direction=column
                            content-left=0rem
                            button-width=3rem
                            button-font-size=85%
                            button-margin-right=0.25rem
                            content-background=#adadad
                            content-border="2px solid #737373"
                            content-padding=0.5rem
                        >
                            <ul class="add-action-over-line-fields">
                                <li>
                                    <p class="add-action-over-line-fields-description">New line number:</p>
                                    <input class="add-line-number" type="number" step="1"/>
                                </li>
                                <li>
                                    <p class="add-action-over-line-fields-description">Select start point:</p>
                                    <div class="line-number-add-start-point">
                                        <select class="selected-start-point-number-for-line-addition" size="3"></select>
                                        <div class="select-and-search-number">  
                                            <input class="search-start-point-number-for-line-addition" type="number" placeholder="Filter..."/>
                                            <p class="selected-start-point-number-for-line-addition-info">You select: </p>
                                        </div>  
                                    </div>
                                </li>
                                <li>
                                    <p class="add-action-over-line-fields-description">Select end point:</p>
                                    <div class="line-number-add-end-point">
                                        <select class="selected-end-point-number-for-line-addition" size="3"></select>
                                        <div class="select-and-search-number">
                                            <input class="search-end-point-number-for-line-addition" type="number" placeholder="Filter..."/>
                                            <p class="selected-end-point-number-for-line-addition-info">You select: </p>
                                        </div>
                                    </div>
                                </li>
                            </ul>
                            <div class="analysis-info">
                                <p class="line-add-message"></p>
                            </div>
                            <div class="add-action-over-line-apply-cancel-buttons-container">
                                <button class="line-add-action-apply">Apply</button>
                                <button class="line-add-action-cancel">Cancel</button>
                            </div> 
                        </hiding-content-button>
                        <hiding-content-button 
                            class=line-update
                            name=${this.state.buttonNames.lineUpdate}
                            full-name=${this.state.buttonFullNames.lineUpdate}
                            content-position=absolute 
                            content-direction=column
                            content-left=0rem
                            button-width=4.5rem
                            button-font-size=85%
                            button-margin-right=0.25rem
                            content-background=#adadad
                            content-border="2px solid #737373"
                            content-padding=0.5rem
                        >
                            <ul class="update-action-over-line-fields">
                                <li>
                                    <p class="update-action-over-line-fields-description">Select line number:</p>
                                    <div class="line-number-update">
                                        <select class="updated-line-number" size="3"></select>
                                        <div class="select-and-search-number">
                                            <input class="search-line-number-for-update" type="number" placeholder="Filter..."/>
                                            <p class="selected-line-number-for-update-info">You select: </p>
                                        </div>
                                    </div>
                                </li>
                                <li>
                                    <p class="update-action-over-line-fields-description">Change line start point:</p>
                                    <div class="line-number-update-start-point">
                                        <select class="selected-start-point-number-for-line-update" size="3"></select>
                                        <div class="select-and-search-number">
                                            <input class="search-start-point-number-for-line-update" type="number" placeholder="Filter..."/>
                                            <p class="selected-start-point-number-for-line-update-info">You select: </p>
                                        </div>
                                    </div>
                                </li>
                                <li>
                                    <p class="update-action-over-line-fields-description">Change line end point:</p>
                                    <div class="line-number-update-end-point">
                                        <select class="selected-end-point-number-for-line-update" size="3"></select>
                                        <div class="select-and-search-number">
                                            <input class="search-end-point-number-for-line-update" type="number" placeholder="Filter..."/>
                                            <p class="selected-end-point-number-for-line-update-info">You select: </p>
                                        </div>
                                    </div>
                                </li>
                            </ul>
                            <div class="analysis-info">
                                <p class="line-update-message"></p>
                            </div>
                            <div class="update-action-over-line-apply-cancel-buttons-container">
                                <button class="line-update-action-apply">Apply</button>
                                <button class="line-update-action-cancel">Cancel</button>
                            </div> 
                        </hiding-content-button>
                        <hiding-content-button 
                            class=line-delete
                            name=${this.state.buttonNames.lineDelete}
                            full-name=${this.state.buttonFullNames.lineDelete}
                            content-position=absolute
                            content-direction=column 
                            content-left=0rem
                            button-width=4rem
                            button-font-size=85%
                            content-background=#adadad
                            content-border="2px solid #737373"
                            content-padding=0.5rem
                        >
                            <ul class="delete-action-over-line-fields">
                                <li>
                                    <p class="delete-action-over-line-fields-description">Select line number:</p>
                                    <div class="line-number-delete">
                                        <select class="deleted-line-number" size="3"></select>
                                        <div class="select-and-search-number">
                                            <input class="search-line-number-for-delete" type="number" placeholder="Filter..."/>
                                            <p class="selected-line-number-for-delete-info">You select: </p>
                                        </div>
                                    </div>
                                </li>
                            </ul>
                            <div class="analysis-info">
                                <p class="line-delete-message"></p>
                            </div>
                            <div class="delete-action-over-line-apply-cancel-buttons-container">
                                <button class="line-delete-action-apply">Apply</button>
                                <button class="line-delete-action-cancel">Cancel</button>
                            </div>
                        </hiding-content-button>
                    </hiding-content-button>
                </hiding-content-button> 
            </div>
        `;
        this.addEventListener("hide siblings", (event) => this.hideSiblings(event));

        this.addEventListener("update content height", (event) => this.updateContentHeight(event));

        this.shadowRoot.querySelector(".point-add-action-apply").addEventListener("click", () => this.addPoint());

        this.shadowRoot.querySelector(".point-add-action-cancel").addEventListener("click", () => this.cancelPointAddition());

        this.shadowRoot.querySelector(".updated-point-number").addEventListener("change", () => this.updatePointCoordinates());

        this.shadowRoot.querySelector(".search-point-number-for-update").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".search-point-number-for-update").value,
                this.shadowRoot.querySelector(".updated-point-number"));
        });

        this.shadowRoot.querySelector(".point-update-action-apply").addEventListener("click", () => this.updatePoint());

        this.shadowRoot.querySelector(".point-update-action-cancel").addEventListener("click", () => this.cancelPointUpdateOrDelete());

        this.shadowRoot.querySelector(".deleted-point-number").addEventListener("change", () => this.showSelectedPointNumberForDelete());

        this.shadowRoot.querySelector(".search-point-number-for-delete").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".search-point-number-for-delete").value,
                this.shadowRoot.querySelector(".deleted-point-number"));
        });

        this.shadowRoot.querySelector(".point-delete-action-apply").addEventListener("click", () => this.deletePoint());

        this.shadowRoot.querySelector(".point-delete-action-cancel").addEventListener("click", () => this.cancelPointUpdateOrDelete());

        this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition").addEventListener("change",
            () => this.showSelectedStartPointNumberForLineAddition());

        this.shadowRoot.querySelector(".search-start-point-number-for-line-addition").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".search-start-point-number-for-line-addition").value,
                this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition"));
        });

        this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition").addEventListener("change",
            () => this.showSelectedEndPointNumberForLineAddition());

        this.shadowRoot.querySelector(".search-end-point-number-for-line-addition").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".search-end-point-number-for-line-addition").value,
                this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition"));
        });

        this.shadowRoot.querySelector(".line-add-action-apply").addEventListener("click", () => this.addLine());

        this.shadowRoot.querySelector(".line-add-action-cancel").addEventListener("click", () => this.cancelLineAddition());

        this.shadowRoot.querySelector(".updated-line-number").addEventListener("change",
            () => this.showSelectedLineNumberForUpdate());

        this.shadowRoot.querySelector(".search-line-number-for-update").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".search-line-number-for-update").value,
                this.shadowRoot.querySelector(".updated-line-number"));
        });

        this.shadowRoot.querySelector(".selected-start-point-number-for-line-update").addEventListener("change",
            () => this.showSelectedStartPointNumberForLineUpdate());

        this.shadowRoot.querySelector(".search-start-point-number-for-line-update").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".search-start-point-number-for-line-update").value,
                this.shadowRoot.querySelector(".selected-start-point-number-for-line-update"));
        });

        this.shadowRoot.querySelector(".selected-end-point-number-for-line-update").addEventListener("change",
            () => this.showSelectedEndPointNumberForLineUpdate());

        this.shadowRoot.querySelector(".search-end-point-number-for-line-update").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".search-end-point-number-for-line-update").value,
                this.shadowRoot.querySelector(".selected-end-point-number-for-line-update"));
        });

        this.shadowRoot.querySelector(".line-update-action-apply").addEventListener("click", () => this.updateLine());

        this.shadowRoot.querySelector(".line-update-action-cancel").addEventListener("click", () => this.cancelLineUpdate());

        this.shadowRoot.querySelector(".deleted-line-number").addEventListener("change",
            () => this.showSelectedLineNumberForDelete());

        this.shadowRoot.querySelector(".search-line-number-for-delete").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".search-line-number-for-delete").value,
                this.shadowRoot.querySelector(".deleted-line-number"));
        });

        this.shadowRoot.querySelector(".line-delete-action-apply").addEventListener("click", () => this.deleteLine());

        this.shadowRoot.querySelector(".line-delete-action-cancel").addEventListener("click", () => this.cancelLineDelete());

        this.shadowRoot.querySelector(".add-point-number").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".add-point-number");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".point-add-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".add-x-coord").addEventListener("click", () => {
            const inputtedXField = this.shadowRoot.querySelector(".add-x-coord");
            this.dropHighlight(inputtedXField);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".point-add-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".add-y-coord").addEventListener("click", () => {
            const inputtedYField = this.shadowRoot.querySelector(".add-y-coord");
            this.dropHighlight(inputtedYField);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".point-add-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".add-z-coord").addEventListener("click", () => {
            const inputtedZField = this.shadowRoot.querySelector(".add-z-coord");
            this.dropHighlight(inputtedZField);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".point-add-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".updated-point-number").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".updated-point-number");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".point-update-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".point-update-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".update-x-coord").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".update-x-coord");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".point-update-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".point-update-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".update-y-coord").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".update-y-coord");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".point-update-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".point-update-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".update-z-coord").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".update-z-coord");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".point-update-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".point-update-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".deleted-point-number").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".deleted-point-number");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".point-delete-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".point-delete-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".add-line-number").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".add-line-number");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".line-add-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".line-add-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".line-add-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".updated-line-number").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".updated-line-number");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".line-update-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".line-update-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".selected-start-point-number-for-line-update").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".selected-start-point-number-for-line-update");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".line-update-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".line-update-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".selected-end-point-number-for-line-update").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".selected-end-point-number-for-line-update");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".line-update-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".line-update-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".deleted-line-number").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".deleted-line-number");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
                this.shadowRoot.querySelector(".line-delete-message").offsetHeight}px;`);
            this.shadowRoot.querySelector(".line-delete-message").innerHTML = "";
        });
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.refreshGeometryFields();
    }

    set addPointFromServer(point) {
        this.props.points.push(point);
        this.refreshGeometryFields();
    }

    set updatePointFromServer(point) {
        let pointInProps = this.props.points.find(existedPoint => existedPoint.number == point.number);
        pointInProps.x = point.x;
        pointInProps.y = point.y;
        pointInProps.z = point.z;
        this.refreshGeometryFields();
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

    refreshGeometryFields() {
        this.shadowRoot.querySelector(".point-update").disable = false;
        this.shadowRoot.querySelector(".point-delete").disable = false;
        this.shadowRoot.querySelector(".line").disable = false;
        this.shadowRoot.querySelector(".line-update").disable = false;
        this.shadowRoot.querySelector(".line-delete").disable = false;
        this.defineNewPointNumber();
        if (this.props.points.length !== 0) {
            this.defineUpdateAndDeletePointNumbers();
            if (this.props.points.length < 2) {
                this.shadowRoot.querySelector(".line").disable = true;
            } else {
                this.defineNewLineNumber();
                const lineStartPointNumberForAddition = this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition");
                const lineEndPointNumberForAddition = this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition");
                this.definePointNumbersForLine(lineStartPointNumberForAddition, lineEndPointNumberForAddition);
                const lineStartPointNumberForUpdate = this.shadowRoot.querySelector(".selected-start-point-number-for-line-update");
                const lineEndPointNumberForUpdate = this.shadowRoot.querySelector(".selected-end-point-number-for-line-update");
                this.definePointNumbersForLine(lineStartPointNumberForUpdate, lineEndPointNumberForUpdate);
                if (this.props.lines.length !== 0) {
                    this.defineUpdateAndDeleteLineNumbers();
                } else {
                    this.shadowRoot.querySelector(".line-update").disable = true;
                    this.shadowRoot.querySelector(".line-delete").disable = true;
                }
            }
        } else {
            this.shadowRoot.querySelector(".point-update").disable = true;
            this.shadowRoot.querySelector(".point-delete").disable = true;
            this.shadowRoot.querySelector(".line").disable = true;
        }
    }

    hideSiblings(event) {
        switch (event.detail.from) {
            case "geometry":
                this.shadowRoot.querySelector(".point").deactivate = "_true";
                this.shadowRoot.querySelector(".point-add").deactivate = "_true";
                this.shadowRoot.querySelector(".point-update").deactivate = "_true";
                this.shadowRoot.querySelector(".point-delete").deactivate = "_true";
                this.shadowRoot.querySelector(".line").deactivate = "_true";
                this.shadowRoot.querySelector(".line-add").deactivate = "_true";
                this.shadowRoot.querySelector(".line-update").deactivate = "_true";
                this.shadowRoot.querySelector(".line-delete").deactivate = "_true";
                event.stopPropagation();
                break;
            case "point":
                this.shadowRoot.querySelector(".line").deactivate = "_true";
                this.shadowRoot.querySelector(".line-add").deactivate = "_true";
                this.shadowRoot.querySelector(".line-update").deactivate = "_true";
                this.shadowRoot.querySelector(".line-delete").deactivate = "_true";
                event.stopPropagation();
                break;
            case "pointAdd":
                this.shadowRoot.querySelector(".point-update").deactivate = "_true";
                this.shadowRoot.querySelector(".point-delete").deactivate = "_true";
                event.stopPropagation();
                break;
            case "pointUpdate":
                this.shadowRoot.querySelector(".point-add").deactivate = "_true";
                this.shadowRoot.querySelector(".point-delete").deactivate = "_true";
                event.stopPropagation();
                break;
            case "pointDelete":
                this.shadowRoot.querySelector(".point-add").deactivate = "_true";
                this.shadowRoot.querySelector(".point-update").deactivate = "_true";
                event.stopPropagation();
                break;
            case "line":
                this.shadowRoot.querySelector(".point").deactivate = "_true";
                this.shadowRoot.querySelector(".point-add").deactivate = "_true";
                this.shadowRoot.querySelector(".point-update").deactivate = "_true";
                this.shadowRoot.querySelector(".point-delete").deactivate = "_true";
                event.stopPropagation();
                break;
            case "lineAdd":
                this.shadowRoot.querySelector(".line-update").deactivate = "_true";
                this.shadowRoot.querySelector(".line-delete").deactivate = "_true";
                event.stopPropagation();
                break;
            case "lineUpdate":
                this.shadowRoot.querySelector(".line-add").deactivate = "_true";
                this.shadowRoot.querySelector(".line-delete").deactivate = "_true";
                event.stopPropagation();
                break;
            case "lineDelete":
                this.shadowRoot.querySelector(".line-add").deactivate = "_true";
                this.shadowRoot.querySelector(".line-update").deactivate = "_true";
                event.stopPropagation();
                break;
            default:
                console.log("Sorry, we are out of button full names.");
        }
    }

    findContentHeight(event) {
        switch (event.detail.from) {
            case "pointAdd":
            case "pointUpdate":
            case "pointDelete":
            case "lineAdd":
            case "lineUpdate":
            case "lineDelete":
                return event.detail.height;
        }
        return 0;
    }

    updateContentHeight(event) {
        const additionalHeight = this.findContentHeight(event);
        const contentTotalHeight =
            this.shadowRoot.querySelector(".geometry").offsetHeight +
            this.shadowRoot.querySelector(".point-add").offsetHeight +
            this.shadowRoot.querySelector(".line-add").offsetHeight +
            this.shadowRoot.querySelector(".add-action-over-point-fields").offsetHeight +
            this.shadowRoot.querySelector(".add-action-over-point-apply-cancel-buttons-container").offsetHeight +
            this.shadowRoot.querySelector(".update-action-over-point-fields").offsetHeight +
            this.shadowRoot.querySelector(".update-action-over-point-apply-cancel-buttons-container").offsetHeight +
            this.shadowRoot.querySelector(".delete-action-over-point-fields").offsetHeight +
            this.shadowRoot.querySelector(".delete-action-over-point-apply-cancel-buttons-container").offsetHeight +
            this.shadowRoot.querySelector(".add-action-over-line-fields").offsetHeight +
            this.shadowRoot.querySelector(".add-action-over-line-apply-cancel-buttons-container").offsetHeight +
            this.shadowRoot.querySelector(".update-action-over-line-fields").offsetHeight +
            this.shadowRoot.querySelector(".update-action-over-line-apply-cancel-buttons-container").offsetHeight +
            this.shadowRoot.querySelector(".delete-action-over-line-fields").offsetHeight +
            this.shadowRoot.querySelector(".delete-action-over-line-apply-cancel-buttons-container").offsetHeight +
            additionalHeight;
        this.shadowRoot.querySelector(".wrapper").setAttribute("style", `height: ${contentTotalHeight}px;`);
    }

    defineNewPointNumber() {
        let newPointNumber = 0;
        const isPointNumberInArray = (point) => point.number === newPointNumber;
        do {
            newPointNumber += 1;
        } while (this.props.points.some(isPointNumberInArray));
        this.shadowRoot.querySelector(".add-point-number").value = newPointNumber;
        this.shadowRoot.querySelector(".add-point-number").min = newPointNumber;
        this.shadowRoot.querySelector(".add-x-coord").value = 0.0;
        this.shadowRoot.querySelector(".add-y-coord").value = 0.0;
        this.shadowRoot.querySelector(".add-z-coord").value = 0.0;
    }

    defineUpdateAndDeletePointNumbers() {
        const pointUpdateNumberSelect = this.shadowRoot.querySelector(".updated-point-number");
        const pointDeleteNumberSelect = this.shadowRoot.querySelector(".deleted-point-number");
        for (let i = pointUpdateNumberSelect.length - 1; i >= 0; i--) {
            pointUpdateNumberSelect.options[i] = null;
        }
        for (let i = pointDeleteNumberSelect.length - 1; i >= 0; i--) {
            pointDeleteNumberSelect.options[i] = null;
        }
        for (let i = 0; i < this.props.points.length; i++) {
            let updateOption = document.createElement("option");
            let deleteOption = document.createElement("option");
            updateOption.value = this.props.points[i].number;
            deleteOption.value = this.props.points[i].number;
            updateOption.innerHTML = `#${this.props.points[i].number}`;
            deleteOption.innerHTML = `#${this.props.points[i].number}`;
            pointUpdateNumberSelect.appendChild(updateOption);
            pointDeleteNumberSelect.appendChild(deleteOption);
        }
        this.shadowRoot.querySelector(".update-x-coord").value = this.props.points[0].x;
        this.shadowRoot.querySelector(".update-y-coord").value = this.props.points[0].y;
        this.shadowRoot.querySelector(".update-z-coord").value = this.props.points[0].z;
    }

    filter(keywordField, selectField) {
        for (let i = 0; i < selectField.length; i++) {
            let txt = selectField.options[i].value;
            if (txt.substring(0, keywordField.length).toLowerCase() !== keywordField.toLowerCase() && keywordField.trim() !== "") {
                selectField.options[i].style.display = "none";
            } else {
                selectField.options[i].style.display = "list-item";
            }
        }
    }

    updatePointCoordinates() {
        const selectedPointNumber = this.shadowRoot.querySelector(".updated-point-number").value;
        this.shadowRoot.querySelector(".selected-point-number-for-update-info").innerHTML = `You select: ${selectedPointNumber}`;
        const pointInProps = this.props.points.find(point => point.number == selectedPointNumber);
        this.shadowRoot.querySelector(".update-x-coord").value = pointInProps.x;
        this.dropHighlight(this.shadowRoot.querySelector(".update-x-coord"));
        this.shadowRoot.querySelector(".update-y-coord").value = pointInProps.y;
        this.dropHighlight(this.shadowRoot.querySelector(".update-y-coord"));
        this.shadowRoot.querySelector(".update-z-coord").value = pointInProps.z;
        this.dropHighlight(this.shadowRoot.querySelector(".update-z-coord"));
    }

    addPoint() {
        const newPointNumberField = this.shadowRoot.querySelector(".add-point-number");
        if (newPointNumberField.value === "") {
            if (newPointNumberField.classList.contains("highlighted") === false) {
                newPointNumberField.classList.add("highlighted");
            }
        }
        const inputtedXField = this.shadowRoot.querySelector(".add-x-coord");
        if (inputtedXField.value === "") {
            if (inputtedXField.classList.contains("highlighted") === false) {
                inputtedXField.classList.add("highlighted");
            }
        }
        const inputtedYField = this.shadowRoot.querySelector(".add-y-coord");
        if (inputtedYField.value === "") {
            if (inputtedYField.classList.contains("highlighted") === false) {
                inputtedYField.classList.add("highlighted");
            }
        }
        const inputtedZField = this.shadowRoot.querySelector(".add-z-coord");
        if (inputtedZField.value === "") {
            if (inputtedZField.classList.contains("highlighted") === false) {
                inputtedZField.classList.add("highlighted");
            }
        }
        if (newPointNumberField.value === "" || inputtedXField.value === "" || inputtedYField.value === "" || inputtedZField.value === "") {
            this.shadowRoot.querySelector(".point-add-message").innerHTML = "The highlighted fields should be filled!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
            return;
        }
        const pointNumberInProps = this.props.points.find(point => point.number == newPointNumberField.value);
        if (pointNumberInProps != null) {
            this.shadowRoot.querySelector(".point-add-message").innerHTML = "The point with the same number does already exist!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
            return;
        }
        const pointCoordinatesInProps = this.props.points.find(point => point.x == inputtedXField.value && point.y == inputtedYField.value &&
            point.z == inputtedZField.value);
        if (pointCoordinatesInProps != null) {
            this.shadowRoot.querySelector(".point-add-message").innerHTML = "The point with the same coordinates does already exist!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
            return;
        }
        const message = JSON.stringify({"add_point": {
                "actionId": this.props.actionId,
                "number": newPointNumberField.value, 
                "x":  inputtedXField.value, "y":  inputtedYField.value, "z": inputtedZField.value
            }});
        this.dispatchEvent(new CustomEvent("client message", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }

    cancelPointAddition() {
        this.defineNewPointNumber();
        this.shadowRoot.querySelector(".wrapper").setAttribute("style",
            `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
            this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
        const newPointNumberField = this.shadowRoot.querySelector(".add-point-number");
        this.dropHighlight(newPointNumberField);
        const inputtedXField = this.shadowRoot.querySelector(".add-x-coord");
        this.dropHighlight(inputtedXField);
        const inputtedYField = this.shadowRoot.querySelector(".add-y-coord");
        this.dropHighlight(inputtedYField);
        const inputtedZField = this.shadowRoot.querySelector(".add-z-coord");
        this.dropHighlight(inputtedZField);
        this.shadowRoot.querySelector(".point-add-message").innerHTML = "";
    }

    updatePoint() {
        const selectedPointNumberField = this.shadowRoot.querySelector(".updated-point-number");
        if (selectedPointNumberField.value == "") {
            if (selectedPointNumberField.classList.contains("highlighted") === false) {
                selectedPointNumberField.classList.add("highlighted");
            }
        }
        const inputtedXField = this.shadowRoot.querySelector(".update-x-coord");
        if (inputtedXField.value == "") {
            if (inputtedXField.classList.contains("highlighted") === false) {
                inputtedXField.classList.add("highlighted");
            }
        }
        const inputtedYField = this.shadowRoot.querySelector(".update-y-coord");
        if (inputtedYField.value == "") {
            if (inputtedYField.classList.contains("highlighted") === false) {
                inputtedYField.classList.add("highlighted");
            }
        }
        const inputtedZField = this.shadowRoot.querySelector(".update-z-coord");
        if (inputtedZField.value == "") {
            if (inputtedZField.classList.contains("highlighted") === false) {
                inputtedZField.classList.add("highlighted");
            }
        }
        if (selectedPointNumberField.value === "" || inputtedXField.value === "" || inputtedYField.value === "" || inputtedZField.value === "") {
            this.shadowRoot.querySelector(".point-update-message").innerHTML = "The highlighted fields should be filled!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".point-update-message").offsetHeight}px;`);
            return;
        }
        const pointCoordinatesInProps = this.props.points.find(point => point.x == inputtedXField.value && point.y == inputtedYField.value &&
            point.z == inputtedZField.value);
        if (pointCoordinatesInProps != null) {
            this.shadowRoot.querySelector(".point-update-message").innerHTML = "The point with the same coordinates does already exist!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".point-update-message").offsetHeight}px;`);
            return;
        }
        const oldPointValues = this.props.points.find(point => point.number == selectedPointNumberField.value);
        const message = JSON.stringify({"update_point": {
            "actionId": this.props.actionId,
            "number": selectedPointNumberField.value, 
            "old_point_values": { "x":  oldPointValues.x, "y": oldPointValues.y, "z": oldPointValues.z },
            "new_point_values": { "x": inputtedXField.value, "y": inputtedYField.value, "z": inputtedZField.value }
        }});
        this.dispatchEvent(new CustomEvent("client message", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }

    showSelectedPointNumberForDelete() {
        const selectedPointNumber = this.shadowRoot.querySelector(".deleted-point-number").value;
        this.shadowRoot.querySelector(".selected-point-number-for-delete-info").innerHTML = `You select: ${selectedPointNumber}`;
    }

    deletePoint() {
        const selectedPointNumberField = this.shadowRoot.querySelector(".deleted-point-number");
        if (selectedPointNumberField.value == "") {
            if (selectedPointNumberField.classList.contains("highlighted") === false) {
                selectedPointNumberField.classList.add("highlighted");
            }
            this.shadowRoot.querySelector(".point-delete-message").innerHTML = "The highlighted fields should be filled!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".point-delete-message").offsetHeight}px;`);
            return;
        }
        const message = JSON.stringify({"delete_point": {
            "actionId": this.props.actionId,
            "number": selectedPointNumberField.value,
        }});
        this.dispatchEvent(new CustomEvent("client message", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }

    cancelPointUpdateOrDelete() {
        this.defineUpdateAndDeletePointNumbers();
        this.shadowRoot.querySelector(".search-point-number-for-update").value = null;
        this.shadowRoot.querySelector(".search-point-number-for-delete").value = null;
        const selectedPointNumberForUpdateField = this.shadowRoot.querySelector(".updated-point-number");
        this.dropHighlight(selectedPointNumberForUpdateField);
        const inputtedXField = this.shadowRoot.querySelector(".update-x-coord");
        this.dropHighlight(inputtedXField);
        const inputtedYField = this.shadowRoot.querySelector(".update-y-coord");
        this.dropHighlight(inputtedYField);
        const inputtedZField = this.shadowRoot.querySelector(".update-z-coord");
        this.dropHighlight(inputtedZField);
        const selectedPointNumberForDeleteField = this.shadowRoot.querySelector(".deleted-point-number");
        this.dropHighlight(selectedPointNumberForDeleteField);
        this.shadowRoot.querySelector(".selected-point-number-for-update-info").innerHTML = "You select:";
        this.shadowRoot.querySelector(".selected-point-number-for-delete-info").innerHTML = "You select:";
        this.shadowRoot.querySelector(".wrapper").setAttribute("style",
            `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
            this.shadowRoot.querySelector(".point-update-message").offsetHeight}px;`);
        this.shadowRoot.querySelector(".point-update-message").innerHTML = "";
        this.shadowRoot.querySelector(".wrapper").setAttribute("style",
            `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
            this.shadowRoot.querySelector(".point-delete-message").offsetHeight}px;`);
        this.shadowRoot.querySelector(".point-delete-message").innerHTML = "";
    }

    defineNewLineNumber() {
        let newLineNumber = 0;
        const isLineNumberInArray = (line) => line.number === newLineNumber;
        do {
            newLineNumber += 1;
        } while (this.props.lines.some(isLineNumberInArray));
        this.shadowRoot.querySelector(".add-line-number").value = newLineNumber;
        this.shadowRoot.querySelector(".add-line-number").min = newLineNumber;
    }

    definePointNumbersForLine(startPointNumberSelector, endPointNumberSelector) {
        for (let i = startPointNumberSelector.length - 1; i >= 0; i--) {
            startPointNumberSelector.options[i] = null;
        }
        for (let i = endPointNumberSelector.length - 1; i >= 0; i--) {
            endPointNumberSelector.options[i] = null;
        }
        for (let i = 0; i < this.props.points.length; i++) {
            let updateOption = document.createElement("option");
            let deleteOption = document.createElement("option");
            updateOption.value = this.props.points[i].number;
            deleteOption.value = this.props.points[i].number;
            updateOption.innerHTML = `#${this.props.points[i].number}`;
            deleteOption.innerHTML = `#${this.props.points[i].number}`;
            startPointNumberSelector.appendChild(updateOption);
            endPointNumberSelector.appendChild(deleteOption);
        }
    }

    defineUpdateAndDeleteLineNumbers() {
        const lineUpdateNumberSelect = this.shadowRoot.querySelector(".updated-line-number");
        const lineDeleteNumberSelect = this.shadowRoot.querySelector(".deleted-line-number");
        for (let i = lineUpdateNumberSelect.length - 1; i >= 0; i--) {
            lineUpdateNumberSelect.options[i] = null;
        }
        for (let i = lineDeleteNumberSelect.length - 1; i >= 0; i--) {
            lineDeleteNumberSelect.options[i] = null;
        }
        for (let i = 0; i < this.props.lines.length; i++) {
            let updateOption = document.createElement("option");
            let deleteOption = document.createElement("option");
            updateOption.value = this.props.lines[i].number;
            deleteOption.value = this.props.lines[i].number;
            updateOption.innerHTML = `#${this.props.lines[i].number}`;
            deleteOption.innerHTML = `#${this.props.lines[i].number}`;
            lineUpdateNumberSelect.appendChild(updateOption);
            lineDeleteNumberSelect.appendChild(deleteOption);
        }
    }

    showSelectedStartPointNumberForLineAddition() {
        const selectedPointNumber = this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition").value;
        this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition-info").innerHTML = `You select: ${selectedPointNumber}`;
    }

    showSelectedEndPointNumberForLineAddition() {
        const selectedPointNumber = this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition").value;
        this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition-info").innerHTML = `You select: ${selectedPointNumber}`;
    }

    addLine() {
        const newLineNumberField = this.shadowRoot.querySelector(".add-line-number");
        if (newLineNumberField.value == "") {
            if (newLineNumberField.classList.contains("highlighted") === false) {
                newLineNumberField.classList.add("highlighted");
            }
        }
        const startPointField = this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition");
        if (startPointField.value == "") {
            if (startPointField.classList.contains("highlighted") === false) {
                startPointField.classList.add("highlighted");
            }
        }
        const endPointField = this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition");
        if (endPointField.value == "") {
            if (endPointField.classList.contains("highlighted") === false) {
                endPointField.classList.add("highlighted");
            }
        }
        if (newLineNumberField.value == "" || startPointField.value == "" || endPointField.value == "") {
            this.shadowRoot.querySelector(".line-add-message").innerHTML = "The highlighted fields should be filled!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
            return;
        }
        const lineNumberInProps = this.props.lines.find(line => line.number == newLineNumberField.value);
        if (lineNumberInProps != null) {
            this.shadowRoot.querySelector(".line-add-message").innerHTML = "The line with the same number does already exist!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
            return;
        }
        const linePointNumbersInProps = this.props.lines.find(line =>
            (line.startPoint == startPointField.value && line.endPoint == endPointField.value) ||
            (line.startPoint == endPointField.value && line.endPoint == startPointField.value));
        if (linePointNumbersInProps != null) {
            this.shadowRoot.querySelector(".line-add-message").innerHTML = "The line with the same start and end points does already exist!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
            return;
        }
        if (startPointField.value === endPointField.value) {
            this.shadowRoot.querySelector(".line-add-message").innerHTML = "The start and the end point numbers should not be the same!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
            return;
        }
        const message = JSON.stringify({"add_line": {
            "actionId": this.props.actionId,
            "number": newLineNumberField.value, 
            "start_point_number": startPointField.value, "end_point_number": endPointField.value
        }});
        this.dispatchEvent(new CustomEvent("client message", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }

    cancelLineAddition() {
        this.defineNewLineNumber();
        const startPoint = this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition");
        const endPoint = this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition");
        this.definePointNumbersForLine(startPoint, endPoint);
        this.shadowRoot.querySelector(".search-start-point-number-for-line-addition").value = null;
        this.shadowRoot.querySelector(".search-end-point-number-for-line-addition").value = null;
        this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition-info").innerHTML = "You select:"
        this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition-info").innerHTML = "You select:"
        const newLineNumberField = this.shadowRoot.querySelector(".add-line-number");
        this.dropHighlight(newLineNumberField);
        const startPointField = this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition");
        this.dropHighlight(startPointField);
        const endPointField = this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition");
        this.dropHighlight(endPointField);
        this.shadowRoot.querySelector(".wrapper").setAttribute("style",
            `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
            this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
        this.shadowRoot.querySelector(".line-add-message").innerHTML = "";
    }

    showSelectedLineNumberForUpdate() {
        const selectedPointNumber = this.shadowRoot.querySelector(".updated-line-number").value;
        this.shadowRoot.querySelector(".selected-line-number-for-update-info").innerHTML = `You select: ${selectedPointNumber}`;
    }

    showSelectedStartPointNumberForLineUpdate() {
        const selectedPointNumber = this.shadowRoot.querySelector(".selected-start-point-number-for-line-update").value;
        this.shadowRoot.querySelector(".selected-start-point-number-for-line-update-info").innerHTML = `You select: ${selectedPointNumber}`;
    }

    showSelectedEndPointNumberForLineUpdate() {
        const selectedPointNumber = this.shadowRoot.querySelector(".selected-end-point-number-for-line-update").value;
        this.shadowRoot.querySelector(".selected-end-point-number-for-line-update-info").innerHTML = `You select: ${selectedPointNumber}`;
    }

    updateLine() {
        const selectedLineNumberField = this.shadowRoot.querySelector(".updated-line-number");
        if (selectedLineNumberField.value == "") {
            if (selectedLineNumberField.classList.contains("highlighted") === false) {
                selectedLineNumberField.classList.add("highlighted");
            }
        }
        const startPointField = this.shadowRoot.querySelector(".selected-start-point-number-for-line-update");
        if (startPointField.value == "") {
            if (startPointField.classList.contains("highlighted") === false) {
                startPointField.classList.add("highlighted");
            }
        }
        const endPointField = this.shadowRoot.querySelector(".selected-end-point-number-for-line-update");
        if (endPointField.value == "") {
            if (endPointField.classList.contains("highlighted") === false) {
                endPointField.classList.add("highlighted");
            }
        }
        if (selectedLineNumberField.value == "" || startPointField.value == "" || endPointField.value == "") {
            this.shadowRoot.querySelector(".line-update-message").innerHTML = "The highlighted fields should be filled!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".line-update-message").offsetHeight}px;`);
            return;
        }
        const linePointNumbersInProps = this.props.lines.find(line =>
            (line.startPoint == startPointField.value && line.endPoint == endPointField.value) ||
            (line.startPoint == endPointField.value && line.endPoint == startPointField.value));
        if (linePointNumbersInProps != null) {
            this.shadowRoot.querySelector(".line-update-message").innerHTML = "The line with the same start and end points does already exist!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".line-update-message").offsetHeight}px;`);
            return;
        }
        if (startPointField.value === endPointField.value) {
            this.shadowRoot.querySelector(".line-update-message").innerHTML = "The start and the end point numbers should not be the same!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".line-update-message").offsetHeight}px;`);
            return;
        }
        const oldLineValues = this.props.lines.find(line => line.number == selectedLineNumberField.value);
        const message = JSON.stringify({"update_line": {
            "actionId": this.props.actionId,
            "number": selectedLineNumberField.value, 
            "old_line_values": { "start_point":  oldLineValues.startPoint, "end_point": oldLineValues.endPoint },
            "new_line_values": { "start_point":  startPointField.value, "end_point": endPointField.value }
        }});
        this.dispatchEvent(new CustomEvent("client message", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }

    cancelLineUpdate() {
        this.defineUpdateAndDeleteLineNumbers();
        this.shadowRoot.querySelector(".search-line-number-for-update").value = null;
        const startPoint = this.shadowRoot.querySelector(".selected-start-point-number-for-line-update");
        const endPoint = this.shadowRoot.querySelector(".selected-end-point-number-for-line-update");
        this.definePointNumbersForLine(startPoint, endPoint);
        this.shadowRoot.querySelector(".search-start-point-number-for-line-update").value = null;
        this.shadowRoot.querySelector(".search-end-point-number-for-line-update").value = null;
        this.shadowRoot.querySelector(".selected-line-number-for-update-info").innerHTML = "You select:";
        this.shadowRoot.querySelector(".selected-start-point-number-for-line-update-info").innerHTML = "You select:";
        this.shadowRoot.querySelector(".selected-end-point-number-for-line-update-info").innerHTML = "You select:";
        const selectedLineNumberField = this.shadowRoot.querySelector(".updated-line-number");
        this.dropHighlight(selectedLineNumberField);
        const startPointField = this.shadowRoot.querySelector(".selected-start-point-number-for-line-update");
        this.dropHighlight(startPointField);
        const endPointField = this.shadowRoot.querySelector(".selected-end-point-number-for-line-update");
        this.dropHighlight(endPointField);
        this.shadowRoot.querySelector(".wrapper").setAttribute("style",
            `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
            this.shadowRoot.querySelector(".line-update-message").offsetHeight}px;`);
        this.shadowRoot.querySelector(".line-update-message").innerHTML = "";
    }

    showSelectedLineNumberForDelete() {
        const selectedPointNumber = this.shadowRoot.querySelector(".deleted-line-number").value;
        this.shadowRoot.querySelector(".selected-line-number-for-delete-info").innerHTML = `You select: ${selectedPointNumber}`;
    }

    deleteLine() {
        const selectedLineNumberField = this.shadowRoot.querySelector(".deleted-line-number");
        if (selectedLineNumberField.value == "") {
            if (selectedLineNumberField.classList.contains("highlighted") === false) {
                selectedLineNumberField.classList.add("highlighted");
            }
            this.shadowRoot.querySelector(".line-delete-message").innerHTML = "The highlighted fields should be filled!";
            this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                this.shadowRoot.querySelector(".line-delete-message").offsetHeight}px;`);
            return;
        }
        const message = JSON.stringify({"delete_line": {
            "actionId": this.props.actionId,
            "number": selectedLineNumberField.value, 
        }});
        this.dispatchEvent(new CustomEvent("client message", {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }

    cancelLineDelete() {
        this.defineUpdateAndDeleteLineNumbers();
        this.shadowRoot.querySelector(".search-line-number-for-delete").value = null;
        this.shadowRoot.querySelector(".selected-line-number-for-delete-info").innerHTML = "You select:";
        const selectedLineNumberField = this.shadowRoot.querySelector(".deleted-line-number");
        this.dropHighlight(selectedLineNumberField);
        this.shadowRoot.querySelector(".wrapper").setAttribute("style",
            `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
            this.shadowRoot.querySelector(".line-delete-message").offsetHeight}px;`);
        this.shadowRoot.querySelector(".line-delete-message").innerHTML = "";
    }

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }
}

export default FeaGeometry;
