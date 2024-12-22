import {invoke} from "./core";

export namespace app {
    /**
     * Gets the application name.
     */
    export const getName = (): Promise<string> => invoke("FLURX|app::get_name");

    /**
     * Gets the application version.
     */
    export const getVersion = (): Promise<string> => invoke("FLURX|app::get_version");


    /**
     *  Exists the application.
     */
    export const exit = (): Promise<void> => invoke("FLURX|app::exit");
}
