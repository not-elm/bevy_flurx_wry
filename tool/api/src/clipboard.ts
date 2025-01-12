import {invoke} from "./core";

export namespace clipboard {
    export const getText = async (): Promise<string> => {
        return await invoke("FLURX|clipboard::get_text");
    }

    export const setText = async (text: string): Promise<void> => {
        await invoke("FLURX|clipboard::set_text", text);
    }
}
