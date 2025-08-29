#!/usr/bin/env node

import { jarmuz } from "jarmuz";

jarmuz({
  once: false,
  pipeline: ["cargo-build", "poet-watch", "tcm", "tsc", "esbuild-development"],
  watch: ["poet.toml", "resources", "src"],
}).decide(function ({ matches, schedule }) {
  switch (true) {
    case matches("src/**/*.rs"):
      schedule("cargo-build");
      return;
    case matches("resources/**/*.{ts,tsx}"):
      schedule("tsc");
      break;
    case matches("resources/ts/**/*.css"):
      schedule("tcm");
    case matches("poet.toml"):
    case matches("resources/**/*.css"):
      schedule("esbuild-development");
      return;
  }
});
