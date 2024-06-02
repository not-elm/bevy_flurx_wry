export const invoke = <Out>(
    id: string,
    args: any = null
): Promise<Out> => {
    return new Promise((resolve, reject) => {
        const resolveId = uid();
        const prop = `_${resolveId}`;

        const convertToArgs = (args: any) => {
            if (args == null) {
                return {
                    type: "Command",
                    message: {
                        id,
                        resolve_id: resolveId,
                    }
                }
            } else {
                return {
                    type: "Command",
                    message: {
                        id,
                        args: JSON.stringify(args),
                        resolve_id: resolveId,
                    }
                }
            }
        };

        window.ipc.postMessage(JSON.stringify(convertToArgs(args)));
        Object.defineProperty(window.__FLURX__, prop, {
            value: (args: Out) => {
                Reflect.deleteProperty(window.__FLURX__, prop);
                resolve(args);
            },
            writable: false,
            configurable: true
        })
    });
}

export const listen = <E>(eventId: string, f: (event: E) => void) => {
    const prop = `_event_${eventId}`;
    Object.defineProperty(window.__FLURX__, prop, {
        value: f,
        writable: false,
        configurable: true
    })
    return () => {
        Reflect.deleteProperty(window.__FLURX__, prop);
    };
};

export const emit = (eventId: string, event: any) => {
    window.ipc.postMessage(JSON.stringify({
        type: "Event",
        message: {
            event_id: eventId,
            payload: JSON.stringify(event)
        }
    }));
};

export const __resolveIpc = (id: string, output: any) => {
    (window.__FLURX__ as any)[`_${id}`]?.(output)
};

export const __emitEvent = (eventId: string, event: any) => {
    (window.__FLURX__ as any)[`_event_${eventId}`]?.(event)
};

const uid = () => {
    return window.crypto.getRandomValues(new Uint32Array(1))[0]
}