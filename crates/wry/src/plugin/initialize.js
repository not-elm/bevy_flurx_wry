;(function () {
    let resolveIdCount = 0;
    const pendingHandlers = {}

    Object.defineProperty(window, "__FLURX__", {
        value: {}
    });

    Object.defineProperty(window.__FLURX__, "resolveIpc", {
            value: Object.freeze((id, output) => {
                pendingHandlers[id](output)
            })
        }
    );

    Object.defineProperty(window.__FLURX__, "invoke", {
            value: Object.freeze((name, args) => {
                return new Promise((resolve, reject) => {
                    const resolveId = resolveIdCount++;
                    window.ipc.postMessage(JSON.stringify({
                        id: name,
                        resolve_id: resolveId,
                        params: args.map(arg => JSON.stringify({
                            arg
                        }))
                    }));

                    pendingHandlers[resolveId] = (args) => {
                        resolve(args);
                        delete pendingHandlers[resolveId]
                    }
                });
            })
        }
    );
})();