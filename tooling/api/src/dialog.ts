import {invoke} from "./core";

export type dialogLevel = "info" | "warn" | "error";

export interface AskDialogOptions {
    title?: string,
    level?: dialogLevel,
}

/**
 *  ユーザーに確認求めるダイアログを表示します。
 * Shows a dialog to confirm yes/no with the user.
 *
 * @example
 * import {dialog} from "@bevy_flurx_wry/api";
 *
 * const yes: boolean = await dialog.ask("question");
 */
export const ask = async (
    questionMessage: string,
    option?: AskDialogOptions,
): Promise<boolean> => {
    return await invoke("FLURX|dialog::ask", {
        questionMessage,
        ...option
    });
};