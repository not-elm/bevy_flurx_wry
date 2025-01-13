import {invoke} from "./core";

export interface SendOptions {
    title?: string,
    icon?: string,
}

export namespace notification {
    /**
     * Sends a notification.
     *
     * @example
     * import {notification} from "bevy_flurx_api";
     * await notification.send("message");
     */
    export const send = async (
        message: string,
        options?: SendOptions,
    ): Promise<void> => {
        await invoke("FLURX|notification::send", {
            message,
            ...options
        });
    }
}