import {fs, app, log, invoke, listen, emit} from "../index";

export {};

declare global {
    interface Window {
        ipc: {
            postMessage: (message: string) => void;
        },
        __FLURX__: {
            app: typeof app,
            fs: typeof fs,
            log: typeof log,
            invoke: typeof invoke,
            listen: typeof listen,
            emit: typeof emit,
        }
    }
}
