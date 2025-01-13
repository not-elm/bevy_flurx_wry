import {invoke} from "./core";

export namespace app {
    /**
     * Gets the application name.
     *
     * @example
     * import {app} from "bevy_flurx_api";
     * const name: string = await app.getName();
     */
    export const getName = (): Promise<string> => invoke("FLURX|app::get_name");

    /**
     * Gets the application version.
     *
     * @example
     * import {app} from "bevy_flurx_api";
     * const version: string = await app.getVersion();
     */
    export const getVersion = (): Promise<string> => invoke("FLURX|app::get_version");


    /**
     *  Exists the application.
     *
     *  @example
     *  import {app} from "bevy_flurx_api";
     *  await app.exit();
     */
    export const exit = (): Promise<void> => invoke("FLURX|app::exit");
}
