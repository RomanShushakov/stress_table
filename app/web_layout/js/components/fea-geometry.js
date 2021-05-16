class FeaGeometry extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
            points: [ ],
            lines: [ ]
        };

        this.state = {
            buttonFullNames: {
                geometry: "geometry", point: "point", line: "line",
                addPoint: "addPoint", updatePoint: "updatePoint", deletePoint: "deletePoint",
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
                    background-color: #2e3440;
                    display: flex;
                    position: relative;
                }

                .geometry-menu-button {
                    margin: 0rem;
                    padding-top: 0.7rem;
                    padding-bottom: 0.7rem;
                    background: #2e3440;
                    border: #3b4453;
                }

                .geometry-menu-button:hover {
                    background: #2d303b;
                }

                .geometry-menu-button:hover .geometry-menu-button-icon {
                    color: #2d303b;
                }

                .active .geometry-menu-button-icon {
                    color: #3b4453;
                }

                .active:hover {
                    background: #242932;
                }

                .active:hover .geometry-menu-button-icon {
                    color: #242932;
                }

                .geometry-menu-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                }

                .geometry-menu-button-icon {
                    color: #2E3440;
                    width: 3rem;
                    height: 3rem;
                }

                .geometry-menu-button-icon-caption {
                    color: #D9D9D9;
                    margin: 0rem;
                    padding: 0rem;
                    width: 3rem;
                    font-size: 85%;
                }








                .geometry-menu-content {
                    display: flex;
                    background-color: #3b4453;
                    padding: 1rem;
                    flex-direction: column;
                    position: absolute;
                    margin-left: 3.7rem;
                }

                .geometry-menu-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    height: 1.5rem;
                    border-bottom: 0.1rem solid #4a5060;
                    font-size: 85%;
                }

                .geometry-menu-buttons-content {
                    margin-top: 1rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    margin-bottom: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    border-bottom: 0.1rem solid #4a5060;
                }

                .geometry-menu-buttons-caption {
                    margin: 0rem;
                    padding: 0rem;
                    width: 4rem;
                    color: #D9D9D9;
                    font-size: 85%;
                }

                .geometry-menu-buttons {
                    display: flex;
                    flex-direction: row;
                    margin: 0rem;
                    padding: 0rem;
                }

                .point-button {
                    margin-left: 0.5rem;
                }

                .point-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                }

                .point-button-icon {
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .point-button:hover .point-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .point-button-icon {
                    color: #8bbee4;
                }

                .active .point-button-icon {
                    color: #72C5FF;
                }

                .active:hover .point-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .point-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }



                

                .line-button {
                    margin-left: 0.5rem;
                }

                .line-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                }

                .line-button-icon {
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .line-button:hover .line-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .line-button-icon {
                    color: #8bbee4;
                }

                .active .line-button-icon {
                    color: #72C5FF;
                }

                .active:hover .line-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .line-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }










                .point-menu-buttons-content {
                    margin-top: 1rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    margin-bottom: 1rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    border-bottom: 0.1rem solid #4a5060;
                }

                .point-menu-buttons-caption {
                    margin: 0rem;
                    padding: 0rem;
                    width: 4rem;
                    color: #D9D9D9;
                    font-size: 85%;
                }

                .point-menu-buttons {
                    display: flex;
                    flex-direction: row;
                    margin: 0rem;
                    padding: 0rem;
                }



                .add-point-button {
                    margin-left: 0.5rem;
                }

                .add-point-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                }

                .add-point-button-icon {
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .add-point-button:hover .add-point-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .add-point-button-icon {
                    color: #8bbee4;
                }

                .active .add-point-button-icon {
                    color: #72C5FF;
                }

                .active:hover .add-point-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .add-point-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }



                .update-point-button {
                    margin-left: 0.5rem;
                }

                .update-point-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                }

                .update-point-button-icon {
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .update-point-button:hover .update-point-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .update-point-button-icon {
                    color: #8bbee4;
                }

                .active .update-point-button-icon {
                    color: #72C5FF;
                }

                .active:hover .update-point-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .update-point-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
                }


                .delete-point-button {
                    margin-left: 0.5rem;
                }

                .delete-point-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    width: 1.5rem;
                    height: 1.5rem;
                }

                .delete-point-button-icon {
                    width: 1.5rem;
                    height: 1.5rem;
                    color: #D9D9D9;
                }

                .delete-point-button:hover .delete-point-button-icon {
                    color: #d1d2d7;
                }

                .active:hover .delete-point-button-icon {
                    color: #8bbee4;
                }

                .active .delete-point-button-icon {
                    color: #72C5FF;
                }

                .active:hover .delete-point-button-icon-content {
                    border-bottom: 0.15rem solid #8bbee4;
                }

                .active .delete-point-button-icon-content {
                    border-bottom: 0.15rem solid #72C5FF;
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

                .active {
                    background: #3b4453;
                }

                .hidden {
                    display: none;
                }
            </style>

            <div class=wrapper>
                <button class="geometry-menu-button">
                    <div class="geometry-menu-button-icon-content">
                        <svg class=geometry-menu-button-icon width="100" height="100" viewBox="0 0 100 100" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Geometry</title>
                            <g fill="currentColor">
                                <path d="M6.77588 74.5L44.4645 1.0947L82.1532 74.5H6.77588Z" stroke="#D9D9D9"/>
                                <rect x="52.5" y="29.5" width="40" height="42" stroke="#72C5FF"/>
                                <circle cx="77.5" cy="65.5" r="22" stroke="#D9D9D9"/>
                                <path d="M33.8771 87.5L21.5761 66L33.8771 44.5H58.4554L69.4385 66L58.4554 
                                    87.5H33.8771Z" stroke="#72C5FF"
                                />
                            </g>
                        </svg>
                        <p class="geometry-menu-button-icon-caption">Geometry</p>
                    </div>
                </button>
                <div class="geometry-menu-content hidden">
                    <p class="geometry-menu-caption">Geometry</p>
                    <div class="geometry-menu-buttons-content">
                        <p class="geometry-menu-buttons-caption">Point</p>
                        <div class="geometry-menu-buttons">
                            <hiding-content-button 
                                class="point-button"
                                full-name=${this.state.buttonFullNames.point}
                                button-default-background=#3b4453
                                button-active-background=#3b4453
                                button-hover-background=#3b4453
                                content-background=#3b4453
                                content-position=absolute
                                content-direction=row
                                content-left=0rem
                                content-top=6rem
                                button-width=1.7rem
                                button-font-size=100%
                            >
                                <div slot="icon-content" class="point-button-icon-content">
                                    <svg class="point-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                                        xmlns="http://www.w3.org/2000/svg"
                                    >
                                        <title>Point</title>
                                        <g stroke="currentColor">
                                            <rect x="15" y="15" width="5" height="5"/>
                                        </g>
                                    </svg>
                                </div>


                                <div slot="content" class="point-menu-buttons-content">
                                    <p class="point-menu-buttons-caption">Add</p>
                                    <div class="point-menu-buttons">
                                        <hiding-content-button 
                                            class=add-point-button
                                            full-name=${this.state.buttonFullNames.addPoint}
                                            button-default-background=#3b4453
                                            button-active-background=#3b4453
                                            button-hover-background=#3b4453
                                            content-background=#3b4453
                                            content-position=absolute
                                            content-direction=row
                                            content-left=0rem
                                            content-top=0rem
                                            button-width=1.7rem
                                            button-font-size=100%
                                        >
                                            <div slot="icon-content" class="add-point-button-icon-content">
                                                <svg class="add-point-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                                                    xmlns="http://www.w3.org/2000/svg"
                                                >  
                                                    <title>Add point</title>
                                                    <g stroke="currentColor">
                                                        <line x1="7" y1="17.5" x2="28" y2="17.5"/>
                                                        <line x1="17.5" y1="7" x2="17.5" y2="28"/>
                                                    </g>
                                                </svg>
                                            </div>
                                        </hiding-content-button>

                                        <hiding-content-button 
                                            class=update-point-button
                                            full-name=${this.state.buttonFullNames.updatePoint}
                                            button-default-background=#3b4453
                                            button-active-background=#3b4453
                                            button-hover-background=#3b4453
                                            content-background=#3b4453
                                            content-position=absolute
                                            content-direction=row
                                            content-left=0rem
                                            button-width=1.7rem
                                            button-font-size=100%
                                        >
                                            <div slot="icon-content" class="update-point-button-icon-content">
                                                <svg class="update-point-button-icon" width="36" height="35" viewBox="0 0 36 35" fill="none" 
                                                    xmlns="http://www.w3.org/2000/svg"
                                                >
                                                    <title>Update point</title>
                                                    <g stroke="currentColor">
                                                        <path d="M17.2055 8L16.9178 11.4247L14.9041 12.1096L12.3151 9.91781L10.3014 
                                                            11.8356L12.4589 14.4384L11.7397 16.2192L8 16.4932V19.2329L11.5959 
                                                            19.5068L12.4589 21.2877L10.0137 23.8904L12.0274 25.8082L14.7603 
                                                            23.6164L16.6301 24.4384L16.9178 28H19.7945L20.0822 24.5753L22.2397 
                                                            23.7534L25.1164 26.0822L27.1301 24.1644L24.5411 21.4247L25.4041 
                                                            19.6438L29 19.3699V16.7671L25.4041 16.4932L24.6849 14.5753L27.1301 
                                                            11.8356L25.1164 9.91781L22.2397 12.2466L20.3699 11.5616L20.0822 
                                                            8H17.2055Z"
                                                        />
                                                        <circle cx="18.5" cy="17.5" r="3.5"/>
                                                        <path d="M34 18.5C34 20.4042 33.6249 22.2897 32.8963 24.0489C32.1676 
                                                            25.8081 31.0995 27.4066 29.753 28.753C28.4066 30.0995 26.8081 
                                                            31.1676 25.0489 31.8963C23.2897 32.6249 21.4042 33 19.5 33"
                                                        />
                                                        <path d="M32.9962 20L34 17.3919L35.0038 20H32.9962Z"/>
                                                        <path d="M2.5 16.5C2.5 14.5958 2.87505 12.7103 3.60375 10.9511C4.33244 
                                                            9.19187 5.4005 7.5934 6.74695 6.24695C8.0934 4.9005 9.69187 
                                                            3.83244 11.4511 3.10375C13.2103 2.37505 15.0958 2 17 2"
                                                        />
                                                        <path d="M3.50384 15L2.5 17.6081L1.49616 15H3.50384Z"/>
                                                    </g>
                                                </svg>
                                            </div>
                                        </hiding-content-button>

                                        <hiding-content-button 
                                            class=delete-point-button
                                            full-name=${this.state.buttonFullNames.deletePoint}
                                            button-default-background=#3b4453
                                            button-active-background=#3b4453
                                            button-hover-background=#3b4453
                                            content-background=#3b4453
                                            content-position=absolute
                                            content-direction=row
                                            content-left=0rem
                                            button-width=1.7rem
                                            button-font-size=100%
                                        >
                                            <div slot="icon-content" class="delete-point-button-icon-content">
                                                <svg class="delete-point-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                                                    xmlns="http://www.w3.org/2000/svg"
                                                >
                                                    <title>Delete point</title>
                                                    <g stroke="currentColor">
                                                        <path d="M7.00184 9.01157C7.00087 9.0055 7.00557 9 7.01172 
                                                            9H27.9885C27.9946 9 27.9993 9.00535 27.9985 9.01135L25.0012 
                                                            30.9914C25.0005 30.9963 24.9963 31 24.9913 31H10.5085C10.5036 
                                                            31 10.4994 30.9964 10.4987 30.9916L7.00184 9.01157Z"
                                                        />
                                                        <rect x="14.5" y="4.5" width="6" height="3" rx="0.5"/>
                                                        <line x1="5" y1="8" x2="30" y2="8" stroke-width="2"/>
                                                        <line x1="11.4942" y1="12.924" x2="13.4942" y2="25.924"/>
                                                        <line x1="23.4942" y1="13.076" x2="21.4942" y2="26.076"/>
                                                        <line x1="17.5" y1="13" x2="17.5" y2="26"/>
                                                    </g>
                                                </svg>


                                            </div>
                                        </hiding-content-button>

                                    </div>
                                </div>
                            </hiding-content-button>







                            <hiding-content-button 
                                class=line-button
                                full-name=${this.state.buttonFullNames.line}
                                button-default-background=#3b4453
                                button-active-background=#3b4453
                                button-hover-background=#3b4453
                                content-background=#3b4453
                                content-position=absolute
                                content-direction=row
                                content-left=0rem
                                button-width=1.7rem
                                button-font-size=100%
                            >
                                <div slot="icon-content" class="line-button-icon-content">
                                    <svg class="line-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                                        xmlns="http://www.w3.org/2000/svg"
                                    >
                                        <title>Line</title>
                                        <g stroke="currentColor">
                                            <rect x="28" y="2" width="5" height="5"/>
                                            <rect x="3" y="27" width="5" height="5"/>
                                            <line x1="8.64645" y1="26.6464" x2="27.6464" y2="7.64645"/>
                                        </g>
                                    </svg>
                                </div>
                            </hiding-content-button>

                        </div>
                    </div>
                </div>
                    <!-- <div slot="icon-content" class="geometry-button-icon-content">
                            <svg slot="icon-content" class=geometry-button-icon width="100" height="100" viewBox="0 0 100 100" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Geometry</title>
                                <g fill="currentColor">
                                    <path d="M6.77588 74.5L44.4645 1.0947L82.1532 74.5H6.77588Z" stroke="#D9D9D9"/>
                                    <rect x="52.5" y="29.5" width="40" height="42" stroke="#72C5FF"/>
                                    <circle cx="77.5" cy="65.5" r="22" stroke="#D9D9D9"/>
                                    <path d="M33.8771 87.5L21.5761 66L33.8771 44.5H58.4554L69.4385 66L58.4554 
                                        87.5H33.8771Z" stroke="#72C5FF"
                                    />
                                </g>
                            </svg>
                        </div>
                        <hiding-content-button
                            slot=content 
                            class=point-add
                            full-name=${this.state.buttonFullNames.pointAdd}
                            button-default-background=#2e3440
                            button-active-background=#3b4453
                            button-hover-background=#2d303b
                            content-background=#3b4453
                            content-position=absolute
                            content-direction=column
                            content-left=0rem
                            button-width=3rem
                            button-font-size=85%
                            button-margin-right=0.25rem
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
                        full-name=${this.state.buttonFullNames.line}
                        content-position=absolute
                        content-direction=row
                        content-left=3rem
                        button-width=5.85rem
                        button-font-size=100%
                    >
                        <hiding-content-button 
                            class=line-add
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
                    </hiding-content-button> -->
            </div>
        `;

        this.shadowRoot.querySelector(".geometry-menu-button").addEventListener("click", (event) => this.toggle(event));

        this.addEventListener("activate menu", (event) => this.activateMenu(event));

        this.addEventListener("hide siblings", (event) => this.hideSiblings(event));

        // this.addEventListener("update content height", (event) => this.updateContentHeight(event));

        // this.shadowRoot.querySelector(".point-add-action-apply").addEventListener("click", () => this.addPoint());

        // this.shadowRoot.querySelector(".point-add-action-cancel").addEventListener("click", () => this.cancelPointAddition());

        // this.shadowRoot.querySelector(".updated-point-number").addEventListener("change", () => this.updatePointCoordinates());

        // this.shadowRoot.querySelector(".search-point-number-for-update").addEventListener("keyup", () => {
        //     this.filter(
        //         this.shadowRoot.querySelector(".search-point-number-for-update").value,
        //         this.shadowRoot.querySelector(".updated-point-number"));
        // });

        // this.shadowRoot.querySelector(".point-update-action-apply").addEventListener("click", () => this.updatePoint());

        // this.shadowRoot.querySelector(".point-update-action-cancel").addEventListener("click", () => this.cancelPointUpdateOrDelete());

        // this.shadowRoot.querySelector(".deleted-point-number").addEventListener("change", () => this.showSelectedPointNumberForDelete());

        // this.shadowRoot.querySelector(".search-point-number-for-delete").addEventListener("keyup", () => {
        //     this.filter(
        //         this.shadowRoot.querySelector(".search-point-number-for-delete").value,
        //         this.shadowRoot.querySelector(".deleted-point-number"));
        // });

        // this.shadowRoot.querySelector(".point-delete-action-apply").addEventListener("click", () => this.deletePoint());

        // this.shadowRoot.querySelector(".point-delete-action-cancel").addEventListener("click", () => this.cancelPointUpdateOrDelete());

        // this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition").addEventListener("change",
        //     () => this.showSelectedStartPointNumberForLineAddition());

        // this.shadowRoot.querySelector(".search-start-point-number-for-line-addition").addEventListener("keyup", () => {
        //     this.filter(
        //         this.shadowRoot.querySelector(".search-start-point-number-for-line-addition").value,
        //         this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition"));
        // });

        // this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition").addEventListener("change",
        //     () => this.showSelectedEndPointNumberForLineAddition());

        // this.shadowRoot.querySelector(".search-end-point-number-for-line-addition").addEventListener("keyup", () => {
        //     this.filter(
        //         this.shadowRoot.querySelector(".search-end-point-number-for-line-addition").value,
        //         this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition"));
        // });

        // this.shadowRoot.querySelector(".line-add-action-apply").addEventListener("click", () => this.addLine());

        // this.shadowRoot.querySelector(".line-add-action-cancel").addEventListener("click", () => this.cancelLineAddition());

        // this.shadowRoot.querySelector(".updated-line-number").addEventListener("change",
        //     () => this.showSelectedLineNumberForUpdate());

        // this.shadowRoot.querySelector(".search-line-number-for-update").addEventListener("keyup", () => {
        //     this.filter(
        //         this.shadowRoot.querySelector(".search-line-number-for-update").value,
        //         this.shadowRoot.querySelector(".updated-line-number"));
        // });

        // this.shadowRoot.querySelector(".selected-start-point-number-for-line-update").addEventListener("change",
        //     () => this.showSelectedStartPointNumberForLineUpdate());

        // this.shadowRoot.querySelector(".search-start-point-number-for-line-update").addEventListener("keyup", () => {
        //     this.filter(
        //         this.shadowRoot.querySelector(".search-start-point-number-for-line-update").value,
        //         this.shadowRoot.querySelector(".selected-start-point-number-for-line-update"));
        // });

        // this.shadowRoot.querySelector(".selected-end-point-number-for-line-update").addEventListener("change",
        //     () => this.showSelectedEndPointNumberForLineUpdate());

        // this.shadowRoot.querySelector(".search-end-point-number-for-line-update").addEventListener("keyup", () => {
        //     this.filter(
        //         this.shadowRoot.querySelector(".search-end-point-number-for-line-update").value,
        //         this.shadowRoot.querySelector(".selected-end-point-number-for-line-update"));
        // });

        // this.shadowRoot.querySelector(".line-update-action-apply").addEventListener("click", () => this.updateLine());

        // this.shadowRoot.querySelector(".line-update-action-cancel").addEventListener("click", () => this.cancelLineUpdate());

        // this.shadowRoot.querySelector(".deleted-line-number").addEventListener("change",
        //     () => this.showSelectedLineNumberForDelete());

        // this.shadowRoot.querySelector(".search-line-number-for-delete").addEventListener("keyup", () => {
        //     this.filter(
        //         this.shadowRoot.querySelector(".search-line-number-for-delete").value,
        //         this.shadowRoot.querySelector(".deleted-line-number"));
        // });

        // this.shadowRoot.querySelector(".line-delete-action-apply").addEventListener("click", () => this.deleteLine());

        // this.shadowRoot.querySelector(".line-delete-action-cancel").addEventListener("click", () => this.cancelLineDelete());

        // this.shadowRoot.querySelector(".add-point-number").addEventListener("click", () => {
        //     const highlightedElement = this.shadowRoot.querySelector(".add-point-number");
        //     this.dropHighlight(highlightedElement);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".point-add-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".add-x-coord").addEventListener("click", () => {
        //     const inputtedXField = this.shadowRoot.querySelector(".add-x-coord");
        //     this.dropHighlight(inputtedXField);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".point-add-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".add-y-coord").addEventListener("click", () => {
        //     const inputtedYField = this.shadowRoot.querySelector(".add-y-coord");
        //     this.dropHighlight(inputtedYField);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".point-add-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".add-z-coord").addEventListener("click", () => {
        //     const inputtedZField = this.shadowRoot.querySelector(".add-z-coord");
        //     this.dropHighlight(inputtedZField);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".point-add-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".updated-point-number").addEventListener("click", () => {
        //     const highlightedElement = this.shadowRoot.querySelector(".updated-point-number");
        //     this.dropHighlight(highlightedElement);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".point-update-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".point-update-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".update-x-coord").addEventListener("click", () => {
        //     const highlightedElement = this.shadowRoot.querySelector(".update-x-coord");
        //     this.dropHighlight(highlightedElement);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".point-update-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".point-update-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".update-y-coord").addEventListener("click", () => {
        //     const highlightedElement = this.shadowRoot.querySelector(".update-y-coord");
        //     this.dropHighlight(highlightedElement);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".point-update-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".point-update-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".update-z-coord").addEventListener("click", () => {
        //     const highlightedElement = this.shadowRoot.querySelector(".update-z-coord");
        //     this.dropHighlight(highlightedElement);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".point-update-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".point-update-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".deleted-point-number").addEventListener("click", () => {
        //     const highlightedElement = this.shadowRoot.querySelector(".deleted-point-number");
        //     this.dropHighlight(highlightedElement);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".point-delete-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".point-delete-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".add-line-number").addEventListener("click", () => {
        //     const highlightedElement = this.shadowRoot.querySelector(".add-line-number");
        //     this.dropHighlight(highlightedElement);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".line-add-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition").addEventListener("click", () => {
        //     const highlightedElement = this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition");
        //     this.dropHighlight(highlightedElement);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".line-add-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition").addEventListener("click", () => {
        //     const highlightedElement = this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition");
        //     this.dropHighlight(highlightedElement);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".line-add-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".updated-line-number").addEventListener("click", () => {
        //     const highlightedElement = this.shadowRoot.querySelector(".updated-line-number");
        //     this.dropHighlight(highlightedElement);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".line-update-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".line-update-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".selected-start-point-number-for-line-update").addEventListener("click", () => {
        //     const highlightedElement = this.shadowRoot.querySelector(".selected-start-point-number-for-line-update");
        //     this.dropHighlight(highlightedElement);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".line-update-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".line-update-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".selected-end-point-number-for-line-update").addEventListener("click", () => {
        //     const highlightedElement = this.shadowRoot.querySelector(".selected-end-point-number-for-line-update");
        //     this.dropHighlight(highlightedElement);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".line-update-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".line-update-message").innerHTML = "";
        // });

        // this.shadowRoot.querySelector(".deleted-line-number").addEventListener("click", () => {
        //     const highlightedElement = this.shadowRoot.querySelector(".deleted-line-number");
        //     this.dropHighlight(highlightedElement);
        //     this.shadowRoot.querySelector(".wrapper").setAttribute("style",
        //         `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight -
        //         this.shadowRoot.querySelector(".line-delete-message").offsetHeight}px;`);
        //     this.shadowRoot.querySelector(".line-delete-message").innerHTML = "";
        // });
    }

    set actionId(value) {
        this.props.actionId = value;
    }

    set addPointToClient(point) {
        this.props.points.push(point);
        this.props.points.sort((a, b) => a.number - b.number);
        this.refreshGeometryFields();
    }

    set updatePointInClient(point) {
        let pointInProps = this.props.points.find(existedPoint => existedPoint.number == point.number);
        pointInProps.x = point.x;
        pointInProps.y = point.y;
        pointInProps.z = point.z;
        this.refreshGeometryFields();
    }

    set deletePointFromClient(point) {
        let pointIndexInProps = this.props.points.findIndex(existedPoint => existedPoint.number == point.number);
        this.props.points.splice(pointIndexInProps, 1);
        this.props.points.sort((a, b) => a.number - b.number);
        this.refreshGeometryFields();
    }

    set addLineToClient(line) {
        this.props.lines.push(line);
        this.props.lines.sort((a, b) => a.number - b.number);
        this.refreshGeometryFields();
    }

    set updateLineInClient(line) {
        let lineInProps = this.props.lines.find(existedLine => existedLine.number == line.number);
        lineInProps.startPointNumber = line.startPointNumber;
        lineInProps.endPointNumber = line.endPointNumber;
        this.refreshGeometryFields();
    }

    set deleteLineFromClient(line) {
        let lineIndexInProps = this.props.lines.findIndex(existedLine => existedLine.number == line.number);
        this.props.lines.splice(lineIndexInProps, 1);
        this.props.lines.sort((a, b) => a.number - b.number);
        this.refreshGeometryFields();
    }

    set close(_empty) {
        this.shadowRoot.querySelector(".geometry").close = "_empty";
        this.refreshGeometryFields();
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

    disconnectedCallback() {
    }

    static get observedAttributes() {
        return [];
    }

    attributeChangedCallback(name, oldValue, newValue) {
    }

    adoptedCallback() {
    }


    activateMenu(event) {
        switch (event.detail.from) {
            case "point":
                if (this.shadowRoot.querySelector(".point-button").classList.contains("active") === false) {
                    this.shadowRoot.querySelector(".point-button").classList.add("active");
                    this.shadowRoot.querySelector(".geometry-menu-buttons-caption").innerHTML = "Point";
                    if (this.shadowRoot.querySelector(".line-button").classList.contains("active") === true) {
                        this.shadowRoot.querySelector(".line-button").classList.remove("active")
                    }
                }
                event.stopPropagation();
                break;
            case "line":
                if (this.shadowRoot.querySelector(".line-button").classList.contains("active") === false) {
                    this.shadowRoot.querySelector(".line-button").classList.add("active");
                    this.shadowRoot.querySelector(".geometry-menu-buttons-caption").innerHTML = "Line";
                    if (this.shadowRoot.querySelector(".point-button").classList.contains("active") === true) {
                        this.shadowRoot.querySelector(".point-button").classList.remove("active")
                    }
                }
                event.stopPropagation();
                break;
            case "properties":
                this.shadowRoot.querySelector("fea-geometry").close = "_empty";
                event.stopPropagation();
                break;
        }
    }

    toggle() {
        const content = this.shadowRoot.querySelector(".geometry-menu-content");
        const button = this.shadowRoot.querySelector(".geometry-menu-button");
        if (content.classList.contains("hidden") === false) {
            content.classList.add("hidden");
            button.classList.remove("active");
        } else {
            content.classList.remove("hidden");
            button.classList.add("active");
            // this.hideSiblings();
            // this.menuOpen();
        }
        // this.updateContentHeight();
        const additionalWidth = button.offsetWidth + content.offsetWidth;
        this.dispatchEvent(new CustomEvent("update preprocessor menu width", {
            bubbles: true,
            composed: true,
            detail: {"additionalWidth": additionalWidth},
        }));
    }

    refreshGeometryFields() {
        // this.shadowRoot.querySelector(".point-update").disable = false;
        // this.shadowRoot.querySelector(".point-delete").disable = false;
        // this.shadowRoot.querySelector(".line").disable = false;
        // this.shadowRoot.querySelector(".line-update").disable = false;
        // this.shadowRoot.querySelector(".line-delete").disable = false;
        // this.defineNewPointNumber();
        // if (this.props.points.length !== 0) {
        //     this.defineUpdateAndDeletePointNumbers();
        //     if (this.props.points.length < 2) {
        //         this.shadowRoot.querySelector(".line").disable = true;
        //     } else {
        //         this.defineNewLineNumber();
        //         const lineStartPointNumberForAddition = this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition");
        //         const lineEndPointNumberForAddition = this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition");
        //         this.definePointNumbersForLine(lineStartPointNumberForAddition, lineEndPointNumberForAddition);
        //         const lineStartPointNumberForUpdate = this.shadowRoot.querySelector(".selected-start-point-number-for-line-update");
        //         const lineEndPointNumberForUpdate = this.shadowRoot.querySelector(".selected-end-point-number-for-line-update");
        //         this.definePointNumbersForLine(lineStartPointNumberForUpdate, lineEndPointNumberForUpdate);
        //         if (this.props.lines.length !== 0) {
        //             this.defineUpdateAndDeleteLineNumbers();
        //         } else {
        //             this.shadowRoot.querySelector(".line-update").disable = true;
        //             this.shadowRoot.querySelector(".line-delete").disable = true;
        //         }
        //     }
        // } else {
        //     this.shadowRoot.querySelector(".point-update").disable = true;
        //     this.shadowRoot.querySelector(".point-delete").disable = true;
        //     this.shadowRoot.querySelector(".line").disable = true;
        // }
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
            // case "point":
            //     this.shadowRoot.querySelector(".line").deactivate = "_true";
            //     this.shadowRoot.querySelector(".line-add").deactivate = "_true";
            //     this.shadowRoot.querySelector(".line-update").deactivate = "_true";
            //     this.shadowRoot.querySelector(".line-delete").deactivate = "_true";
            //     event.stopPropagation();
            //     break;
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
            // case "line":
            //     this.shadowRoot.querySelector(".point").deactivate = "_true";
            //     this.shadowRoot.querySelector(".point-add").deactivate = "_true";
            //     this.shadowRoot.querySelector(".point-update").deactivate = "_true";
            //     this.shadowRoot.querySelector(".point-delete").deactivate = "_true";
            //     event.stopPropagation();
            //     break;
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
            if (this.shadowRoot.querySelector(".point-add-message").innerHTML === "") {
                this.shadowRoot.querySelector(".point-add-message").innerHTML = "The highlighted fields should be filled!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
                return;
            }
        }
        const pointNumberInProps = this.props.points.find(point => point.number == newPointNumberField.value);
        if (pointNumberInProps != null) {
            if (this.shadowRoot.querySelector(".point-add-message").innerHTML === "") {
                this.shadowRoot.querySelector(".point-add-message").innerHTML = "The point with the same number does already exist!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
                return;
            }
        }
        const pointCoordinatesInProps = this.props.points.find(point => point.x == inputtedXField.value && point.y == inputtedYField.value &&
            point.z == inputtedZField.value);
        if (pointCoordinatesInProps != null) {
            if (this.shadowRoot.querySelector(".point-add-message").innerHTML === "") {
                this.shadowRoot.querySelector(".point-add-message").innerHTML = "The point with the same coordinates does already exist!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".point-add-message").offsetHeight}px;`);
                return;
            } else {
                return;
            }
        }
        const message = {"add_point": {
            "actionId": this.props.actionId,
            "number": newPointNumberField.value, 
            "x":  inputtedXField.value, "y":  inputtedYField.value, "z": inputtedZField.value
        }};

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
            if (this.shadowRoot.querySelector(".point-update-message").innerHTML === "") {
                this.shadowRoot.querySelector(".point-update-message").innerHTML = "The highlighted fields should be filled!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".point-update-message").offsetHeight}px;`);
                return;
            } else {
                return;
            }
        }
        const pointCoordinatesInProps = this.props.points.find(point => point.x == inputtedXField.value && point.y == inputtedYField.value &&
            point.z == inputtedZField.value);
        if (pointCoordinatesInProps != null) {
            if (this.shadowRoot.querySelector(".point-update-message").innerHTML === "") {
                this.shadowRoot.querySelector(".point-update-message").innerHTML = "The point with the same coordinates does already exist!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".point-update-message").offsetHeight}px;`);
                return;
            } else {
                return;
            }
        }
        const oldPointValues = this.props.points.find(point => point.number == selectedPointNumberField.value);
        const message = {"update_point": {
            "actionId": this.props.actionId,
            "number": selectedPointNumberField.value, 
            "old_point_values": { "x":  oldPointValues.x, "y": oldPointValues.y, "z": oldPointValues.z },
            "new_point_values": { "x": inputtedXField.value, "y": inputtedYField.value, "z": inputtedZField.value }
        }};
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
            if (this.shadowRoot.querySelector(".point-delete-message").innerHTML === "") {
                this.shadowRoot.querySelector(".point-delete-message").innerHTML = "The highlighted fields should be filled!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".point-delete-message").offsetHeight}px;`);
                return;
            } else {
                return;
            }
        }
        const deletedPointValues = this.props.points.find(point => point.number == selectedPointNumberField.value);
        const message = {"delete_point": { "actionId": this.props.actionId, "number": deletedPointValues.number }};
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
            if (this.shadowRoot.querySelector(".line-add-message").innerHTML === "") {
                this.shadowRoot.querySelector(".line-add-message").innerHTML = "The highlighted fields should be filled!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
                return;
            } else {
                return;
            }
        }
        const lineNumberInProps = this.props.lines.find(line => line.number == newLineNumberField.value);
        if (lineNumberInProps != null) {
            if (this.shadowRoot.querySelector(".line-add-message").innerHTML === "") {
                this.shadowRoot.querySelector(".line-add-message").innerHTML = "The line with the same number does already exist!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
                return;
            } else {
                return;
            }
        }
        const linePointNumbersInProps = this.props.lines.find(line =>
            (line.startPointNumber == startPointField.value && line.endPointNumber == endPointField.value) ||
            (line.startPointNumber == endPointField.value && line.endPointNumber == startPointField.value));
        if (linePointNumbersInProps != null) {
            if (this.shadowRoot.querySelector(".line-add-message").innerHTML === "") {
                this.shadowRoot.querySelector(".line-add-message").innerHTML = "The line with the same start and end points does already exist!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
                return;
            } else {
                return;
            }
        }
        if (startPointField.value === endPointField.value) {
            if (this.shadowRoot.querySelector(".line-add-message").innerHTML === "") {
                this.shadowRoot.querySelector(".line-add-message").innerHTML = "The start and the end point numbers should not be the same!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".line-add-message").offsetHeight}px;`);
                return;
            } else {
                return;
            }
        }
        const message = {"add_line": {
            "actionId": this.props.actionId,
            "number": newLineNumberField.value, 
            "start_point_number": startPointField.value, "end_point_number": endPointField.value
        }};
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
        const startPointNumber = this.shadowRoot.querySelector(".selected-start-point-number-for-line-addition");
        const endPointNumber = this.shadowRoot.querySelector(".selected-end-point-number-for-line-addition");
        this.definePointNumbersForLine(startPointNumber, endPointNumber);
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
            if (this.shadowRoot.querySelector(".line-update-message").innerHTML === "") {
                this.shadowRoot.querySelector(".line-update-message").innerHTML = "The highlighted fields should be filled!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".line-update-message").offsetHeight}px;`);
                return;
            } else {
                return;
            }
        }
        const linePointNumbersInProps = this.props.lines.find(line =>
            (line.startPointNumber == startPointField.value && line.endPointNumber == endPointField.value) ||
            (line.startPointNumber == endPointField.value && line.endPointNumber == startPointField.value));
        if (linePointNumbersInProps != null) {
            if (this.shadowRoot.querySelector(".line-update-message").innerHTML === "") {
                this.shadowRoot.querySelector(".line-update-message").innerHTML = "The line with the same start and end points does already exist!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".line-update-message").offsetHeight}px;`);
                return;
            } else {
                return;
            }
        }
        if (startPointField.value === endPointField.value) {
            if (this.shadowRoot.querySelector(".line-update-message").innerHTML === "") {
                this.shadowRoot.querySelector(".line-update-message").innerHTML = "The start and the end point numbers should not be the same!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".line-update-message").offsetHeight}px;`);
                return;
            } else {
                return;
            }
        }
        const oldLineValues = this.props.lines.find(line => line.number == selectedLineNumberField.value);
        const message = {"update_line": {
            "actionId": this.props.actionId,
            "number": selectedLineNumberField.value, 
            "old_line_values": { "start_point":  oldLineValues.startPointNumber, "end_point": oldLineValues.endPointNumber },
            "new_line_values": { "start_point":  startPointField.value, "end_point": endPointField.value }
        }};
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
        const startPointNumber = this.shadowRoot.querySelector(".selected-start-point-number-for-line-update");
        const endPointNumber = this.shadowRoot.querySelector(".selected-end-point-number-for-line-update");
        this.definePointNumbersForLine(startPointNumber, endPointNumber);
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
            if (this.shadowRoot.querySelector(".line-delete-message").innerHTML === "") {
                this.shadowRoot.querySelector(".line-delete-message").innerHTML = "The highlighted fields should be filled!";
                this.shadowRoot.querySelector(".wrapper").setAttribute("style",
                    `height: ${this.shadowRoot.querySelector(".wrapper").offsetHeight +
                    this.shadowRoot.querySelector(".line-delete-message").offsetHeight}px;`);
                return;
            } else {
                return;
            }
        }
        const message = {"delete_line": {
            "actionId": this.props.actionId,
            "number": selectedLineNumberField.value, 
        }};
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
