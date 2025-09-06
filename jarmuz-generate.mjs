#!/usr/bin/env node

import { jarmuz } from "jarmuz";

jarmuz({
  once: true,
  pipeline: [
    "cargo-build",
    "tcm",
    "tsc",
    "esbuild-development",
    "poet-generate",
  ],
  watch: ["resources", "src"],
}).decide(function ({ schedule }) {
  schedule("cargo-build");
});
