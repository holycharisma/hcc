
const ORIGIN = String(document.referrer).replace(/\/$/, "");

let init_keyring = false;

const [antiForgeryKey, encryptionKey] = ["au", "af", "ec"];
const TOKEN_DB = {
  [antiForgeryKey]: undefined,
  [encryptionKey]: undefined,
};

function recvTokenMessage(event) {
  if (event.origin !== ORIGIN) return;
  else if (!window.recv_claims || !window.EphemeralSharedKeyring) return;
  else if (init_keyring) return;

  init_keyring = true;

  let claims = recv_claims(ORIGIN, event.data.token);

  let keyring = new EphemeralSharedKeyring(claims);

  TOKEN_DB[encryptionKey] = keyring;
  TOKEN_DB[antiForgeryKey] = keyring.encrypt_header(event.data.token);

  event.source.postMessage("ack-token", event.origin);
  window.removeEventListener("message", recvTokenMessage);
}

window.addEventListener("message", recvTokenMessage);

export default {
    getKeyring: function() {
        return TOKEN_DB[encryptionKey];
    },
    getAntiForgeryToken: function() {
        return TOKEN_DB[antiForgeryKey];
    }
};
