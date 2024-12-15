import {invoke, PhysicalSize} from "./core";


export class WebWindow {
    private constructor(
        private readonly identifier: string,
    ) {
    }

    async title(): Promise<string> {
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
    async center(): Promise<void> {
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
    async hide(): Promise<void> {
        await invoke("FLURX|web_window::hide", this.identifier);
    }

    /**
     *  @example
     * import {WebWindow, PhysicalSize} from "@bevy_flurx_wry/api";
     *
     * const size: PhysicalSize = await WebWindow.current().innerSize();
     */
    async innerSize(): Promise<PhysicalSize> {
        return await invoke("FLURX|web_window::inner_size", this.identifier);
    }

    /**
     *  Get a current decorated state.
     *
     *  @example
     * import {WebWindow} from "@bevy_flurx_wry/api";
     *
     * const isDecorated: boolean = await WebWindow.current().isDecorated();
     */
    async isDecorated(): Promise<boolean> {
        return await invoke("FLURX|web_window::is_decorated", this.identifier);
    }

    /**
     *  Get a current focus state.
     *
     *  @example
     * import {WebWindow} from "@bevy_flurx_wry/api";
     *
     * const isDecorated: boolean = await WebWindow.current().isFocused();
     */
    async isFocused(): Promise<boolean> {
        return await invoke("FLURX|web_window::is_focused", this.identifier);
    }

    /**
     *  Get a current fullscreen state.
     *
     *  @example
     * import {WebWindow} from "@bevy_flurx_wry/api";
     *
     * const isFullscreen: boolean = await WebWindow.current().isFullscreen();
     */
    async isFullscreen(): Promise<boolean> {
        return await invoke("FLURX|web_window::is_fullscreen", this.identifier);
    }

    /**
     *  Return the window is maximized.
     *
     *  @example
     * import {WebWindow} from "@bevy_flurx_wry/api";
     *
     * const isMaximized: boolean = await WebWindow.current().isMaximized();
     */
    async isMaximized(): Promise<boolean> {
        return await invoke("FLURX|web_window::is_maximized", this.identifier);
    }

    static current(): WebWindow {
        return new WebWindow("<CURRENT_IDENTIFIER>");
    }
}
