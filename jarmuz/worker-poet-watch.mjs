import { spawner } from "jarmuz/job-types";

spawner(function ({ baseDirectory, background }) {
  return background(`target/debug/poet watch ${baseDirectory}`);
});
