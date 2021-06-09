class FeaAppMenuBar extends HTMLElement {
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

                .wrapper {
                    margin: 0;
                    padding-top: 0.5rem;
                    padding-bottom: 0.5rem;
                    padding-right: 0.5rem;
                    background: #212933;
                    border-left: 0.1rem solid #5c687a;
                    border-right: 0.1rem solid #5c687a;
                    border-bottom: 0.1rem solid #5c687a;
                    height: 1.8rem;
                }

                .menu-bar-buttons {
                    margin: 0;
                    padding: 0;
                    float: right;
                    position: relative;
                }

                .account-button {
                    margin: 0;
                    padding: 0;
                    background: #507b16;
                    color: #eaeee3;
                    width: 1.8rem;
                    height: 1.8rem;
                    border: none;
                    border-radius: 0.9rem;
                    text-align: center;
                }

                .account-button:hover {
                    background: #4c6028;
                    color: #acb1ab;
                }

                .account-dropdown-content {
                    margin-top: 0.3rem;
                    position: absolute;
                    right: 0.8rem;
                    z-index: 1;
                }

                .account-details-button {
                    background: #454f61;
                    border: none;
                    color: #eaeee3;
                    border-bottom: 0.1rem solid #637084;
                    width: 8rem;
                    height: 1.9rem;
                    border-radius: 0.2rem 0.2rem 0 0;
                }

                .account-details-button:hover {
                    background: #3c4251;
                }

                .sign-out-button {
                    background: #454f61;
                    border: none;
                    color: #eaeee3;
                    width: 8rem;
                    height: 1.8rem;
                    border-radius: 0 0 0.2rem 0.2rem;
                }

                .sign-out-button:hover {
                    background: #3c4251;
                }

                .hidden {
                    display: none;
                }

            </style>
            <div class="wrapper">
                <div class="menu-bar-buttons">
                    <button class="account-button">${this.getAttribute("username")}</button>
                    <div class="account-dropdown-content hidden">
                        <button class="account-details-button">Account details</button>
                        <button class="sign-out-button">Sign out</button>
                    </div>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".account-button").addEventListener("click", () => this.toggleAccountDropdownContent());

        this.shadowRoot.querySelector(".sign-out-button").addEventListener("click", () => this.signOut());

        document.addEventListener("click", (event) => this.closeAccountDropdownContent(event));
    }

    connectedCallback() {
    }

    disconnectedCallback() {
    }
    
    static get observedAttributes() {
        return ["username"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
    }

    adoptedCallback() {
    }

    toggleAccountDropdownContent() {
        const accountDropdownContent = this.shadowRoot.querySelector(".account-dropdown-content");
        if (accountDropdownContent.classList.contains("hidden") === true) {
            accountDropdownContent.classList.remove("hidden");
        } else {
            accountDropdownContent.classList.add("hidden");
        }

           
    }

    closeAccountDropdownContent(event) {
        const paths = event.composedPath();
        let accountButtonClicked = false;
        for(let i = 0; i < paths.length; i++) {
            const currentClassList = paths[i].classList;
            if (currentClassList !== undefined) {
                if (currentClassList.contains("account-button") === true) {
                    accountButtonClicked = true;
                    break;
                }
            }
        }
        if (accountButtonClicked === false) {
            const accountDropdownContent = this.shadowRoot.querySelector(".account-dropdown-content");
            if (accountDropdownContent.classList.contains("hidden") === false) {
                accountDropdownContent.classList.add("hidden");
            }
        }
        event.stopPropagation();
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

    signOut() {
        this.postData("/auth/logout", {})
            .then(response => {
                window.location.href = response.url;
        });
    }
}

export default FeaAppMenuBar;
