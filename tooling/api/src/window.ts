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
 * import {window} from "@bevy_flurx_wry/api";
 *
 * const monitors: window.Monitor[] = await window.availableMonitors();
 */
export const availableMonitors = async (): Promise<Monitor[]> => {
    return await invoke("FLURX|window::available_monitors");
}