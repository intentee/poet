import { persist } from "jarmuz/job-types";

persist(function ({ baseDirectory, keepAlive }) {
  return keepAlive(`target/debug/poet watch ${baseDirectory}`);
});
