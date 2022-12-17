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
  commands?: string[];
  steps: Step[];
  trigger?: Trigger;
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
  "post-receive"
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

export { Config, Pipeline, Step, ExecOptions, ExecContext, Action };
