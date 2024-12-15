import {invoke} from "./core";

export type Arch =
    "x86" |
    "x86_64" |
    "arm" |
    "aarch64" |
    "m68k" |
    "mips" |
    "mips32r6" |
    "mips64" |
    "mips64r6" |
    "csky" |
    "powerpc" |
    "powerpc64" |
    "riscv32" |
    "riscv64" |
    "s390x" |
    "sparc" |
    "sparc64" |
    "hexagon" |
    "loongarch64";

export type Family = "unix" | "windows" | "itron" | "wasm";

/**
 * Get a describing the architecture of the CPU.
 *
 * @example
 * import {os} from "@bevy_flurx_wry/api";
 *
 * const arch: os.Arch = await os.arch();
 */
export const arch = async (): Promise<Arch> => {
    return await invoke("FLURX|os::arch");
}

/**
 * Get a describing the family of the operating system.
 *
 *  This value may be null if the family is unknown.
 *
 * @example
 * import {os} from "@bevy_flurx_wry/api";
 *
 * const arch: os.Family | null = await os.family();
 */
export const family = async (): Promise<Family | null> => {
    return await invoke("FLURX|os::family");
}

/**
 * Get the system version.
 *
 * @example
 * import {os} from "@bevy_flurx_wry/api";
 *
 * const osVersion: string | null = await os.version();
 */
export const version = async (): Promise<string | null> => {
    return await invoke("FLURX|os::os_version");
}

/**
 * Get the system version.
 *
 * @example
 * import {os} from "@bevy_flurx_wry/api";
 *
 * const osVersion: string | null = await os.longOsVersion();
 */
export const longOsVersion = async (): Promise<string | null> => {
    return await invoke("FLURX|os::long_os_version");
}

/**
 * Get the kernel version.
 *
 * @example
 * import {os} from "@bevy_flurx_wry/api";
 *
 * const kernelVersion: string | null = await os.kernelVersion();
 */
export const kernelVersion = async (): Promise<string | null> => {
    return await invoke("FLURX|os::kernel_version");
}

/**
 * Get the system name.
 *
 * @example
 * import {os} from "@bevy_flurx_wry/api";
 *
 * const name: string | null = await os.systemName();
 */
export const systemName = async (): Promise<string | null> => {
    return await invoke("FLURX|os::system_name");
}

/**
 * Get the system host name.
 *
 * @example
 * import {os} from "@bevy_flurx_wry/api";
 *
 * const name: string | null = await os.hostName();
 */
export const hostName = async (): Promise<string | null> => {
    return await invoke("FLURX|os::host_name");
}

/**
 * Get the preferred locale for the system.
 *
 * @example
 * import {os} from "@bevy_flurx_wry/api";
 *
 * const locale: string | null = await os.locale();
 */
export const locale = async (): Promise<string | null> => {
    return await invoke("FLURX|os::locale");
}