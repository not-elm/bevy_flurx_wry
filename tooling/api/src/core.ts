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
                    id,
                    resolve_id: resolveId,
                }
            } else {
                return {
                    id,
                    resolve_id: resolveId,
                    args: JSON.stringify(args)
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

export const resolveIpc = (id: string, output: any) => {
    pendingHandlers[id]?.(output)
};

export const listen = (eventId: string, f: (event: any) => void) => {
    eventHandlers[eventId] = f;
    return () => {
        delete eventHandlers[eventId];
    };
};

export const emitEvent = (eventId: string, event: any) => {
    eventHandlers[eventId]?.(event);
};