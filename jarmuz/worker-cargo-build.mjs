import { command } from "jarmuz/job-types";

const features = process.env.FEATURES;
const featuresFlag = features ? ` --features ${features}` : "";
command(`cargo build --all-targets${featuresFlag}`);
