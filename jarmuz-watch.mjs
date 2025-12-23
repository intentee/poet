#!/usr/bin/env node

import { jarmuz } from "jarmuz";

jarmuz({
  once: false,
  pipeline: ["cargo-build", "poet-watch", "tcm", "tsc", "esbuild-development"],
  watch: ["resources", "poet", "rhai_components"],
}).decide(function ({ matches, schedule }) {
  switch (true) {
    case matches("**/*.rs"):
      schedule("cargo-build");
      return;
    case matches("resources/**/*.{ts,tsx}"):
      schedule("tsc");
      break;
    case matches("resources/ts/**/*.css"):
      schedule("tcm");
    case matches("resources/**/*.{avif,css,gif,jpg,jpeg,svg,webp}"):
      schedule("esbuild-development");
      return;
  }
});
