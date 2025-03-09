import type { Reporter, Task, TaskStatus } from "../src/reporters/types.js";

export class TestReporter implements Reporter {
  log(message: string): void {}
  startTask(message: string): Task {
    const task: Task = {
      message,
      succeed: () => {},
      fail: () => {},
      warn: () => {},
      skip: () => {},
      stop: () => {},
    };
    return task;
  }
  async task(message: string, action: (task: Task) => Promise<TaskStatus | void>): Promise<void> {
    const task = this.startTask(message);
    await action(task);
  }
}
