import { join } from "pathe";
import { beforeAll, describe, expect, it } from "vitest";
import which from "which";
import { createTestDir } from "../../../test/test-utils.js";
import { execSuccess } from "../../utils/exec-async.js";
import { packDebArchive } from "./pack.js";

const tempDir = createTestDir("deb/pack");

let target: string;

const options = {
  name: "test",
  version: "1.0.0",
  description: "Test package",
  architecture: "all",
  maintainer: {
    name: "John Smith",
    email: "john.smith@example.com",
  },
};

const hasDebDpkg = which.sync("dpkg-deb");

beforeAll(async () => {
  await tempDir.delete();
  const dir = await tempDir.mkdir();
  target = join(dir, "test.deb");
  await packDebArchive(target, options);
});

describe.runIf(hasDebDpkg)("dpkg-deb verification", () => {
  beforeAll(async () => {
    const { stdout } = await execSuccess("dpkg-deb", ["-f", target]);
    console.log("Created deb package at", target);
    console.log(stdout.toString().trim());
  });
  async function askDpkgDebForField(field: string) {
    const { stdout } = await execSuccess("dpkg-deb", ["-f", target, field]);
    return stdout.toString().trim();
  }

  it("has correct package name", async () => {
    expect(await askDpkgDebForField("Package")).toBe(options.name);
  });

  it("has correct version", async () => {
    expect(await askDpkgDebForField("Version")).toBe(options.version);
  });

  it("has correct description", async () => {
    expect(await askDpkgDebForField("Description")).toBe(options.description);
  });

  it("has correct maintainer", async () => {
    expect(await askDpkgDebForField("Maintainer")).toBe(`${options.maintainer.name} <${options.maintainer.email}>`);
  });

  it("has correct architecture", async () => {
    expect(await askDpkgDebForField("Architecture")).toBe(options.architecture);
  });
});
