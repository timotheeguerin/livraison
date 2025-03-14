import { join } from "pathe";
import { beforeAll, describe, expect, it } from "vitest";
import { createTestDir, hasTool } from "../../../test/test-utils.js";
import { execSuccess } from "../../utils/exec-async.js";
import { packDebArchive } from "./pack.js";
import type { DebOptions } from "./types.js";

const tempDir = createTestDir("deb/pack");

let target: string;

const options: DebOptions = {
  name: "test",
  version: "1.0.0",
  revision: "12",
  description: "Great test package\nWith nice description",
  architecture: "all",
  priority: "optional",
  section: "misc",
  maintainer: {
    name: "John Smith",
    email: "john.smith@example.com",
  },
  depends: ["libc6", "libstdc++6"],
  conffiles: [
    {
      archivePath: "/etc/init.d/test",
      stats: {
        mode: 0o755,
      },
      content: `#! /bin/sh
do_start() {
  :
}

do_stop() {
  :
}

do_restart() {
  :
}

do_reload() {
  :
}

case $1 in
  start) do_start ;;
  stop) do_stop ;;
  force-reload) do_reload ;;
esac    
`,
    },
  ],
};

const hasDebDpkg = hasTool("dpkg-deb", "linux");
const hasLintian = hasTool("lintian", "linux");

beforeAll(async () => {
  await tempDir.delete();
  const dir = await tempDir.mkdir();
  target = join(dir, "test.deb");
  await packDebArchive(target, options);
});

describe.runIf(hasDebDpkg)("dpkg-deb verification", () => {
  // beforeAll(async () => {
  //   const { stdout } = await execSuccess("dpkg-deb", ["-f", target]);
  //   console.log("Created deb package at", target);
  //   console.log(stdout.toString().trim());
  // });
  async function askDpkgDebForField(field: string) {
    const { stdout } = await execSuccess("dpkg-deb", ["-f", target, field]);
    return stdout.toString().trim();
  }

  it("has correct package name", async () => {
    expect(await askDpkgDebForField("Package")).toBe(options.name);
  });

  it("has correct version", async () => {
    expect(await askDpkgDebForField("Version")).toBe(`${options.version}-${options.revision}`);
  });

  it("has correct maintainer", async () => {
    expect(await askDpkgDebForField("Maintainer")).toBe(`${options.maintainer.name} <${options.maintainer.email}>`);
  });

  it("has correct architecture", async () => {
    expect(await askDpkgDebForField("Architecture")).toBe(options.architecture);
  });

  it("has correct priority", async () => {
    expect(await askDpkgDebForField("Priority")).toBe(options.priority);
  });

  it("has correct section", async () => {
    expect(await askDpkgDebForField("Section")).toBe(options.section);
  });
});

describe.runIf(hasLintian)("lintian verification", () => {
  it("pass linter", async () => {
    const exclude = [
      "no-copyright-file",
      "no-changelog",
      "script-in-etc-init.d-not-registered-via-update-rc.d",
      "missing-systemd-service-for-init.d-script",
    ];
    const { code } = await execSuccess("lintian", ["-i", target, "--suppress-tags", exclude.join(",")]);
    expect(code).toBe(0);
  });
});
