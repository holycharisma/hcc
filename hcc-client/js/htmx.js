import 'htmx.org';

window.htmx = require('htmx.org');

const [authKey, antiForgeryKey] = ["au", "af"];
const TOKEN_DB = {
    [authKey]: undefined,
    [antiForgeryKey]: undefined
};

const ORIGIN = String(document.referrer).replace(/\/$/, "");

function recvTokenMessage(event) {
    if(event.origin !== ORIGIN) return;
    TOKEN_DB[antiForgeryKey] = event.data;
    event.source.postMessage("ack-token", event.origin);
    window.removeEventListener('message', recvTokenMessage);
}

window.addEventListener('message', recvTokenMessage);

function signRequestHeaders(evt) {
    if (evt && evt.detail && evt.detail.headers) {

        // todo: only sign request if we are requesting a secure asset
        const jwt = TOKEN_DB[authKey];
        if (jwt) {
            evt.detail.headers['x-auth-token'] = jwt;
        }

        // todo: only sign with CSRF if unsafe request method
        const csrf = TOKEN_DB[antiForgeryKey];
        if (csrf) {
            evt.detail.headers['x-anti-forgery-token'] = csrf;
        }
    }
}

function readAuthHeader(evt) {
    if (evt && evt.detail && evt.detail.xhr instanceof XMLHttpRequest) {
        const jwt = evt.detail.xhr.getResponseHeader("x-auth-token");
        if (jwt) {
            TOKEN_DB[authKey] = jwt;
        }
    }
}

document.body.addEventListener('htmx:configRequest', signRequestHeaders);

document.body.addEventListener('htmx:beforeSwap', readAuthHeader);