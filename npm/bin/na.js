#!/usr/bin/env node
import { spawn } from "child_process";
const ls = spawn(require.resolve("../dist/na"), process.argv.slice(2));

ls.stdout.on("data", (data) => {
  process.stdout.write(data);
});
