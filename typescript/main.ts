// Load the config file and transform object to JSON
import type { Config } from "./types";
const cwd = process.cwd();
const promess = import(`${cwd}/pipelight.config`);
promess
  .then((res) => {
    const config = res.default as Config;
    const json = JSON.stringify(config, null, 2);
    console.log(json);
  })
  .catch((err) => {
    console.log(err);
  });
