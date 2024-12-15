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