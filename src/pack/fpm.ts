import { execAppBuilder } from "./utils/app-builder.js";

interface FpmConfiguration {
  target: string;
  args: string[];
  customDepends?: string[];
  customRecommends?: string[];
  compression?: string | null;
}

export async function execFpm() {
  const artifactPath = "dist/foo";
  const fpmConfiguration: FpmConfiguration = {
    target: "deb",
    args: ["-t", "deb", `${artifactPath}=/usr/local/bin/foo`],
  };
  await execAppBuilder(["fpm", "--configuration", JSON.stringify(fpmConfiguration)]);
}
