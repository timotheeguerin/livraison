import pc from "picocolors";
import { isCI } from "std-env";
import { F_CHECK, F_CROSS, F_WARN } from "../utils/figures.js";
import type { Reporter, Task, TaskStatus } from "./types.js";

export const StatusIcons = {
  success: pc.green(F_CHECK),
  failure: pc.red(F_CROSS),
  warn: pc.gray(F_WARN),
  skipped: pc.gray("•"),
};

export class BasicReporter implements Reporter {
  isTTY = process.stdout?.isTTY && !isCI;
  protected stream = process.stderr;

  log(message: string) {
    this.stream.write(message + "\n");
  }

  startTask(message: string): Task {
    this.log(`${pc.yellow("-")} ${message}`);

    const task: Task = {
      message,
      succeed: (message) => {
        task.stop("success", message);
      },
      fail: (message) => {
        task.stop("failure", message);
      },
      warn: (message) => {
        task.stop("warn", message);
      },
      skip: (message) => {
        task.stop("skipped", message);
      },
      stop: (taskStatus, message) => {
        if (message) {
          task.message = message;
        }
        this.log(`${StatusIcons[taskStatus]} ${task.message}`);
      },
    };

    return task;
  }

  async task(message: string, action: (task: Task) => Promise<TaskStatus | void>) {
    const task = this.startTask(message);
    const status = await action(task);
    task.stop(status ?? "success");
  }

  getStatusChar(status: TaskStatus) {
    return StatusIcons[status];
  }
}
