import {invoke} from "./core";


/**
 *  Returns user's config path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const config = async (): Promise<string | null> => {
    return await invoke("FLURX|path::config");
}

/**
 *  Returns user's config local path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const configLocal = async (): Promise<string | null> => {
    return await invoke("FLURX|path::config_local");
}

/**
 *  Returns user's data path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const data = async (): Promise<string | null> => {
    return await invoke("FLURX|path::data");
}

/**
 *  Returns user's data local path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const dataLocal = async (): Promise<string | null> => {
    return await invoke("FLURX|path::data_local");
}

/**
 *  Returns user's audio path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const audio = async (): Promise<string | null> => {
    return await invoke("FLURX|path::audio");
}

/**
 *  Returns user's cache path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const cache = async (): Promise<string | null> => {
    return await invoke("FLURX|path::cache");
}

/**
 *  Returns user's desktop path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const desktop = async (): Promise<string | null> => {
    return await invoke("FLURX|path::desktop");
}

/**
 *  Returns user's document path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const document = async (): Promise<string | null> => {
    return await invoke("FLURX|path::document");
}

/**
 *  Returns user's download path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const download = async (): Promise<string | null> => {
    return await invoke("FLURX|path::download");
}

/**
 *  Returns user's executable path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const executable = async (): Promise<string | null> => {
    return await invoke("FLURX|path::executable");
}

/**
 *  Returns user's public path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const publicDir = async (): Promise<string | null> => {
    return await invoke("FLURX|path::public");
}

/**
 *  Returns user's runtime path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const runtime = async (): Promise<string | null> => {
    return await invoke("FLURX|path::runtime");
}

/**
 *  Returns user's temp path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const temp = async (): Promise<string | null> => {
    return await invoke("FLURX|path::temp");
}

/**
 *  Returns user's template path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const template = async (): Promise<string | null> => {
    return await invoke("FLURX|path::template");
}

/**
 *  Returns user's video path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const video = async (): Promise<string | null> => {
    return await invoke("FLURX|path::video");
}

/**
 *  Returns user's home path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const home = async (): Promise<string | null> => {
    return await invoke("FLURX|path::home");
}

/**
 *  Returns user's picture path.
 *
 *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
 */
export const picture = async (): Promise<string | null> => {
    return await invoke("FLURX|path::picture");
}
