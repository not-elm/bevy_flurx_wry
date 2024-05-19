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
                window.ipc.postMessage(JSON.stringify({
                    id: name,
                    resolve_id: resoleId,
                    params: args.map(arg => JSON.stringify({
                        arg
                    }))
                }));
                pendingHandlers[resoleId] = (args) => {
                    resolve(args);
                    delete pendingHandlers[resoleId]
                }
            });
        }
    }
)
