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
     *  Returns whether the window is decorated.
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
     *  Returns whether the window has focused.
     *
     *  @example
     * import {WebWindow} from "@bevy_flurx_wry/api";
     *
     * const hasFocused: boolean = await WebWindow.current().hasFocused();
     */
    async hasFocused(): Promise<boolean> {
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
     *  Returns whether the window is maximized.
     *
     *  @example
     * import {WebWindow} from "@bevy_flurx_wry/api";
     *
     * const isMaximized: boolean = await WebWindow.current().isMaximized();
     */
    async isMaximized(): Promise<boolean> {
        return await invoke("FLURX|web_window::is_maximized", this.identifier);
    }

    /**
     *  Returns whether the window can be maximized.
     *
     *  @example
     * import {WebWindow} from "@bevy_flurx_wry/api";
     *
     * const isMaximizable: boolean = await WebWindow.current().isMaximizable();
     */
    async isMaximizable(): Promise<boolean> {
        return await invoke("FLURX|web_window::is_maximizable", this.identifier);
    }

    /**
     *  Returns whether the window can be minimized.
     *
     *  @example
     * import {WebWindow} from "@bevy_flurx_wry/api";
     *
     * const isMaximizable: boolean = await WebWindow.current().isMinimizable();
     */
    async isMinimizable(): Promise<boolean> {
        return await invoke("FLURX|web_window::is_minimizable", this.identifier);
    }

    /**
     *  Returns whether the window is minimized.
     *
     *  @example
     * import {WebWindow} from "@bevy_flurx_wry/api";
     *
     * const isMinimized: boolean = await WebWindow.current().isMinimized();
     */
    async isMinimized(): Promise<boolean> {
        return await invoke("FLURX|web_window::is_minimized", this.identifier);
    }

    /**
     *  Returns whether the window can be resized.
     *
     *  @example
     * import {WebWindow} from "@bevy_flurx_wry/api";
     *
     * const isResizable: boolean = await WebWindow.current().isResizable();
     */
    async isResizable(): Promise<boolean> {
        return await invoke("FLURX|web_window::is_resizable", this.identifier);
    }

    /**
     *  Returns whether the window is visible.
     *
     *  @example
     * import {WebWindow} from "@bevy_flurx_wry/api";
     *
     * const isVisible: boolean = await WebWindow.current().isVisible();
     */
    async isVisible(): Promise<boolean> {
        return await invoke("FLURX|web_window::is_visible", this.identifier);
    }

    static current(): WebWindow {
        return new WebWindow("<CURRENT_IDENTIFIER>");
    }
}
