import {emit} from "./core";

export namespace log {
    /**
     * Requests to execute `println!` to aa process.
     */
    export const println = (message: string) => emit("FLURX|log::println", {
        message
    });
}
