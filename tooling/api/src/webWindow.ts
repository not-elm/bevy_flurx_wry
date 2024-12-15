import {invoke} from "./core";


export class WebWindow{
    private constructor(
        private readonly identifier: string,
    ) {
    }
    async title(): Promise<string>{
        return await invoke("FLURX|web_window::title", this.identifier);
    }

    /**
     *  Center the window.
     *
     *  @example
     * import {WebWindow} from "@bevy_flurx_wry/api";
     *
     * await WebWindow.current().center();
     */
    async center(): Promise<void>{
        await invoke("FLURX|web_window::center", this.identifier);
    }

        /**
     *  Hide the window.
     *
     *  @example
     * import {WebWindow} from "@bevy_flurx_wry/api";
     *
     * await WebWindow.current().hide();
     */
    async hide(): Promise<void>{
        await invoke("FLURX|web_window::hide", this.identifier);
    }

    static current(): WebWindow{
        return new WebWindow("<CURRENT_IDENTIFIER>");
    }
}
