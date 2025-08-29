import { spawner } from "jarmuz/job-types";

spawner(function ({ baseDirectory, command }) {
  return command(`target/debug/poet watch ${baseDirectory}`);
});
