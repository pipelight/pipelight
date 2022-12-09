// Load the config file and transform object to JSON
import { Config } from "./types";
const cwd = process.cwd();
const promess = import(`${cwd}/pipelight.config`);
promess
  .then((res) => {
    const config: Config = res.default;
    const json = JSON.stringify(res, null, 2);
    console.log(json);
  })
  .catch((err) => {
    console.log("Couldn't load the config file");
    console.log(err);
  });
