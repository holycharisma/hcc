import("../css/app.css");

import("./encryption.js");

import("./audioplayer.js");

// console.log("time to start to import index.js")

try {
    import("../pkg/index.js").then((x => {
        // console.log("imported index.js");
        // global exports for JS interop here:
        window.recv_claims = x.recv_claims;
        window.EphemeralSharedKeyring = x.SharedKeyring;
        window.render_media_node = x.render_media_node;
        // console.log("window exports complete")
    }));
} catch (e) {
    console.log(e);
}

import("./htmx.js");