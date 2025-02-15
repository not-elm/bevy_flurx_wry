import {invoke, PhysicalPosition, PhysicalSize} from "./core";

export type WindowMode = "fullscreen" | "borderless" | "windowed";

export interface WindowResolution {
    size?: PhysicalSize,
    scaleFactorOverride?: number,
}

export interface CreateWebWindowOptions {
    identifier: string,
    url: string,
    resolution?: WindowResolution,
    autoPlay?: boolean,
    // TODO: works later
    background?: string,
    /**
     * Use browser-specific accelerator keys?
     */
    browserAcceleratorKeys?: boolean,
    /**
     *  Whether not use devtools.
     */
    useDevtools?: boolean,
    /**
     *   Whether not is open devtools
     */
    isOpenDevtools?: boolean,
    /**
     *   Whether not is visible webview.
     */
    visible?: boolean,
    /**
     *  The useragent.
     */
    userAgent?: string,
    /**
     * Represents the theme apply the webview.
     */
    theme?: "auto" | "dark" | "light",
    initializeFocused?: boolean,
    incognito?: boolean,
    hotkeysZoom?: boolean,
    useHttpsScheme?: boolean,
    // OnDownload,
    // OnDragDrop,
    // OnNavigation,
    // OnNewWindowRequest,
}

export class WebWindow {
    private constructor(
        private readonly identifier: string,
    ) {
    }

    /**
     *  Listen the event where from bevy process.
     *
     *  @example
     * import {WebWindow} from "bevy_flurx_api";
     *
     * await WebWindow.current().listen("eventId", (e) => {
     *    console.log(e);
     * });
     */
    listen<E>(eventId: string, f: (event: E) => void) {
        const prop = `_event_${this.identifier}_${eventId}`;
        Object.defineProperty(window.__FLURX__, prop, {
            value: f,
            writable: false,
            configurable: true
        })
        return () => {
            Reflect.deleteProperty(window.__FLURX__, prop);
        };
    };

    /**
     *  Returns the position of the window.
     *
     *  The position could be null value if window already not exists or the platform not supported.
     *
     *  @example
     * import {WebWindow, PhysicalPosition} from "bevy_flurx_api";
     *
     * const position: PhysicalPosition | null = await WebWindow.current().innerPosition();
     */
    async innerPosition(): Promise<PhysicalPosition | null> {
        return await invoke("FLURX|web_window::inner_position", this.identifier);
    }

    /**
     *  Returns the position of the window.
     *
     *  The position could be null value if window already not exists or the platform not supported.
     *
     *  @example
     * import {WebWindow, PhysicalPosition} from "bevy_flurx_api";
     *
     * const position: PhysicalPosition | null = await WebWindow.current().outerPosition();
     */
    async outerPosition(): Promise<PhysicalPosition | null> {
        return await invoke("FLURX|web_window::outer_position", this.identifier);
    }

    /**
     *  Sets the position of the window.
     *
     *  @example
     * import {WebWindow} from "bevy_flurx_api";
     *
     * await WebWindow.current().setPosition({
     *    x: 100,
     *    y: 100,
     * });
     */
    async setPosition(position: PhysicalPosition): Promise<void> {
        await invoke("FLURX|web_window::set_position", [this.identifier, position]);
    }

    async title(): Promise<string> {
        return await invoke("FLURX|web_window::title", this.identifier);
    }

    /**
     *  Center the window.
     *
     *  @example
     * import {WebWindow} from "bevy_flurx_api";
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
     * import {WebWindow} from "bevy_flurx_api";
     *
     * await WebWindow.current().hide();
     */
    async hide(): Promise<void> {
        await invoke("FLURX|web_window::hide", this.identifier);
    }

    /**
     *  Shows the window.
     *
     *  @example
     * import {WebWindow} from "bevy_flurx_api";
     *
     * await WebWindow.current().show();
     */
    async show(): Promise<void> {
        await invoke("FLURX|web_window::show", this.identifier);
    }

    /**
     *  @example
     * import {WebWindow, PhysicalSize} from "bevy_flurx_api";
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
     * import {WebWindow} from "bevy_flurx_api";
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
     * import {WebWindow} from "bevy_flurx_api";
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
     * import {WebWindow} from "bevy_flurx_api";
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
     * import {WebWindow} from "bevy_flurx_api";
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
     * import {WebWindow} from "bevy_flurx_api";
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
     * import {WebWindow} from "bevy_flurx_api";
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
     * import {WebWindow} from "bevy_flurx_api";
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
     * import {WebWindow} from "bevy_flurx_api";
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
     * import {WebWindow} from "bevy_flurx_api";
     *
     * const isVisible: boolean = await WebWindow.current().isVisible();
     */
    async isVisible(): Promise<boolean> {
        return await invoke("FLURX|web_window::is_visible", this.identifier);
    }

    /**
     *  Maximizes the window.
     *
     *  @example
     * import {WebWindow} from "bevy_flurx_api";
     *
     *  await WebWindow.current().maximize();
     */
    async maximize(): Promise<void> {
        await invoke("FLURX|web_window::maximize", this.identifier);
    }

    /**
     *  UnMaximizes the window.
     *
     *  @example
     * import {WebWindow} from "bevy_flurx_api";
     *
     *  await WebWindow.current().unMaximize();
     */
    async unMaximize(): Promise<void> {
        await invoke("FLURX|web_window::un_maximize", this.identifier);
    }

    /**
     *  Minimizes the window.
     *
     *  @example
     * import {WebWindow} from "bevy_flurx_api";
     *
     *  await WebWindow.current().minimize();
     */
    async minimize(): Promise<void> {
        await invoke("FLURX|web_window::minimize", this.identifier);
    }

    /**
     *  UnMinimizes the window.
     *
     *  @example
     * import {WebWindow} from "bevy_flurx_api";
     *
     *  await WebWindow.current().unMinimize();
     */
    async unMinimize(): Promise<void> {
        await invoke("FLURX|web_window::un_minimize", this.identifier);
    }

    /**
     *  Sets whether window have enabled decorations?
     *
     *  @example
     * import {WebWindow} from "bevy_flurx_api";
     *
     *  await WebWindow.current().setDecorations(true);
     */
    async setDecorations(decorations: boolean): Promise<void> {
        await invoke("FLURX|web_window::set_decorations", [this.identifier, decorations]);
    }

    /**
     *  Sets  the window focus.
     *
     *  @example
     * import {WebWindow} from "bevy_flurx_api";
     *
     *  await WebWindow.current().focus();
     */
    async focus(): Promise<void> {
        await invoke("FLURX|web_window::focus", this.identifier);
    }

    /**
     *  UnFocus the window.
     *
     *  @example
     * import {WebWindow} from "bevy_flurx_api";
     *
     *  await WebWindow.current().unFocus();
     */
    async unFocus(): Promise<void> {
        await invoke("FLURX|web_window::un_focus", this.identifier);
    }

    /**
     *  Sets the window fullscreen.
     *
     *  @example
     * import {WebWindow} from "bevy_flurx_api";
     *
     *  await WebWindow.current().setWindowMode("fullscreen");
     */
    async setWindowMode(mode: WindowMode): Promise<void> {
        await invoke("FLURX|web_window::set_window_mode", [this.identifier, mode]);
    }

    /**
     *  Modifies whether the window catches cursor events.
     *
     *  @example
     * import {WebWindow} from "bevy_flurx_api";
     *
     *  await WebWindow.current().setCursorHitTest(true);
     */
    async setCursorHitTest(hitTest: boolean): Promise<void> {
        await invoke("FLURX|web_window::set_cursor_hit_test", [this.identifier, hitTest]);
    }

    static current(): WebWindow {
        return new WebWindow(window.__FLURX__.windowIdentifier);
    }

    static async newWindow(options: CreateWebWindowOptions): Promise<WebWindow> {
        await invoke("FLURX|webWindow::create", options);
        return new WebWindow(options.identifier)
    }
}

