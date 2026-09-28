import { command } from "jarmuz/job-types";

command(`
  npm exec prettier --
    --write
    jarmuz
    resources
    *.mjs
`);
