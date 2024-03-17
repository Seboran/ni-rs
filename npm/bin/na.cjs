#!/usr/bin/env node
const { spawn } = require("node:child_process");

const ls = spawn(require.resolve("../dist/na"), process.argv.slice(2));

ls.stdout.on("data", (/** @type {string | Uint8Array} */ data) => {
  process.stdout.write(data);
});
