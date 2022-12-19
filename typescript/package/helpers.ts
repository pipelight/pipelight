import {
  defaultImport,
  Config,
  Pipeline,
  Step,
  ExecOptions,
  ExecContext,
  Action,
} from "./types";
const ssh = ({ remote: string, command: string }) => {
  return `ssh ${remote} -C \"${command}\"`;
};

export { ssh };
