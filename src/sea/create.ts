import { copyFile, mkdir, mkdtemp, readdir } from "fs/promises";
import { readFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { basename, join, resolve } from "pathe";
import { inject } from "postject";
import { execSuccess } from "../utils/exec-async.js";
import { writeSeaConfig } from "./sea-config.js";

export interface CreateExecutableOptions {
  readonly entrypoint: string;
  /** Name of the exe (Without .exe extension) */
  readonly name: string;
  readonly outDir: string;
}

const projectRoot = resolve(import.meta.dirname, "..", "..");

export async function createExecutable(options: CreateExecutableOptions) {
  await mkdir(options.outDir, { recursive: true });
  const tempDir = await createTempDir();

  const seaConfigPath = join(tempDir, "sea-config.json");
  const blobPath = join(tempDir, "sea-prep.blob");
  await createSeaConfig(options, seaConfigPath, blobPath);
  const tempNodeExe = await copyNodeExecutable(tempDir);
  await removeSignature(tempNodeExe);
  await createBlob(seaConfigPath);
  await injectBlob(tempNodeExe, blobPath);
  await postProcessing(tempNodeExe);
  await resign(tempNodeExe);

  const exeName = process.platform === "win32" ? `${options.name}.exe` : options.name;
  const exePath = join(options.outDir, exeName);
  await copyFile(tempNodeExe, exePath);

  return exePath;
}

async function createTempDir() {
  const dir = join(tmpdir(), "livraison-sea-");
  return await mkdtemp(dir);
}

async function copyNodeExecutable(tempDir: string) {
  const nodeExe = process.execPath;
  const name = basename(nodeExe);
  const newFile = join(tempDir, name);
  await copyFile(nodeExe, newFile);
  return newFile;
}

async function removeSignature(exePath: string) {
  if (process.platform === "darwin") {
    await execSuccess("codesign", ["--remove-signature", exePath]);
  } else if (process.platform === "win32") {
    const signToolPath = await findWindowsSigntool();
    if (signToolPath) {
      await execSuccess(signToolPath, [`remove`, `/s`, exePath]);
    } else {
      throw new Error("Cannot find signtool.exe in CI");
    }
  }
}

async function resign(exePath: string) {
  if (process.platform === "darwin") {
    // await execSuccess("codesign", ["--sign", "-", exePath]);
  } else if (process.platform === "win32") {
    // This should be left to a signing service
  }
}

async function createBlob(seaConfigPath: string) {
  await execSuccess("node", ["--experimental-sea-config", seaConfigPath]);
}

async function injectBlob(exePath: string, blobPath: string) {
  const blob = await readFile(blobPath);
  if (process.platform === "darwin") {
    await inject(exePath, "NODE_SEA_BLOB", blob, {
      sentinelFuse: "NODE_SEA_FUSE_fce680ab2cc467b6e072b8b5df1996b2",
      machoSegmentName: "NODE_SEA",
    });
  } else {
    await inject(exePath, "NODE_SEA_BLOB", blob, {
      sentinelFuse: "NODE_SEA_FUSE_fce680ab2cc467b6e072b8b5df1996b2",
    });
  }
  // if (process.platform === "darwin") {
  //   await execSuccess("postject", [
  //     exePath,
  //     "NODE_SEA_BLOB",
  //     blobPath,
  //     "--sentinel-fuse",
  //     "NODE_SEA_FUSE_fce680ab2cc467b6e072b8b5df1996b2",
  //     "--macho-segment-name",
  //     "NODE_SEA",
  //   ]);
  // } else {
  //   await execSuccess("postject", [
  //     exePath,
  //     "NODE_SEA_BLOB",
  //     blobPath,
  //     "--sentinel-fuse",
  //     "NODE_SEA_FUSE_fce680ab2cc467b6e072b8b5df1996b2",
  //   ]);
  // }
}

async function postProcessing(exePath: string) {
  // On osx we need to register some entitlements for the app to run.
  if (process.platform === "darwin") {
    const entitlementsPath = join(projectRoot, "assets/osx-entitlements.plist");
    await execSuccess("codesign", [
      "--deep",
      "-s",
      "-",
      "-f",
      "--options",
      "runtime",
      "--entitlements",
      entitlementsPath,
      exePath,
    ]);
  }
}
async function createSeaConfig(options: CreateExecutableOptions, seaConfigPath: string, blobPath: string) {
  await writeSeaConfig(seaConfigPath, {
    main: options.entrypoint,
    output: blobPath,
    disableExperimentalSEAWarning: true,
    useCodeCache: false,
  });
}

async function findWindowsSigntool() {
  try {
    const base = "C:/Program Files (x86)/Windows Kits/10/bin/";

    const files = await readdir(base);
    const latest = files
      .filter((f) => f.startsWith("1"))
      .sort()
      .reverse()[0];

    const resolved = join(base, latest, "x64/signtool.exe");
    return resolved;
  } catch {
    return undefined;
  }
}
