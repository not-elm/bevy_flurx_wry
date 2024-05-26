// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import {defineConfig, Plugin} from 'rollup'
import typescript from '@rollup/plugin-typescript'
import fg from 'fast-glob'
import {join} from 'path'
import {copyFileSync, Dir, opendirSync, rmSync} from 'fs'
import {fileURLToPath} from "node:url";
import * as path from "node:path";
import terser from "@rollup/plugin-terser";


cleanDir(join(path.dirname(fileURLToPath(import.meta.url)), './dist'))


export default defineConfig([
    {
        input: join(path.dirname(fileURLToPath(import.meta.url)), 'src', 'index.ts'),
        output: [
            {
                format: 'esm',
                dir: './dist',
                preserveModules: true,
                preserveModulesRoot: 'src',
                entryFileNames: (chunkInfo) => {
                    if (chunkInfo.name.includes('node_modules')) {
                        return chunkInfo.name.replace('node_modules', 'external') + '.js'
                    }

                    return '[name].js'
                }
            },
            {
                format: 'cjs',
                dir: './dist',
                preserveModules: true,
                preserveModulesRoot: 'src',
                entryFileNames: (chunkInfo) => {
                    if (chunkInfo.name.includes('node_modules')) {
                        return chunkInfo.name.replace('node_modules', 'external') + '.cjs'
                    }

                    return '[name].cjs'
                }
            }
        ],
        plugins: [
            typescript({
                declaration: true,
                declarationDir: './dist',
                rootDir: 'src'
            }),
            makeFlatPackageInDist()
        ],
    },
    {
        input: 'src/index.ts',
        output: {
            format: 'iife',
            name: '__FLURX_IIFE__',
            footer: 'Object.defineProperty(window, "__FLURX__", { value: __FLURX_IIFE__ })',
            file: '../../crates/common/scripts/api.js'
        },
        plugins: [typescript(), terser()],
    }
])


function makeFlatPackageInDist(): Plugin {
    return {
        name: 'makeFlatPackageInDist',
        writeBundle() {
            // copy necessary files like `CHANGELOG.md` , `README.md` and Licenses to `./dist`
            fg.sync('(LICENSE*|*.md|package.json)').forEach((f) =>
                copyFileSync(f, `dist/${f}`)
            )
        }
    }
}

function cleanDir(path: string) {
    let dir: Dir
    try {
        dir = opendirSync(path)
    } catch (err: any) {
        switch (err.code) {
            case 'ENOENT':
                return
            case 'ENOTDIR':
                throw new Error(`'${path}' is not a directory.`)
            default:
                throw err
        }
    }

    let file = dir.readSync()
    while (file) {
        const filePath = join(path, file.name)
        rmSync(filePath, {recursive: true})
        file = dir.readSync()
    }
    dir.closeSync()
}
