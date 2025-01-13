import {invoke} from "./core";

export namespace path {
    /**
     *  Returns user's config path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const configPath: string | null = await path.config();
     */
    export const config = async (): Promise<string | null> => {
        return await invoke("FLURX|path::config");
    }

    /**
     *  Returns user's config local path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const configLocalPath: string | null = await path.configLocal();
     */
    export const configLocal = async (): Promise<string | null> => {
        return await invoke("FLURX|path::config_local");
    }

    /**
     *  Returns user's data path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const dataPath: string | null = await path.data();
     */
    export const data = async (): Promise<string | null> => {
        return await invoke("FLURX|path::data");
    }

    /**
     *  Returns user's data local path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const dataLocalPath: string | null = await path.dataLocal();
     */
    export const dataLocal = async (): Promise<string | null> => {
        return await invoke("FLURX|path::data_local");
    }

    /**
     *  Returns user's audio path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const audioPath: string | null = await path.audio();
     */
    export const audio = async (): Promise<string | null> => {
        return await invoke("FLURX|path::audio");
    }

    /**
     *  Returns user's cache path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const cachePath: string | null = await path.cache();
     */
    export const cache = async (): Promise<string | null> => {
        return await invoke("FLURX|path::cache");
    }

    /**
     *  Returns user's desktop path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const desktopPath: string | null = await path.desktop();
     */
    export const desktop = async (): Promise<string | null> => {
        return await invoke("FLURX|path::desktop");
    }

    /**
     *  Returns user's document path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const documentPath: string | null = await path.document();
     */
    export const document = async (): Promise<string | null> => {
        return await invoke("FLURX|path::document");
    }

    /**
     *  Returns user's download path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const downloadPath: string | null = await path.download();
     */
    export const download = async (): Promise<string | null> => {
        return await invoke("FLURX|path::download");
    }

    /**
     *  Returns user's executable path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const executablePath: string | null = await path.executable();
     */
    export const executable = async (): Promise<string | null> => {
        return await invoke("FLURX|path::executable");
    }

    /**
     *  Returns user's public path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const publicPath: string | null = await path.public();
     */
    export const publicDir = async (): Promise<string | null> => {
        return await invoke("FLURX|path::public");
    }

    /**
     *  Returns user's runtime path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const runtimePath: string | null = await path.runtime();
     */
    export const runtime = async (): Promise<string | null> => {
        return await invoke("FLURX|path::runtime");
    }

    /**
     *  Returns user's temp path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const tempPath: string | null = await path.temp();
     */
    export const temp = async (): Promise<string | null> => {
        return await invoke("FLURX|path::temp");
    }

    /**
     *  Returns user's template path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const templatePath: string | null = await path.template();
     */
    export const template = async (): Promise<string | null> => {
        return await invoke("FLURX|path::template");
    }

    /**
     *  Returns user's video path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const videoPath: string | null = await path.video();
     */
    export const video = async (): Promise<string | null> => {
        return await invoke("FLURX|path::video");
    }

    /**
     *  Returns user's home path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const homePath: string | null = await path.home();
     */
    export const home = async (): Promise<string | null> => {
        return await invoke("FLURX|path::home");
    }

    /**
     *  Returns user's picture path.
     *
     *  If the path doesn't exist or is not permitted by `AllowPaths`, will be null.
     *
     *  @example
     *  import {path} from "bevy_flurx_api";
     *  const picturePath: string | null = await path.picture();
     */
    export const picture = async (): Promise<string | null> => {
        return await invoke("FLURX|path::picture");
    }
}