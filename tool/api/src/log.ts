import {emit} from "./core";

export namespace log {
    /**
     * Requests to execute `println!` on main process.
     *
     * @example
     * import {log} from "bevy_flurx_api";
     * log.println("message");
     */
    export const println = (message: any) => emit("FLURX|log::println", {
        message: convertToString(message)
    });

    /**
     * Requests to execute `log::trace!` on main process.
     *
     * @example
     * import {log} from "bevy_flurx_api";
     * log.trace("message");
     */
    export const trace = (message: any) => emitLog(message, "trace");

    /**
     * Requests to execute `log::info!` on main process.
     *
     * @example
     * import {log} from "bevy_flurx_api";
     * log.info("message");
     */
    export const info = (message: any) => emitLog(message, "info");

    /**
     * Requests to execute `log::warn!` on main process.
     *
     * @example
     * import {log} from "bevy_flurx_api";
     * log.warn("message");
     */
    export const warn = (message: any) => emitLog(message, "warn");

    /**
     * Requests to execute `log::error!` on main process.
     *
     * @example
     * import {log} from "bevy_flurx_api";
     * log.error("message");
     */
    export const error = (message: any) => emitLog(message, "error");

    type LogLevel = "trace" | "debug" | "info" | "warn" | "error";

    const emitLog = (message: any, level: LogLevel) => {
        emit("FLURX|log::log", {
            message: convertToString(message),
            level
        })
    };

    const convertToString = (message: any) => typeof message === "object" ? JSON.stringify(message, null, 2) : message.toString()
}
