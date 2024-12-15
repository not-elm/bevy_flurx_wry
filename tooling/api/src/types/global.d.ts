import {
    app,
    clipboard,
    dialog,
    emit,
    fs,
    invoke,
    listen,
    log,
    monitor,
    notification,
    os,
    path,
    WebWindow
} from "../index";

export {};

declare global {
    interface Window {
        ipc: {
            postMessage: (message: string) => void;
        },
        __FLURX__: {
            app: typeof app,
            fs: typeof fs,
            path: typeof path,
            log: typeof log,
            clipboard: typeof clipboard,
            dialog: typeof dialog,
            notification: typeof notification,
            os: typeof os,
            monitor: typeof monitor,
            WebWindow: typeof WebWindow,
            invoke: typeof invoke,
            listen: typeof listen,
            emit: typeof emit,
        }
    }
}
