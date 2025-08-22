import { jarmuz } from "jarmuz";

export function run({ development, once = false, rustJobs }) {
  const esbuildJob = development ? "esbuild-development" : "esbuild-production";

  jarmuz({
    once,
    pipeline: ["tcm", "tsc", "eslint", esbuildJob, ...rustJobs],
    watch: ["resources", "src", "templates"],
  }).decide(function ({ matches, schedule }) {
    switch (true) {
      case matches("resources/**/*.{ts,tsx}"):
        schedule("tsc");
        schedule("eslint");
        break;
      case matches("resources/css/**/*.css"):
        schedule("tcm");
        schedule(esbuildJob);
        return;
      case matches("src/**/*.rs"):
        for (const job of rustJobs) {
          schedule(job);
        }
        return;
    }
  });
}
