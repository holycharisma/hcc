import "htmx.org";

window.htmx = require("htmx.org");

const [authKey, antiForgeryKey, encryptionKey] = ["au", "af", "ec"];
const TOKEN_DB = {
  [authKey]: undefined,
  [antiForgeryKey]: undefined,
  [encryptionKey]: undefined,
};

const ORIGIN = String(document.referrer).replace(/\/$/, "");

let init_keyring = false;

function recvTokenMessage(event) {
  if (event.origin !== ORIGIN) return
  else if (!window.recv_claims || !window.EphemeralSharedKeyring) return
  else if (init_keyring) return;
  
  init_keyring = true;

  let claims = recv_claims(
    ORIGIN,
    event.data.token,
    event.data.pubkey
  );

  let keyring = new EphemeralSharedKeyring(claims);

  TOKEN_DB[encryptionKey] = keyring;
  TOKEN_DB[antiForgeryKey] = keyring.encrypt(event.data.token);

  event.source.postMessage("ack-token", event.origin);
  window.removeEventListener("message", recvTokenMessage);
}

window.addEventListener("message", recvTokenMessage);

function signRequestHeaders(evt) {

  // notice: both AUTH_TOKEN and ANTI_FORGERY_TOKEN 
  // are already encrypted using the client encryption key received by the JWT session handshake

  if (evt && evt.detail && evt.detail.headers) {
    // todo: only sign request if we are requesting a secure asset
    const jwt = TOKEN_DB[authKey];
    if (jwt) {
      evt.detail.headers["x-auth-token"] = jwt;
    }

    // todo: only sign with CSRF if unsafe request method
    const csrf = TOKEN_DB[antiForgeryKey];
    if (csrf) {
      evt.detail.headers["x-anti-forgery-token"] = csrf;
    }
  }
}

function recvAuthToken(jwt) {
  let encryption = TOKEN_DB[encryptionKey];
  if (encryption) {
    if (jwt) {
      let decrypted = encryption.decrypt(jwt);
      TOKEN_DB[authKey] = encryption.encrypt(decrypted);
    }
  } else {
    setTimeout(recvAuthToken.bind(null, jwt), 100);
  }
}

function readAuthHeader(evt) {
  let authHeader = "";
  if (evt && evt.detail && evt.detail.xhr instanceof XMLHttpRequest) {
    authHeader = evt.detail.xhr.getResponseHeader("x-auth-token");
  } else {
  }
  recvAuthToken(authHeader);
}

document.body.addEventListener("htmx:configRequest", signRequestHeaders);

document.body.addEventListener("htmx:beforeSwap", readAuthHeader);
