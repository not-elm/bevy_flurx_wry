interface Ok<Output> {
    "Ok": Output
}

interface Err {
    "Err": any
}

export interface PhysicalPosition {
    x: number,
    y: number,
}

export interface PhysicalSize {
    width: number,
    height: number,
}

const isOk = <Out>(args: unknown): args is Ok<Out> => {
    if (!args || (args && typeof (args) !== "object")) {
        return false;
    }
    const ok = args as Ok<Out>;
    return ok.Ok !== undefined;
}

const isErr = (args: unknown): args is Err => {
    if (!args || (args && typeof (args) !== "object")) {
        return false;
    }
    const err = args as Err;
    return err.Err !== undefined;
}

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
            value: (args: Out | Ok<Out> | Err) => {
                Reflect.deleteProperty(window.__FLURX__, prop);
                if (isOk(args)) {
                    resolve(args.Ok);
                } else if (isErr(args)) {
                    reject(args.Err);
                } else {
                    resolve(args);
                }
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
