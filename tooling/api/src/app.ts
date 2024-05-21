import {invoke} from "./core";

/**
 * Show the window.
 */
export const show = (): Promise<void> => invoke("plugin_window_show", []);


/**
 *  Hides the window.
 */
export const hide = (): Promise<void> => invoke("plugin_window_hide", []);
