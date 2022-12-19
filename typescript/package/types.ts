// Config, every types definition
/**
 * @typedef {object} defaultImport
 * @property {Config} config
 */
type defaultImport = {
  default: Config;
};
/**
 * @typedef {object} Config
 * @property {Pipeline[]} pipelines
 */
type Config = {
  pipelines: Pipeline[];
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
  steps: Step[];
  trigger?: Trigger | Trigger[];
};

/**
 * @typedef {object} Step
 * @property {boolean} non-blocking
 * @property {string} name
 * @property {string[]} commands
 */
type Step = {
  name: string;
  commands: string[];
  non_blocking?: boolean;
  on_failure: string[];
};

/**
 * @typedef {object} Trigger
 * @property {Branch[]} branches - the branch that will trigger the pipe
 * @property {string[]} actions - the action that will trigger the pipe
 */
type Trigger = {
  branch?: string | string[];
  hook?: Action | Action[];
};

/**
 * @typedef {object} ExecContext
 * @property {boolean} verbose
 */
type ExecContext = {
  verbose?: boolean;
};

/**
 * @typedef {object} ExecOptions
 * @property {boolean} non-blocking
 */
type ExecOptions = {
  "non-blocking"?: boolean;
};

const GitHooks = [
  "pre-commit",
  "pre-push",
  "pre-receive",
  "update",
  "post-receive",
];
/**
 * @typedef {string} Action - Define a trigger event
 */
type Action =
  | "pre-commit"
  | "pre-push"
  | "pre-receive"
  | "update"
  | "post-receive";

export {
  defaultImport,
  Config,
  Pipeline,
  Step,
  ExecOptions,
  ExecContext,
  Action,
};
