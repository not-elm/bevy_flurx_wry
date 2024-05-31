import {app, log, invoke, listen, emit} from "../index";

export {};

declare global {
    interface Window {
        ipc: {
            postMessage: (message: string) => void;
        },
        __FLURX__: {
            app: typeof app,
            log: typeof log,
            invoke: typeof invoke,
            listen: typeof listen,
            emit: typeof emit
        }
    }
}
