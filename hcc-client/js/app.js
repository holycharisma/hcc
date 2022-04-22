import("../css/app.css");

import("./encryption.js");

try {
    import("../pkg/index.js").then((x => {
        // global exports for JS interop here:
        window.recv_claims = x.recv_claims;
        window.EphemeralSharedKeyring = x.SharedKeyring;
        window.render_media_node = x.render_media_node;
    }));
} catch (e) {
    console.log(e);
}

import("./htmx.js");