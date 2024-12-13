import {invoke} from "./core";

export interface FsWriteFileOptions {
    append?: boolean,
    recursive?: boolean,
}

export interface FsBaseDirectoryOption {
    dir?: BaseDirectory,
}

export interface FsDirOptions {
    dir?: BaseDirectory,
    recursive?: boolean,
}

export type BaseDirectory =
    "ConfigLocal" |
    "Data" |
    "LocalData" |
    "Audio" |
    "Cache" |
    "Config" |
    "Desktop" |
    "Document" |
    "Download" |
    "Executable" |
    "Font" |
    "Home" |
    "Picture" |
    "Public" |
    "Runtime" |
    "Template" |
    "Video"


export interface CopyFileOptions {
    fromBaseDir?: BaseDirectory,
    toBaseDir?: BaseDirectory,
}

export interface RenameFileOptions {
    oldDir?: BaseDirectory,
    newDir?: BaseDirectory,
}

/**
 * Copies a file to a destination.
 */
export const copyFile = async (
    from: string,
    to: string,
    options?: CopyFileOptions,
): Promise<void> => {
    await invoke("FLURX|fs::copy_file", {
        from,
        to,
        ...options
    })
}

/**
 * Creates a directory.
 *
 *  If you need to create the parent directory recursively, set `recursive` to `true`.
 */
export const createDir = async (
    path: string,
    options?: FsDirOptions
): Promise<void> => {
    await invoke("FLURX|fs::create_dir", {path, ...options});
}

/**
 *  Check if a path exists.
 */
export const exists = async (
    path: string,
    options?: FsBaseDirectoryOption,
): Promise<boolean> => {
    return await invoke("FLURX|fs::exists", {
        path,
        ...options
    });
}

/**
 * Reads a file as byte array.
 */
export const readBinaryFile = async (filePath: string): Promise<Uint8Array> => {
    return await invoke("FLURX|fs::read_binary_file", filePath);
}

/**
 * Reads a file as a UTF-8 encoded string.
 */
export const readTextFile = async (
    path: string,
    options?: FsBaseDirectoryOption,
): Promise<string> => {
    return await invoke("FLURX|fs::read_text_file", {
        path,
        ...options
    });
}

/**
 * Removes a file.
 */
export const removeFile = async (
    path: string,
    options?: FsBaseDirectoryOption,
): Promise<void> => {
    await invoke("FLURX|fs::remove_file", {
        path,
        ...options
    });
}

/**
 * Renames a file.
 */
export const renameFile = async (
    oldPath: string,
    newPath: string,
    options?: RenameFileOptions,
): Promise<void> => {
    await invoke("FLURX|fs::rename_file", {
        oldPath,
        newPath,
        ...options
    });
}

/**
 * Writes a UTF-8 text file.
 */
export const writeTextFile = async (
    path: string,
    contents: string,
    options?: FsWriteFileOptions
): Promise<void> => {
    await invoke("FLURX|fs::write_text_file", {
        path,
        contents,
        ...options
    });
}

/**
 * Writes a file.
 */
export const writeBinaryFile = async (
    path: string,
    contents: Uint8Array,
    options?: FsWriteFileOptions
): Promise<void> => {
    await invoke("FLURX|fs::write_binary_file", {
        path,
        contents,
        ...options
    });
}

/**
 * List directory files.
 */
export const readDir = async (dirPath: string) => {
    throw new Error("not impl")
}

/**
 * Remove a directory.
 */
export const removeDir = async (
    path: string,
    options?: FsDirOptions,
): Promise<void> => {
    await invoke("FLURX|fs::remove_dir", {path, ...options});
}