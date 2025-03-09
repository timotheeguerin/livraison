export interface Reporter {
  readonly log: (message: string) => void;
  readonly startTask: (message: string) => Task;
  readonly task: (message: string, action: (task: Task) => Promise<TaskStatus | void>) => Promise<void>;
}

export interface Task {
  get message(): string;
  set message(value: string);

  readonly succeed: (message?: string) => void;
  readonly fail: (message?: string) => void;
  readonly warn: (message?: string) => void;
  readonly skip: (message?: string) => void;
  readonly stop: (taskStatus: TaskStatus, message?: string) => void;
}

export type TaskStatus = "success" | "failure" | "skipped" | "warn";
