import {invoke} from "./core";


/**
 *  Hides the window.
 */
export const hide = (): Promise<void> => invoke("FLURX|app_window::hide");
