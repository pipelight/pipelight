// Load the config file and transform object to JSON
"use strict";
import type {
  defaultImport,
  Config,
  Pipeline,
  Step,
  ExecOptions,
  ExecContext,
  Action
} from "../package/types";

const cwd = process.cwd();

const stock = console;
console = {} as any;
const promess = import(`${cwd}/pipelight.config`);

promess
  .then((res: defaultImport) => {
    console = stock;
    const config: Config = res.default as Config;
    const json = JSON.stringify(config, null, 2);
    console.log(json);
  })
  .catch((err) => {
    console.log(err);
  });
