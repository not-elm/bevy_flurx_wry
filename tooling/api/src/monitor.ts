import {invoke} from "./core";

export interface PhysicalPosition {
    x: number,
    y: number,
}

export interface PhysicalSize {
    width: number,
    height: number,
}

export interface Monitor {
    name: string | null,
    position: PhysicalPosition,
    scaleFactor: number,
    size: PhysicalSize,
}

/**
 *  Returns the monitor infos.
 *
 *  @example
 * import {appWindow, Monitor} from "@bevy_flurx_wry/api";
 *
 * const monitors: Monitor[] = await appWindow.availableMonitors();
 */
export const availableMonitors = async (): Promise<Monitor[]> => {
    return await invoke("FLURX|window::available_monitors");
}

/**
 *  Returns the current monitor info.
 *
 *  @example
 * import {appWindow, Monitor} from "@bevy_flurx_wry/api";
 *
 * const monitor: Monitor | null = await appWindow.currentMonitor();
 */
export const currentMonitor = async (): Promise<Monitor | null> => {
    return await invoke("FLURX|window::current_monitor");
}

/**
 *  Returns the primary monitor info.
 *
 *  @example
 * import {monitor, Monitor} from "@bevy_flurx_wry/api";
 *
 * const primaryMonitor: Monitor | null = await monitor.primary();
 */
export const primary = async (): Promise<Monitor | null> => {
    return await invoke("FLURX|monitor::primary");
}