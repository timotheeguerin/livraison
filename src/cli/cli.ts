import { resolve } from "pathe";
import yargs from "yargs";
import { pack } from "../pack/deb/pack.js";
import { DynamicReporter } from "../reporters/dynamic.js";
import { createExecutable } from "../sea/create.js";
import { withErrors } from "./utils.js";

try {
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  await import("source-map-support/register.js");
} catch {
  // package only present in dev.
}

export const DEFAULT_PORT = 3000;

async function main() {
  await yargs(process.argv.slice(2))
    .scriptName("livraison")
    .strict()
    .help()
    .parserConfiguration({
      "greedy-arrays": false,
      "boolean-negation": false,
    })
    .option("debug", {
      type: "boolean",
      description: "Output debug log messages.",
      default: false,
    })
    .command(
      "compile <entrypoint>",
      "Compile a js script to a node standalone executable",
      (cmd) =>
        cmd
          .positional("entrypoint", {
            type: "string",
            demandOption: true,
          })
          .option("name", {
            type: "string",
            demandOption: true,
          })
          .option("out-dir", {
            type: "string",
          }),
      withErrors(async (args) => {
        const exePath = await createExecutable(new DynamicReporter(), {
          entrypoint: resolve(process.cwd(), args.entrypoint),
          name: args.name,
          outDir: resolve(process.cwd(), args.outDir ?? "dist"),
        });
        log(`Executable created at ${exePath}`);
        return [] as any;
      }),
    )
    .command(
      "pack",
      "Compile a js script to a node standalone executable",
      (cmd) =>
        cmd.option("out-dir", {
          type: "string",
        }),
      withErrors(async (args) => {
        await pack();
        return [] as any;
      }),
    )
    .demandCommand(1, "You need at least one command before moving on")
    .parse();
}

function log(message: string) {
  // eslint-disable-next-line no-console
  console.log(message);
}
main().catch((error) => {
  // eslint-disable-next-line no-console
  console.log("Error", error);
  process.exit(1);
});

process.on("unhandledRejection", (error: unknown) => {
  // eslint-disable-next-line no-console
  console.error("Unhandled promise rejection!", error);
  process.exit(1);
});

process.on("SIGTERM", () => process.exit(2));
process.on("SIGINT", () => process.exit(2));
process.on("SIGUSR1", () => process.exit(2));
process.on("SIGUSR2", () => process.exit(2));
