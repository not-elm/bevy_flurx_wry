let resolveIdCount = 0;
let pendingHandlers = {}

Object.defineProperty(window, "__FLURX__", {
    value: {}
});

Object.defineProperty(window.__FLURX__, "resolveIpc", {
        value: function (id, output) {
            pendingHandlers[id](output)
        }
    }
);

Object.defineProperty(window.__FLURX__, "invoke", {
        value: function (name, args) {
            return new Promise((resolve, reject) => {
                let resoleId = resolveIdCount++;
                fetch("http://localhost:9900", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/type"
                    },
                    body: JSON.stringify({
                        entity,
                        body: {
                            id: name,
                            resolve_id: resoleId,
                            params: args.map(arg => JSON.stringify({
                                arg
                            }))
                        }
                    })
                })
                    .catch(e => {
                        reject(e)
                    })
                pendingHandlers[resoleId] = (args) => {
                    resolve(args);
                    delete pendingHandlers[resoleId]
                }
            });
        }
    }
)
