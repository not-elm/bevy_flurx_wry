let resolveIdCount = 0;
const pendingHandlers: Record<string, any> = {};
const eventHandlers: Record<string, (event: any) => void> = {};

export const invoke = <Out>(
    id: string,
    args: any = null
): Promise<Out> => {
    return new Promise((resolve, reject) => {
        const resolveId = resolveIdCount++;
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

        pendingHandlers[resolveId] = (args: Out) => {
            resolve(args);
            delete pendingHandlers[resolveId]
        }
    });
}

export const listen = (eventId: string, f: (event: any) => void) => {
    eventHandlers[eventId] = f;
    return () => {
        delete eventHandlers[eventId];
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
    pendingHandlers[id]?.(output)
};

export const __emitEvent = (eventId: string, event: any) => {
    eventHandlers[eventId]?.(event);
};