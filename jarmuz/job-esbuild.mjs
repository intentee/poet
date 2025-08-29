import * as esbuild from "esbuild";
import { emptyDir } from "fs-extra";
import { readFile } from "fs/promises";
import { glob } from "glob";
import { load } from "js-toml";
import { writeFile } from "node:fs/promises";
import path from "path";

import { basic } from "jarmuz/job-types";

const metafileFilename = "esbuild-meta.json";

export function jobEsbuild({ development }) {
  basic(async function ({
    baseDirectory,
    buildId,
    printSubtreeList,
    resetConsole,
  }) {
    await resetConsole();

    console.log(`Building with ID: ${buildId}`);

    const { static_files_directory, static_files_public_path } = load(
      await readFile(`${baseDirectory}/poet.toml`, {
        encoding: "utf-8",
      }),
    );

    const outdir = path.join(baseDirectory, static_files_directory);
    const relativePath = path.relative(baseDirectory, static_files_directory);

    if (relativePath.startsWith(".") || path.isAbsolute(relativePath)) {
      console.error(
        "Suspicious static files directory path. Not cleaning it up.",
      );

      return false;
    }

    await emptyDir(outdir);

    const inject = await glob(["resources/ts/polyfill_*.{ts,tsx}"]);

    const entryPoints = await glob([
      "resources/css/{fragment,global,layout,page}-*.css",
      "resources/ts/{controller,global,worker}{_,-}*.{ts,tsx}",
    ]);

    printSubtreeList({
      title: "Entry points",
      items: entryPoints,
    });

    const settings = {
      outdir,
      bundle: true,
      entryPoints,
      minify: !development,
      sourcemap: true,
      splitting: true,
      format: "esm",
      target: "es2024",
      loader: {
        ".jpg": "file",
        ".otf": "file",
        ".png": "file",
        ".svg": "file",
        ".ttf": "file",
        ".webp": "file",
        ".woff2": "file",
      },
      assetNames: `[name]_${buildId}`,
      entryNames: `[name]_${buildId}`,
      metafile: true,
      define: {
        "process.env.NODE_ENV": JSON.stringify(
          development ? "development" : "production",
        ),
        __BUILD_ID: JSON.stringify(buildId),
        __DEV__: JSON.stringify(String(development)),
        __PUBLIC_PATH: JSON.stringify(static_files_public_path),
      },
      inject,
      preserveSymlinks: true,
      treeShaking: true,
      tsconfig: "tsconfig.json",
    };

    console.log("");

    const result = await esbuild.build(settings);

    await writeFile(metafileFilename, JSON.stringify(result.metafile));

    console.log(`Build metafile written to: ${metafileFilename}`);
    console.log(`Build finished with ID: ${buildId}`);

    if (result.errors.length > 0 || result.warnings.length > 0) {
      return false;
    }
  });
}
