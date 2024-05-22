import {invoke} from "./core";


/**
 *  Hides the window.
 */
export const hide = (): Promise<void> => invoke("plugin_app_window_hide");
