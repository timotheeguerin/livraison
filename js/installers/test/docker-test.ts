import { execa } from "execa";
import { copyFile, mkdir, readdir, rm } from "fs/promises";
import { basename, dirname, join } from "path";
import pc from "picocolors";
import { parseArgs } from "util";

const packageRoot = dirname(import.meta.dirname);
const testDir = join(packageRoot, "test");

// Configuration
interface TestConfig {
  shellToTest: string;
  ubuntuVersion: string;
  verbose: boolean;
  cleanup: boolean;
}

const defaultConfig: TestConfig = {
  shellToTest: "bash",
  ubuntuVersion: "22.04",
  verbose: false,
  cleanup: true,
};

const validShells = ["bash", "zsh"] as const;
const validUbuntuVersions = ["20.04", "22.04", "24.04"] as const;

type Shell = (typeof validShells)[number];
type UbuntuVersion = (typeof validUbuntuVersions)[number];

// Logger functions
const log = {
  info: (message: string): void => {
    process.stdout.write(`${pc.green("[info]")} ${message}\n`);
  },
  warn: (message: string): void => {
    process.stdout.write(`${pc.yellow("[warn]")} ${message}\n`);
  },
  error: (message: string): void => {
    process.stderr.write(`${pc.red("[error]")} ${message}\n`);
  },
  verbose: (message: string, isVerbose: boolean): void => {
    if (isVerbose) {
      process.stdout.write(`${pc.yellow("[verbose]")} ${message}\n`);
    }
  },
};

// Usage display
const showUsage = (): void => {
  const scriptName = basename(process.argv[1]);
  process.stdout.write(`Usage: ${scriptName} [OPTIONS]\n`);
  process.stdout.write("\n");
  process.stdout.write("Test the Livraison installer with a specific shell and Ubuntu version using Docker\n");
  process.stdout.write("\n");
  process.stdout.write("OPTIONS:\n");
  process.stdout.write("  --shell, -s SHELL        Shell to test (bash, zsh) [default: bash]\n");
  process.stdout.write("  --ubuntu, -u VERSION     Ubuntu version to test (20.04, 22.04, 24.04) [default: 22.04]\n");
  process.stdout.write("  --verbose, -v            Enable verbose output\n");
  process.stdout.write("  --no-cleanup             Don't cleanup Docker containers after test\n");
  process.stdout.write("  --help, -h               Show this help message\n");
  process.stdout.write("\n");
  process.stdout.write("EXAMPLES:\n");
  process.stdout.write(`  ${scriptName}                          # Test bash on Ubuntu 22.04\n`);
  process.stdout.write(`  ${scriptName} --shell zsh              # Test zsh on Ubuntu 22.04\n`);
  process.stdout.write(`  ${scriptName} -s zsh -u 24.04          # Test zsh on Ubuntu 24.04 (short options)\n`);
  process.stdout.write(`  ${scriptName} --ubuntu 24.04 --verbose # Test bash on Ubuntu 24.04 with verbose output\n`);
};

// Argument parsing
const parseCliArgs = (config: TestConfig): TestConfig => {
  const { values } = parseArgs({
    args: process.argv.slice(2),
    options: {
      "shell": {
        type: "string",
        short: "s",
        default: config.shellToTest,
      },
      "ubuntu": {
        type: "string",
        short: "u",
        default: config.ubuntuVersion,
      },
      "verbose": {
        type: "boolean",
        short: "v",
        default: config.verbose,
      },
      "no-cleanup": {
        type: "boolean",
        default: !config.cleanup,
      },
      "help": {
        type: "boolean",
        short: "h",
        default: false,
      },
    },
    allowPositionals: false,
  });

  if (values.help) {
    showUsage();
    process.exit(0);
  }

  const newConfig = { ...config };

  if (values.shell) {
    const shell = values.shell as Shell;
    if (!validShells.includes(shell)) {
      log.error(`Invalid shell: ${shell}. Valid options: ${validShells.join(", ")}`);
      process.exit(1);
    }
    newConfig.shellToTest = shell;
  }

  if (values.ubuntu) {
    const ubuntu = values.ubuntu as UbuntuVersion;
    if (!validUbuntuVersions.includes(ubuntu)) {
      log.error(`Invalid Ubuntu version: ${ubuntu}. Valid options: ${validUbuntuVersions.join(", ")}`);
      process.exit(1);
    }
    newConfig.ubuntuVersion = ubuntu;
  }

  if (values.verbose) {
    newConfig.verbose = true;
  }

  if (values["no-cleanup"]) {
    newConfig.cleanup = false;
  }

  return newConfig;
};

// Dependency checking
const checkDependencies = async (): Promise<void> => {
  log.info("Checking dependencies...");

  try {
    await execa("docker", ["--version"], { stdio: "ignore" });
  } catch {
    log.error("Docker is required but not installed");
    process.exit(1);
  }

  try {
    await execa("docker", ["info"], { stdio: "ignore" });
  } catch {
    log.error("Docker daemon is not running");
    process.exit(1);
  }

  log.info("All dependencies satisfied");
};

// Docker image creation
const buildTestImage = async (config: TestConfig, ubuntuVersion: string, shellName: string): Promise<void> => {
  const imageName = `livraison-test:ubuntu-${ubuntuVersion}-${shellName}`;

  log.verbose(`Building Docker image: ${imageName}`, config.verbose);

  // Create temporary directory for Docker context
  const contextDir = join(testDir, "temp", `docker-context-${ubuntuVersion}-${shellName}`);
  await mkdir(contextDir, { recursive: true });

  try {
    // Copy test scripts
    const testFiles = await readdir(testDir);
    const shellFiles = testFiles.filter((file: string) => file.endsWith(".sh"));

    for (const file of shellFiles) {
      await copyFile(join(testDir, file), join(contextDir, file));
    }

    // Copy install.sh from parent directory
    await copyFile(join(testDir, "..", "install.sh"), join(contextDir, "install.sh"));

    // Copy the Dockerfile to context directory
    await copyFile(join(testDir, "Dockerfile"), join(contextDir, "Dockerfile"));

    // Build the image with build arguments
    const buildArgs = [
      "build",
      "--platform", "linux/amd64",
      "--build-arg", `UBUNTU_VERSION=${ubuntuVersion}`,
      "--build-arg", `SHELL_NAME=${shellName}`,
      "-t", imageName,
      contextDir
    ];

    if (config.verbose) {
      await execa("docker", buildArgs, { stdio: "inherit" });
    } else {
      await execa("docker", buildArgs, { stdio: "ignore" });
    }
  } finally {
    // Cleanup temporary context directory
    await rm(contextDir, { recursive: true, force: true });
  }
};

// Test execution
const runDockerTest = async (config: TestConfig, ubuntuVersion: string, shellName: string): Promise<void> => {
  const testName = `Ubuntu ${ubuntuVersion} with ${shellName}`;
  const imageName = `livraison-test:ubuntu-${ubuntuVersion}-${shellName}`;
  const containerName = `livraison-test-${ubuntuVersion.replace(/\./g, "")}-${shellName}-${Date.now()}`;

  log.info(`Running test: ${testName}`);

  // Build test image
  await buildTestImage(config, ubuntuVersion, shellName);

  log.verbose(`Starting container: ${containerName}`, config.verbose);

  const runArgs = [
    "run",
    "--platform",
    "linux/amd64",
    "--rm",
    "--name",
    containerName,
    imageName,
    "bash",
    "-c",
    `./install-and-verify.sh`,
  ];

  try {
    if (config.verbose) {
      await execa("docker", runArgs, { stdio: "inherit" });
    } else {
      await execa("docker", runArgs, { stdio: "ignore" });
    }
  } catch (error) {
    log.error(`Test failed: ${testName}`);
    throw error;
  }

  // Cleanup image if requested
  if (config.cleanup) {
    log.verbose(`Cleaning up Docker image: ${imageName}`, config.verbose);
    try {
      await execa("docker", ["rmi", imageName], { stdio: "ignore" });
    } catch {
      // Ignore cleanup failures
    }
  }

  log.info(`✓ Test passed: ${testName}`);
};

// Global cleanup
const cleanupDockerImages = async (config: TestConfig): Promise<void> => {
  if (config.cleanup) {
    log.verbose("Cleaning up any remaining Docker images...", config.verbose);
    try {
      const { stdout } = await execa("docker", ["images", "--filter", "reference=livraison-test:*", "-q"]);
      const imageIds = stdout
        .trim()
        .split("\n")
        .filter((id: string) => id);

      if (imageIds.length > 0) {
        await execa("docker", ["rmi", ...imageIds], { stdio: "ignore" });
      }
    } catch {
      // Ignore cleanup failures
    }
  }
};

// Main execution function
const runTests = async (): Promise<void> => {
  const config = parseCliArgs({ ...defaultConfig });

  try {
    log.info("Testing Livraison installer with Docker");
    log.info(`Test directory: ${testDir}`);

    await checkDependencies();

    // Run the single test
    await runDockerTest(config, config.ubuntuVersion, config.shellToTest);

    log.info("Test completed successfully! 🎉");
  } catch (error) {
    log.error(`Test failed: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  } finally {
    await cleanupDockerImages(config);
  }
};

// Handle cleanup on exit
const setupCleanupHandlers = (): void => {
  const config = parseCliArgs({ ...defaultConfig });

  process.on("SIGINT", async () => {
    await cleanupDockerImages(config);
    process.exit(130);
  });

  process.on("SIGTERM", async () => {
    await cleanupDockerImages(config);
    process.exit(143);
  });
};

// Main execution
setupCleanupHandlers();

await runTests();
