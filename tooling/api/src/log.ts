import {emit} from "./core";

/**
 * Requests to execute `println!` to core process.
 */
export const println = (message: string) => emit("FLURX|log::println", {
    message
});

