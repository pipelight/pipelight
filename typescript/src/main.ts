// Load the config file and transform object to JSON
import type { Config } from "./types";
const cwd = Deno.cwd();
const promess = import(`${cwd}/pipelight.config.ts`);

promess
  .then((res) => {
    const data = res.default;
    const config: Config = data;
    const json = JSON.stringify(config, null, 2);
    console.log(json);
  })
  .catch((err) => {
    console.log(err);
  });
