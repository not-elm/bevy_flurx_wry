import {invoke} from "./core";

export namespace clipboard {
    /**
     * Gets the text from the clipboard.
     *
     * @example
     * import {clipboard} from "bevy_flurx_api";
     * const text: string = await clipboard.getText();
     */
    export const getText = async (): Promise<string> => {
        return await invoke("FLURX|clipboard::get_text");
    }

    /**
     * Sets the text to the clipboard.
     *
     * @example
     * import {clipboard} from "bevy_flurx_api";
     * await clipboard.setText("text");
     */
    export const setText = async (text: string): Promise<void> => {
        await invoke("FLURX|clipboard::set_text", text);
    }
}
