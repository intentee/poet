import notifier from "node-notifier";

import { spawner } from "jarmuz/job-types";

spawner(function ({ baseDirectory, buildId, command }) {
  notifier.notify({
    title: "poet",
    message: `Build ${buildId} finished`,
    timeout: 1,
  });

  return command(`target/debug/poet watch ${baseDirectory}`);
});
