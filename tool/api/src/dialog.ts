import {invoke} from "./core";

export type dialogLevel = "info" | "warn" | "error";

export interface ConfirmDialogOptions {
    title?: string,
    level?: dialogLevel,
}

export interface OpenFileDialogOptions {
    title?: string,
    defaultPath?: string,
    directory?: boolean,
    multiple?: boolean,
    filters?: DialogFilter[],
}

export interface SaveFileDialogOptions {
    title?: string,
    defaultPath?: string,
    filters?: DialogFilter[],
}

export interface DialogFilter {
    name: string,
    extensions: string[]
}

interface Single {
    Single: string | null
}

interface Multiple {
    Multiple: string[] | null
}

type OpenDialogResult<T extends OpenFileDialogOptions> =
    T["multiple"] extends true ? Multiple : Single


export namespace dialog {
    /**
     * Shows a dialog to confirm yes/no with the user.
     *
     * @example
     * import {dialog} from "@aa/api";
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
     * import {dialog} from "@aa/api";
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
     * import {dialog} from "@aa/api";
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

    /**
     * Shows a message dialog.
     *
     * @example
     * import {dialog} from "@aa/api";
     *
     *  const selectedPaths: string[] | null = await dialog.open({multiple: true});
     *  const selectedPath: string | null = await dialog.open({multiple: false});
     */
    export const open = async <T extends OpenFileDialogOptions>(
        options?: T
    ): Promise<T["multiple"] extends true ? string[] | null : string | null> => {
        const result: OpenDialogResult<T> = await invoke("FLURX|dialog::open", options);
        const isSingle = (r: Single | Multiple): r is Single => !!(r as Single)?.Single
        // @ts-ignore
        return isSingle(result) ? result.Single : result.Multiple;
    };

    /**
     * Open a file save dialog.
     *
     *
     *  @return string: select file path ; null: if canceled
     *
     * @example
     * import {dialog} from "@aa/api";
     *
     *  const selectedPath: string | null = await dialog.save();
     */
    export const save = async (
        options?: SaveFileDialogOptions
    ): Promise<string | null> => {
        return await invoke("FLURX|dialog::save", {
            ...options,
        });
    };
}