class FeaAnalysisMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            isFEModelLoaded: false,     // load status of wasm module "fe_model";
            jobNames: [],               // array of: [String, ...];
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
                    padding: 1rem;
                }

                .analysis-menu-caption {
                    margin: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0.3rem;
                    padding-left: 0rem;
                    padding-right: 0rem;
                    color: #D9D9D9;
                    border-bottom: 0.1rem solid #4a5060;
                    font-size: 85%;
                }

                .new-job-name-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                }

                .new-job-name-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .new-job-name {
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

                .new-job-name:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .new-job-name:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .new-job-buttons {
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    align-self: center;
                }

                .check-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 5rem;
                    height: 1.7rem;
                }

                .check-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .submit-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 5.5rem;
                    height: 1.7rem;
                }

                .submit-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .new-job-info {
                    display: flex;
                    margin: 0rem;
                    padding: 0rem;
                    flex-direction: column;
                    align-items: center;
                }

                .new-job-info-message {
                    margin-top: 1rem;
                    margin-bottom: 0.5rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 80%;
                    width: 12rem;
                }

                .new-job-hide-message-button {
                    background: #3d5d78;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 6rem;
                    height: 1.7rem;
                    font-size: 70%;
                }

                .new-job-hide-message-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .new-job-yes-button {
                    background: #3d5d78;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 4rem;
                    height: 1.7rem;
                    font-size: 70%;
                }

                .new-job-yes-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .new-job-no-button {
                    background: #3d5d78;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 4rem;
                    height: 1.7rem;
                    font-size: 70%;
                }

                .new-job-no-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .job-name-field-content {
                    display: flex;
                    flex-direction: row;
                    background-color: #3b4453;
                    padding: 0rem;
                    margin-top: 5rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    align-items: center;
                }

                .job-name-caption {
                    margin: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 85%;
                    width: 6rem;
                }

                .job-name-select-filter-content {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 1rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    display: flex;
                    flex-direction: column;
                }

                .job-name-filter-label {
                    position: relative;
                }
                  
                .job-name-filter-label:before {
                    content: "";
                    position: absolute;
                    left: 0rem;
                    top: 0rem;
                    bottom: 0rem;
                    width: 0.8rem;
                    background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
                }

                .job-name-filter {
                    margin-top: 0rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding-left: 1.3rem;
                    width: 3.5rem;
                    background-color: #3b4453;
                    border: #4a5060;
                    border-bottom: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                }

                .job-name-filter::placeholder {
                    font-size: 85%;
                }

                .job-name-filter:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .job-name-filter:focus {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .job-name {
                    width: 5rem;
                    margin-top: 0.5rem;
                    background-color: #3b4453;
                    border: #4a5060;
                    border-bottom: 0.1rem solid #4a5060;
                    outline: none;
                    color: #D9D9D9;
                    -webkit-appearance: none;
                    -moz-appearance: none;
                    background: url('data:image/svg+xml,<svg width="4" height="4" viewBox="0 0 4 4" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M1 1L2 2L3 1" stroke="rgb(112, 112, 114)" stroke-width="0.5"/></svg>') right / contain no-repeat;
                }

                .job-name option {
                    background-color: #484f60;
                }

                .job-name:hover {
                    box-shadow: 0rem 0.15rem 0rem #4a5060;
                }

                .job-buttons {
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    align-self: center;
                }

                .show-result-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 5.5rem;
                    height: 1.7rem;
                }

                .show-result-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .delete-job-button {
                    background: #0996d7;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 5.5rem;
                    height: 1.7rem;
                }

                .delete-job-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .job-info {
                    display: flex;
                    margin: 0rem;
                    padding: 0rem;
                    flex-direction: column;
                    align-items: center;
                }

                .job-info-message {
                    margin-top: 1rem;
                    margin-bottom: 0.5rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    color: #D9D9D9;
                    font-size: 80%;
                    width: 12rem;
                }

                .job-hide-message-button {
                    background: #3d5d78;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 6rem;
                    height: 1.7rem;
                    font-size: 70%;
                }

                .job-hide-message-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .job-yes-button {
                    background: #3d5d78;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 4rem;
                    height: 1.7rem;
                    font-size: 70%;
                }

                .job-yes-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .job-no-button {
                    background: #3d5d78;
                    border: 0.2rem solid #3b4453;
                    border-radius: 0.3rem;
                    color: #D9D9D9;
                    padding: 0rem;
                    margin: 0rem;
                    width: 4rem;
                    height: 1.7rem;
                    font-size: 70%;
                }

                .job-no-button:hover {
                    border: 0.2rem solid #4a5060;
                }

                .highlighted {
                    box-shadow: 0rem 0.1rem 0rem #72C5FF;
                }
            </style>
            <div class=wrapper>
                <p class="analysis-menu-caption">Analysis</p>

                <div class="new-job-name-field-content">
                    <p class="new-job-name-caption">New job name</p>
                    <input class="new-job-name" type="text"/>
                </div>

                <div class="new-job-buttons">
                    <button class="submit-button">Submit</button>
                </div>

                <div class="new-job-info">
                    <p class="new-job-info-message"></p>
                </div>

                <div class="job-name-field-content">
                    <p class="job-name-caption">Job name</p>
                    <div class="job-name-select-filter-content">
                        <label class="job-name-filter-label">
                            <input class="job-name-filter" type="text" placeholder="Filter..."/>
                        </label>
                        <select class="job-name"></select>
                    </div>
                </div>

                <div class="job-buttons">
                    <button class="show-result-button">Show result</button>
                    <button class="delete-job-button">Delete job</button>
                </div>

                <div class="job-info">
                    <p class="job-info-message"></p>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".new-job-name").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".new-job-name");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".new-job-info-message").innerHTML = "";
            if (this.shadowRoot.querySelector(".new-job-hide-message-button") != undefined) {
                this.shadowRoot.querySelector(".new-job-hide-message-button").remove();
            }
            if (this.shadowRoot.querySelector(".new-job-yes-button") != undefined) {
                this.shadowRoot.querySelector(".new-job-yes-button").remove();
            }
            if (this.shadowRoot.querySelector(".new-job-no-button") != undefined) {
                this.shadowRoot.querySelector(".new-job-no-button").remove();
            }
        });

        this.shadowRoot.querySelector(".submit-button").addEventListener("click", () => this.submitJob());

        this.shadowRoot.querySelector(".job-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".job-name-filter").value,
                this.shadowRoot.querySelector(".job-name"));
        });
        this.shadowRoot.querySelector(".show-result-button").addEventListener("click", () => this.showJobAnalysisResult());
        this.shadowRoot.querySelector(".delete-job-button").addEventListener("click", () => this.deleteJob());
    }

    set isFEModelLoaded(value) {
        this.props.isFEModelLoaded = value;
    }

    set jobNames(value) {
        this.props.jobNames = value;
        this.props.jobNames.sort((a, b) => a.name - b.name);
    }

    set submitJobError(error) {
        this.shadowRoot.querySelector(".new-job-info-message").innerHTML = error;
        if (this.shadowRoot.querySelector(".new-job-hide-message-button") == undefined) {
            const newJobHideMessageButton = document.createElement("button");
            newJobHideMessageButton.className = "new-job-hide-message-button";
            newJobHideMessageButton.innerHTML = "Hide message";
            newJobHideMessageButton.addEventListener("click", () => {
                this.shadowRoot.querySelector(".new-job-info-message").innerHTML = "";
                if (this.shadowRoot.querySelector(".new-job-hide-message-button") != undefined) {
                    this.shadowRoot.querySelector(".new-job-hide-message-button").remove();
                }
                if (this.shadowRoot.querySelector(".new-job-yes-button") != undefined) {
                    this.shadowRoot.querySelector(".new-job-yes-button").remove();
                }
                if (this.shadowRoot.querySelector(".new-job-no-button") != undefined) {
                    this.shadowRoot.querySelector(".new-job-no-button").remove();
                }
                this.dropHighlight(this.shadowRoot.querySelector(".new-job-name"));
            });
            this.shadowRoot.querySelector(".new-job-info").append(newJobHideMessageButton);
        }
    }

    set submitJobSuccess(message) {
        this.shadowRoot.querySelector(".new-job-info-message").innerHTML = message;
        if (this.shadowRoot.querySelector(".new-job-hide-message-button") == undefined) {
            const newJobHideMessageButton = document.createElement("button");
            newJobHideMessageButton.className = "new-job-hide-message-button";
            newJobHideMessageButton.innerHTML = "Hide message";
            newJobHideMessageButton.addEventListener("click", () => {
                this.shadowRoot.querySelector(".new-job-info-message").innerHTML = "";
                if (this.shadowRoot.querySelector(".new-job-hide-message-button") != undefined) {
                    this.shadowRoot.querySelector(".new-job-hide-message-button").remove();
                }
                if (this.shadowRoot.querySelector(".new-job-yes-button") != undefined) {
                    this.shadowRoot.querySelector(".new-job-yes-button").remove();
                }
                if (this.shadowRoot.querySelector(".new-job-no-button") != undefined) {
                    this.shadowRoot.querySelector(".new-job-no-button").remove();
                }
                this.dropHighlight(this.shadowRoot.querySelector(".new-job-name"));
            });
            this.shadowRoot.querySelector(".new-job-info").append(newJobHideMessageButton);
        }
    }

    set jobError(error) {
        this.shadowRoot.querySelector(".job-info-message").innerHTML = error;
        if (this.shadowRoot.querySelector(".job-hide-message-button") == undefined) {
            const jobHideMessageButton = document.createElement("button");
            jobHideMessageButton.className = "job-hide-message-button";
            jobHideMessageButton.innerHTML = "Hide message";
            jobHideMessageButton.addEventListener("click", () => {
                this.shadowRoot.querySelector(".job-info-message").innerHTML = "";
                if (this.shadowRoot.querySelector(".job-hide-message-button") != undefined) {
                    this.shadowRoot.querySelector(".job-hide-message-button").remove();
                }
                this.dropHighlight(this.shadowRoot.querySelector(".job-name"));
            });
            this.shadowRoot.querySelector(".job-info").append(jobHideMessageButton);
        }
    }

    set addJobNameToClient(jobName) {
        if (!this.props.jobNames.includes(jobName)) {
            this.props.jobNames.push(jobName);
            this.props.jobNames.sort((a, b) => a.name - b.name);
        }
        this.shadowRoot.querySelector(".new-job-name").value = "";
        this.defineJobNameOptions();
    }

    set deleteJobNameFromClient(jobName) {
        let jobNameIndexInProps = this.props.jobNames.findIndex(existedJobName => existedJobName == jobName);
        this.props.jobNames.splice(jobNameIndexInProps, 1);
        this.props.jobNames.sort((a, b) => a.name - b.name);
        this.defineJobNameOptions();
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        const frame = () => {
            this.getFEModelLoadStatus();
            if (this.props.isFEModelLoaded === true) {
                this.getJobNames();
                this.defineJobNameOptions();
                clearInterval(id);
            }
        }
        const id = setInterval(frame, 10);
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

    getFEModelLoadStatus() {
        this.dispatchEvent(new CustomEvent("getFEModelLoadStatus", {
            bubbles: true,
            composed: true,
        }));
    }

    getJobNames() {
        this.dispatchEvent(new CustomEvent("getJobNames", {
            bubbles: true,
            composed: true,
        }));
    }

    defineJobNameOptions() {
        const jobNameSelect = this.shadowRoot.querySelector(".job-name");
        for (let i = jobNameSelect.length - 1; i >= 0; i--) {
            jobNameSelect.options[i] = null;
        }
        for (let i = 0; i < this.props.jobNames.length; i++) {
            let option = document.createElement("option");
            option.value = this.props.jobNames[i].replace(/['"]+/g, "");
            option.innerHTML = this.props.jobNames[i].replace(/['"]+/g, "");
            jobNameSelect.appendChild(option);
        }
    }

    submitJob() {
        const newJobNameField = this.shadowRoot.querySelector(".new-job-name");
        if (newJobNameField.value === "") {
            if (newJobNameField.classList.contains("highlighted") === false) {
                newJobNameField.classList.add("highlighted");
            }
        }

        if (newJobNameField.value === "") {
            if (this.shadowRoot.querySelector(".new-job-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".new-job-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                if (this.shadowRoot.querySelector(".new-job-hide-message-button") == undefined) {
                    const newJobHideMessageButton = document.createElement("button");
                    newJobHideMessageButton.className = "new-job-hide-message-button";
                    newJobHideMessageButton.innerHTML = "Hide message";
                    newJobHideMessageButton.addEventListener("click", () => {
                        this.shadowRoot.querySelector(".new-job-info-message").innerHTML = "";
                        if (this.shadowRoot.querySelector(".new-job-hide-message-button") != undefined) {
                            this.shadowRoot.querySelector(".new-job-hide-message-button").remove();
                        }
                        if (this.shadowRoot.querySelector(".new-job-yes-button") != undefined) {
                            this.shadowRoot.querySelector(".new-job-yes-button").remove();
                        }
                        if (this.shadowRoot.querySelector(".new-job-no-button") != undefined) {
                            this.shadowRoot.querySelector(".new-job-no-button").remove();
                        }
                        this.dropHighlight(this.shadowRoot.querySelector(".new-job-name"));
                    });
                    this.shadowRoot.querySelector(".new-job-info").append(newJobHideMessageButton);
                }
                return;
            } else {
                return;
            }
        }

        const newJobName = newJobNameField.value;

        if (this.props.jobNames.includes(newJobName)) {
            if (newJobNameField.classList.contains("highlighted") === false) {
                newJobNameField.classList.add("highlighted");
            }
            if (this.shadowRoot.querySelector(".new-job-hide-message-button") != undefined) {
                this.shadowRoot.querySelector(".new-job-hide-message-button").remove();
            }
            if (this.shadowRoot.querySelector(".new-job-yes-button") != undefined) {
                this.shadowRoot.querySelector(".new-job-yes-button").remove();
            }
            if (this.shadowRoot.querySelector(".new-job-no-button") != undefined) {
                this.shadowRoot.querySelector(".new-job-no-button").remove();
            }
            this.shadowRoot.querySelector(".new-job-info-message").innerHTML = 
                `Note: The ${newJobName} does already exist! Do you want to overwrite ${newJobName}?`;
            const newJobYesButton = document.createElement("button");
            newJobYesButton.className = "new-job-yes-button";
            newJobYesButton.innerHTML = "Yes";
            newJobYesButton.addEventListener("click", () => {
                this.dropHighlight(this.shadowRoot.querySelector(".new-job-name"));
                this.shadowRoot.querySelector(".new-job-info-message").innerHTML = "";
                if (this.shadowRoot.querySelector(".new-job-yes-button") != undefined) {
                    this.shadowRoot.querySelector(".new-job-yes-button").remove();
                }
                if (this.shadowRoot.querySelector(".new-job-no-button") != undefined) {
                    this.shadowRoot.querySelector(".new-job-no-button").remove();
                }
                
                this.dispatchEvent(new CustomEvent("submitJob",
                {
                    bubbles: true,
                    composed: true,
                    detail: {
                        message: newJobName,
                    },        
                }));
            });
            const newJobNoButton = document.createElement("button");
            newJobNoButton.className = "new-job-no-button";
            newJobNoButton.innerHTML = "No";
            newJobNoButton.addEventListener("click", () => {
                this.dropHighlight(this.shadowRoot.querySelector(".new-job-name"));
                this.shadowRoot.querySelector(".new-job-info-message").innerHTML = "";
                if (this.shadowRoot.querySelector(".new-job-yes-button") != undefined) {
                    this.shadowRoot.querySelector(".new-job-yes-button").remove();
                }
                if (this.shadowRoot.querySelector(".new-job-no-button") != undefined) {
                    this.shadowRoot.querySelector(".new-job-no-button").remove();
                }
            });
            this.shadowRoot.querySelector(".new-job-info").append(newJobYesButton);
            this.shadowRoot.querySelector(".new-job-info").append(newJobNoButton);
            return;    
        }

        this.dispatchEvent(new CustomEvent("submitJob",
            {
                bubbles: true,
                composed: true,
                detail: {
                    message: newJobName,
                },        
            }));
    }

    showJobAnalysisResult() {
        const selectedJobNameField = this.shadowRoot.querySelector(".job-name");
        if (selectedJobNameField.value == "") {
            if (selectedJobNameField.classList.contains("highlighted") === false) {
                selectedJobNameField.classList.add("highlighted");
            }
        }

        if (selectedJobNameField.value === "") {
            if (this.shadowRoot.querySelector(".job-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".job-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                if (this.shadowRoot.querySelector(".job-hide-message-button") == undefined) {
                    const jobHideMessageButton = document.createElement("button");
                    jobHideMessageButton.className = "job-hide-message-button";
                    jobHideMessageButton.innerHTML = "Hide message";
                    jobHideMessageButton.addEventListener("click", () => {
                        this.shadowRoot.querySelector(".job-info-message").innerHTML = "";
                        if (this.shadowRoot.querySelector(".job-hide-message-button") != undefined) {
                            this.shadowRoot.querySelector(".job-hide-message-button").remove();
                        }
                        this.dropHighlight(this.shadowRoot.querySelector(".job-name"));
                    });
                    this.shadowRoot.querySelector(".job-info").append(jobHideMessageButton);
                }
                return;
            } else {
                return;
            }
        }
        this.dispatchEvent(new CustomEvent("showJobAnalysisResult", 
            {
                bubbles: true,
                composed: true,
                detail: {
                    message: selectedJobNameField.value,
                },
            }));
        this.shadowRoot.querySelector(".job-name-filter").value = null;
    }

    deleteJob() {
        const selectedJobNameField = this.shadowRoot.querySelector(".job-name");
        if (selectedJobNameField.value == "") {
            if (selectedJobNameField.classList.contains("highlighted") === false) {
                selectedJobNameField.classList.add("highlighted");
            }
        }

        if (selectedJobNameField.value === "") {
            if (this.shadowRoot.querySelector(".job-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".job-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                if (this.shadowRoot.querySelector(".job-hide-message-button") == undefined) {
                    const jobHideMessageButton = document.createElement("button");
                    jobHideMessageButton.className = "job-hide-message-button";
                    jobHideMessageButton.innerHTML = "Hide message";
                    jobHideMessageButton.addEventListener("click", () => {
                        this.shadowRoot.querySelector(".job-info-message").innerHTML = "";
                        if (this.shadowRoot.querySelector(".job-hide-message-button") != undefined) {
                            this.shadowRoot.querySelector(".job-hide-message-button").remove();
                        }
                        this.dropHighlight(this.shadowRoot.querySelector(".job-name"));
                    });
                    this.shadowRoot.querySelector(".job-info").append(jobHideMessageButton);
                }
                return;
            } else {
                return;
            }
        }

        this.shadowRoot.querySelector(".job-info-message").innerHTML = 
            `Note: Do you really want to delete analysis result for ${selectedJobNameField.value}?`;
        const jobYesButton = document.createElement("button");
        jobYesButton.className = "job-yes-button";
        jobYesButton.innerHTML = "Yes";
        jobYesButton.addEventListener("click", () => {
            this.shadowRoot.querySelector(".job-info-message").innerHTML = "";
            if (this.shadowRoot.querySelector(".job-yes-button") != undefined) {
                this.shadowRoot.querySelector(".job-yes-button").remove();
            }
            if (this.shadowRoot.querySelector(".job-no-button") != undefined) {
                this.shadowRoot.querySelector(".job-no-button").remove();
            }
            
            this.dispatchEvent(new CustomEvent("deleteJob", 
                {
                    bubbles: true,
                    composed: true,
                    detail: {
                        message: selectedJobNameField.value,
                    },
                }));
            });
        const jobNoButton = document.createElement("button");
        jobNoButton.className = "job-no-button";
        jobNoButton.innerHTML = "No";
        jobNoButton.addEventListener("click", () => {
            this.shadowRoot.querySelector(".job-info-message").innerHTML = "";
            if (this.shadowRoot.querySelector(".job-yes-button") != undefined) {
                this.shadowRoot.querySelector(".job-yes-button").remove();
            }
            if (this.shadowRoot.querySelector(".job-no-button") != undefined) {
                this.shadowRoot.querySelector(".job-no-button").remove();
            }
        });
        this.shadowRoot.querySelector(".job-info").append(jobYesButton);
        this.shadowRoot.querySelector(".job-info").append(jobNoButton);

        this.shadowRoot.querySelector(".job-name-filter").value = null;
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

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }
}

export default FeaAnalysisMenu;
