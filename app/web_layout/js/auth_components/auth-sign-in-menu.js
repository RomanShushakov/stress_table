class AuthSignInMenu extends HTMLElement {
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

                .sign-in-menu-header-content {
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

                .sign-in-caption {
                    margin-top: 0rem;
                    margin-bottom: 0;
                    margin-left: 4rem;
                    margin-right: 0;
                    padding: 0;
                    color: #3a3e5d;
                    font-size: 125%;
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

                .sign-in-menu-buttons {
                    margin-top: 1rem;
                    margin-bottom: 0rem;
                    margin-left: 0rem;
                    margin-right: 0rem;
                    padding: 0rem;
                }

                .sign-in-button {
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

                .sign-in-button:hover {
                    box-shadow: 0 0 0.2rem #bfbfbf;
                }

                .sign-in-button:focus {
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

                <div class=sign-in-menu-header-content>
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
                    <p class="sign-in-caption">Sign in</p>
                </div>
                
                <div class="email-field-content">
                    <p class="email-caption">Email</p>
                    <input class="email" type="text" placeholder="name@example.com"/>
                </div>

                <div class="password-field-content">
                    <p class="password-caption">Password</p>
                    <input class="password" type="password"/>
                </div>
                
                <div class="sign-in-menu-buttons">
                    <button class="sign-in-button">Sign in</button>
                </div>

                <div class="auth-info">
                    <p class="auth-info-message"></p>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".sign-in-button").addEventListener("click", () => this.signIn());

        this.shadowRoot.querySelector(".back-button").addEventListener("click", () => this.back());

        this.shadowRoot.querySelector(".email").addEventListener("click", () => {
            const inputtedEmail = this.shadowRoot.querySelector(".email");
            this.dropHighlight(inputtedEmail);
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

    async postData(url = "", data = {}) {
        const response = await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(data)
        });
        return response;
    }
  
    signIn() {
        const emailField = this.shadowRoot.querySelector(".email");
        if (emailField.value === "") {
            if (emailField.classList.contains("highlighted") === false) {
                emailField.classList.add("highlighted");
            }
        }
        const passwordField = this.shadowRoot.querySelector(".password");
        if (passwordField.value === "") {
            if (passwordField.classList.contains("highlighted") === false) {
                passwordField.classList.add("highlighted");
            }
        }

        if (emailField.value === "" || passwordField.value === "") {
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

        this.postData("/auth/login", { email: emailField.value, password: passwordField.value })
            .then(response => {
                if (response.ok) {
                    window.location.href = "/";
                } else {
                    if (this.shadowRoot.querySelector(".auth-info-message").innerHTML === "") {
                        this.shadowRoot.querySelector(".auth-info-message").innerHTML = 
                            "Note: You input incorrect username or password!";
                        return;
                    } else {
                        return;
                    }
                }  
        });
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

export default AuthSignInMenu;
