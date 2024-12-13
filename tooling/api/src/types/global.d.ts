import {fs, app, log, invoke, listen, emit, clipboard} from "../index";

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
            clipboard: typeof clipboard,
            invoke: typeof invoke,
            listen: typeof listen,
            emit: typeof emit,
        }
    }
}
