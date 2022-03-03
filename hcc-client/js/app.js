import("../css/app.css");

import("./encryption.js");

try {
    import("../pkg/index.js").then((x => {
        window.recv_claims = x.recv_claims;
        window.EphemeralSharedKeyring = x.SharedKeyring;
    }));
} catch (e) {
    console.log(e);
}

import("./htmx.js");