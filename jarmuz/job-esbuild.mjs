import * as esbuild from "esbuild";
import { emptyDir } from "fs-extra";
import { readFile, writeFile } from "fs/promises";
import { glob } from "glob";
import { load } from "js-toml";
import path from "path";

import { basic } from "jarmuz/job-types";

const METAFILE_FILENAME = "esbuild-meta.json";
const PUBLIC_PATH = "/assets/";

export function jobEsbuild({ development }) {
  basic(async function ({ baseDirectory, buildId, printSubtreeList }) {
    let start = performance.now();

    console.log(`Building assets: ${buildId}`);

    const outdir = path.join(baseDirectory, "assets");

    await emptyDir(outdir);

    const inject = await glob(["resources/ts/polyfill_*.{ts,tsx}"]);

    const entryPoints = await glob([
      "resources/css/{component,fragment,global,layout,page}-*.css",
      "resources/media/**/*.{avif,gif,jpg,jpeg,png,svg,webp}",
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
      assetNames: `[name]_[hash]`,
      entryNames: `[name]_[hash]`,
      metafile: true,
      define: {
        "process.env.NODE_ENV": JSON.stringify(
          development ? "development" : "production",
        ),
        __BUILD_ID: JSON.stringify(buildId),
        __DEV__: JSON.stringify(String(development)),
        __PUBLIC_PATH: JSON.stringify(PUBLIC_PATH),
      },
      inject,
      preserveSymlinks: true,
      publicPath: PUBLIC_PATH,
      treeShaking: true,
      tsconfig: "tsconfig.json",
    };

    console.log("");

    const result = await esbuild.build(settings);

    await writeFile(METAFILE_FILENAME, JSON.stringify(result.metafile));

    console.log(`Build metafile written to: ${METAFILE_FILENAME}`);
    console.log(
      `Build finished with ID: ${buildId} in ${Math.round(performance.now() - start)} milliseconds`,
    );

    if (result.errors.length > 0 || result.warnings.length > 0) {
      return false;
    }
  });
}
