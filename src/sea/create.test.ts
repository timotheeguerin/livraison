import { join } from "pathe";
import { beforeAll, expect, it } from "vitest";
import { TestReporter } from "../../test/test-reporter.js";
import { createTestDir } from "../../test/test-utils.js";
import { projectRoot } from "../utils/constants.js";
import { execAsync } from "../utils/exec-async.js";
import { isPathAccessible } from "../utils/fs-utils.js";
import { createExecutable } from "./create.js";

const tempDir = createTestDir("ar/create");

beforeAll(async () => {
  await tempDir.delete();
});

it("create basic executable", { timeout: 60_000 }, async () => {
  const dir = await tempDir.mkdir();
  const exePath = await createExecutable(new TestReporter(), {
    entrypoint: join(projectRoot, "test/fixtures/simple-app.js"),
    name: "test",
    outDir: dir,
  });

  expect(await isPathAccessible(exePath)).toBeTruthy();

  const { stdall } = await execAsync(exePath, []);
  expect(stdall.toString()).toBe("Worked\n");
});
