import * as core from "../core";

export {};

declare global {
    interface Window {
        ipc: {
            postMessage: (message: string) => void;
        },
        __FLURX__: {
            core: typeof core
        }
    }
}
