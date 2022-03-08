import encryption from "./encryption";

import "htmx.org";
const htmx = (window.htmx = require("htmx.org"));

let jwt;

function signRequestHeaders(evt) {
  // notice: both AUTH_TOKEN and ANTI_FORGERY_TOKEN
  // are already encrypted using the client encryption key received by the JWT session handshake

  if (evt && evt.detail && evt.detail.headers) {
    // todo: only sign request if we are requesting a secure asset
    if (jwt) {
      evt.detail.headers["x-auth-token"] = jwt;
    }

    // todo: only sign with CSRF if unsafe request method
    const csrf = encryption.getAntiForgeryToken();

    if (csrf) {
      evt.detail.headers["x-anti-forgery-token"] = csrf;
    }

    let keyring = encryption.getKeyring();

    for (const [key, value] of Object.entries(evt.detail.parameters)) {
      evt.detail.parameters[key] = keyring.encrypt(value);
    }
  }
}

function decryptResponse(evt) {
  let header = evt.detail.xhr.getResponseHeader("x-auth-token");

  let keyring = encryption.getKeyring();

  if (header) {
    jwt = keyring.encrypt_header(keyring.decrypt_header(header));
  }

  let obj = evt.detail;
  if (obj && obj.serverResponse) {
    let target = obj.target;
    let serverHtml = keyring.decrypt(obj.serverResponse);

    let node = document.createElement("div");
    node.className = "hcc-htmx";
    node.innerHTML = serverHtml;

    obj["serverResponse"] = serverHtml;
    obj["target"] = node;

    htmx.process(obj.target);

    target.innerHTML = "";
    target.appendChild(obj.target);
  }
}

document.body.addEventListener("htmx:configRequest", signRequestHeaders);

document.body.addEventListener("htmx:beforeSwap", decryptResponse);
