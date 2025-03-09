import type { SpawnOptions } from "child_process";
import crosspawn from "cross-spawn";
import { stdout } from "process";

export interface ExecResult {
  readonly code: number | null;
  readonly stdall: Buffer;
  readonly stdout: Buffer;
  readonly stderr: Buffer;
}
export function execAsync(cmd: string, args: string[], opts: SpawnOptions = {}): Promise<ExecResult> {
  return new Promise((resolve, reject) => {
    const child = crosspawn(cmd, args, opts);
    let stdall = Buffer.from("");
    let stdout = Buffer.from("");
    let stderr = Buffer.from("");

    if (child.stdout) {
      child.stdout.on("data", (data) => {
        stdout = Buffer.concat([stdout, data]);
        stdall = Buffer.concat([stdall, data]);
      });
    }

    if (child.stderr) {
      child.stderr.on("data", (data) => {
        stderr = Buffer.concat([stderr, data]);
        stdall = Buffer.concat([stdall, data]);
      });
    }

    child.on("error", (err) => {
      reject(err);
    });

    child.on("close", (code) => {
      resolve({ code, stdout, stderr, stdall });
    });
  });
}

export async function execSuccess(cmd: string, args: string[], opts: SpawnOptions = {}): Promise<ExecResult> {
  const result = await execAsync(cmd, args, opts);
  if (result.code !== 0) {
    throw new Error(`Command ${cmd} ${args.join(" ")} failed with code ${result.code}\n${result.stdall.toString()}`);
  }
  return result;
}
