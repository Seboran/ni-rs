#!/usr/bin/env node
const proc = Bun.spawn(
  [require.resolve("../dist/nrm"), ...process.argv.slice(2)],
  {
    stdout: "pipe",
  },
);

for await (const chunk of proc.stdout) {
  process.stdout.write(new TextDecoder().decode(chunk));
}
