/**
 * @typedef {object} Config
 * @property {Pipeline[]} pipelines
 */
type Config = {
  pipelines?: Pipeline[];
};

/**
 * @typedef {object} Pipeline
 * @property {string} name
 * @property {string[]} commands
 * @property {Step[]} steps
 * @property {Trigger} trigger
 */
type Pipeline = {
  name: string;
  steps: StepOrParallel[];
  triggers?: Trigger[];
  on_failure?: StepOrParallel[];
  on_success?: StepOrParallel[];
  on_abortion?: StepOrParallel[];
};

type StepOrParallel = Step | Parallel;
type Parallel = {
  parallel: Step[];
  on_failure?: StepOrParallel[];
  on_success?: StepOrParallel[];
  on_abortion?: StepOrParallel[];
};
/**
 * @typedef {object} Step
 * @property {boolean} non-blocking
 * @property {string} name
 * @property {string[]} commands
 */
type Step = {
  "non-blocking"?: boolean;
  name: string;
  commands: string[];
  on_failure?: StepOrParallel[];
  on_success?: StepOrParallel[];
  on_abortion?: StepOrParallel[];
};
/**
 * @typedef {object} Trigger
 * @property {Branch[]} branches - the branch that will trigger the pipe
 * @property {string[]} actions - the action that will trigger the pipe
 */
type Trigger = {
  branches?: string[];
  actions?: Action[];
};

/**
 * @typedef {string} Action - Define a trigger event
 */
type Action =
  | "applypatch-msg"
  | "pre-applypatch"
  | "post-apply-patch"
  | "pre-commit"
  | "prepare-commit-msg"
  | "commit-msg"
  | "post-commit"
  | "pre-rebase"
  | "post-checkout"
  | "post-merge"
  | "pre-receive"
  | "update"
  | "post-receive"
  | "post-update"
  | "pre-auto-gc"
  | "post-rewrite"
  | "pre-push"
  | "manual";

export type { Config, Pipeline, Step, Action };
