import AuthSignInMenu from "./auth_components/auth-sign-in-menu.js";
customElements.define("auth-sign-in-menu", AuthSignInMenu);

import AuthCreateAccountMenu from "./auth_components/auth-create-account-menu.js";
customElements.define("auth-create-account-menu", AuthCreateAccountMenu);

const mainContainer = document.querySelector(".main-container");

const authMenuContainer = document.querySelector(".auth-menu-container");

const signInButton = document.querySelector(".sign-in-menu-button");

const createAccountButton = document.querySelector(".create-account-menu-button");

document.addEventListener("activateAuthMenu", (event) => activateAuthMenu(event));

document.addEventListener("activateSignInMenu", (event) => activateSignInMenuFromCreateAccountMenu(event));

signInButton.addEventListener("click", () => activateSignInMenu());

createAccountButton.addEventListener("click", () => activateCreateAccountMenu());

function activateSignInMenu() {
    mainContainer.innerHTML = "";
    const authSignInMenu = document.createElement("auth-sign-in-menu");
    mainContainer.append(authSignInMenu);
}

function activateCreateAccountMenu() {
    mainContainer.innerHTML = "";
    const authCreateAccountMenu = document.createElement("auth-create-account-menu");
    mainContainer.append(authCreateAccountMenu);
}

function activateAuthMenu(event) {
    mainContainer.innerHTML = "";
    mainContainer.append(authMenuContainer);
    event.stopPropagation();
}

function activateSignInMenuFromCreateAccountMenu(event) {
    event.stopPropagation();
    activateSignInMenu();
}