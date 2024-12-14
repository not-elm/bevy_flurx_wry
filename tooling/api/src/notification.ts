import {invoke} from "./core";

export interface SendOptions {
    title?: string,
    icon?: string,
}

export const send = async (
    message: string,
    options?: SendOptions,
): Promise<void> => {
    await invoke("FLURX|notification::send", {
        message,
        ...options
    });
}