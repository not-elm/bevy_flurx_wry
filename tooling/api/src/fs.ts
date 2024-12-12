import {invoke} from "./core";

export interface FsWriteFileOptions{
    append?: boolean,
    recursive?: boolean,
}

/**
 * Copies a file to a destination.
 */
export const copyFile = async (source: string, destination: string): Promise<void> => {
    await invoke("FLURX|fs::copy_file", [source, destination])
}

/**
 * Creates a directory.
 *
 *  If you need to create the parent directory recursively, set `recursive` to `true`.
 */
export const createDir = async (
    dir: string,
    options?: {
        recursive?: boolean,
    }
): Promise<void> => {
    await invoke("FLURX|fs::create_dir", {dir, ...options});
}

/**
 *  Check if a path exists.
 */
export const exists = async (path: string): Promise<boolean> => {
    return await invoke("FLURX|fs::exists", path);
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
export const readTextFile = async (filePath: string): Promise<string> => {
    return await invoke("FLURX|fs::read_text_file", filePath);
}

/**
 * Removes a file.
 */
export const removeFile = async (filePath: string): Promise<void> => {
    await invoke("FLURX|fs::remove_file", filePath);
}

/**
 * Renames a file.
 */
export const renameFile = async (oldPath: string, newPath: string): Promise<void> => {
    await invoke("FLURX|fs::rename_file", [oldPath, newPath]);
}

/**
 * Writes a UTF-8 text file.
 */
export const writeFile = async (
    path: string,
    contents: string,
    options?: FsWriteFileOptions
): Promise<void> => {
    await invoke("FLURX|fs::write_file", {
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
    dir: string,
    options: {
        recursive: boolean
    }
): Promise<void> => {
    await invoke("FLURX|fs::remove_dir", {dir, ...options});
}