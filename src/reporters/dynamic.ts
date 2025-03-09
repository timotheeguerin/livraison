import pc from "picocolors";
import { BasicReporter, StatusIcons } from "./basic.js";
import type { Reporter, Task, TaskStatus } from "./types.js";
import { createSpinner } from "./utils.js";

class DynamicTask implements Task {
  #stream: NodeJS.WriteStream;
  #message: string;
  #spinner: () => string;
  #interval: NodeJS.Timeout | undefined;

  constructor(message: string, stream: NodeJS.WriteStream) {
    this.#message = message;
    this.#stream = stream;
    this.#spinner = createSpinner();
  }

  get message() {
    return this.#message;
  }

  set message(newMessage: string) {
    this.#message = newMessage;
    this.#printProgress();
  }

  start() {
    this.#interval = setInterval(() => {
      this.#printProgress();
    }, 300);
  }

  succeed(message?: string) {
    this.stop("success", message);
  }
  fail(message?: string) {
    this.stop("failure", message);
  }
  warn(message?: string) {
    this.stop("warn", message);
  }
  skip(message?: string) {
    this.stop("skipped", message);
  }

  stop(status: TaskStatus, message?: string) {
    if (message) {
      this.#message = message;
    }
    if (this.#interval) {
      clearInterval(this.#interval);
      this.#interval = undefined;
    }
    this.#stream.write(`\r${StatusIcons[status]} ${this.#message}\n`);
  }

  #printProgress() {
    this.#stream.clearLine(0);
    this.#stream.cursorTo(0);
    this.#stream.write(`\r${pc.yellow(this.#spinner())} ${this.#message}`);
  }
}
export class DynamicReporter extends BasicReporter implements Reporter {
  startTask(message: string): Task {
    if (!this.isTTY) {
      return super.startTask(message);
    }

    return new DynamicTask(message, this.stream);
  }
}
