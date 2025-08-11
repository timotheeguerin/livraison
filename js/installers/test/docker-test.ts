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
  osVersion: string;
  platform: "linux" | "windows";
  verbose: boolean;
  cleanup: boolean;
}

const defaultConfig: TestConfig = {
  shellToTest: "bash",
  osVersion: "22.04",
  platform: "linux",
  verbose: false,
  cleanup: true,
};

const validLinuxShells = ["bash", "zsh"] as const;
const validWindowsShells = ["pwsh", "powershell", "cmd"] as const;
const validUbuntuVersions = ["20.04", "22.04", "24.04"] as const;
const validWindowsVersions = ["ltsc2019", "ltsc2022"] as const;

type LinuxShell = (typeof validLinuxShells)[number];
type WindowsShell = (typeof validWindowsShells)[number];
type Shell = LinuxShell | WindowsShell;
type UbuntuVersion = (typeof validUbuntuVersions)[number];
type WindowsVersion = (typeof validWindowsVersions)[number];

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
  process.stdout.write("Test the Livraison installer with Docker containers\n");
  process.stdout.write("\n");
  process.stdout.write("OPTIONS:\n");
  process.stdout.write("  --platform, -p PLATFORM  Platform to test (linux, windows) [default: linux]\n");
  process.stdout.write("  --shell, -s SHELL        Shell to test:\n");
  process.stdout.write("                           Linux: bash, zsh [default: bash]\n");
  process.stdout.write("                           Windows: pwsh, powershell, cmd (for display only) [default: pwsh]\n");
  process.stdout.write("                           Note: Windows tests use PowerShell internally for all shells\n");
  process.stdout.write("  --ubuntu, -u VERSION     Ubuntu version (20.04, 22.04, 24.04) [default: 22.04]\n");
  process.stdout.write("  --windows, -w VERSION    Windows version (ltsc2019, ltsc2022) [default: ltsc2022]\n");
  process.stdout.write("  --verbose, -v            Enable verbose output\n");
  process.stdout.write("  --no-cleanup             Don't cleanup Docker containers after test\n");
  process.stdout.write("  --help, -h               Show this help message\n");
  process.stdout.write("\n");
  process.stdout.write("EXAMPLES:\n");
  process.stdout.write(`  ${scriptName}                              # Test bash on Ubuntu 22.04\n`);
  process.stdout.write(`  ${scriptName} --shell zsh                  # Test zsh on Ubuntu 22.04\n`);
  process.stdout.write(`  ${scriptName} --platform windows           # Test pwsh on Windows Server 2022\n`);
  process.stdout.write(`  ${scriptName} -p windows -s powershell     # Test Windows PowerShell 5.x\n`);
  process.stdout.write(`  ${scriptName} -p windows -w ltsc2019       # Test pwsh on Windows Server 2019\n`);
  process.stdout.write(
    `  ${scriptName} --ubuntu 24.04 --verbose     # Test bash on Ubuntu 24.04 with verbose output\n`,
  );
};

// Argument parsing
const parseCliArgs = (config: TestConfig): TestConfig => {
  const { values } = parseArgs({
    args: process.argv.slice(2),
    options: {
      "platform": {
        type: "string",
        short: "p",
        default: config.platform,
      },
      "shell": {
        type: "string",
        short: "s",
        default: config.shellToTest,
      },
      "ubuntu": {
        type: "string",
        short: "u",
        default: config.platform === "linux" ? config.osVersion : "22.04",
      },
      "windows": {
        type: "string",
        short: "w",
        default: config.platform === "windows" ? config.osVersion : "ltsc2022",
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

  // Handle platform
  if (values.platform) {
    const platform = values.platform as "linux" | "windows";
    if (platform !== "linux" && platform !== "windows") {
      log.error(`Invalid platform: ${platform}. Valid options: linux, windows`);
      process.exit(1);
    }
    newConfig.platform = platform;

    // Set default shell for platform if not specified
    if (!values.shell) {
      newConfig.shellToTest = platform === "windows" ? "pwsh" : "bash";
    }

    // Set default OS version for platform if not specified
    if (!values.ubuntu && !values.windows) {
      newConfig.osVersion = platform === "windows" ? "ltsc2022" : "22.04";
    }
  }

  // Handle shell validation based on platform
  if (values.shell) {
    const shell = values.shell as Shell;

    if (newConfig.platform === "windows") {
      if (!validWindowsShells.includes(shell as WindowsShell)) {
        log.error(`Invalid shell for Windows: ${shell}. Valid options: ${validWindowsShells.join(", ")}`);
        process.exit(1);
      }
    } else {
      if (!validLinuxShells.includes(shell as LinuxShell)) {
        log.error(`Invalid shell for Linux: ${shell}. Valid options: ${validLinuxShells.join(", ")}`);
        process.exit(1);
      }
    }
    newConfig.shellToTest = shell;
  }

  // Handle OS version based on platform
  if (newConfig.platform === "linux" && values.ubuntu) {
    const ubuntu = values.ubuntu as UbuntuVersion;
    if (!validUbuntuVersions.includes(ubuntu)) {
      log.error(`Invalid Ubuntu version: ${ubuntu}. Valid options: ${validUbuntuVersions.join(", ")}`);
      process.exit(1);
    }
    newConfig.osVersion = ubuntu;
  }

  if (newConfig.platform === "windows" && values.windows) {
    const windows = values.windows as WindowsVersion;
    if (!validWindowsVersions.includes(windows)) {
      log.error(`Invalid Windows version: ${windows}. Valid options: ${validWindowsVersions.join(", ")}`);
      process.exit(1);
    }
    newConfig.osVersion = windows;
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
const checkDependencies = async (config: TestConfig): Promise<void> => {
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

  // Check Docker OS compatibility
  if (config.platform === "windows") {
    try {
      const { stdout } = await execa("docker", ["info", "--format", "{{.OSType}}"]);
      if (!stdout.includes("windows")) {
        log.warn("Docker is not configured for Windows containers. Please switch to Windows containers mode.");
        log.warn("You can switch using: Docker Desktop → right-click → Switch to Windows containers");
      }
    } catch {
      log.warn("Could not determine Docker OS type. Ensure Docker is configured for Windows containers.");
    }
  }

  log.info("All dependencies satisfied");
};

// Docker image creation
const buildTestImage = async (config: TestConfig, osVersion: string, shellName: string): Promise<void> => {
  const platform = config.platform;
  const imageName =
    platform === "windows"
      ? `livraison-test-windows:${osVersion}-${shellName}`
      : `livraison-test:ubuntu-${osVersion}-${shellName}`;

  log.verbose(`Building Docker image: ${imageName}`, config.verbose);

  // Create temporary directory for Docker context
  const contextDir = join(testDir, "temp", `docker-context-${platform}-${osVersion}-${shellName}`);
  await mkdir(contextDir, { recursive: true });

  try {
    // Copy test scripts based on platform
    const testFiles = await readdir(testDir);
    let scriptFiles: string[];

    if (platform === "windows") {
      scriptFiles = testFiles.filter((file: string) => file.endsWith(".ps1"));
    } else {
      scriptFiles = testFiles.filter((file: string) => file.endsWith(".sh"));
    }

    for (const file of scriptFiles) {
      await copyFile(join(testDir, file), join(contextDir, file));
    }

    // Copy installer script from parent directory
    const installerScript = platform === "windows" ? "install.ps1" : "install.sh";
    await copyFile(join(testDir, "..", installerScript), join(contextDir, installerScript));

    // Copy the appropriate Dockerfile to context directory
    const dockerfileName = platform === "windows" ? "Dockerfile.windows" : "Dockerfile";
    await copyFile(join(testDir, dockerfileName), join(contextDir, "Dockerfile"));

    // Build the image with build arguments
    const buildArgs = [
      "build",
      ...(platform === "linux" ? ["--platform", "linux/amd64"] : []),
      "--build-arg",
      platform === "windows" ? `WINDOWS_VERSION=${osVersion}` : `UBUNTU_VERSION=${osVersion}`,
      "--build-arg",
      `SHELL_NAME=${shellName}`,
      "-t",
      imageName,
      contextDir,
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
const runDockerTest = async (config: TestConfig, osVersion: string, shellName: string): Promise<void> => {
  const platform = config.platform;
  const testName =
    platform === "windows" ? `Windows Server ${osVersion} with ${shellName}` : `Ubuntu ${osVersion} with ${shellName}`;

  const imageName =
    platform === "windows"
      ? `livraison-test-windows:${osVersion}-${shellName}`
      : `livraison-test:ubuntu-${osVersion}-${shellName}`;

  const containerName =
    platform === "windows"
      ? `livraison-test-windows-${osVersion}-${shellName}-${Date.now()}`
      : `livraison-test-${osVersion.replace(/\./g, "")}-${shellName}-${Date.now()}`;

  log.info(`Running test: ${testName}`);

  // Build test image
  await buildTestImage(config, osVersion, shellName);

  log.verbose(`Starting container: ${containerName}`, config.verbose);

  let runArgs: string[];

  if (platform === "windows") {
    // For all Windows shells, use the PowerShell script
    runArgs = [
      "run",
      "--rm",
      "--name",
      containerName,
      imageName,
      "pwsh",
      "-Command",
      `& C:\\test\\install-and-verify.ps1`,
    ];
  } else {
    // Linux containers
    runArgs = [
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
  }

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
      const filter =
        config.platform === "windows" ? "reference=livraison-test-windows:*" : "reference=livraison-test:*";

      const { stdout } = await execa("docker", ["images", "--filter", filter, "-q"]);
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
    log.info(`Testing Livraison installer with Docker on ${config.platform} containers`);
    log.info(`Test directory: ${testDir}`);

    await checkDependencies(config);

    // Run the single test
    await runDockerTest(config, config.osVersion, config.shellToTest);

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
