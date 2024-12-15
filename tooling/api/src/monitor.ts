import {invoke, PhysicalPosition, PhysicalSize} from "./core";

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
 * import {monitor, Monitor} from "@bevy_flurx_wry/api";
 *
 * const monitors: Monitor[] = await monitor.availables();
 */
export const availables = async (): Promise<Monitor[]> => {
    return await invoke("FLURX|monitor::availables");
}

/**
 *  Returns the current monitor info.
 *
 *  @example
 * import {monitor, Monitor} from "@bevy_flurx_wry/api";
 *
 * const currentMonitor: Monitor | null = await monitor.current();
 */
export const current = async (): Promise<Monitor | null> => {
    return await invoke("FLURX|monitor::current");
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