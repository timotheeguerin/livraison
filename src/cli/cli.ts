import "source-map-support/register.js";
import yargs from "yargs";
import { withErrors } from "./utils.js";

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
        cmd.positional("entrypoint", {
          type: "string",
        }),
      withErrors((args) => {
        return [] as any;
      }),
    )
    .demandCommand(1, "You need at least one command before moving on")
    .parse();
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
