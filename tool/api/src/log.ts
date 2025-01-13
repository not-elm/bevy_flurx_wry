import {emit} from "./core";

export namespace log {
    /**
     * Requests to execute `println!` to aa process.
     *
     * @example
     * import {log} from "bevy_flurx_api";
     * log.println("message");
     */
    export const println = (message: string) => emit("FLURX|log::println", {
        message
    });
}
