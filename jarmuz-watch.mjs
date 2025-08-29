#!/usr/bin/env node

import { jarmuz } from "jarmuz";

jarmuz({
  once: false,
  pipeline: ["cargo-build", "poet-watch", "tcm", "tsc", "esbuild-development"],
  watch: ["resources", "src"],
}).decide(function ({ matches, schedule }) {
  switch (true) {
    case matches("src/**/*.rs"):
      schedule("cargo-build");
      schedule("poet-watch");
      return;
    case matches("resources/**/*.{ts,tsx}"):
      schedule("tsc");
      break;
    case matches("resources/ts/**/*.css"):
      schedule("tcm");
    case matches("resources/**/*.css"):
      schedule("esbuild-development");
      return;
  }
});
