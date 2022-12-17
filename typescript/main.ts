// Load the config file and transform object to JSON
import type {
  defaultImport,
  Config,
  Pipeline,
  Step,
  ExecOptions,
  ExecContext,
  Action
} from "./types";

const cwd = process.cwd();
const promess = import(`${cwd}/pipelight.config`);

promess
  .then((res: defaultImport) => {
    const config: Config = res.default as Config;
    const json = JSON.stringify(config, null, 2);
    console.log(json);
  })
  .catch((err) => {
    console.log(err);
  });
