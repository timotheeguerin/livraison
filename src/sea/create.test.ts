import { mkdir, rm } from "fs/promises";
import { join } from "pathe";
import { beforeAll, expect, it } from "vitest";
import { projectRoot } from "../utils/constants.js";
import { execAsync } from "../utils/exec-async.js";
import { isPathAccessible } from "../utils/fs-utils.js";
import { createExecutable } from "./create.js";
import { TestReporter } from "../../test/test-reporter.js";

const tempDir = join(projectRoot, "temp/test");

async function mkTestDir() {
  const r = (Math.random() + 1).toString(36).substring(7);
  const testTempDir = join(tempDir, r);
  await mkdir(testTempDir, { recursive: true });
  return testTempDir;
}

beforeAll(async () => {
  await rm(tempDir, { force: true, recursive: true });
});

it("create basic executable", { timeout: 60_000 }, async () => {
  const dir = await mkTestDir();
  const exePath = await createExecutable(new TestReporter(), {
    entrypoint: join(projectRoot, "test/fixtures/simple-app.js"),
    name: "test",
    outDir: dir,
  });

  expect(await isPathAccessible(exePath)).toBeTruthy();

  const { stdall } = await execAsync(exePath, []);
  expect(stdall.toString()).toBe("Worked\n");
});
