import {invoke} from "./core";

export type dialogLevel = "info" | "warn" | "error";

export interface ConfirmDialogOptions {
    title?: string,
    level?: dialogLevel,
}

/**
 * Shows a dialog to confirm yes/no with the user.
 *
 * @example
 * import {dialog} from "@bevy_flurx_wry/api";
 *
 * const yes: boolean = await dialog.ask("question");
 */
export const ask = async (
    questionMessage: string,
    option?: ConfirmDialogOptions,
): Promise<boolean> => {
    return await invoke("FLURX|dialog::ask", {
        questionMessage,
        ...option
    });
};

/**
 * Shows a dialog to confirm ok/cancel with the user.
 *
 * @example
 * import {dialog} from "@bevy_flurx_wry/api";
 *
 * const yes: boolean = await dialog.confirm("question");
 */
export const confirm = async (
    questionMessage: string,
    option?: ConfirmDialogOptions,
): Promise<boolean> => {
    return await invoke("FLURX|dialog::confirm", {
        questionMessage,
        ...option
    });
};

/**
 * Shows a message dialog.
 *
 * @example
 * import {dialog} from "@bevy_flurx_wry/api";
 *
 *  await dialog.message("message");
 */
export const message = async (
    message: string,
    option?: ConfirmDialogOptions,
): Promise<void> => {
    await invoke("FLURX|dialog::message", {
        questionMessage: message,
        ...option
    });
};