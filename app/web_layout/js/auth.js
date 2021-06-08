import AuthSignInMenu from "./auth_components/auth-sign-in-menu.js";
customElements.define("auth-sign-in-menu", AuthSignInMenu);

const mainContainer = document.querySelector(".main-container");

const authMenuContainer = document.querySelector(".auth-menu-container");

const signInButton = document.querySelector(".sign-in-menu-button");

const backButton = document.querySelector(".back");

signInButton.addEventListener("click", () => activateLoginMenu());

backButton.addEventListener("click", () => activateAuthMenu());

function activateLoginMenu() {
    mainContainer.innerHTML = "";
    const authSignInMenu = document.createElement("auth-sign-in-menu");
    mainContainer.append(authSignInMenu);
}


function activateAuthMenu() {
    mainContainer.innerHTML = "";
    mainContainer.append(authMenuContainer);
}