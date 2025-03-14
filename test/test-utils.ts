import { mkdir, rm } from "fs/promises";
import { join } from "pathe";
import { isCI } from "std-env";
import which from "which";
import { projectRoot } from "../src/utils/constants.js";

const tempDir = join(projectRoot, "temp/test");

export interface TestDir {
  delete(): Promise<void>;
  mkdir(): Promise<string>;
}

export function createTestDir(name: string): TestDir {
  const baseDir = join(tempDir, name);

  return {
    async delete() {
      await rm(baseDir, { force: true, recursive: true });
    },
    async mkdir() {
      const r = (Math.random() + 1).toString(36).substring(7);
      const dir = join(baseDir, r);
      await mkdir(dir, { recursive: true });
      return dir;
    },
  };
}

export function hasTool(name: string, platform: typeof process.platform): boolean {
  const exists = !!which.sync(name, { nothrow: true });
  if (!exists && isCI && platform === process.platform) {
    throw new Error(`Tool ${name} is required for tests on ${platform} in CI`);
  }
  return exists;
}
