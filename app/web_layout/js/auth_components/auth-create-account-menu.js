class AuthCreateAccountMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {};

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
                    padding: 0rem;
                    margin: 0rem;
                }

                .create-account-menu-header-content {
                    margin-top: 1rem;
                    display: flex;
                    flex-direction: row;
                }

                .back-button {
                    margin: 0rem;
                    padding: 0rem;
                    border: #ffffff;
                }

                .back-button-icon-content {
                    margin: 0rem;
                    padding: 0rem;
                    background: #ffffff;
                }

                .back-button-icon {
                    width: 1rem;
                    height: 1rem;
                }

                .back-button-icon-caption {
                    color: #3a3e5d;
                    margin: 0rem;
                    padding: 0rem;
                    font-size: 85%;
                }

                .create-account-caption {
                    margin-top: 0rem;
                    margin-bottom: 0;
                    margin-left: 2rem;
                    margin-right: 0;
                    padding: 0;
                    color: #3a3e5d;
                    font-size: 125%;
                }

                .first-name-field-content {
                    display: flex;
                    flex-direction: column;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                }

                .first-name-caption {
                    margin: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0rem;
                    padding-left: 0.3rem;
                    padding-right: 0rem;
                    color: #9096aa;
                    font-size: 85%;
                }

                .first-name {
                    margin-top: 0.3rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0rem;
                    padding-left: 0.3rem;
                    padding-right: 0rem;
                    height: 2rem;
                    border: 0.1rem solid #bfbfbf;
                    border-radius: 0.3rem;
                    outline: none;
                    color: #9096aa;
                }

                .first-name:hover {
                    box-shadow: 0 0 0.2rem #bfbfbf;
                }

                .first-name:focus {
                    border: 0.1rem solid #0996d7;
                    box-shadow: 0 0 0.2rem #0996d7;
                }

                .last-name-field-content {
                    display: flex;
                    flex-direction: column;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                }

                .last-name-caption {
                    margin: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0rem;
                    padding-left: 0.3rem;
                    padding-right: 0rem;
                    color: #9096aa;
                    font-size: 85%;
                }

                .last-name {
                    margin-top: 0.3rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0rem;
                    padding-left: 0.3rem;
                    padding-right: 0rem;
                    height: 2rem;
                    border: 0.1rem solid #bfbfbf;
                    border-radius: 0.3rem;
                    outline: none;
                    color: #9096aa;
                }

                .last-name:hover {
                    box-shadow: 0 0 0.2rem #bfbfbf;
                }

                .last-name:focus {
                    border: 0.1rem solid #0996d7;
                    box-shadow: 0 0 0.2rem #0996d7;
                }

                .email-field-content {
                    display: flex;
                    flex-direction: column;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                }

                .email-caption {
                    margin: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0rem;
                    padding-left: 0.3rem;
                    padding-right: 0rem;
                    color: #9096aa;
                    font-size: 85%;
                }

                .email {
                    margin-top: 0.3rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0rem;
                    padding-left: 0.3rem;
                    padding-right: 0rem;
                    height: 2rem;
                    border: 0.1rem solid #bfbfbf;
                    border-radius: 0.3rem;
                    outline: none;
                    color: #9096aa;
                }

                .email:hover {
                    box-shadow: 0 0 0.2rem #bfbfbf;
                }

                .email:focus {
                    border: 0.1rem solid #0996d7;
                    box-shadow: 0 0 0.2rem #0996d7;
                }

                .confirm-email-field-content {
                    display: flex;
                    flex-direction: column;
                    padding: 0rem;
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                }

                .confirm-email-caption {
                    margin: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0rem;
                    padding-left: 0.3rem;
                    padding-right: 0rem;
                    color: #9096aa;
                    font-size: 85%;
                }

                .confirm-email {
                    margin-top: 0.3rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0rem;
                    padding-left: 0.3rem;
                    padding-right: 0rem;
                    height: 2rem;
                    border: 0.1rem solid #bfbfbf;
                    border-radius: 0.3rem;
                    outline: none;
                    color: #9096aa;
                }

                .confirm-email:hover {
                    box-shadow: 0 0 0.2rem #bfbfbf;
                }

                .confirm-email:focus {
                    border: 0.1rem solid #0996d7;
                    box-shadow: 0 0 0.2rem #0996d7;
                }

                .password-field-content {
                    display: flex;
                    flex-direction: column;
                    padding: 0rem;
                    margin-top: 0.5rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                }

                .password-caption {
                    margin: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0rem;
                    padding-left: 0.3rem;
                    padding-right: 0rem;
                    color: #9096aa;
                    font-size: 85%;
                }

                .password {
                    margin-top: 0.3rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding-top: 0rem;
                    padding-bottom: 0rem;
                    padding-left: 0.3rem;
                    padding-right: 0rem;
                    height: 2rem;
                    border: 0.1rem solid #bfbfbf;
                    border-radius: 0.3rem;
                    outline: none;
                    color: #9096aa;
                }

                .password:hover {
                    box-shadow: 0 0 0.2rem #bfbfbf;
                }

                .password:focus {
                    border: 0.1rem solid #0996d7;
                    box-shadow: 0 0 0.2rem #0996d7;
                }

                .create-account-menu-buttons {
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                }

                .create-account-button {
                    margin-top: 0rem;
                    margin-bottom: 0;
                    margin-left: 0;
                    margin-right: 0;
                    width: 15rem;
                    height: 2rem;
                    background: #0996d7;
                    color: #fffdf2;
                    border: 0.1rem solid;
                    border-radius: 0.2rem; 
                    border-color: #0996d7;
                }

                .create-account-button:hover {
                    box-shadow: 0 0 0.2rem #bfbfbf;
                }

                .create-account-button:focus {
                    box-shadow: 0 0 0.4rem #bfbfbf;
                }

                .auth-info {
                    display: flex;
                    margin: 0rem;
                    padding: 0rem;
                }

                .auth-info-message {
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                    color: #9096aa;
                    font-size: 80%;
                    width: 15rem;
                }

                .highlighted {
                    border: 0.1rem solid #0996d7;
                    box-shadow: 0 0 0.2rem #0996d7;
                }
            </style>

            <div class=wrapper>

                <div class=create-account-menu-header-content>
                    <button class="back-button">
                        <div class="back-button-icon-content">
                            <svg class="back-button-icon" width="20" height="21" viewBox="0 0 20 21" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Back</title>
                                <path d="M10.5 1.30761L1.30762 10.5L10.5 19.6924" stroke="#0996D7" stroke-width="3"/>
                            </svg>
                            <p class="back-button-icon-caption">Back</p>
                        </div>
                    </button>
                    <p class="create-account-caption">Create account</p>
                </div>

                <div class="first-name-field-content">
                    <p class="first-name-caption">First name</p>
                    <input class="first-name" type="text"/>
                </div>
                
                <div class="last-name-field-content">
                    <p class="last-name-caption">Last name</p>
                    <input class="last-name" type="text"/>
                </div>

                <div class="email-field-content">
                    <p class="email-caption">Email</p>
                    <input class="email" type="text"/>
                </div>

                <div class="confirm-email-field-content">
                    <p class="confirm-email-caption">Confirm email</p>
                    <input class="confirm-email" type="text"/>
                </div>

                <div class="password-field-content">
                    <p class="password-caption">Password</p>
                    <input class="password" type="password"/>
                </div>
                
                <div class="create-account-menu-buttons">
                    <button class="create-account-button">Create account</button>
                </div>

                <div class="auth-info">
                    <p class="auth-info-message"></p>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".create-account-button").addEventListener("click", () => this.createAccount());

        this.shadowRoot.querySelector(".back-button").addEventListener("click", () => this.back());

        this.shadowRoot.querySelector(".first-name").addEventListener("click", () => {
            const inputtedFirstName = this.shadowRoot.querySelector(".first-name");
            this.dropHighlight(inputtedFirstName);
            this.shadowRoot.querySelector(".auth-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".last-name").addEventListener("click", () => {
            const inputtedLastName = this.shadowRoot.querySelector(".last-name");
            this.dropHighlight(inputtedLastName);
            this.shadowRoot.querySelector(".auth-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".email").addEventListener("click", () => {
            const inputtedEmail = this.shadowRoot.querySelector(".email");
            this.dropHighlight(inputtedEmail);
            this.shadowRoot.querySelector(".auth-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".confirm-email").addEventListener("click", () => {
            const inputtedConfirmEmail = this.shadowRoot.querySelector(".confirm-email");
            this.dropHighlight(inputtedConfirmEmail);
            this.shadowRoot.querySelector(".auth-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".password").addEventListener("click", () => {
            const inputtedPassword = this.shadowRoot.querySelector(".password");
            this.dropHighlight(inputtedPassword);
            this.shadowRoot.querySelector(".auth-info-message").innerHTML = "";
        });
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
  
    createAccount() {
        const firstNameField = this.shadowRoot.querySelector(".first-name");
        if (firstNameField.value === "") {
            if (firstNameField.classList.contains("highlighted") === false) {
                firstNameField.classList.add("highlighted");
            }
        }

        const lastNameField = this.shadowRoot.querySelector(".last-name");
        if (lastNameField.value === "") {
            if (lastNameField.classList.contains("highlighted") === false) {
                lastNameField.classList.add("highlighted");
            }
        }

        const emailField = this.shadowRoot.querySelector(".email");
        if (emailField.value === "") {
            if (emailField.classList.contains("highlighted") === false) {
                emailField.classList.add("highlighted");
            }
        }

        const confirmEmailField = this.shadowRoot.querySelector(".confirm-email");
        if (confirmEmailField.value === "") {
            if (confirmEmailField.classList.contains("highlighted") === false) {
                confirmEmailField.classList.add("highlighted");
            }
        }

        const passwordField = this.shadowRoot.querySelector(".password");
        if (passwordField.value === "") {
            if (passwordField.classList.contains("highlighted") === false) {
                passwordField.classList.add("highlighted");
            }
        }

        if (firstNameField.value === "" || lastNameField.value === "" || emailField.value === "" || 
            confirmEmailField.value === "" || passwordField.value === "") {
            if (this.shadowRoot.querySelector(".auth-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".auth-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if (this.validateEmail(emailField.value) === false) {
            if (this.shadowRoot.querySelector(".auth-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".auth-info-message").innerHTML = 
                    "Note: This email is not recognized!";
                emailField.classList.add("highlighted");
                return;
            } else {
                return;
            }
        }

        if (this.validateEmail(confirmEmailField.value) === false) {
            if (this.shadowRoot.querySelector(".auth-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".auth-info-message").innerHTML = 
                    "Note: This email is not recognized!";
                confirmEmailField.classList.add("highlighted");
                return;
            } else {
                return;
            }
        }

        if (emailField.value !== confirmEmailField.value) {
            if (this.shadowRoot.querySelector(".auth-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".auth-info-message").innerHTML = 
                    "Note: Your email and confirmation email must match!";
                emailField.classList.add("highlighted");
                confirmEmailField.classList.add("highlighted");
                return;
            } else {
                return;
            }
        }

        this.dispatchEvent(new CustomEvent("activateSignInMenu", {
            bubbles: true,
            composed: true,
        }));
    }

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }

    validateEmail(email) {
        const re = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
        return re.test(String(email).toLowerCase());
    }


    back() {
        this.dispatchEvent(new CustomEvent("activateAuthMenu", {
            bubbles: true,
            composed: true,
        }));
    }
}

export default AuthCreateAccountMenu;
