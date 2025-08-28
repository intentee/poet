import { jarmuz } from "jarmuz";

export function run({ development, once = false, rustJobs }) {
  const esbuildJob = development ? "esbuild-development" : "esbuild-production";

  jarmuz({
    once,
    pipeline: ["tcm", "tsc", esbuildJob, ...rustJobs],
    watch: ["resources", "src"],
  }).decide(function ({ matches, schedule }) {
    switch (true) {
      case matches("resources/**/*.{ts,tsx}"):
        schedule("tsc");
        break;
      case matches("resources/ts/**/*.css"):
        schedule("tcm");
      case matches("resources/**/*.css"):
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
